use super::motion::rigged;
use super::{act, ask, layer, undone};
use serde_json::{Value, json};

fn sculpt(p: &mut wprs_editor::Project, args: Value) -> anyhow::Result<()> {
    act(p, json!({ "scene3d": { "sculpt": args } }))
}

fn carved(p: &wprs_editor::Project) -> Vec<wprs_scene::Carved> {
    let path = p.doc["scene"]["assets"]["rig"]["shapes"]["smile"].as_str().unwrap();
    wprs_scene::carve_decode(&std::fs::read(p.dir.join(path)).unwrap()).unwrap()
}

#[test]
fn a_brush_sculpts_a_new_blend_shape_beside_the_model_with_undo() {
    let root = tempfile::tempdir().unwrap();
    let mut p = rigged(&root);
    let shown = ask(&p, json!({ "stage3d": { "time": 0.0 } }));
    let tip =
        shown["models"][0]["bones"].as_array().unwrap().iter().find(|b| b["name"] == "tip").unwrap()["at"]
            .clone();
    let at = [tip[0].as_f64().unwrap() as f32, tip[1].as_f64().unwrap() as f32];
    let stroke = |mode: &str, strength: f32| json!({ "layer": "doll", "shape": "smile", "mode": mode, "points": [at], "radius": 60, "strength": strength });
    let before = p.doc.clone();
    sculpt(&mut p, stroke("push", 0.1)).unwrap();
    undone(&mut p, &before);
    assert_eq!(layer(&p, "doll")["shapes"]["smile"], json!(1));
    let first = carved(&p);
    assert!(!first.is_empty() && first.iter().all(|c| c.node == 2 && c.primitive == 0), "{first:?}");
    let out = |c: &wprs_scene::Carved| c.at[0] * c.at[0] + c.at[2] * c.at[2];
    assert!(first.iter().any(|c| out(c) > 1e-6), "the push moves vertices out of the column: {first:?}");
    sculpt(&mut p, stroke("push", 0.1)).unwrap();
    let most = |list: &[wprs_scene::Carved]| list.iter().map(out).fold(0.0f32, f32::max);
    assert!(most(&carved(&p)) > most(&first) * 1.5, "a second stroke adds to the shape");
    sculpt(&mut p, stroke("smooth", 1.0)).unwrap();
    sculpt(&mut p, stroke("pull", 0.2)).unwrap();
    let m = ask(&p, json!({ "motion": { "layer": "doll" } }));
    assert!(m["shapes"].as_array().unwrap().contains(&json!("smile")), "{m}");
    for bad in [
        json!({ "layer": "doll", "shape": "bulge", "mode": "push", "points": [at], "radius": 60, "strength": 0.1 }),
        json!({ "layer": "doll", "shape": "smile", "mode": "push", "points": [], "radius": 60, "strength": 0.1 }),
        json!({ "layer": "doll", "shape": "smile", "mode": "smooth", "points": [at], "radius": 60, "strength": 2 }),
        json!({ "layer": "doll", "shape": "a b", "mode": "push", "points": [at], "radius": 60, "strength": 0.1 }),
    ] {
        assert!(sculpt(&mut p, bad.clone()).is_err(), "{bad}");
    }
}
