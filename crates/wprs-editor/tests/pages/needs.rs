use super::{act, manifest, page};
use serde_json::json;

#[test]
fn required_packages_take_a_range_or_the_range_of_a_version() {
    let root = tempfile::tempdir().unwrap();
    let mut project = page(root.path());
    let packages = json!({"requires": {"packages": [{"id": "org.example.fx", "version": "1.2.7"}]}});
    manifest(&mut project, packages).unwrap();
    let m = &project.doc["manifest"];
    assert_eq!(m["requires"]["packages"], json!([{"id": "org.example.fx", "version": "^1.2"}]));
    assert!(m["requires"]["features"].as_array().unwrap().contains(&json!("packages")));
    let more = json!({"requires": {"packages": [{"id": "org.example.sky", "version": "0.4.1"},
        {"id": "org.example.fx", "version": ">=1.3, <2"}], "add": true}});
    manifest(&mut project, more).unwrap();
    assert_eq!(
        project.doc["manifest"]["requires"]["packages"],
        json!([{"id": "org.example.fx", "version": ">=1.3, <2"}, {"id": "org.example.sky", "version": "^0.4"}])
    );
    for refused in [
        json!({"requires": {"packages": [{"id": "org.example.page", "version": "1"}], "add": true}}),
        json!({"requires": {"packages": [{"id": "org.example.x", "version": "soon"}], "add": true}}),
        json!({"requires": {"packages": [{"id": "Not An Id", "version": "1.0.0"}], "add": true}}),
    ] {
        assert!(manifest(&mut project, refused.clone()).is_err(), "{refused} was accepted");
    }
    manifest(&mut project, json!({"requires": {"packages": [{"id": "org.example.fx"}], "remove": true}}))
        .unwrap();
    manifest(&mut project, json!({"requires": {"packages": [{"id": "org.example.sky"}], "remove": true}}))
        .unwrap();
    assert!(project.doc["manifest"]["requires"]["packages"].is_null());
    assert!(
        !project.doc["manifest"]["requires"]["features"]
            .as_array()
            .is_some_and(|f| f.contains(&json!("packages")))
    );
    act(&mut project, json!({"undo": {}})).unwrap();
    assert_eq!(project.doc["manifest"]["requires"]["packages"][0]["id"], "org.example.sky");
}
