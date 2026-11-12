use super::{act, at, scene};
use serde_json::{Value, json};

#[test]
fn the_canvas_takes_a_screen_size_a_still_frame_from_the_playhead_and_a_fresh_seed() {
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(&root, "clock");
    let phone = wprs_editor::actions::SCREENS.iter().find(|(n, _)| n.starts_with("Phone")).unwrap().1;
    act(&mut p, json!({"canvas": {"size": {"size": phone}}})).unwrap();
    assert_eq!(at(&p, "/scene/canvas/size"), json!(phone));
    assert!(act(&mut p, json!({"canvas": {"size": {"size": [0, 100]}}})).is_err());
    act(&mut p, json!({"canvas": {"clock": {"poster": 2.3456}}})).unwrap();
    assert_eq!(at(&p, "/scene/clock/poster_time"), json!(2.346));
    act(&mut p, json!({"canvas": {"clock": {"fresh": true}}})).unwrap();
    assert_eq!(at(&p, "/scene/clock/seed"), Value::Null);
    assert!(p.doc.pointer("/scene/clock/seed").is_some());
    act(&mut p, json!({"canvas": {"clock": {"seed": 7}}})).unwrap();
    assert_eq!(at(&p, "/scene/clock/seed"), json!(7));
    assert!(act(&mut p, json!({"canvas": {"clock": {"poster": -1}}})).is_err());
    assert!(wprs_editor::check::check(&p.doc).is_empty());
}
