use crate::told::{fill, told, tr};
use serde_json::Value;
use wprs_core::TransitionKind;

pub fn unset(list: &str) -> &'static str {
    if list == "transitions" { TransitionKind::GENERAL } else { "none" }
}

pub fn lines(state: &Value, only: Option<&str>) {
    let Some(outputs) = state["outputs"].as_object() else { return };
    for (name, o) in outputs.iter().filter(|(name, _)| only.is_none_or(|n| n == name.as_str())) {
        let t = &o["transition"];
        let text = |key: &str| t[key].as_str().unwrap_or_default().to_owned();
        let line =
            fill(told("%1: %2; the last change: %3"), &[name, &choice(&text("choice")), &why(&text("why"))]);
        println!("{line}");
    }
}

fn choice(choice: &str) -> String {
    match choice {
        "own" => tr(told("the transition of each wallpaper")),
        "fade" => tr(told("the plain fade")),
        "none" => tr(told("a cut")),
        other => other.to_owned(),
    }
}

fn why(why: &str) -> String {
    match why {
        "own" => tr(told("the wallpaper's own transition played")),
        "chosen" => tr(told("what you chose played")),
        "reduced" => tr(told("the plain fade played, because of less motion")),
        "fallen" => tr(told("the plain fade played, because the wallpaper's transition could not play")),
        "plain" => tr(told("the plain fade played: neither wallpaper brings a transition")),
        "cut" => tr(told("a cut: the screen was not ready for a transition")),
        other => other.to_owned(),
    }
}
