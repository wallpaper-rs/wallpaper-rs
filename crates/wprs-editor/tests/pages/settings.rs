use super::{manifest, page};
use serde_json::json;

#[test]
fn a_page_sets_its_preload_background_and_storage() {
    let root = tempfile::tempdir().unwrap();
    let mut project = page(root.path());
    std::fs::write(project.dir.join("early.js"), "1").unwrap();
    let web =
        json!({"web": {"values": {"preload": "early.js", "background": "#102030", "storage_kib": 256}}});
    manifest(&mut project, web).unwrap();
    assert_eq!(
        project.doc["manifest"]["web"],
        json!({"preload": "early.js", "background": "#102030", "storage_kib": 256})
    );
    let listed = wprs_editor::pages::files(&project.dir, &project.doc);
    assert!(listed.iter().any(|f| f["path"] == "early.js" && f["role"] == "preload"));
    manifest(&mut project, json!({"web": {"clear": ["preload", "storage_kib"]}})).unwrap();
    assert_eq!(project.doc["manifest"]["web"], json!({"background": "#102030"}));
    for refused in [
        json!({"web": {"values": {"background": "red"}}}),
        json!({"web": {"values": {"storage_kib": 99999}}}),
        json!({"web": {"values": {"volume": 0.5}}}),
    ] {
        assert!(manifest(&mut project, refused.clone()).is_err(), "{refused} was accepted");
    }
    assert_eq!(project.doc["manifest"]["web"], json!({"background": "#102030"}));
}
