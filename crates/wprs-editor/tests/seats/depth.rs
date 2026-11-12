use super::{act, card, near, scene, seat};
use serde_json::json;
use wprs_editor::seats::depth::{depth, handles};

fn grip(p: &mut wprs_editor::Project, id: &str, grip: serde_json::Value, to: [f64; 2]) {
    act(p, json!({"layer": {"grip": {"layer": id, "grip": grip, "to": to}}})).unwrap();
}

fn tilted(p: &mut wprs_editor::Project) {
    let leaning = card("a", json!({"position": [380, 420], "tilt": [25, -30], "z": 120, "rotation": 15}));
    let child = json!({"id": "b", "type": "solid", "color": "#ffffff", "size": [80, 60], "parent": "a",
        "transform": {"position": [40, 10]}});
    act(p, json!({"set": {"pointer": "/scene/layers", "value": [leaning, child]}})).unwrap();
}

#[test]
fn grips_on_a_tilted_layer_land_under_the_pointer() {
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(&root, json!([]));
    tilted(&mut p);
    let ids: Vec<String> = wprs_editor::seats::seats(&p.dir, &p.doc, 0.0).into_iter().map(|s| s.id).collect();
    assert_eq!(ids, ["a", "b"], "{}", p.doc["scene"]["layers"]);
    grip(&mut p, "a", json!("move"), [500.0, 450.0]);
    assert!(near(handles(&seat(&p, "a")).pivot, [500.0, 450.0], 0.05), "{:?}", handles(&seat(&p, "a")));
    grip(&mut p, "a", json!({"scale": {"corner": [1, 1]}}), [640.0, 560.0]);
    assert!(near(seat(&p, "a").at([1.0, 1.0]), [640.0, 560.0], 0.5), "{:?}", seat(&p, "a").at([1.0, 1.0]));
    grip(&mut p, "a", json!("origin"), [520.0, 470.0]);
    assert!(near(handles(&seat(&p, "a")).pivot, [520.0, 470.0], 0.05));
    for (g, u) in [
        (json!({"scale": {"corner": [1, 0]}}), [1.0, 0.0]),
        (json!({"skew": {"edge": [1, 0.5]}}), [1.0, 0.5]),
    ] {
        let b = seat(&p, "b").at(u);
        let to = [b[0] + 12.0, b[1] - 9.0];
        grip(&mut p, "b", g.clone(), to);
        let edge = seat(&p, "b").at(u);
        if u == [1.0, 0.0] {
            assert!(near(edge, to, 0.5), "{edge:?} {to:?}");
        }
    }
    grip(&mut p, "b", json!("move"), [470.0, 430.0]);
    assert!(near(handles(&seat(&p, "b")).pivot, [470.0, 430.0], 0.05));
}

#[test]
fn the_depth_gizmo_tilts_and_lifts_through_the_renderer_projection() {
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(&root, json!([card("a", json!({"position": [380, 420], "tilt": [35, 20], "z": 150}))]));
    let aimed = depth(&seat(&p, "a"));
    let shown = seat(&p, "a");
    let projected = shown.lean.unwrap().apply(shown.flat([0.5, 0.5]).map(|v| v as f32));
    assert!(near(shown.at([0.5, 0.5]), projected.map(f64::from), 1e-6));
    act(&mut p, json!({"layer": {"transform": {"layer": "a", "tilt": [10, 20]}}})).unwrap();
    grip(&mut p, "a", json!({"tilt": {"axis": 0}}), aimed.height);
    let t = &p.doc["scene"]["layers"][0]["transform"]["tilt"];
    assert!((t[0].as_f64().unwrap() - 35.0).abs() < 0.3 && t[1] == json!(20.0), "{t}");
    let width = depth(&seat(&p, "a")).width;
    grip(&mut p, "a", json!({"tilt": {"axis": 1}}), width);
    let t = &p.doc["scene"]["layers"][0]["transform"]["tilt"];
    assert!((t[1].as_f64().unwrap() - 20.0).abs() < 0.3, "{t}");
    act(&mut p, json!({"layer": {"transform": {"layer": "a", "tilt": [35, 20], "z": -40}}})).unwrap();
    grip(&mut p, "a", json!("lift"), aimed.lift);
    let z = p.doc["scene"]["layers"][0]["transform"]["z"].as_f64().unwrap();
    assert!((z - 150.0).abs() < 0.5, "{z}");
    assert!(
        act(&mut p, json!({"layer": {"grip": {"layer": "a", "grip": {"tilt": {"axis": 2}}, "to": [0, 0]}}}))
            .is_err()
    );
}

#[test]
fn a_keyed_tilt_gets_a_key_at_the_playhead() {
    let root = tempfile::tempdir().unwrap();
    let keys = json!({"keys": [[0.0, [0, 0]], [4.0, [40, 0]]]});
    let mut p = scene(&root, json!([card("a", json!({"tilt": keys}))]));
    let to = depth(&seat(&p, "a")).height;
    let grab = json!({"layer": {"grip": {"layer": "a", "grip": {"tilt": {"axis": 0}}, "to": [to[0], to[1] + 30.0], "at": 2}}});
    act(&mut p, grab).unwrap();
    let keys = p.doc.pointer("/scene/layers/0/transform/tilt/keys").unwrap().as_array().unwrap();
    assert_eq!(keys.len(), 3, "{keys:?}");
}
