use serde_json::{Value, json};
use wprs_editor::actions::{Action, plan};
use wprs_editor::inspect::fields;
use wprs_editor::ops::apply;
use wprs_editor::starters::starter;

fn act(doc: &mut Value, action: Value) {
    let action: Action = serde_json::from_value(action).unwrap();
    apply(doc, &plan(doc, &action).unwrap()).unwrap();
}

fn with_shape() -> Value {
    let mut doc = starter("empty", "org.example.test", "Test", "en").unwrap();
    act(&mut doc, json!({"add_layer": {"kind": "shape"}}));
    doc
}

#[test]
fn a_first_keyframe_on_an_untouched_position_takes_the_resting_value() {
    let mut doc = with_shape();
    let position = "/scene/layers/0/transform/position";
    assert!(doc.pointer(position).is_none());
    act(&mut doc, json!({"keyframe": {"pointer": position, "time": 0}}));
    assert_eq!(doc.pointer(&format!("{position}/keys/0/1")), Some(&json!([960.0, 540.0])));
}

#[test]
fn the_shape_choices_offer_a_size_you_can_see() {
    let doc = with_shape();
    let list = fields(&doc, "/scene/layers/0");
    let shape = list.iter().find(|f| f.pointer == "/scene/layers/0/shape").unwrap();
    for kind in ["ellipse", "ring", "polygon", "star"] {
        let inner = &shape.sample[kind][kind];
        let size = &inner["size"];
        let seen = size.as_f64().is_some_and(|n| n > 0.0)
            || size.as_array().is_some_and(|a| a.iter().all(|n| n.as_f64().unwrap_or(0.0) > 0.0));
        assert!(seen, "{kind}: {inner}");
    }
}
