use super::{act, card, near, scene, seat};
use serde_json::json;

fn layered(root: &tempfile::TempDir) -> wprs_editor::Project {
    let mut p = scene(
        root,
        json!([card("a", json!({"position": [500, 500]})), card("b", json!({"position": [300, 300]}))]),
    );
    act(&mut p, json!({"canvas": {"parallax": {"amount": 1, "pointer": 1, "gyro": true}}})).unwrap();
    p
}

#[test]
fn depth_is_set_per_side_and_kept_short() {
    let root = tempfile::tempdir().unwrap();
    let mut p = layered(&root);
    act(&mut p, json!({"layer": {"depth": {"layer": "a", "depth": [0.2, -0.5]}}})).unwrap();
    assert_eq!(p.doc["scene"]["layers"][0]["parallax"], json!([0.2, -0.5]));
    let inputs = p.doc["manifest"]["inputs"].as_array().unwrap();
    assert!(inputs.contains(&json!("pointer")), "{inputs:?}");
    act(&mut p, json!({"layer": {"depth": {"layer": "a", "depth": [3, 3]}}})).unwrap();
    assert_eq!(p.doc["scene"]["layers"][0]["parallax"], json!(3.0));
    act(&mut p, json!({"layer": {"depth": {"layer": "a", "depth": [0, 0]}}})).unwrap();
    assert!(p.doc["scene"]["layers"][0].get("parallax").is_none());
    assert!(act(&mut p, json!({"layer": {"depth": {"layer": "a", "depth": [11, 0]}}})).is_err());
}

#[test]
fn a_bound_depth_is_kept_unless_the_write_replaces_it() {
    let root = tempfile::tempdir().unwrap();
    let mut p = layered(&root);
    let bound = json!({"set": {"pointer": "/scene/layers/0/parallax", "value": {"keys": [[0, 1], [2, 3]]}}});
    act(&mut p, bound).unwrap();
    assert!(act(&mut p, json!({"layer": {"depth": {"layer": "a", "depth": [1, 2]}}})).is_err());
    assert_eq!(p.doc["scene"]["layers"][0]["parallax"], json!({"keys": [[0, 1], [2, 3]]}));
    act(&mut p, json!({"layer": {"depth": {"layer": "a", "depth": [1, 2], "replace": true}}})).unwrap();
    assert_eq!(p.doc["scene"]["layers"][0]["parallax"], json!([1.0, 2.0]));
}

#[test]
fn a_simulated_pointer_and_tilt_shift_each_layer_by_its_depth() {
    let root = tempfile::tempdir().unwrap();
    let mut p = layered(&root);
    act(&mut p, json!({"layer": {"depth": {"layer": "a", "depth": [0.2, 0.1]}}})).unwrap();
    act(&mut p, json!({"canvas": {"probe": {"pointer": [1, 0.5]}}})).unwrap();
    assert!(near(seat(&p, "a").pivot(), [600.0, 500.0], 0.01), "{:?}", seat(&p, "a").pivot());
    assert!(near(seat(&p, "b").pivot(), [300.0, 300.0], 0.01));
    act(&mut p, json!({"canvas": {"probe": {"pointer": [0.5, 0.5], "tilt": [0, 1]}}})).unwrap();
    assert!(near(seat(&p, "a").pivot(), [500.0, 550.0], 0.01), "{:?}", seat(&p, "a").pivot());
    let shown = p.doc.pointer("/scene/ext/editor/probe").cloned().unwrap();
    assert_eq!(shown, json!({"pointer": [0.5, 0.5], "tilt": [0.0, 1.0]}));
    act(&mut p, json!({"layer": {"grip": {"layer": "a", "grip": "move", "to": [450, 450]}}})).unwrap();
    assert!(near(seat(&p, "a").pivot(), [450.0, 450.0], 0.01));
    assert!(act(&mut p, json!({"canvas": {"probe": {"pointer": [2, 0]}}})).is_err());
    act(&mut p, json!({"canvas": {"probe": {"off": true}}})).unwrap();
    assert!(near(seat(&p, "a").pivot(), [450.0, 400.0], 0.01), "{:?}", seat(&p, "a").pivot());
}
