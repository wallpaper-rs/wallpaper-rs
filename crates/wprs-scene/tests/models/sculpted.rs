use super::*;

fn adopted(shapes: Value, shape: Vec<u8>) -> Result<Scene, SceneError> {
    let glb = ForgeRig::new(2.0, 0.5).glb();
    let doc = json!({ "schema": SCHEMA, "canvas": { "size": [200, 100] },
        "assets": { "toy": { "path": "toy.glb", "shapes": shapes } },
        "layers": [model("m", json!({}))] });
    let mut scene = Scene::from_json(&doc.to_string()).unwrap();
    let read = move |path: &str, _: usize| {
        Ok::<_, String>(if path.ends_with(".glb") { glb.clone() } else { shape.clone() })
    };
    scene.adopt(&BTreeMap::new(), read).map(|_| scene)
}

#[test]
fn a_sculpted_shape_reaches_a_model_that_sets_no_weights() {
    let carved: Vec<Carved> = (0..36)
        .map(|v| Carved { node: 2, primitive: 0, vertex: v, at: [0.5, 0.0, 0.0], normal: [0.0; 3] })
        .collect();
    let scene = adopted(json!({ "smile": "smile.wpsh" }), carve_encode(&carved)).unwrap();
    let rig = scene.solids["toy"].rig.as_ref().expect("a sculpted model is posed");
    assert!(rig.shapes.iter().any(|s| s == "smile"));
}

#[test]
fn sculpted_bytes_count_in_the_mesh_budget() {
    let shapes = json!({ "a": "a.wpsh", "b": "b.wpsh" });
    let big = vec![0u8; MAX_RIG_BYTES / 2 + 1];
    let error = adopted(shapes, big).unwrap_err().to_string();
    assert!(error.contains("MiB of meshes"), "{error}");
}
