use super::*;

#[test]
fn unsaved_text_is_checked_as_readers_do_with_its_line_and_column() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    shader(&mut p, json!({ "new": { "name": "tone", "template": "color" } })).unwrap();
    let broken =
        "fn color_tone(color: vec4<f32>, uv: vec2<f32>) -> vec4<f32> {\n    return color *  nope;\n}\n";
    let found = ask(&p, json!({ "shader_check": { "effect": "tone", "wgsl": broken } }));
    assert_eq!((found["ok"].clone(), found["line"].clone()), (json!(false), json!(2)), "{found}");
    assert!(found["column"].as_u64().unwrap() > 1, "{found}");
    let saved = std::fs::read_to_string(p.dir.join("effects/tone.wgsl")).unwrap();
    assert!(!saved.contains("nope"), "a check writes nothing");
    let big = "a".repeat(70 << 10);
    let huge = ask(&p, json!({ "shader_check": { "effect": "tone", "wgsl": big } }));
    assert!(huge["error"].as_str().unwrap().contains("64 KiB"), "{huge}");
    let looped = saved.replace("return", "loop { }\n    return");
    let endless =
        wprs_editor::wgsl::check("tone", &serde_json::to_string(&descriptor(&p, "tone")).unwrap(), &looped);
    assert_eq!(endless["ok"], false, "{endless}");
    let material =
        "fn material(s: Surface) -> Surface {\n    var o = s;\n    o.albedo = tap(s.uv);\n    return o;\n}\n";
    shader(&mut p, json!({ "new": { "name": "coat", "template": "material" } })).unwrap();
    let wrong = ask(&p, json!({ "shader_check": { "effect": "coat", "wgsl": material } }));
    assert_eq!(wrong["line"], 3, "{wrong}");
    assert!(wrong["error"].as_str().unwrap().contains("material shader"), "{wrong}");
}

#[test]
fn written_shaders_are_undone_and_kept_within_their_limits() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    shader(&mut p, json!({ "new": { "name": "tone", "template": "color" } })).unwrap();
    let path = p.dir.join("effects/tone.wgsl");
    let first = std::fs::read_to_string(&path).unwrap();
    let second = first.replace("param(0u).x", "param(0u).x * 0.5");
    shader(&mut p, json!({ "write": { "effect": "tone", "wgsl": second } })).unwrap();
    assert_eq!(std::fs::read_to_string(&path).unwrap(), second);
    act(&mut p, json!({ "undo": {} })).unwrap();
    assert_eq!(std::fs::read_to_string(&path).unwrap(), first);
    act(&mut p, json!({ "redo": {} })).unwrap();
    assert_eq!(std::fs::read_to_string(&path).unwrap(), second);
    let big = "a".repeat(65 << 10);
    assert!(shader(&mut p, json!({ "write": { "effect": "tone", "wgsl": big } })).is_err());
    let mut d = descriptor(&p, "tone");
    d["id"] = json!("other");
    assert!(shader(&mut p, json!({ "write": { "effect": "tone", "descriptor": d } })).is_err());
    let unknown = json!({ "id": "tone", "cost": {}, "secret": 1 });
    assert!(shader(&mut p, json!({ "write": { "effect": "tone", "descriptor": unknown } })).is_err());
    assert!(shader(&mut p, json!({ "write": { "effect": "nothing", "wgsl": "" } })).is_err());
}
