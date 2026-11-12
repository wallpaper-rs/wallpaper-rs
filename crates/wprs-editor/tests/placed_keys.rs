use serde_json::{Value, json};
use wprs_editor::actions::{Action, plan};
use wprs_editor::inspect::fields_at;
use wprs_editor::ops::apply;
use wprs_editor::starters::starter;

fn act(doc: &mut Value, action: Value) {
    let action: Action = serde_json::from_value(action).unwrap();
    apply(doc, &plan(doc, &action).unwrap()).unwrap();
}

#[test]
fn a_key_placed_near_a_whole_second_shows_its_own_value_in_the_field() {
    let mut doc = starter("clock", "org.example.test", "Test", "en").unwrap();
    let rotation = "/scene/layers/1/transform/rotation";
    act(&mut doc, json!({"keyframe": {"pointer": rotation, "time": 0, "value": 0}}));
    act(&mut doc, json!({"keyframe": {"pointer": rotation, "time": 7.99999, "value": 360}}));
    let keys = doc.pointer(&format!("{rotation}/keys")).unwrap().clone();
    assert_eq!(keys[1], json!([8.0, 360]));
    for time in [7.99999, 8.0, 8.00004] {
        let shown = fields_at(&doc, "/scene/layers/1", time)
            .into_iter()
            .find(|f| f.pointer == rotation)
            .map(|f| f.shown);
        assert_eq!(shown, Some(json!(360)), "at {time}");
    }
}
