use super::kit::{act, info, parts};
use serde_json::json;

#[test]
fn expressions_are_named_sets_of_blend_shape_weights() {
    let root = tempfile::tempdir().unwrap();
    let mut p = parts(&root);
    assert_eq!(
        (info(&p, "body")["shapes"].clone(), info(&p, "body")["defaults"].clone()),
        (json!(["smile"]), json!([0.0]))
    );
    let set = |name: &str, w: f32| json!({ "expression": { "layer": "body", "name": name, "shapes": { "smile": w } } });
    act(&mut p, set("happy", 0.8)).unwrap();
    let layer = |p: &wprs_editor::Project| p.doc["scene"]["layers"][0].clone();
    assert_eq!(layer(&p)["expressions"], json!({ "happy": { "smile": 0.8 } }));
    act(&mut p, json!({ "set": { "pointer": "/scene/layers/0/shapes", "value": { "happy": 1 } } })).unwrap();
    act(&mut p, json!({ "expression": { "layer": "body", "name": "happy", "to": "glad" } })).unwrap();
    assert_eq!(
        (layer(&p)["shapes"].clone(), info(&p, "body")["expressions"].as_object().unwrap().len()),
        (json!({ "glad": 1 }), 1)
    );
    act(&mut p, json!({ "expression": { "layer": "body", "name": "glad", "remove": true } })).unwrap();
    assert!(layer(&p).get("expressions").is_none() && layer(&p).get("shapes").is_none());
    act(&mut p, json!({ "undo": {} })).unwrap();
    assert_eq!(layer(&p)["shapes"], json!({ "glad": 1 }));
    act(&mut p, json!({ "redo": {} })).unwrap();
    for bad in [
        set("sad", 2.0),
        json!({ "expression": { "layer": "body", "name": "sad", "shapes": { "frown": 1 } } }),
        json!({ "expression": { "layer": "body", "name": "", "shapes": { "smile": 1 } } }),
        json!({ "expression": { "layer": "body", "name": "nobody", "remove": true } }),
    ] {
        assert!(act(&mut p, bad).is_err());
    }
    act(&mut p, json!({ "clip": { "layer": "body", "id": "grin", "add": true } })).unwrap();
    act(
        &mut p,
        json!({ "key": { "layer": "body", "clip": "grin", "time": 0.5, "shape": "smile", "weight": 0.5 } }),
    )
    .unwrap();
    assert_eq!(layer(&p)["clips"][0]["shapes"], json!({ "smile": [[0.5, 0.5]] }));
}

#[test]
fn a_brush_sculpts_a_new_blend_shape_into_the_mesh() {
    let root = tempfile::tempdir().unwrap();
    let mut p = super::kit::rigged(&root);
    let stroke = |mode: &str, by: f32, shape: &str| json!({ "shape_sculpt": { "layer": "net", "shape": shape, "points": [[0.5, 0.5]], "mode": mode, "by": by, "radius": 0.3 } });
    assert_eq!(info(&p, "net")["shapes"], json!([]));
    act(&mut p, stroke("push", 20.0, "puff")).unwrap();
    assert_eq!(info(&p, "net")["shapes"], json!(["puff"]));
    let spread = |p: &wprs_editor::Project, shape: &str| {
        let path = p.doc["scene"]["assets"]["net"]["path"].as_str().unwrap();
        let rig = wprs_scene::Rigged::decode(&std::fs::read(p.dir.join(path)).unwrap()).unwrap();
        let t = rig.shape(shape).unwrap();
        let mut total = 0.0;
        for piece in &rig.pieces {
            for (s, base) in &piece.targets {
                if *s as usize == t {
                    let deltas = &rig.morphs[*base as usize..(*base + piece.vertices) as usize];
                    total += deltas.iter().map(|d| d[0].abs() + d[1].abs()).sum::<f32>();
                }
            }
        }
        total
    };
    let first = spread(&p, "puff");
    assert!(first > 0.0);
    act(&mut p, stroke("push", 20.0, "puff")).unwrap();
    assert!(spread(&p, "puff") > first);
    act(&mut p, stroke("smooth", 1.0, "puff")).unwrap();
    act(&mut p, stroke("pull", 5.0, "puff")).unwrap();
    act(&mut p, stroke("erase", 1.0, "puff")).unwrap();
    assert!(spread(&p, "puff") < first * 2.0);
    act(&mut p, stroke("push", 10.0, "dent")).unwrap();
    assert_eq!(info(&p, "net")["shapes"], json!(["puff", "dent"]));
    act(&mut p, json!({ "undo": {} })).unwrap();
    assert_eq!(info(&p, "net")["shapes"], json!(["puff"]));
    for bad in [
        stroke("push", -1.0, "puff"),
        stroke("smooth", 2.0, "puff"),
        stroke("push", 5000.0, "puff"),
        stroke("push", 5.0, ""),
        stroke("push", 5.0, "a/b"),
        json!({ "shape_sculpt": { "layer": "net", "shape": "x", "points": [], "mode": "push", "by": 1 } }),
        json!({ "shape_sculpt": { "layer": "nothing", "shape": "x", "points": [[0.5, 0.5]], "mode": "push", "by": 1 } }),
    ] {
        assert!(act(&mut p, bad).is_err());
    }
}
