use super::{act, card, near, scene, seat};
use serde_json::json;
use wprs_editor::seats::lens::{frame, lens};

fn camera(p: &mut wprs_editor::Project, camera: serde_json::Value) {
    act(p, json!({"set": {"pointer": "/scene/canvas/camera", "value": camera}})).unwrap();
}

#[test]
fn the_frame_is_what_the_renderer_camera_shows() {
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(&root, json!([card("a", json!({"position": [300, 400]}))]));
    camera(&mut p, json!({"center": [400, 450], "zoom": 2, "rotation": 30}));
    let f = frame(&p.doc, 0.0).unwrap();
    let l = lens(&p.doc, 0.0).unwrap();
    let shown = |q: [f64; 2]| l.apply(q.map(|v| v as f32)).map(f64::from);
    assert!(near(f.center, [400.0, 450.0], 1e-3), "{:?}", f.center);
    assert!((f.zoom - 2.0).abs() < 1e-4 && (f.rotation - 30.0).abs() < 1e-3, "{f:?}");
    for (c, s) in f.corners.iter().zip([[0.0, 0.0], [1000.0, 0.0], [1000.0, 1000.0], [0.0, 1000.0]]) {
        assert!(near(shown(*c), s, 1e-2), "{c:?}");
    }
    assert!(near(seat(&p, "a").pivot(), shown([300.0, 400.0]), 1e-2));
}

#[test]
fn grips_of_the_frame_move_zoom_and_turn_the_camera() {
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(&root, json!([card("a", json!({}))]));
    let grip = |p: &mut wprs_editor::Project, hold: serde_json::Value, to: [f64; 2]| {
        act(p, json!({"canvas": {"camera_grip": {"hold": hold, "to": to}}})).unwrap();
        frame(&p.doc, 0.0).unwrap()
    };
    let f = grip(&mut p, json!("move"), [600.0, 300.0]);
    assert!(near(f.center, [600.0, 300.0], 1e-2));
    let f = grip(&mut p, json!({"zoom": {"corner": [1, 1]}}), [850.0, 550.0]);
    assert!((f.zoom - 2.0).abs() < 1e-3 && near(f.corners[2], [850.0, 550.0], 0.1), "{f:?}");
    let f = grip(&mut p, json!("turn"), [800.0, 300.0]);
    assert!((f.rotation + 90.0).abs() < 1e-2, "{f:?}");
    let top = f.top;
    let (dx, dy) = (top[0] - f.center[0], top[1] - f.center[1]);
    assert!(dx > 0.0 && dy.abs() < 1e-2, "{top:?}");
}

#[test]
fn a_keyed_camera_gets_a_key_at_the_playhead() {
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(&root, json!([card("a", json!({}))]));
    let keys = json!({"keys": [[0.0, 1], [4.0, 3]]});
    camera(&mut p, json!({"zoom": keys}));
    act(&mut p, json!({"canvas": {"camera": {"zoom": 2.5, "at": 2}}})).unwrap();
    let keys = p.doc.pointer("/scene/canvas/camera/zoom/keys").unwrap().as_array().unwrap().clone();
    assert_eq!(keys.len(), 3, "{keys:?}");
    assert!((frame(&p.doc, 2.0).unwrap().zoom - 2.5).abs() < 1e-3);
    let plain = json!({"canvas": {"camera": {"zoom": 2, "replace": true}}});
    act(&mut p, plain).unwrap();
    assert_eq!(p.doc.pointer("/scene/canvas/camera/zoom"), Some(&json!(2.0)));
    assert!(act(&mut p, json!({"canvas": {"camera": {"zoom": 40}}})).is_err());
}

#[test]
fn the_whole_canvas_view_places_layers_without_the_camera_and_keeps_the_frame() {
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(&root, json!([card("a", json!({"position": [300, 400]}))]));
    camera(&mut p, json!({"center": [300, 400], "zoom": 2}));
    assert!(near(seat(&p, "a").pivot(), [500.0, 500.0], 1e-2));
    act(&mut p, json!({"canvas": {"camera_view": {"whole": true}}})).unwrap();
    assert!(near(seat(&p, "a").pivot(), [300.0, 400.0], 1e-2));
    let f = frame(&p.doc, 0.0).unwrap();
    assert!(f.whole && near(f.corners[0], [50.0, 150.0], 1e-2), "{f:?}");
    let mut shown = p.doc["scene"].clone();
    wprs_editor::rigs::preview(&mut shown, &json!({}));
    assert!(shown["canvas"].get("camera").is_none());
    act(&mut p, json!({"canvas": {"camera_view": {"whole": false}}})).unwrap();
    assert!(p.doc.pointer("/scene/ext/editor/camera2d").is_none());
    act(&mut p, json!({"canvas": {"camera": {"off": true}}})).unwrap();
    assert!(frame(&p.doc, 0.0).is_none());
}
