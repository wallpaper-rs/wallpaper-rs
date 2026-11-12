use super::{act, near, scene, seat};
use serde_json::{Value, json};
use wprs_editor::Project;

fn region() -> Value {
    json!([{"id": "lens", "type": "adjustment", "size": [300, 200], "redraw": true,
        "effects": [{"effect": "vignette"}],
        "transform": {"position": [500, 400], "origin": [0.5, 0.5], "rotation": 20, "scale": [1.5, 1.5]}}])
}

fn view(p: &Project) -> Value {
    let q = json!({"capture_view": {"layer": "lens", "time": 0.0}});
    wprs_editor::rigs::query(&p.dir, &p.doc, "lens", &q)
}

fn size(p: &Project) -> Value {
    p.doc.pointer("/scene/layers/0/size").cloned().unwrap()
}

fn point(v: &Value) -> [f64; 2] {
    [v[0].as_f64().unwrap(), v[1].as_f64().unwrap()]
}

#[test]
fn the_captured_zone_ignores_the_turn_and_the_scale_and_the_redraw_follows_them() {
    let root = tempfile::tempdir().unwrap();
    let p = scene(&root, region());
    let v = view(&p);
    let zone: Vec<[f64; 2]> = v["zone"].as_array().unwrap().iter().map(point).collect();
    assert!(near(zone[0], [350.0, 300.0], 0.05) && near(zone[2], [650.0, 500.0], 0.05), "{zone:?}");
    assert_eq!(v["max"], json!([2000.0, 2000.0]));
    assert_eq!(v["over"], json!(false));
    let drawn = point(&v["drawn"][0]);
    assert!(near(drawn, seat(&p, "lens").corners()[0], 1e-6));
    assert!(!near(drawn, zone[0], 1.0));
}

#[test]
fn a_handle_edits_the_size_and_leaves_the_scale_and_the_pivot() {
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(&root, region());
    act(
        &mut p,
        json!({"layer": {"grip": {"layer": "lens", "grip": {"size": {"corner": [1, 1]}}, "to": [700, 560]}}}),
    )
    .unwrap();
    assert_eq!(size(&p), json!([400.0, 320.0]));
    assert_eq!(p.doc.pointer("/scene/layers/0/transform/scale").cloned().unwrap(), json!([1.5, 1.5]));
    assert!(near(seat(&p, "lens").pivot(), [500.0, 400.0], 0.05));
    act(&mut p, json!({"layer": {"grip": {"layer": "lens", "grip": {"size": {"corner": [0.5, 0]}}, "to": [999, 100]}}}))
        .unwrap();
    assert_eq!(size(&p), json!([400.0, 600.0]));
}

#[test]
fn the_size_stops_at_twice_the_canvas() {
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(&root, region());
    act(&mut p, json!({"layer": {"grip": {"layer": "lens", "grip": {"size": {"corner": [1, 1]}}, "to": [9000, 9000]}}}))
        .unwrap();
    assert_eq!(size(&p), json!([2000.0, 2000.0]));
    assert_eq!(view(&p)["over"], json!(false));
    let other = tempfile::tempdir().unwrap();
    let plain = scene(&other, json!([{"id": "lens", "type": "solid", "color": "#ffffff", "size": [10, 10]}]));
    assert!(view(&plain).is_null());
}
