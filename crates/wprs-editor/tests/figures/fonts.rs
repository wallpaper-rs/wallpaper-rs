use super::{act, at, scene};
use serde_json::{Value, json};

fn list(p: &wprs_editor::Project, layer: &str) -> Value {
    wprs_editor::rigs::query(&p.dir, &p.doc, layer, &json!({"font_list": {"layer": layer}}))
}

#[test]
fn the_picker_lists_system_families_package_fonts_and_the_state_of_the_chain() {
    let root = tempfile::tempdir().unwrap();
    let layer = json!({"id": "t", "type": "text", "text": "x", "font": "sans-serif", "fallback": ["Nothing Installed Here"]});
    let mut p = scene(&root, json!([layer]));
    act(&mut p, json!({"set": {"pointer": "/scene/assets/body", "value": {"path": "assets/body.ttf"}}}))
        .unwrap();
    let found = list(&p, "t");
    assert_eq!(found["generic"], json!(["sans-serif", "serif", "monospace"]));
    assert_eq!(found["package"], json!([{"name": "body", "path": "assets/body.ttf"}]));
    assert_eq!(found["most"], 8);
    assert_eq!(
        found["chain"],
        json!([{"name": "sans-serif", "source": "generic"}, {"name": "Nothing Installed Here", "source": "missing"}])
    );
    let names: Vec<&str> = found["system"].as_array().unwrap().iter().filter_map(Value::as_str).collect();
    assert!(names.windows(2).all(|w| w[0].to_lowercase() <= w[1].to_lowercase()));
}

#[test]
fn the_font_and_its_fallback_list_are_set_and_cleared() {
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(&root, json!([{"id": "t", "type": "text", "text": "x"}]));
    act(&mut p, json!({"layer": {"figure": {"fonts": {"layer": "t", "font": "serif", "fallback": ["monospace", "sans-serif"]}}}})).unwrap();
    assert_eq!(at(&p, "/scene/layers/0/font"), json!("serif"));
    assert_eq!(at(&p, "/scene/layers/0/fallback"), json!(["monospace", "sans-serif"]));
    let many: Vec<String> = (0..9).map(|i| format!("f{i}")).collect();
    assert!(act(&mut p, json!({"layer": {"figure": {"fonts": {"layer": "t", "fallback": many}}}})).is_err());
    act(&mut p, json!({"layer": {"figure": {"fonts": {"layer": "t", "font": "", "fallback": []}}}})).unwrap();
    assert_eq!(at(&p, "/scene/layers/0/font"), Value::Null);
    assert_eq!(at(&p, "/scene/layers/0/fallback"), Value::Null);
    let shape = json!({"id": "s", "type": "shape", "shape": {"rect": {"size": [10, 10]}}});
    let mut q = scene(&root, json!([shape]));
    assert!(act(&mut q, json!({"layer": {"figure": {"fonts": {"layer": "s", "font": "serif"}}}})).is_err());
}
