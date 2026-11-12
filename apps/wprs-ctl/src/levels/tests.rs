use super::hdr;
use super::notes::{measured, note, off, page_notes, sandbox_lines, sandbox_note};
use serde_json::json;
use wprs_core::model::{OutputState, SceneInputs};

fn shown(reason: &str, cost: f64, auto: &str) -> OutputState {
    let state = json!({ "width": 64, "height": 32, "image": null, "fit": "fill", "kind": "scene",
        "playing": true, "active": true, "covered": false, "clients": 0, "error": null, "item": null,
        "preset": null, "playlist": null, "credit": null, "quality": "q0", "quality_reason": reason,
        "quality_cost": cost, "quality_budget": if cost > 0.0 { 4.0 } else { 0.0 }, "quality_auto": auto });
    serde_json::from_value(state).unwrap()
}

#[test]
fn a_chosen_level_on_a_scene_heavy_only_in_model_triangles_still_warns() {
    for reason in ["user", "device", "slow"] {
        let triangles = note(&shown(reason, 0.0, "stop")).unwrap();
        assert!(triangles.starts_with("this scene draws more model triangles than this device holds"));
        assert!(triangles.ends_with("automatic quality would keep it still at its poster"), "{triangles}");
    }
    let stopped = note(&shown("device", 0.0, "still")).unwrap();
    assert!(
        stopped.starts_with("this scene is too heavy for this device and automatic quality keeps it still")
    );
    let over = note(&shown("user", 6.0, "stop")).unwrap();
    assert!(over.starts_with("this scene costs 6.0 units a frame at q0, more than the 4"), "{over}");
    assert!(over.ends_with("automatic quality would keep it still at its poster"), "{over}");
    assert_eq!(note(&shown("user", 0.0, "")), None);
}

#[test]
fn a_level_measured_too_slow_tells_its_frame_time_and_rate_instead_of_the_estimate() {
    let mut o = shown("user", 6.0, "lower");
    o.quality_timing = serde_json::from_value(json!({ "quality_frame_ms": 25.0, "quality_period_ms": 16.7,
        "quality_step": "steady", "quality_pace": 1 }))
    .unwrap();
    let slow = note(&o).unwrap();
    assert!(
        slow.starts_with("its frames take 25.0 ms on the graphics card at q0, more than the 16.7 ms"),
        "{slow}"
    );
    assert!(slow.contains("at 60 frames a second") && slow.ends_with("would lower it"), "{slow}");
    assert_eq!(measured(&o), "25.0 of 16.7 ms, steady");
    o.quality_timing.quality_pace = 2;
    assert_eq!(measured(&o), "25.0 of 16.7 ms, half pace, steady");
    o.quality_timing.quality_frame_ms = None;
    assert!(
        note(&o).unwrap().starts_with("this scene costs 6.0 units"),
        "no measure falls back to the estimate"
    );
    assert_eq!(measured(&o), "not yet, half pace, steady");
    assert_eq!(note(&shown("user", 6.0, "")), None, "an estimate that measured frames disprove still warned");
    assert_eq!(measured(&shown("user", 0.0, "")), "");
}

#[test]
fn an_input_a_scene_uses_but_is_not_granted_shows_as_off() {
    let used: SceneInputs = serde_json::from_value(json!({ "scene": "org.example.sea", "audio": true,
        "pointer": true, "network": true, "granted": ["pointer"] }))
    .unwrap();
    assert_eq!(off(&used), ["audio", "network", "audio_network"]);
    let none: SceneInputs = serde_json::from_value(json!({ "scene": "org.example.sea", "audio": false,
        "pointer": true, "granted": ["pointer"] }))
    .unwrap();
    assert!(off(&none).is_empty());
}

#[test]
fn a_web_page_whose_sandbox_could_not_fully_start_warns_about_it() {
    assert_eq!(sandbox_note(&shown("user", 0.0, "")), None);
    let mut reduced = shown("user", 0.0, "");
    reduced.sandbox = "reduced".into();
    assert_eq!(
        sandbox_note(&reduced),
        Some("its web engine runs without part of its sandbox on this computer".to_owned())
    );
}

#[test]
fn the_reduced_sandbox_line_names_only_the_screens_asked_about() {
    let mut reduced = shown("user", 0.0, "");
    reduced.sandbox = "reduced".into();
    let mut state = std::collections::BTreeMap::new();
    state.insert("DP-1".into(), reduced);
    state.insert("DP-2".into(), shown("user", 0.0, ""));
    assert_eq!(sandbox_lines(&state, |_| true).len(), 1);
    assert!(sandbox_lines(&state, |n| n == "DP-2").is_empty());
    assert!(sandbox_lines(&state, |n| n == "DP-1")[0].starts_with("DP-1: its web engine"));
}

#[test]
fn the_hdr_line_says_how_the_colors_reach_the_desktop() {
    let mut o = shown("user", 0.0, "");
    o.hdr = serde_json::from_value(json!({ "choice": "on", "own": null, "active": true, "offered": true,
        "display": true, "peak_nits": 800.0, "reference_nits": 203.0, "reason": "on", "encoding": "pq-own",
        "max_cll": 800, "max_fall": 203 }))
    .unwrap();
    assert_eq!(hdr(&o), "on on (on, 800 nits, PQ with its own description, MaxCLL 800)");
    o.hdr.encoding = "scrgb".into();
    assert_eq!(hdr(&o), "on on (on, 800 nits, scRGB)");
    o.hdr.active = false;
    assert_eq!(hdr(&o), "on off (on, 800 nits)");
}

#[test]
fn a_page_that_threw_and_a_picture_held_by_software_say_so_in_the_state_notes() {
    let mut o = shown("user", 0.0, "");
    assert!(page_notes(&o).is_empty());
    o.page_errors = vec!["RangeError: Incorrect locale information provided".into()];
    o.still = Some(wprs_core::travel::Still::Software);
    let said = page_notes(&o);
    assert!(said[0].starts_with("the page reported an error and may not be drawing: RangeError"), "{said:?}");
    assert!(said[1].contains("playback.software_parallax true"), "{said:?}");
}
