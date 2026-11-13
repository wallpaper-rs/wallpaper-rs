use super::{ok, run};
use serde_json::Value;

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
