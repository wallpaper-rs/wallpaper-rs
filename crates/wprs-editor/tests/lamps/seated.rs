use super::{layer, light, project};
use serde_json::json;

fn size(p: &wprs_editor::Project) -> [f64; 2] {
    [0, 1].map(|i| p.doc["scene"]["canvas"]["size"][i].as_f64().unwrap())
}

#[test]
fn an_anchored_light_moves_to_the_canvas_point_it_is_dragged_to() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    light(&mut p, json!({ "add": { "light": "spot", "id": "s" } })).unwrap();
    let pointer = wprs_editor::actions::layer(&p.doc, "s").unwrap().0;
    let anchor = json!({ "set": { "pointer": format!("{pointer}/anchor"), "value": "bottom_right" } });
    super::act(&mut p, anchor).unwrap();
    light(&mut p, json!({ "set": { "layer": "s", "at": [400, 300] } })).unwrap();
    let [w, h] = size(&p);
    assert_eq!(layer(&p, "s")["transform"]["position"], json!([400.0 - w, 300.0 - h]));
    let shown = wprs_editor::lamps::lights(&p.dir, &p.doc, 0.0);
    let at = &shown["lights"][0]["at"];
    assert!(
        (at[0].as_f64().unwrap() - 400.0).abs() < 0.01 && (at[1].as_f64().unwrap() - 300.0).abs() < 0.01,
        "{at}"
    );
}

#[test]
fn a_light_in_a_skewed_group_lands_where_it_is_dragged() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    let group = json!({ "id": "g", "type": "group", "transform": { "position": [500, 400], "skew": [30, 0] }, "layers": [] });
    super::act(&mut p, json!({ "add": { "pointer": "/scene/layers/-", "value": group } })).unwrap();
    light(&mut p, json!({ "add": { "light": "point", "id": "l", "into": "g", "at": [0, 0] } })).unwrap();
    light(&mut p, json!({ "set": { "layer": "l", "at": [700, 600] } })).unwrap();
    let at = &wprs_editor::lamps::lights(&p.dir, &p.doc, 0.0)["lights"][0]["at"];
    assert!(
        (at[0].as_f64().unwrap() - 700.0).abs() < 0.05 && (at[1].as_f64().unwrap() - 600.0).abs() < 0.05,
        "{at}"
    );
}

#[test]
fn the_tilt_knob_writes_a_key_at_the_playhead_when_the_tilt_is_keyed() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    light(&mut p, json!({ "add": { "light": "spot", "id": "s" } })).unwrap();
    let pointer = wprs_editor::actions::layer(&p.doc, "s").unwrap().0;
    let keyed = json!({ "keys": [[0, [10, 0]], [2, [40, 0]]] });
    let set = json!({ "set": { "pointer": format!("{pointer}/transform/tilt"), "value": keyed } });
    super::act(&mut p, set).unwrap();
    light(&mut p, json!({ "set": { "layer": "s", "tilt": 25, "time": 1 } })).unwrap();
    let keys = layer(&p, "s")["transform"]["tilt"]["keys"].as_array().unwrap().clone();
    assert_eq!(keys.len(), 3, "{keys:?}");
    assert!(keys.iter().any(|k| k[0] == 1.0 && k[1] == json!([25.0, 0])), "{keys:?}");
    light(&mut p, json!({ "set": { "layer": "s", "tilt": 30, "time": 1, "replace": true } })).unwrap();
    assert_eq!(layer(&p, "s")["transform"]["tilt"], json!([30.0, 0]));
}

#[test]
fn the_radius_and_tube_end_handles_write_keys_at_the_playhead_when_keyed() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    light(&mut p, json!({ "add": { "light": "tube", "id": "t", "at": [400, 300] } })).unwrap();
    let pointer = wprs_editor::actions::layer(&p.doc, "t").unwrap().0;
    for (field, keys) in [
        ("radius", json!([[0, 100], [2, 300]])),
        ("transform/position", json!([[0, [400, 300]], [2, [600, 300]]])),
    ] {
        let set = json!({ "set": { "pointer": format!("{pointer}/{field}"), "value": { "keys": keys } } });
        super::act(&mut p, set).unwrap();
    }
    light(&mut p, json!({ "set": { "layer": "t", "radius": 220, "time": 1 } })).unwrap();
    let radius = layer(&p, "t")["radius"]["keys"].as_array().unwrap().clone();
    assert!(radius.len() == 3 && radius.iter().any(|k| k[0] == 1.0 && k[1] == 220.0), "{radius:?}");
    let end =
        json!({ "set": { "layer": "t", "at": [500, 320], "aim": [560, 320], "length": 120, "time": 1 } });
    light(&mut p, end).unwrap();
    let t = layer(&p, "t");
    let keys = t["transform"]["position"]["keys"].as_array().unwrap();
    assert_eq!(keys.len(), 3, "{keys:?}");
    assert_eq!(t["length"], json!(120.0));
    let unkeyed = json!({ "set": { "layer": "t", "radius": 50 } });
    assert!(light(&mut p, unkeyed).is_err(), "a keyed radius is not overwritten without replace");
}
