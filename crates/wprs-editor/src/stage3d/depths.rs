use serde_json::{Value, json};

fn inside(doc: &Value, pointer: &str) -> bool {
    let mut at = pointer;
    while let Some((up, _)) = at.rsplit_once("/layers/") {
        if doc.pointer(up).is_some_and(|g| g["space"] == "3d") {
            return true;
        }
        at = up;
    }
    false
}

pub fn depth(doc: &Value, q: &Value) -> Value {
    let id = q["layer"].as_str().unwrap_or_default();
    let Ok((pointer, found)) = crate::actions::layer(doc, id) else { return Value::Null };
    let Some(scene) = crate::seats::parsed(&doc["scene"]) else { return Value::Null };
    let Some(layer) = scene.layer(id) else { return Value::Null };
    let particles = found["type"] == "particles";
    let why = particles.then(|| wprs_scene::particle_plane(&scene, layer)).flatten();
    let way = if particles && why.is_none() { "one_by_one" } else { "plane" };
    json!({ "layer": id, "inside": inside(doc, &pointer), "depth_test": found["depth_test"] == true,
        "particles": particles, "way": way, "reason": why, "lowest": "plane" })
}
