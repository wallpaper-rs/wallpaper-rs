use super::{act, at, project, refused};
use serde_json::{Value, json};
use wprs_editor::Project;

const ONE: &str = "/scene/layers/0/opacity";
const TWO: &str = "/scene/layers/1/opacity";

fn keyed(p: &mut Project, pointer: &str, keys: &[(f64, Value)]) {
    for (t, v) in keys {
        act(p, json!({"keyframe": {"pointer": pointer, "time": t, "value": v}})).unwrap();
    }
}

fn values(p: &Project, pointer: &str) -> Vec<Value> {
    at(p, &format!("{pointer}/keys")).as_array().unwrap().iter().map(|k| k[1].clone()).collect()
}

#[test]
fn keys_of_several_values_of_one_kind_lift_and_scale_together() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    act(&mut p, json!({"add_layer": {"kind": "solid", "id": "back"}})).unwrap();
    keyed(&mut p, ONE, &[(0.0, json!(0.2)), (1.0, json!(0.6))]);
    keyed(&mut p, TWO, &[(0.0, json!(0.4)), (2.0, json!(1))]);
    let picks = json!([{"pointer": ONE, "times": [0, 1]}, {"pointer": TWO, "times": [0]}]);
    let before = p.history.done.len();
    act(&mut p, json!({"edit_keys": {"picks": picks, "lift": 0.1}})).unwrap();
    assert_eq!(p.history.done.len(), before + 1);
    assert_eq!(
        (values(&p, ONE), values(&p, TWO)),
        (vec![json!(0.3), json!(0.7)], vec![json!(0.5), json!(1)])
    );
    act(&mut p, json!({"edit_keys": {"picks": picks, "value_scale": 0.5}})).unwrap();
    assert_eq!(values(&p, ONE), [json!(0.4), json!(0.6)], "around the middle of the picked values, 0.5");
    assert_eq!(values(&p, TWO)[0], json!(0.5));
    act(&mut p, json!({"edit_keys": {"picks": picks, "value_scale": 2, "value_pivot": 0}})).unwrap();
    assert_eq!(values(&p, ONE), [json!(0.8), json!(1.2)]);
    let place = "/scene/layers/0/transform/position";
    keyed(&mut p, place, &[(0.0, json!([0, 0])), (1.0, json!([10, 20]))]);
    let mixed = json!([{"pointer": ONE, "times": [0]}, {"pointer": place, "times": [0]}]);
    let unlike = refused(&mut p, json!({"edit_keys": {"picks": mixed, "lift": 1}}));
    assert!(unlike.contains("of one kind and unit"), "{unlike}");
    act(&mut p, json!({"edit_keys": {"picks": mixed, "shift": 0.5}})).unwrap();
}

#[test]
fn scaling_the_values_of_a_part_scales_its_own_handles() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    let place = "/scene/layers/0/transform/position";
    keyed(&mut p, place, &[(0.0, json!([0, 0])), (2.0, json!([10, 20]))]);
    let pick = json!([{"pointer": place, "times": [0]}]);
    act(&mut p, json!({"edit_keys": {"picks": pick, "part": 1, "out_handle": [1, 6]}})).unwrap();
    let both = json!([{"pointer": place, "times": [0, 2]}]);
    act(&mut p, json!({"edit_keys": {"picks": both, "part": 1, "value_scale": 2}})).unwrap();
    assert_eq!(values(&p, place), [json!([0.0, -10.0]), json!([10.0, 30.0])]);
    assert_eq!(at(&p, &format!("{place}/keys/0/2/out")), json!([[0.667, 3.333333], [1.0, 12.0]]));
}
