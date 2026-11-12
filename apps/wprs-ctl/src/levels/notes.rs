use wprs_core::model::{Input, InputsReply, OutputState, State};

pub fn note(o: &OutputState) -> Option<String> {
    let over = o.quality_budget > 0.0 && o.quality_cost > o.quality_budget;
    let would = |auto: &str| if auto == "stop" { "keep it still at its poster" } else { "lower it" };
    let cost = |auto: &str| {
        format!(
            "this scene costs {:.1} units a frame at {}, more than the {:.0} this device is estimated to hold: frames may drop and battery use rise; automatic quality would {}",
            o.quality_cost,
            o.quality,
            o.quality_budget,
            would(auto)
        )
    };
    let note = match (o.quality_auto.as_str(), o.quality_timing.quality_frame_ms) {
        ("still", _) => STOPPED.to_owned(),
        ("lower", Some(ms)) => format!(
            "its frames take {ms:.1} ms on the graphics card at {}, more than the {:.1} ms a frame has at {} frames a second: frames drop and battery use rises; automatic quality would lower it",
            o.quality,
            o.quality_timing.quality_period_ms.unwrap_or_default(),
            o.quality_timing
                .quality_period_ms
                .filter(|p| *p > 0.0)
                .map_or(0, |p| (1000.0 / p).round() as u32)
        ),
        ("stop", _) if !over => TRIANGLES.to_owned(),
        (auto @ ("stop" | "lower"), _) => cost(auto),
        _ => return None,
    };
    let left = o.left_out.len();
    Some(match left {
        0 => note,
        _ => format!(
            "{note}; {left} {} out at every level to protect the graphics card",
            WORDS[usize::from(left > 1)]
        ),
    })
}

pub fn measured(o: &OutputState) -> String {
    let t = &o.quality_timing;
    let half = if t.quality_pace > 1 { ", half pace" } else { "" };
    match (t.quality_frame_ms, t.quality_period_ms) {
        _ if t.quality_step.is_empty() => String::new(),
        (Some(ms), Some(period)) => format!("{ms:.1} of {period:.1} ms{half}, {}", t.quality_step),
        _ => format!("not yet{half}, {}", t.quality_step),
    }
}

pub fn sandbox_note(o: &OutputState) -> Option<&'static str> {
    (o.sandbox == "reduced").then_some("its web engine runs without part of its sandbox on this computer")
}
const STOPPED: &str = "this scene is too heavy for this device and automatic quality keeps it still at its poster; a fixed level such as wprs-ctl quality q0 plays it anyway, with fewer frames per second and more battery use";
const TRIANGLES: &str = "this scene draws more model triangles than this device holds: frames may drop and battery use rise; automatic quality would keep it still at its poster";
const WORDS: [&str; 2] = ["effect stays", "effects stay"];

pub(super) fn off(used: &wprs_core::model::SceneInputs) -> Vec<&'static str> {
    let simple = [
        (used.audio, Input::Audio),
        (used.pointer, Input::Pointer),
        (used.clicks, Input::Clicks),
        (used.network, Input::Network),
        (used.rgb, Input::Rgb),
        (used.media, Input::Media),
    ];
    let combo = [
        (used.media && used.network, Input::MediaNetwork),
        (used.audio && used.network, Input::AudioNetwork),
    ];
    simple
        .into_iter()
        .chain(combo)
        .filter(|(on, i)| *on && !used.granted.contains(i))
        .map(|(_, i)| i.as_str())
        .collect()
}

pub fn notes(state: &str, inputs: &str) {
    let Ok(state) = serde_json::from_str::<State>(state) else {
        return;
    };
    state.outputs.iter().for_each(|(name, o)| {
        note(o).into_iter().for_each(|n| eprintln!("{name}: {n}"));
        let more = sandbox_note(o).map(String::from).into_iter().chain(o.error.clone());
        more.chain(o.away.as_deref().map(crate::away::note)).for_each(|n| eprintln!("{name}: {n}"));
    });
    let Ok(inputs) = serde_json::from_str::<InputsReply>(inputs) else {
        return;
    };
    inputs.shown.iter().for_each(|(name, used)| {
        let off = off(used);
        if !off.is_empty() {
            eprintln!(
                "{name}: {} wants {} but it is not granted here, so that part of the scene is off",
                used.scene,
                off.join(", ")
            );
        }
    });
}
