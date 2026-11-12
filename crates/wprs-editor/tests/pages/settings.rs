use super::{act, manifest, page};
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

#[test]
fn network_origins_keep_the_network_input_in_step() {
    let root = tempfile::tempdir().unwrap();
    let mut project = page(root.path());
    let origins = json!({"network": {"origins": ["https://API.example.org/", "https://api.example.org"]}});
    manifest(&mut project, origins).unwrap();
    let m = &project.doc["manifest"];
    assert_eq!(m["permissions"]["network"]["allow"], json!(["https://api.example.org"]));
    assert_eq!(m["inputs"], json!(["network"]));
    manifest(&mut project, json!({"network": {"origins": ["https://b.example.org"], "add": true}})).unwrap();
    assert_eq!(project.doc["manifest"]["permissions"]["network"]["allow"].as_array().unwrap().len(), 2);
    assert!(manifest(&mut project, json!({"network": {"origins": ["http://plain.example.org"]}})).is_err());
    assert!(manifest(&mut project, json!({"network": {"origins": ["not a url"], "add": true}})).is_err());
    assert!(manifest(&mut project, json!({"inputs": {"inputs": ["audio"]}})).is_err());
    manifest(&mut project, json!({"network": {"origins": ["https://api.example.org"], "remove": true}}))
        .unwrap();
    manifest(&mut project, json!({"network": {"origins": ["https://b.example.org"], "remove": true}}))
        .unwrap();
    let m = &project.doc["manifest"];
    assert!(m.get("permissions").is_none() && m["inputs"].as_array().is_none_or(Vec::is_empty), "{m}");
    act(&mut project, json!({"undo": {}})).unwrap();
    assert_eq!(project.doc["manifest"]["inputs"], json!(["network"]));
}
