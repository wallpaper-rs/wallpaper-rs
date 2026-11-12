use super::*;

fn sparks(depth: f32, render: Value) -> Value {
    json!({ "particles": { "s": { "capacity": 4, "seed": 1, "perspective": { "focal": 100 },
        "emitters": [{ "shape": { "point": [32, 32] }, "bursts": [{ "count": 1 }] }], "render": render,
        "init": { "life": [50, 50], "speed": [0, 0], "size": [20, 20], "color": ["#00ff00", "#00ff00"], "depth": [depth, depth] } } } })
}

pub fn group(particles: Value) -> Value {
    let cube = json!({ "id": "cube", "type": "model", "model": "red", "position": [0, 0, 0] });
    json!([{ "id": "world", "type": "group", "space": "3d", "layers": [cube, particles] }])
}

pub fn red() -> BTreeMap<&'static str, Vec<u8>> {
    BTreeMap::from([("red", cube(unlit("paint", [1.0, 0.0, 0.0, 1.0])))])
}

fn tested(lighting: Option<Value>) -> Value {
    let mut layer = json!({ "id": "p", "type": "particles", "system": "s", "depth_test": true });
    if let Some(l) = lighting {
        layer["lighting"] = l;
    }
    layer
}

pub fn red_at(px: &[u8]) -> bool {
    near(rgb(px, 32, 32), [255, 0, 0], 8)
}

#[test]
fn lit_particles_in_a_3d_group_hide_one_by_one_behind_the_models_in_front_of_them() {
    let Some(r) = renderer() else { return };
    let lights = json!({ "id": "sun", "type": "light", "light": "point", "intensity": 2, "radius": 400, "z": 40,
        "transform": { "position": [32, 32] } });
    for (render, lighting) in [(json!({ "lit": true }), None), (json!({}), Some(json!({ "strength": 1 })))] {
        let layer = tested(lighting);
        let mut shown = Vec::new();
        for (depth, quality) in [(-50.0, Quality::Q3), (50.0, Quality::Q3), (-50.0, Quality::Q0)] {
            let mut layers = group(layer.clone());
            layers.as_array_mut().unwrap().insert(0, lights.clone());
            let mut shot = shoot(r, scene(sparks(depth, render.clone()), layers, red()), &[]);
            let px = (0..3).map(|_| shot.frame(r, 0.05, quality)).last().unwrap();
            shown.push((red_at(&px), rgb(&px, 32, 32)));
        }
        assert!(!shown[0].0, "{render}: a nearer lit particle shows: {shown:?}");
        assert!(shown[1].0, "{render}: a farther lit particle hides: {shown:?}");
        assert!(shown[2].0, "{render}: the lowest quality keeps the plane: {shown:?}");
    }
}

const SAME: &str = "fn particle(p: Sprite) -> Sprite {\n    return p;\n}\n";

fn owned(depth: f32) -> Scene {
    let mut extra = sparks(depth, json!({ "shader": "same" }));
    extra["effects"] = json!({ "same": { "descriptor": "effects/same.json", "wgsl": "effects/same.wgsl" } });
    let layer = json!({ "id": "p", "type": "particles", "system": "s", "depth_test": true });
    let doc = json!({ "schema": SCHEMA, "canvas": { "size": [SIDE, SIDE], "background": "#000000" },
        "assets": { "red": { "path": "red.glb" } }, "layers": group(layer), "particles": extra["particles"],
        "effects": extra["effects"] });
    let mut scene = Scene::from_json(&doc.to_string()).unwrap();
    let glb = cube(unlit("paint", [1.0, 0.0, 0.0, 1.0]));
    let files = move |path: &str, _: usize| match path {
        "red.glb" => Ok(glb.clone()),
        "effects/same.json" => Ok(br#"{ "id": "same", "stage": "particle", "cost": {} }"#.to_vec()),
        "effects/same.wgsl" => Ok(SAME.as_bytes().to_vec()),
        _ => Err(format!("no {path}")),
    };
    scene.adopt(&BTreeMap::new(), files).unwrap();
    scene
}

#[test]
fn particles_with_a_shader_of_their_own_in_a_3d_group_hide_one_by_one() {
    let Some(r) = renderer() else { return };
    let mut shown = Vec::new();
    for (depth, quality) in [(-50.0, Quality::Q3), (50.0, Quality::Q3), (-50.0, Quality::Q0)] {
        let mut shot = shoot(r, owned(depth), &[]);
        let px = (0..12).map(|_| shot.frame(r, 0.05, quality)).last().unwrap();
        shown.push(rgb(&px, 32, 32));
        assert!(shot.view.left_out().is_empty(), "{:?}", shot.view.left_out());
    }
    assert!(near(shown[0], [0, 255, 0], 8), "a nearer particle shows: {shown:?}");
    assert!(near(shown[1], [255, 0, 0], 8), "a farther particle hides: {shown:?}");
    assert!(near(shown[2], [255, 0, 0], 8), "the lowest quality keeps the plane: {shown:?}");
}
