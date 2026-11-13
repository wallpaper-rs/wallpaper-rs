use crate::allocations::allocations;
use serde_json::json;
use std::sync::Arc;
use wprs_scene::{Forge, ForgeMaterial, ForgeShape, Quality, Runtime, SCHEMA, Scene, Spare, Values};

fn deep() -> Scene {
    let glb = Forge::default()
        .material(ForgeMaterial::new("paint", [0.8, 0.3, 0.2, 1.0]))
        .part("box", ForgeShape::Cube, [0.0; 3], 1.0, 0)
        .glb();
    let turn = json!({ "keys": [[0, [0, 0, 0]], [4, [0, 360, 0]]], "loop": "repeat" });
    let model = |id: &str, at: [f32; 3]| json!({ "id": id, "type": "model", "model": "box", "position": at, "rotation": turn });
    let card = |id: &str, z: f32| {
        json!({ "id": id, "type": "solid", "color": "#40ff40", "size": [20, 20], "depth_test": true,
            "transform": { "position": [32, 32], "z": z } })
    };
    let doc = json!({ "schema": SCHEMA, "canvas": { "size": [64, 64] },
        "assets": { "box": { "path": "box.glb" } },
        "layers": [
            { "id": "back", "type": "solid", "color": "#102030" },
            { "id": "world", "type": "group", "space": "3d", "layers": [
                model("a", [0.0, 0.0, 0.0]), card("near", 0.5), model("b", [1.0, 0.0, -2.0]), card("far", -1.0),
                { "id": "flat", "type": "solid", "color": "#ffffff", "size": [8, 8] } ] } ] });
    let mut scene = Scene::from_json(&doc.to_string()).unwrap();
    scene.adopt(&Default::default(), move |_: &str, _: usize| Ok::<_, String>(glb.clone())).unwrap();
    scene
}

#[test]
fn a_3d_group_evaluates_its_steady_frames_without_allocating() {
    let mut runtime = Runtime::new(Arc::new(deep()), Values::new());
    let mut spare = Spare::default();
    let mut counts = Vec::new();
    for _ in 0..12 {
        runtime.advance(1.0 / 30.0);
        let mut frame = None;
        let kept = std::mem::take(&mut spare);
        let made = allocations(|| frame = Some(runtime.frame_in(kept, [0.0, 0.0, 64.0, 64.0], Quality::Q3)));
        let frame = frame.unwrap();
        assert_eq!(frame.stages.len(), 1, "{:?}", frame.stages);
        assert!(frame.stages[0].slices >= 2, "{:?}", frame.stages[0]);
        spare = frame.spare();
        counts.push(made);
    }
    assert!(counts[4..].iter().all(|c| *c == 0), "{counts:?}");
}
