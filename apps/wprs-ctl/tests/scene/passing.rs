use super::{fails, ok};
use serde_json::{Value, json};
use std::path::Path;

fn show(dir: &Path, pointer: &str) -> Value {
    serde_json::from_str(&ok(dir, &["scene", "show", "s", pointer])).unwrap()
}

#[test]
fn a_transition_is_made_changed_asked_and_removed_from_the_command_line() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();
    ok(dir, &["scene", "new", "s", "--from", "empty"]);
    ok(dir, &["scene", "act", "s", "transition", r#"{"way": "in", "make": true}"#]);
    let change =
        r#"{"way": "in", "duration_ms": 1800, "curve": [0.2, 0.8, 0.4, 1], "params": {"edge": 0.4}}"#;
    ok(dir, &["scene", "act", "s", "transition", change]);
    let made = show(dir, "/scene/transition/in");
    assert_eq!(made["descriptor"], "transitions/opening.json");
    assert_eq!((made["duration_ms"].clone(), made["params"]["edge"].clone()), (json!(1800), json!(0.4)));
    assert!(dir.join("s/transitions/opening.wgsl").exists());
    let asked: Value = serde_json::from_str(&ok(dir, &["scene", "ask", "s", "transition"])).unwrap();
    assert_eq!(asked["in"]["plays"], json!([true, true, true, true]));
    assert_eq!(asked["out"], Value::Null);
    let why = fails(dir, &["scene", "act", "s", "transition", r#"{"way": "in", "duration_ms": 50}"#]);
    assert!(why.contains("0.1"), "{why}");
    assert_eq!(show(dir, "/scene/transition/in/duration_ms"), json!(1800));
    ok(dir, &["scene", "act", "s", "transition", r#"{"way": "in", "remove": true}"#]);
    let scene = show(dir, "/scene");
    assert!(scene.get("transition").is_none(), "{scene}");
}
