use crate::told::{fill, told, tr};
use wprs_core::model::{Input, InputsReply, OutputState, State};

const SHOWN_ERRORS: usize = 5;

pub fn page_notes(o: &OutputState) -> Vec<String> {
    let first = o.page_errors.len().saturating_sub(SHOWN_ERRORS);
    let line = |l: &String| fill(told("the page reported an error and may not be drawing: %1"), &[l]);
    let errors = o.page_errors[first..].iter().map(line);
    let still = o.still.map(|s| wprs_depth::still_note(&wprs_depth::Places::default(), s).local());
    errors.chain(still).collect()
}

pub fn note(o: &OutputState) -> Option<String> {
    let over = o.quality_budget > 0.0 && o.quality_cost > o.quality_budget;
    let would = |auto: &str| {
        if auto == "stop" { tr(told("keep it still at its poster")) } else { tr(told("lower it")) }
    };
    let cost = |auto: &str| {
        fill(
            told(
                "this scene costs %1 units a frame at %2, more than the %3 this device is estimated to hold: frames may drop and battery use rise; automatic quality would %4",
            ),
            &[
                &format!("{:.1}", o.quality_cost),
                &o.quality,
                &format!("{:.0}", o.quality_budget),
                &would(auto),
            ],
        )
    };
    let note = match (o.quality_auto.as_str(), o.quality_timing.quality_frame_ms) {
        ("still", _) => tr(STOPPED),
        ("lower", Some(ms)) => fill(
            told(
                "its frames take %1 ms on the graphics card at %2, more than the %3 ms a frame has at %4 frames a second: frames drop and battery use rises; automatic quality would lower it",
            ),
            &[
                &format!("{:.1}", ms),
                &o.quality,
                &format!("{:.1}", o.quality_timing.quality_period_ms.unwrap_or_default()),
                &o.quality_timing
                    .quality_period_ms
                    .filter(|p| *p > 0.0)
                    .map_or(0, |p| (1000.0 / p).round() as u32),
            ],
        ),
        ("stop", _) if !over => TRIANGLES.to_owned(),
        (auto @ ("stop" | "lower"), _) => cost(auto),
        _ => return None,
    };
    let left = o.left_out.len();
    Some(match left {
        0 => note,
        _ => fill(
            told("%1; %2 %3 out at every level to protect the graphics card"),
            &[&note, &left, &WORDS[usize::from(left > 1)]],
        ),
    })
}

pub fn measured(o: &OutputState) -> String {
    let t = &o.quality_timing;
    let half = if t.quality_pace > 1 { tr(told(", half pace")) } else { String::new() };
    match (t.quality_frame_ms, t.quality_period_ms) {
        _ if t.quality_step.is_empty() => String::new(),
        (Some(ms), Some(period)) => format!("{ms:.1} of {period:.1} ms{half}, {}", t.quality_step),
        _ => fill(told("not yet%1, %2"), &[&half, &t.quality_step]),
    }
}

pub fn sandbox_note(o: &OutputState) -> Option<String> {
    (o.sandbox == "reduced")
        .then(|| tr(told("its web engine runs without part of its sandbox on this computer")))
}
const STOPPED: &str = told(
    "this scene is too heavy for this device and automatic quality keeps it still at its poster; a fixed level such as wprs-ctl quality q0 plays it anyway, with fewer frames per second and more battery use",
);
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
        let more = sandbox_note(o).into_iter().chain(o.error.clone());
        let more = more.chain(o.away.as_deref().map(crate::away::note)).chain(page_notes(o));
        more.for_each(|n| eprintln!("{name}: {n}"));
    });
    let Ok(inputs) = serde_json::from_str::<InputsReply>(inputs) else {
        return;
    };
    inputs.shown.iter().for_each(|(name, used)| {
        let off = off(used);
        if !off.is_empty() {
            eprintln!(
                "{}",
                fill(
                    told("%1: %2 wants %3 but it is not granted here, so that part of the scene is off"),
                    &[&name, &used.scene, &off.join(", ")]
                )
            );
        }
    });
}
