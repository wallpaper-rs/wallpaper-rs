use crate::cli::JSON;
use crate::show::{self, list};
use crate::told::{fill, told, tr};
use crate::usage;
use anyhow::{Result, bail};
use clap::{Subcommand, ValueEnum};
use std::collections::BTreeSet;
use wprs_client::{Client, error_name, parse};
use wprs_core::dbus::error::UNCONFIRMED;
use wprs_core::model::{Input, InputsReply};

mod asking;
mod position;
mod shown;

pub use asking::{asking, asking_for};
use position::at_fraction;
use shown::status;

#[derive(Subcommand, Debug)]
pub enum Inputs {
    #[command(
        about = "Let the scene package shown now use an input whenever it is shown; another package with the same id does not get it"
    )]
    Grant {
        #[arg(help = "Package id of the scene")]
        scene: String,
        #[arg(help = "Input to let the scene read")]
        input: Kind,
        #[arg(
            short,
            long,
            help = "The screen whose package gets the input, when several show one with this id"
        )]
        output: Option<String>,
    },
    #[command(
        about = "Take an input back from a scene; it stops at once and clients stop asking until it is granted"
    )]
    Revoke {
        #[arg(help = "Package id of the scene")]
        scene: String,
        #[arg(help = "Input to take back from the scene")]
        input: Kind,
    },
    #[command(
        about = "Show what each screen's scene may use and whether audio is captured and the pointer followed"
    )]
    Status {
        #[arg(long, help = JSON)]
        json: bool,
    },
    #[command(
        about = "Let clicks and taps on a screen reach its wallpaper, for packages allowed the clicks input; off by default"
    )]
    Clicks {
        #[arg(help = "Pass clicks on the desktop to scenes or not")]
        state: Switch,
        #[arg(short, long, help = "The screen; every screen when left out")]
        output: Option<String>,
    },
    #[command(about = "Report the pointer position in logical desktop coordinates")]
    Pointer {
        #[arg(allow_hyphen_values = true, help = "Horizontal position in logical pixels")]
        x: i32,
        #[arg(allow_hyphen_values = true, help = "Vertical position in logical pixels")]
        y: i32,
    },
    #[command(
        about = "Show a made-up pointer on the wallpaper of a screen instead of the real one, to try input.cursor, click:, enter: and leave: without a mouse there; --stop ends it"
    )]
    Touch {
        #[arg(help = "The screen")]
        output: String,
        #[arg(
            required_unless_present = "stop",
            help = "Position as X,Y in fractions of the screen, 0..=1 for each"
        )]
        at: Option<String>,
        #[arg(long, default_value_t = 0, help = "Bitmask of held buttons: 1 left, 2 right, 4 middle")]
        buttons: u32,
        #[arg(long, conflicts_with = "at", help = "Stop the simulated pointer")]
        stop: bool,
    },
    #[command(
        about = "Fire a trigger on the scene of a screen, such as event:boom, click:button or media:play, starting the timelines and sounds it starts"
    )]
    Trigger {
        #[arg(help = "The screen, a preview stream included")]
        output: String,
        #[arg(help = "The trigger, as a timeline or a sound layer names it")]
        trigger: String,
    },
    #[command(
        about = "Print the levels of the 64 bands of the sound a scene on a screen follows, as the editor meter reads them; nothing is recorded or kept, and it needs the audio input of the scene. listening says whether the daemon has the sound capture open for it, sound whether sound is arriving now"
    )]
    Bands {
        #[arg(help = "The screen, such as a preview output named preview:1")]
        output: String,
    },
}

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum Kind {
    Audio,
    Pointer,
    Clicks,
    Network,
    Rgb,
    Media,
    #[value(name = "media_network")]
    MediaNetwork,
    #[value(name = "audio_network")]
    AudioNetwork,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum Switch {
    On,
    Off,
}

impl Kind {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Kind::Audio => Input::Audio.as_str(),
            Kind::Pointer => Input::Pointer.as_str(),
            Kind::Clicks => Input::Clicks.as_str(),
            Kind::Network => Input::Network.as_str(),
            Kind::Rgb => Input::Rgb.as_str(),
            Kind::Media => Input::Media.as_str(),
            Kind::MediaNetwork => Input::MediaNetwork.as_str(),
            Kind::AudioNetwork => Input::AudioNetwork.as_str(),
        }
    }
}

pub async fn run(client: &Client, command: Inputs) -> Result<()> {
    let inputs = &client.inputs;
    match command {
        Inputs::Grant { scene, input, output } => {
            grant(client, &scene, input.name(), output.as_deref()).await?
        }
        Inputs::Revoke { scene, input } => inputs.revoke(&scene, input.name()).await?,
        Inputs::Status { json } => show::reply(inputs.get_inputs().await?, json, status)?,
        Inputs::Pointer { x, y } => inputs.set_pointer(x, y).await?,
        Inputs::Clicks { state, output } => {
            inputs.set_clicks(output.as_deref().unwrap_or_default(), matches!(state, Switch::On)).await?
        }
        Inputs::Bands { output } => println!("{}", inputs.bands(&output).await?),
        Inputs::Touch { output, at: None, .. } => inputs.simulate_touch(&output, -1.0, -1.0, 0).await?,
        Inputs::Touch { output, at: Some(at), buttons, .. } => {
            let (x, y) = at_fraction(&at)?;
            inputs.simulate_touch(&output, x, y, buttons).await?
        }
        Inputs::Trigger { output, trigger } => inputs.simulate_trigger(&output, &trigger).await?,
    }
    Ok(())
}

#[cfg(test)]
mod tests;

async fn grant(client: &Client, scene: &str, input: &str, output: Option<&str>) -> Result<()> {
    let reply: InputsReply = parse(&client.inputs.get_inputs().await?)?;
    let shown = reply.shown.iter().filter(|(name, s)| s.scene == scene && output.is_none_or(|o| o == *name));
    let keys: BTreeSet<&str> = shown.map(|(_, s)| s.key.as_str()).filter(|k| !k.is_empty()).collect();
    let sent = match (Vec::from_iter(keys).as_slice(), output) {
        ([key], _) => client.inputs.grant_package(scene, key, input).await,
        ([], Some(output)) => bail!("{}", fill(told("%1 does not show %2"), &[&output, &scene])),
        ([], None) => client.inputs.grant(scene, input).await,
        _ => {
            let outputs = reply.shown.iter().filter(|(_, s)| s.scene == scene).map(|(name, _)| name);
            let many = fill(
                told("more than one package with the id %1 is shown; choose one with --output"),
                &[&scene],
            );
            return Err(usage(format!("{many} ({})", list(outputs))));
        }
    };
    match sent.map_err(wprs_client::explain) {
        Err(e) if error_name(&e) == Some(UNCONFIRMED) => {
            eprintln!(
                "{}",
                tr(told(
                    "Asked in a desktop notification: the scene gets the input once you allow it there."
                ))
            );
            Ok(())
        }
        sent => {
            sent?;
            println!("{}", fill(told("%1 may use %2"), &[&scene, &input]));
            Ok(())
        }
    }
}
