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

#[test]
fn declared_inputs_are_checked_and_never_drop_the_network_one_alone() {
    let root = tempfile::tempdir().unwrap();
    let mut project = page(root.path());
    manifest(&mut project, json!({"inputs": {"inputs": ["audio", "pointer", "clicks"]}})).unwrap();
    assert_eq!(project.doc["manifest"]["inputs"], json!(["audio", "pointer", "clicks"]));
    manifest(&mut project, json!({"inputs": {"inputs": ["pointer"], "remove": true}})).unwrap();
    assert_eq!(project.doc["manifest"]["inputs"], json!(["audio", "clicks"]));
    assert!(manifest(&mut project, json!({"inputs": {"inputs": ["lasers"], "add": true}})).is_err());
    assert!(manifest(&mut project, json!({"inputs": {"inputs": ["network"], "add": true}})).is_err());
}

#[test]
fn a_page_sets_its_playback() {
    let root = tempfile::tempdir().unwrap();
    let mut project = page(root.path());
    let values = json!({"playback": {"values": {"max_fps": 30, "volume": 0.4, "muted": true}}});
    manifest(&mut project, values).unwrap();
    let playback = json!({"max_fps": 30, "volume": 0.4, "muted": true});
    assert_eq!(project.doc["manifest"]["playback"], playback);
    manifest(&mut project, json!({"playback": {"values": {"max_fps": "display"}}})).unwrap();
    assert_eq!(project.doc["manifest"]["playback"]["max_fps"], "display");
    for refused in [
        json!({"playback": {"values": {"speed": 2.0}}}),
        json!({"playback": {"values": {"max_fps": 500}}}),
        json!({"playback": {"values": {"volume": 3.0}}}),
    ] {
        assert!(manifest(&mut project, refused.clone()).is_err(), "{refused} was accepted");
    }
    manifest(&mut project, json!({"playback": {"clear": ["max_fps", "volume", "muted"]}})).unwrap();
    assert!(project.doc["manifest"].get("playback").is_none());
}

#[test]
fn the_preview_of_a_page_carries_its_manifest_quality_rate_and_reload() {
    let root = tempfile::tempdir().unwrap();
    let project = page(root.path());
    assert!(wprs_editor::pages::paged(&project.doc));
    let look = json!({"quality": "q1", "fps": 30, "restart": 2, "media": {"title": "Song"}});
    let document = wprs_editor::pages::preview_document(&project.doc, &look);
    assert_eq!(document["manifest"], project.doc["manifest"]);
    assert_eq!(
        document["preview"],
        json!({"quality": "q1", "restart": 2, "fps": 30, "media": {"title": "Song"}})
    );
    assert!(document.get("scene").is_none());
    let plain = wprs_editor::pages::preview_document(&project.doc, &json!({}));
    assert!(plain.get("preview").is_none());
}

#[test]
fn a_page_takes_properties_and_a_folder_property_like_a_scene() {
    let root = tempfile::tempdir().unwrap();
    let mut project = page(root.path());
    act(&mut project, json!({"add_property": {"kind": "slider", "id": "speed"}})).unwrap();
    act(&mut project, json!({"add_property": {"kind": "folder", "id": "photos"}})).unwrap();
    let properties = &project.doc["manifest"]["properties"];
    assert_eq!(properties["speed"]["type"], "slider");
    assert_eq!(properties["photos"]["type"], "folder");
    let rows = wprs_editor::listing::rows(&project.doc);
    let ids: Vec<_> = rows.iter().map(|r| r["id"].as_str().unwrap().to_owned()).collect();
    assert!(ids.contains(&"speed".to_owned()) && ids.contains(&"photos".to_owned()), "{ids:?}");
    act(&mut project, json!({"set": {"pointer": "/manifest/properties/photos/mode", "value": "all"}}))
        .unwrap();
    assert_eq!(project.doc["manifest"]["properties"]["photos"]["mode"], "all");
    let problems = wprs_editor::package::problems(&project.dir, &project.doc);
    assert!(problems.iter().all(|p| p.warning), "{problems:?}");
    act(&mut project, json!({"remove": {"pointer": "/manifest/properties/photos"}})).unwrap();
    assert!(project.doc["manifest"]["properties"].get("photos").is_none());
}
