use serde_json::{Value, json};
use std::fs;
use wprs_editor::Project;
use wprs_editor::actions::Action;
use wprs_editor::outline::outline;
use wprs_editor::starters::starter;

const WGSL: &str = "override soft: bool = false;
@fragment fn fs_glow(in: VertexOut) -> @location(0) vec4<f32> {
    return tap(in.uv) * select(param(0u).x, 1.0, soft);
}";

fn descriptor(id: &str) -> Value {
    json!({ "id": id, "name": { "en": "Glow", "es": "Brillo" }, "cost": { "taps": 1 }, "passes": [{ "entry": "fs_glow" }],
        "params": [{ "id": "gain", "type": "float", "default": 1, "min": 0, "max": 4,
            "label": { "en": "Gain", "es": "Ganancia" } }] })
}

#[test]
fn package_effects_are_removed_unless_a_layer_still_uses_them() {
    let root = tempfile::tempdir().unwrap();
    unsafe { std::env::set_var("XDG_STATE_HOME", root.path().join("state")) };
    let dir = root.path().join("glow");
    let mut project =
        Project::create(&dir, starter("empty", "org.example.glow", "Glow", "en").unwrap()).unwrap();
    let source = root.path().join("source");
    fs::create_dir_all(&source).unwrap();
    fs::write(source.join("glow.json"), descriptor("glow").to_string()).unwrap();
    fs::write(source.join("glow.wgsl"), WGSL).unwrap();
    project.act(&Action::ImportEffect { descriptor: source.join("glow.json"), wgsl: None }).unwrap();
    let listed = outline(&project, 0.0, &[]);
    let package_effects = listed["package_effects"].as_array().unwrap();
    assert!(package_effects.iter().any(|e| e["id"] == "glow" && e["size"].as_u64().unwrap() > 0));
    let remove = |p: &mut Project, id: &str| p.act(&Action::RemoveEffect { effect: id.to_owned() });
    assert!(remove(&mut project, "missing").unwrap_err().to_string().contains("is no effect"));
    let layer = json!({"add_layer": {"kind": "solid", "id": "fill"}});
    project.act(&serde_json::from_value::<Action>(layer).unwrap()).unwrap();
    let pointer = wprs_editor::layers::find(&project.doc, "fill").unwrap();
    let effect = json!({"add": {"pointer": format!("{pointer}/effects"), "value": [{ "effect": "glow" }]}});
    project.act(&serde_json::from_value::<Action>(effect).unwrap()).unwrap();
    let used = remove(&mut project, "glow").unwrap_err().to_string();
    assert!(used.contains("still used by fill"), "{used}");
    let drop = json!({"remove": {"pointer": format!("{pointer}/effects/0")}});
    project.act(&serde_json::from_value::<Action>(drop).unwrap()).unwrap();
    remove(&mut project, "glow").unwrap();
    assert!(project.doc["scene"]["effects"].get("glow").is_none());
    assert!(dir.join("effects/glow.json").is_file(), "prune, not removal, clears the files");
}
