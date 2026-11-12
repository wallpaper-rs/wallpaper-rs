use serde_json::json;
use wprs_editor::keys::value_at;

#[test]
fn emitter_keyframes_blend_each_emitter_of_the_same_fields() {
    let a = json!({"keys": [[0, [{"from": [0, 1], "angle": -90, "strength": 2}]],
        [2, [{"from": [1, 0], "angle": 0, "strength": 6}]]]});
    assert_eq!(value_at(&a, 1.0), json!([{"from": [0.5, 0.5], "angle": -45.0, "strength": 4.0}]));
    let b = json!({"keys": [[0, [{"from": [0, 1], "strength": 2}]], [2, [{"from": [1, 0], "to": [1, 1], "strength": 6}]]]});
    assert_eq!(value_at(&b, 1.0), json!([{"from": [0.0, 1.0], "angle": -90.0, "strength": 2.0}]));
}
