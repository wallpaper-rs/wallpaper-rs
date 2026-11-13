use super::{ok, run};
use serde_json::json;

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
