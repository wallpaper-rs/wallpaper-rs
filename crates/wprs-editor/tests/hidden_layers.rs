use serde_json::{Value, json};
use wprs_editor::actions::{Action, plan};
use wprs_editor::check::check;
use wprs_editor::ops::apply;
use wprs_editor::starters::starter;

fn act(doc: &mut Value, action: Value) {
    let action: Action = serde_json::from_value(action).unwrap();
    apply(doc, &plan(doc, &action).unwrap()).unwrap();
}

fn warned(doc: &Value) -> Vec<(String, String)> {
    let all = check(doc);
    assert!(all.iter().all(|p| p.warning || !p.message.contains("never shows")), "{all:?}");
    all.into_iter()
        .filter(|p| p.warning && p.message.contains("never shows"))
        .map(|p| (p.pointer, p.message))
        .collect()
}

#[test]
fn a_layer_that_covers_the_canvas_names_what_it_hides() {
    let mut doc = starter("parallax", "org.example.test", "Test", "en").unwrap();
    assert_eq!(warned(&doc), []);
    act(&mut doc, json!({"add_layer": {"kind": "solid", "id": "wall"}}));
    act(&mut doc, json!({"set": {"pointer": "/scene/layers/5/color", "value": "#223344"}}));
    let found = warned(&doc);
    let pointers: Vec<&str> = found.iter().map(|(p, _)| p.as_str()).collect();
    assert_eq!(pointers, ["/scene/layers/0", "/scene/layers/1", "/scene/layers/2", "/scene/layers/3", "/scene/layers/4"]);
    assert!(found[0].1.contains("\"sky\"") && found[0].1.contains("\"wall\""), "{found:?}");
}

#[test]
fn a_cover_that_lets_light_through_hides_nothing() {
    let mut doc = starter("parallax", "org.example.test", "Test", "en").unwrap();
    act(&mut doc, json!({"add_layer": {"kind": "solid", "id": "wall"}}));
    let wall = "/scene/layers/5";
    act(&mut doc, json!({"set": {"pointer": format!("{wall}/color"), "value": "#223344"}}));
    for (key, value) in [
        ("opacity", json!(0.5)),
        ("blend", json!("multiply")),
        ("visible", json!(false)),
        ("color", json!("#22334480")),
        ("size", json!([800, 600])),
    ] {
        let mut other = doc.clone();
        act(&mut other, json!({"set": {"pointer": format!("{wall}/{key}"), "value": value}}));
        assert_eq!(warned(&other), [], "{key}");
    }
}

#[test]
fn a_shape_wholly_outside_the_canvas_is_named() {
    let mut doc = starter("empty", "org.example.test", "Test", "en").unwrap();
    act(&mut doc, json!({"add_layer": {"kind": "shape", "id": "lost"}}));
    act(&mut doc, json!({"set": {"pointer": "/scene/layers/0/transform", "value": {"position": [-400, -300]}}}));
    let size = json!({"rect": {"size": [100, 100]}});
    act(&mut doc, json!({"set": {"pointer": "/scene/layers/0/shape", "value": size}}));
    let found = warned_off(&doc);
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(found[0].contains("\"lost\""), "{found:?}");
    act(&mut doc, json!({"set": {"pointer": "/scene/layers/0/transform", "value": {"position": [-20, 40]}}}));
    assert_eq!(warned_off(&doc), Vec::<String>::new());
}

fn warned_off(doc: &Value) -> Vec<String> {
    let all = check(doc);
    all.into_iter().filter(|p| p.warning && p.message.contains("outside the canvas")).map(|p| p.message).collect()
}
