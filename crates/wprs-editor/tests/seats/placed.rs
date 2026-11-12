use super::{card, near, scene, seat};
use serde_json::{Value, json};
use std::sync::Arc;
use wprs_scene::{Affine, Quality, Runtime, Scene, Values};

fn drawn(doc: &Value) -> Vec<(String, [[f64; 2]; 4])> {
    let scene = Scene::from_json(&doc["scene"].to_string()).unwrap();
    let mut rt = Runtime::new(Arc::new(scene), Values::new());
    rt.advance(0.0);
    let frame = rt.frame([0.0, 0.0, 1000.0, 1000.0], Quality::Q3);
    let sized = |d: &wprs_scene::Draw| match &d.content {
        wprs_scene::Content::Solid { size, .. } => Some(*size),
        _ => None,
    };
    frame
        .draws
        .iter()
        .filter_map(|d| {
            let [w, h] = sized(d)?;
            let unit = d
                .world
                .then(Affine::translate([-d.origin[0] * w, -d.origin[1] * h]))
                .then(Affine::scale([w, h]));
            let corners =
                [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]].map(|u| unit.apply(u).map(f64::from));
            Some((d.layer.id.clone(), corners))
        })
        .collect()
}

fn family() -> Value {
    json!([
        card("base", json!({"position": [500, 500], "rotation": 90, "scale": [2, 0.5], "skew": [10, 0]})),
        {"id": "kid", "type": "solid", "color": "#ff0000", "size": [40, 40], "parent": "base",
            "transform": {"position": [100, 0], "origin": [0, 1], "rotation": 15}},
        {"id": "box", "type": "group", "transform": {"position": [100, 50], "rotation": 30}, "layers": [
            card("inner", json!({"skew": [0, 20], "origin": [0.25, 0.75]})),
            {"id": "corner", "type": "solid", "color": "#00ff00", "size": [30, 30], "anchor": "bottom_right",
                "transform": {"position": [-20, -20]}}
        ]},
        {"id": "far", "type": "solid", "color": "#0000ff", "size": [50, 50], "parallax": 2, "parent": "kid",
            "transform": {"position": [10, 10]}}
    ])
}

#[test]
fn every_outline_sits_where_the_scene_draws_the_layer() {
    let root = tempfile::tempdir().unwrap();
    let p = scene(&root, family());
    let found = drawn(&p.doc);
    assert_eq!(found.len(), 5, "{found:?}");
    for (id, corners) in found {
        let outline = seat(&p, &id).outline();
        let close = outline.iter().zip(corners).all(|(a, b)| near(*a, b, 0.01));
        assert!(close, "{id}: the canvas puts {outline:?}, the scene draws {corners:?}");
    }
}

#[test]
fn a_child_follows_its_turned_and_scaled_parent() {
    let root = tempfile::tempdir().unwrap();
    let p = scene(&root, family());
    let kid = seat(&p, "kid");
    assert!(near(kid.pivot(), [500.0, 700.0], 0.01), "{:?}", kid.pivot());
    assert!(near(kid.position(), [100.0, 0.0], 0.01), "{:?}", kid.position());
    let corner = seat(&p, "corner");
    let wanted = seat(&p, "box").world.apply([980.0, 980.0]).map(f64::from);
    assert!(near(corner.pivot(), wanted, 0.01), "{:?} {wanted:?}", corner.pivot());
}

#[test]
fn a_group_box_holds_its_layers() {
    let root = tempfile::tempdir().unwrap();
    let p = scene(&root, family());
    let group = seat(&p, "box");
    for id in ["inner", "corner"] {
        for c in seat(&p, id).outline() {
            let u = group.fraction(c);
            assert!(
                u.iter().all(|v| (-1e-3..=1.001).contains(v)),
                "{id} {c:?} is outside the group at {u:?}"
            );
        }
    }
}

#[test]
fn a_tilted_parent_leans_the_outline_of_its_child() {
    let root = tempfile::tempdir().unwrap();
    let layers = json!([
        card("lean", json!({"position": [500, 500], "tilt": [0, 40]})),
        {"id": "rider", "type": "solid", "color": "#ffffff", "size": [50, 50], "parent": "lean",
            "transform": {"position": [80, 0]}}
    ]);
    let p = scene(&root, layers);
    let rider = seat(&p, "rider");
    assert_ne!(rider.corners(), rider.outline(), "the child outline did not lean with its parent");
    let back = rider.fraction(rider.at([0.25, 0.75]));
    assert!(near(back, [0.25, 0.75], 1e-3), "{back:?}");
}
