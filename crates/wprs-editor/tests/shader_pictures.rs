use serde_json::{Value, json};
use std::fs;
use wprs_editor::Project;
use wprs_editor::actions::Action;
use wprs_editor::inspect::fields_with;
use wprs_editor::starters::starter;

const WGSL: &str = "@fragment fn fs_pic(in: VertexOut) -> @location(0) vec4<f32> {
    return tap(in.uv);
}";

fn descriptor() -> Value {
    json!({ "id": "pics", "name": { "en": "Pictures" }, "cost": { "taps": 1 }, "passes": [{ "entry": "fs_pic" }],
        "params": [
            { "id": "at", "type": "position", "default": [0.5, 0.5] },
            { "id": "cover", "type": "mask", "default": "", "channels": 1 },
            { "id": "tex", "type": "texture", "default": "" },
            { "id": "brush", "type": "texture", "default": "", "tool": "gray" }] })
}

fn act(project: &mut Project, action: Value) {
    project.act(&serde_json::from_value::<Action>(action).unwrap()).unwrap();
}

#[test]
fn position_mask_and_texture_params_of_a_package_effect_offer_the_generic_pictures_and_the_canvas_point() {
    let root = tempfile::tempdir().unwrap();
    unsafe { std::env::set_var("XDG_STATE_HOME", root.path().join("state")) };
    let dir = root.path().join("pics");
    let mut project = Project::create(&dir, starter("empty", "org.example.pics", "Pics", "en").unwrap()).unwrap();
    let source = root.path().join("source");
    fs::create_dir_all(&source).unwrap();
    fs::write(source.join("pics.json"), descriptor().to_string()).unwrap();
    fs::write(source.join("pics.wgsl"), WGSL).unwrap();
    act(&mut project, json!({ "import_effect": { "descriptor": source.join("pics.json") } }));
    act(&mut project, json!({"add_layer": {"kind": "solid", "id": "fill"}}));
    let pointer = wprs_editor::layers::find(&project.doc, "fill").unwrap();
    act(&mut project, json!({"add": {"pointer": format!("{pointer}/effects"), "value": [{ "effect": "pics" }]}}));
    let at = format!("{pointer}/effects/0");
    let fields = fields_with(&project.doc, &at, 0.0, &project.effects());
    let field = |key: &str| fields.iter().find(|f| f.key == key).unwrap();
    assert_eq!(field("at").widget, "vec2");
    for (key, widget) in [("cover", "ref:mask"), ("tex", "ref:image"), ("brush", "ref:mask")] {
        let f = field(key);
        assert_eq!(f.widget, widget, "{key}");
        for generic in ["white", "black", "noise", "normal_flat"] {
            assert!(f.options.contains(&json!(generic)), "{key} does not offer {generic}: {:?}", f.options);
        }
    }
    for (key, name) in [("cover", "white"), ("tex", "noise"), ("brush", "black")] {
        act(&mut project, json!({"set": {"pointer": format!("{at}/params/{key}"), "value": name}}));
        assert_eq!(project.doc.pointer(&format!("{at}/params/{key}")), Some(&json!(name)));
    }
    let found = wprs_editor::check::review(&project.doc, Some(&dir));
    assert!(found.iter().all(|p| p.warning), "a generic picture is refused: {found:?}");
    let stroke = json!({"pointer": format!("{at}/params/brush"), "radius": 0.2, "points": [[0.5, 0.5]]});
    act(&mut project, json!({ "paint_mask": stroke }));
    let painted = project.doc.pointer(&format!("{at}/params/brush")).and_then(Value::as_str).unwrap().to_owned();
    assert!(!["white", "black", "noise", "normal_flat"].contains(&painted.as_str()), "{painted}");
    assert!(project.doc["scene"]["assets"][&painted]["path"].is_string());
    let shown = project.ask("fill", &json!({ "effect_handles": { "layer": "fill", "index": 0, "time": 0.0 } }));
    let handles = shown["handles"].as_array().unwrap();
    assert!(handles.iter().any(|h| h["param"] == "at" && h["kind"] == "point"), "{shown}");
}
