use super::{act, at, project, refused};
use serde_json::json;
use wprs_editor::rigs::query;

#[test]
fn pools_are_added_changed_listed_drawn_and_removed() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    assert!(refused(&mut p, json!({"add_layer": {"kind": "shapes"}})).contains("add a pool first"));
    act(&mut p, json!({"pool": {"id": "bars", "add": true, "max": 8, "max_vertices": 64}})).unwrap();
    assert_eq!(at(&p, "/scene/pools/bars"), json!({"type": "mesh", "max": 8, "max_vertices": 64}));
    act(&mut p, json!({"pool": {"id": "bars", "size": [128, 64], "max": 12}})).unwrap();
    assert_eq!(at(&p, "/scene/pools/bars/size"), json!([128, 64]));
    act(&mut p, json!({"pool": {"id": "bars", "clear": ["size"]}})).unwrap();
    assert_eq!(at(&p, "/scene/pools/bars").get("size"), None);
    for bad in [
        json!({"pool": {"id": "bars", "add": true}}),
        json!({"pool": {"id": "Big One", "add": true}}),
        json!({"pool": {"id": "nowhere", "max": 2}}),
        json!({"pool": {"id": "bars", "max": 9000}}),
        json!({"pool": {"id": "bars", "max_vertices": 2}}),
        json!({"pool": {"id": "bars", "texture": "missing"}}),
        json!({"pool": {"id": "bars", "add": true, "remove": true}}),
        json!({"pool": {"id": "nowhere", "remove": true}}),
    ] {
        refused(&mut p, bad);
    }
    act(&mut p, json!({"add_layer": {"kind": "shapes", "id": "drawn"}})).unwrap();
    assert_eq!(at(&p, "/scene/layers/1"), json!({"id": "drawn", "type": "video", "video": "bars"}));
    let rows = query(&root.path().join("glow"), &p.doc, "", &json!({"pools": {}}));
    assert_eq!(rows["pools"][0]["layers"], json!(["drawn"]));
    assert_eq!(rows["pools"][0]["max"], 12);
    assert!(refused(&mut p, json!({"pool": {"id": "bars", "remove": true}})).contains("drawn by drawn"));
    act(&mut p, json!({"remove_layer": {"id": "drawn"}})).unwrap();
    act(&mut p, json!({"pool": {"id": "bars", "remove": true}})).unwrap();
    assert_eq!(at(&p, "/scene/pools"), serde_json::Value::Null);
}

#[test]
fn the_source_of_a_video_layer_offers_the_pools() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    act(&mut p, json!({"pool": {"id": "bars", "add": true}})).unwrap();
    act(&mut p, json!({"add_layer": {"kind": "shapes", "id": "drawn", "source": "bars"}})).unwrap();
    let fields = wprs_editor::inspect::fields(&p.doc, "/scene/layers/1");
    let video = fields.iter().find(|f| f.key == "video").unwrap();
    assert_eq!(video.options, vec![json!("bars")]);
}
