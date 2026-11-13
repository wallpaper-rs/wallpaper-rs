use crate::{ok, run};
use serde_json::{Value, json};

#[test]
fn removing_a_layer_names_the_followers_that_could_not_be_kept() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();
    ok(dir, &["scene", "new", "s", "--from", "empty"]);
    let layers = json!([
        {"id": "base", "type": "solid", "color": "#ffffff", "size": [100, 100], "solid": true},
        {"id": "kid", "type": "solid", "color": "#ff0000", "size": [40, 40], "parent": "base",
            "transform": {"position": {"script": "wobble"}}}
    ]);
    let set = json!({"pointer": "/scene/layers", "value": layers}).to_string();
    ok(dir, &["scene", "act", "s", "set", &set]);
    let asked = ok(dir, &["scene", "ask", "s", "unkept", r#"{"id":"base"}"#]);
    assert!(asked.contains("kid"), "{asked}");
    let out = run(dir, &["scene", "act", "s", "remove_layer", r#"{"id":"base"}"#]);
    assert!(out.status.success());
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(err.contains("warning: kid followed the removed layer"), "{err}");
    let quiet = run(dir, &["scene", "act", "s", "remove_layer", r#"{"id":"kid"}"#]);
    assert!(!String::from_utf8_lossy(&quiet.stderr).contains("followed"));
}

#[test]
fn queries_show_their_arguments_and_refuse_others() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();
    ok(dir, &["scene", "new", "s", "--from", "empty"]);
    let text = ok(dir, &["scene", "actions", "--queries", "pose"]);
    assert!(
        text.contains("  clip: string (required)") && text.contains("  rotation: number (optional)"),
        "{text}"
    );
    let schema: Value =
        serde_json::from_str(&ok(dir, &["scene", "actions", "--queries", "pose", "--json"])).unwrap();
    assert_eq!(schema["schema"]["required"][0], "clip");
    let out = run(dir, &["scene", "ask", "s", "wind", r#"{"speed": 1}"#]);
    assert_eq!(out.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&out.stderr).contains("speed is not an argument of the query wind"));
    assert!(run(dir, &["scene", "ask", "s", "wind", "{}"]).status.success());
}
