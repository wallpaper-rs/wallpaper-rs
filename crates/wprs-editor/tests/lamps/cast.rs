use super::{act, layer, light, project, undone};
use serde_json::json;

#[test]
fn spots_project_pictures_videos_and_layers_without_loops() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    std::fs::write(p.dir.join("clip.mp4"), b"").unwrap();
    act(&mut p, json!({ "set": { "pointer": "/scene/assets/clip", "value": { "path": "clip.mp4" } } }))
        .unwrap();
    light(&mut p, json!({ "add": { "light": "spot", "id": "s" } })).unwrap();
    let before = p.doc.clone();
    light(&mut p, json!({ "projection": { "layer": "s", "asset": "pic" } })).unwrap();
    undone(&mut p, &before);
    assert_eq!(layer(&p, "s")["texture"], "pic");
    light(&mut p, json!({ "projection": { "layer": "s", "asset": "clip" } })).unwrap();
    assert_eq!(layer(&p, "s")["texture"], "clip");
    light(&mut p, json!({ "projection": { "layer": "s", "from": "logo" } })).unwrap();
    assert_eq!(layer(&p, "s")["texture"], json!({ "layer": "logo" }));
    assert!(light(&mut p, json!({ "projection": { "layer": "s", "asset": "song" } })).is_err());
    assert!(light(&mut p, json!({ "projection": { "layer": "s", "from": "nothing" } })).is_err());
    assert!(light(&mut p, json!({ "projection": { "layer": "s" } })).is_err());
    act(&mut p, json!({ "lighting": { "layer": "logo", "roughness": 0.4 } })).unwrap();
    let cast = wprs_editor::lamps::cast(&p.dir, &p.doc, "s", 0.0);
    assert!(!cast["layers"].as_array().unwrap().contains(&json!("logo")), "a lit layer would light itself");
    assert!(light(&mut p, json!({ "projection": { "layer": "s", "from": "logo" } })).is_err());
    light(&mut p, json!({ "projection": { "layer": "s", "off": true } })).unwrap();
    assert!(layer(&p, "s").get("texture").is_none());
    light(&mut p, json!({ "add": { "light": "point", "id": "dot" } })).unwrap();
    assert!(
        light(&mut p, json!({ "projection": { "layer": "dot", "asset": "pic" } })).is_err(),
        "only spots"
    );
}

#[test]
fn the_cone_preview_lands_the_picture_on_the_canvas() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    light(&mut p, json!({ "add": { "light": "spot", "id": "s" } })).unwrap();
    light(&mut p, json!({ "set": { "layer": "s", "tilt": 89, "cone": [30, 40], "z": 200 } })).unwrap();
    light(&mut p, json!({ "projection": { "layer": "s", "asset": "pic" } })).unwrap();
    let cast = wprs_editor::lamps::cast(&p.dir, &p.doc, "s", 0.0);
    assert_eq!(cast["kind"], "picture");
    assert!(cast["image"].as_str().unwrap().starts_with("data:image/png"));
    assert!(cast["pictures"].as_array().unwrap().contains(&json!("pic")));
    let quad: Vec<[f64; 2]> = serde_json::from_value(cast["quad"].clone()).unwrap();
    let width = quad[1][0] - quad[0][0];
    let expected = 2.0 * 200.0 * 20f64.to_radians().tan();
    assert!((width - expected).abs() < 6.0, "{quad:?} {expected}");
    assert!(quad[2][1] > quad[1][1], "the picture stays upright: {quad:?}");
    let m = wprs_editor::lamps::homography([[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]]);
    assert!((m[0] - 10.0).abs() < 1e-4 && (m[5] - 10.0).abs() < 1e-4 && m[12].abs() < 1e-6, "{m:?}");
    light(&mut p, json!({ "set": { "layer": "s", "tilt": 0 } })).unwrap();
    let level = wprs_editor::lamps::cast(&p.dir, &p.doc, "s", 0.0);
    assert_eq!(level["landed"], false, "a level beam does not land on the canvas");
    let quad: Vec<[f64; 2]> = serde_json::from_value(level["quad"].clone()).unwrap();
    let at = &wprs_editor::lamps::lights(&p.dir, &p.doc, 0.0)["lights"][0]["at"];
    let far = quad.iter().map(|q| (q[0] - at[0].as_f64().unwrap()).hypot(q[1] - at[1].as_f64().unwrap()));
    let far = far.fold(0.0, f64::max);
    assert!((far - 900.0).abs() < 1.0, "the far edge is drawn out to the radius: {quad:?}");
}

#[test]
fn the_cone_preview_shows_a_projected_layer() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    light(&mut p, json!({ "add": { "light": "spot", "id": "s" } })).unwrap();
    light(&mut p, json!({ "set": { "layer": "s", "tilt": 89, "z": 200 } })).unwrap();
    let title = json!({ "id": "title", "type": "text", "text": "Hi", "size": 64 });
    act(&mut p, json!({ "add": { "pointer": "/scene/layers/-", "value": title } })).unwrap();
    light(&mut p, json!({ "projection": { "layer": "s", "from": "title" } })).unwrap();
    let cast = wprs_editor::lamps::cast(&p.dir, &p.doc, "s", 0.0);
    assert_eq!((cast["kind"].as_str(), cast["landed"].as_bool()), (Some("layer"), Some(true)));
    assert!(cast["image"].as_str().unwrap().starts_with("data:image/png"), "{cast}");
}
