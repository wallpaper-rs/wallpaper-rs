use super::shown::{AUDIO, AUDIO_NETWORK, CLICKS, MEDIA, MEDIA_NETWORK, NETWORK, POINTER, RGB, SLOW};
use crate::show;
use crate::told::{fill, told, tr};
use anyhow::Result;
use std::collections::BTreeSet;
use std::time::{Duration, Instant};
use wprs_client::{Client, parse};
use wprs_core::model::{Input, InputsReply, OutputState, SceneInputs, State};

const SETTLE: Duration = Duration::from_secs(1);

pub async fn asking(
    client: &Client,
    output: &str,
    action: impl Future<Output = zbus::Result<()>>,
) -> Result<()> {
    asking_for(client, output, None, action).await
}

pub async fn asking_for(
    client: &Client,
    output: &str,
    only: Option<&str>,
    action: impl Future<Output = zbus::Result<()>>,
) -> Result<()> {
    let since = client.daemon.serial().await?;
    action.await.map_err(wprs_client::explain)?;
    if let Ok(asks) = asks(client, output, only, since).await {
        asks.iter().for_each(|ask| eprintln!("{}", show::clean(ask)));
    }
    Ok(())
}

async fn asks(
    client: &Client,
    output: &str,
    only: Option<&str>,
    mut serial: u64,
) -> Result<BTreeSet<String>> {
    let ours = |name: &String| output.is_empty() || name == output;
    let state: State = parse(&client.daemon.get_state().await?)?;
    let scene =
        |o: &OutputState| o.image.as_ref().is_some_and(|p| p.extension().is_some_and(|e| e == "wpak"));
    if !state.outputs.iter().any(|(name, o)| ours(name) && scene(o)) {
        return Ok(BTreeSet::new());
    }
    let deadline = Instant::now() + SETTLE;
    while let Some(left) = deadline.checked_duration_since(Instant::now()) {
        let (now, topics) = client.daemon.wait_changed(serial, left.as_millis() as u32).await?;
        if now == serial || topics.iter().any(|t| t == "inputs") {
            break;
        }
        serial = now;
    }
    let reply: InputsReply = parse(&client.inputs.get_inputs().await?)?;
    let mut after: State = parse(&client.daemon.get_state().await?)?;
    let unseen = |s: &State| {
        s.outputs.iter().any(|(n, o)| ours(n) && scene(o) && o.frames == 0 && o.particles.is_empty())
    };
    let deadline = Instant::now() + SETTLE;
    while let Some(left) = deadline.checked_duration_since(Instant::now()).filter(|_| unseen(&after)) {
        client.daemon.wait_changed(after.serial, left.as_millis().min(100) as u32).await?;
        after = parse(&client.daemon.get_state().await?)?;
    }
    let slow = after.outputs.iter().filter(|(name, o)| ours(name) && o.particles == "cpu");
    let slow = slow.map(|(name, _)| format!("{name}: {}", tr(SLOW)));
    let sandbox = crate::levels::sandbox_lines(&after.outputs, &ours);
    let asks = requests(&reply, &ours, only);
    Ok(asks.into_iter().chain(slow).chain(sandbox).collect())
}

pub(super) fn requests(
    reply: &InputsReply,
    ours: &dyn Fn(&String) -> bool,
    only: Option<&str>,
) -> Vec<String> {
    let declined =
        |scene: &String, input: Input| reply.declined.get(scene).is_some_and(|set| set.contains(&input));
    let twins = |s: &SceneInputs| reply.shown.values().any(|o| o.scene == s.scene && o.key != s.key);
    let shown = reply.shown.iter().filter(|(name, s)| ours(name) && only.is_none_or(|id| s.scene == id));
    let asks = shown.flat_map(|(name, s)| {
        let title = if s.title.is_empty() { &s.scene } else { &s.title };
        let used = [
            (s.audio, Input::Audio, tr(AUDIO)),
            (s.pointer, Input::Pointer, tr(POINTER)),
            (s.clicks, Input::Clicks, tr(CLICKS)),
            (s.network, Input::Network, tr(NETWORK)),
            (s.rgb, Input::Rgb, tr(RGB)),
            (s.media, Input::Media, tr(MEDIA)),
            (s.media && s.network, Input::MediaNetwork, tr(MEDIA_NETWORK)),
            (s.audio && s.network, Input::AudioNetwork, tr(AUDIO_NETWORK)),
        ];
        let decided = move |input: Input| s.granted.contains(&input) || declined(&s.scene, input);
        let open = used.into_iter().filter(move |(on, input, _)| *on && !decided(*input));
        let output = if twins(s) { format!(" -o {name}") } else { String::new() };
        let screen = !reply.clicks.contains(name.as_str());
        open.map(move |(_, input, what)| {
            let mut grant = format!("wprs-ctl input grant {} {}{output}", s.scene, input.as_str());
            if input == Input::Clicks && screen {
                grant += &fill(told(" && wprs-ctl input clicks on -o %1"), &[&name]);
            }
            fill(told("%1 follows %2 once you allow it: %3"), &[&title, &what, &grant])
        })
    });
    asks.collect()
}
