use super::{act, ask, edit, layer, project, undone};
use serde_json::{Value, json};

fn shown(p: &wprs_editor::Project) -> Value {
    ask(p, json!({ "stage3d": { "time": 0 } }))
}

fn xy(v: &Value) -> [f64; 2] {
    [v[0].as_f64().unwrap(), v[1].as_f64().unwrap()]
}

fn grab(p: &mut wprs_editor::Project, what: Value, handle: &str, from: [f64; 2], to: [f64; 2]) {
    edit(p, json!({ "grab": { "what": what, "handle": handle, "from": from, "to": to } })).unwrap();
}

fn doll(root: &tempfile::TempDir) -> wprs_editor::Project {
    let mut p = project(root);
    edit(&mut p, json!({ "model": { "add": { "asset": "toy", "id": "doll" } } })).unwrap();
    edit(&mut p, json!({ "view": { "orbit": [35, 20] } })).unwrap();
    p
}

fn near(a: [f64; 2], b: [f64; 2], by: f64) -> bool {
    (a[0] - b[0]).hypot(a[1] - b[1]) < by
}

#[test]
fn the_model_moves_along_its_arrows_in_the_view_plane_and_in_depth() {
    let root = tempfile::tempdir().unwrap();
    let mut p = doll(&root);
    let m = shown(&p)["models"][0].clone();
    let (at, arm) = (xy(&m["at"]), xy(&m["arms"][0]));
    let before = p.doc.clone();
    grab(&mut p, json!({ "model": { "layer": "doll" } }), "x", at, arm);
    undone(&mut p, &before);
    let x = layer(&p, "doll")["position"][0].as_f64().unwrap();
    assert!((x - m["length"].as_f64().unwrap()).abs() < 0.02 * x.abs().max(0.1), "{x} {m}");
    let now = shown(&p)["models"][0].clone();
    assert!(near(xy(&now["at"]), arm, 1.5), "the renderer places it under the pointer: {now}");
    let at = xy(&now["at"]);
    let to = [at[0] + 40.0, at[1] - 25.0];
    grab(&mut p, json!({ "model": { "layer": "doll" } }), "plane", at, to);
    assert!(near(xy(&shown(&p)["models"][0]["at"]), to, 1.0));
    let deep = shown(&p)["models"][0]["depth"].as_f64().unwrap();
    grab(&mut p, json!({ "model": { "layer": "doll" } }), "depth", at, [at[0], at[1] - 120.0]);
    let after = shown(&p)["models"][0].clone();
    assert!(after["depth"].as_f64().unwrap() > deep * 1.1, "{after}");
    assert!(near(xy(&after["at"]), to, 2.0), "depth keeps it under the pointer: {after}");
}

#[test]
fn the_rings_turn_the_model_around_each_axis() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    edit(&mut p, json!({ "model": { "add": { "asset": "toy", "id": "doll" } } })).unwrap();
    let m = shown(&p)["models"][0].clone();
    let rings = &m["turns"];
    assert_eq!(rings["rings"].as_array().unwrap().len(), 3);
    let (c, g) = (xy(&m["at"]), xy(&rings["grips"][2]));
    let to = [c[0] + (g[1] - c[1]), c[1] - (g[0] - c[0])];
    grab(&mut p, json!({ "model": { "layer": "doll" } }), "turn_z", g, to);
    let r = layer(&p, "doll")["rotation"].clone();
    assert!((r[2].as_f64().unwrap() - 90.0).abs() < 1.0, "{r}");
    edit(&mut p, json!({ "view": { "orbit": [30, 25] } })).unwrap();
    let m = shown(&p)["models"][0].clone();
    let g = xy(&m["turns"]["grips"][1]);
    grab(&mut p, json!({ "model": { "layer": "doll" } }), "turn_y", g, [g[0] + 30.0, g[1] + 10.0]);
    let r = layer(&p, "doll")["rotation"].clone();
    assert_ne!(r, json!([0.0, 0.0, 90.0]), "{r}");
}

#[test]
fn keyed_values_get_a_key_and_bound_ones_are_kept_unless_replaced() {
    let root = tempfile::tempdir().unwrap();
    let mut p = doll(&root);
    let pointer = wprs_editor::actions::layer(&p.doc, "doll").unwrap().0 + "/position";
    act(&mut p, json!({ "keyframe": { "pointer": pointer, "time": 0, "value": [0, 0, 0] } })).unwrap();
    act(&mut p, json!({ "keyframe": { "pointer": pointer, "time": 2, "value": [0, 1, 0] } })).unwrap();
    let m = shown(&p)["models"][0].clone();
    let (a, b) = (xy(&m["at"]), xy(&m["arms"][0]));
    let g = json!({ "what": { "model": { "layer": "doll" } }, "handle": "x", "from": a, "to": b, "time": 1 });
    edit(&mut p, json!({ "grab": g })).unwrap();
    let keys = p.doc.pointer(&format!("{pointer}/keys")).unwrap().as_array().unwrap().clone();
    assert_eq!(keys.len(), 3, "{keys:?}");
    let mut replace = g.clone();
    replace["replace"] = json!(true);
    edit(&mut p, json!({ "grab": replace.clone() })).unwrap();
    assert!(p.doc.pointer(&pointer).unwrap().is_array());
    let bound = json!({ "prop": "spot", "map": { "a": [0, 0, 0] } });
    act(&mut p, json!({ "set": { "pointer": pointer, "value": bound.clone() } })).unwrap();
    assert!(edit(&mut p, json!({ "grab": g })).is_err());
    assert_eq!(p.doc.pointer(&pointer), Some(&bound));
    edit(&mut p, json!({ "grab": replace })).unwrap();
    assert!(p.doc.pointer(&pointer).unwrap().is_array());
}

#[test]
fn the_camera_slides_deepens_orbits_dollies_and_rolls() {
    let root = tempfile::tempdir().unwrap();
    let mut p = doll(&root);
    let h = shown(&p)["handles"].clone();
    let eye = xy(&h["eye"]);
    let to = [eye[0] + 30.0, eye[1] + 12.0];
    grab(&mut p, json!("eye"), "plane", eye, to);
    assert!(near(xy(&shown(&p)["handles"]["eye"]), to, 1.0));
    let dist = |p: &wprs_editor::Project| {
        let c = &p.doc["scene"]["camera3d"];
        let (e, m) = (&c["eye"], c.get("center").cloned().unwrap_or(json!([0, 0, 0])));
        (0..3).map(|i| (e[i].as_f64().unwrap() - m[i].as_f64().unwrap()).powi(2)).sum::<f64>().sqrt()
    };
    let d0 = dist(&p);
    let rig = shown(&p)["handles"]["rig"].clone();
    let o = xy(&rig["orbit"]);
    grab(&mut p, json!("eye"), "orbit", o, [o[0] + 60.0, o[1]]);
    assert!((dist(&p) - d0).abs() < 0.01 * d0, "orbit keeps the distance");
    let d = xy(&shown(&p)["handles"]["rig"]["dolly"]);
    grab(&mut p, json!("eye"), "dolly", d, [d[0], d[1] + 100.0]);
    assert!(dist(&p) > d0 * 1.05, "dolly backs away");
    let d1 = dist(&p);
    let c = xy(&shown(&p)["handles"]["center"]);
    grab(&mut p, json!("center"), "depth", c, [c[0], c[1] + 100.0]);
    assert!(dist(&p) < d1, "the aim comes closer along its ray");
    let rig = shown(&p)["handles"]["rig"].clone();
    let (g, e) = (xy(&rig["roll_grip"]), xy(&shown(&p)["handles"]["eye"]));
    let to = [g[0] - (g[1] - e[1]) * 0.5, g[1] + (g[0] - e[0]) * 0.5];
    grab(&mut p, json!("eye"), "roll", g, to);
    let up = shown(&p)["camera"]["up"].clone();
    assert!(up[1].as_f64().unwrap() < 0.97, "{up}");
}

#[test]
fn path_keys_move_in_depth() {
    let root = tempfile::tempdir().unwrap();
    let mut p = doll(&root);
    edit(&mut p, json!({ "path": { "add": { "id": "fly" } } })).unwrap();
    edit(&mut p, json!({ "view": { "orbit": [50, 10], "zoom": 0.5 } })).unwrap();
    let keys = ask(&p, json!({ "paths3d": { "time": 0 } }))["paths"][0]["keys"].clone();
    let at = xy(&keys[1]["at"]);
    let d = |p: &wprs_editor::Project| {
        let k = &p.doc["scene"]["camera3d"]["paths"][0]["keys"][1];
        (0..3)
            .map(|i| (k["eye"][i].as_f64().unwrap() - k["center"][i].as_f64().unwrap()).powi(2))
            .sum::<f64>()
    };
    let before = d(&p);
    let key = json!({ "key": { "path": "fly", "key": 2, "aim": false } });
    grab(&mut p, key.clone(), "depth", at, [at[0], at[1] - 80.0]);
    assert!((d(&p) - before).abs() > 1e-3);
    let at = xy(&ask(&p, json!({ "paths3d": { "time": 0 } }))["paths"][0]["keys"][1]["at"]);
    let to = [at[0] - 25.0, at[1] + 15.0];
    grab(&mut p, key, "plane", at, to);
    let now = xy(&ask(&p, json!({ "paths3d": { "time": 0 } }))["paths"][0]["keys"][1]["at"]);
    assert!(near(now, to, 1.0), "{now:?} {to:?}");
}
