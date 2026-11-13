use crate::told::{told, tr};
use wprs_core::model::{OutputState, StillReason};

pub fn note(o: &OutputState) -> Option<String> {
    let text = match o.still_reason? {
        StillReason::Covered => told("it is still because a window covers the screen"),
        StillReason::Battery => told("it is still because the computer is on battery"),
        StillReason::Locked => told("it is still because the screen is locked"),
        StillReason::Paused => told("it is still because it was paused (wprs-ctl resume)"),
    };
    Some(tr(text))
}

#[cfg(test)]
mod tests {
    use super::note;
    use wprs_core::model::{OutputState, StillReason};

    #[test]
    fn each_reason_has_its_words_and_a_playing_screen_has_none() {
        let mut o: OutputState = serde_json::from_value(serde_json::json!({
            "width": 1, "height": 1, "image": null, "fit": "fill", "kind": "image", "playing": true,
            "active": true, "covered": false, "clients": 0, "error": null, "item": null, "preset": null,
            "playlist": null, "credit": null
        }))
        .unwrap();
        assert_eq!(note(&o), None);
        let words: Vec<_> =
            [StillReason::Covered, StillReason::Battery, StillReason::Locked, StillReason::Paused]
                .into_iter()
                .map(|r| {
                    o.still_reason = Some(r);
                    note(&o).unwrap()
                })
                .collect();
        assert!(words[0].contains("window") && words[1].contains("battery"));
        assert!(words[2].contains("locked") && words[3].contains("paused"));
    }
}
