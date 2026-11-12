use super::{act, at, project, refused};
use serde_json::{Value, json};
use wprs_editor::Project;
use wprs_scene::{Key, Vec2, sample};

const PLACE: &str = "/scene/layers/0/transform/position";

fn keyed(p: &mut Project, keys: &[(f64, Value)]) {
    for (t, v) in keys {
        act(p, json!({"keyframe": {"pointer": PLACE, "time": t, "value": v}})).unwrap();
    }
}

fn third(p: &Project, i: usize) -> Value {
    at(p, &format!("{PLACE}/keys/{i}/2"))
}

fn pick(t: f64) -> Value {
    json!([{"pointer": PLACE, "times": [t]}])
}

#[test]
fn a_handle_of_one_part_bends_that_track_alone() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    keyed(&mut p, &[(0.0, json!([0, 0])), (2.0, json!([40, 0])), (4.0, json!([0, 20]))]);
    act(&mut p, json!({"edit_keys": {"picks": pick(2.0), "part": 1, "out_handle": [0.5, 30]}})).unwrap();
    assert_eq!(
        third(&p, 1),
        json!({"out": [[0.667, -13.333333], [0.5, 30.0]]}),
        "the other part keeps its curve"
    );
    let keys: Vec<Key<Vec2>> = serde_json::from_value(at(&p, &format!("{PLACE}/keys"))).unwrap();
    let curves = wprs_editor::graph::curves(&p.doc, &json!({"pointer": PLACE, "samples": 9}));
    for (track, part) in [(0, 0), (1, 1)] {
        for point in curves["tracks"][track]["points"].as_array().unwrap() {
            let t = point[0].as_f64().unwrap() as f32;
            let drawn = f64::from(sample(&keys, t).0[part]);
            assert!((point[1].as_f64().unwrap() - drawn).abs() < 1e-3, "part {part} at {t}: {point} {drawn}");
        }
    }
    let out = &curves["keys"][1]["out"];
    assert_eq!((out["split"].clone(), out["at"][1].clone()), (json!(true), json!([2.5, 30.0])));
    assert_eq!(out["at"][0], json!([2.667, 26.666667]));
}

#[test]
fn aligned_part_handles_turn_the_same_part_on_the_other_side() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    keyed(&mut p, &[(0.0, json!([0, 0])), (1.0, json!([10, 10])), (3.0, json!([20, 0]))]);
    act(&mut p, json!({"edit_keys": {"picks": pick(1.0), "part": 0, "out_handle": [1, 4], "aligned": true}}))
        .unwrap();
    let both = third(&p, 1);
    assert_eq!(both["out"], json!([[1.0, 4.0], [0.667, -3.333333]]));
    assert_eq!(both["in"], json!([[0.333, 1.333333], [0.333, 3.333333]]));
    let refused_part =
        refused(&mut p, json!({"edit_keys": {"picks": pick(1.0), "part": 5, "out_handle": [1, 4]}}));
    assert!(refused_part.contains("no part 5"), "{refused_part}");
    let lonely = refused(&mut p, json!({"edit_keys": {"picks": pick(3.0), "part": 0, "out_handle": [1, 4]}}));
    assert!(lonely.contains("no neighbour"), "{lonely}");
}

#[test]
fn linking_the_parts_keeps_the_shape_of_one_part_for_all() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    keyed(&mut p, &[(0.0, json!([0, 0])), (2.0, json!([40, 20]))]);
    act(&mut p, json!({"edit_keys": {"picks": pick(0.0), "part": 1, "out_handle": [1, 10]}})).unwrap();
    act(&mut p, json!({"edit_keys": {"picks": pick(0.0), "part": 1, "link": true}})).unwrap();
    assert_eq!(third(&p, 0), json!({"out": [1.0, 0.5]}), "a share of the way taken from y");
    act(&mut p, json!({"edit_keys": {"picks": pick(0.0), "part": 0, "out_handle": [1, 10]}})).unwrap();
    act(&mut p, json!({"edit_keys": {"picks": pick(0.0), "out_handle": [0.5, 0.2]}})).unwrap();
    assert_eq!(third(&p, 0), json!({"out": [0.5, 0.2]}), "a handle for every part links them again");
    let mixed =
        refused(&mut p, json!({"edit_keys": {"picks": pick(0.0), "link": true, "out_handle": [1, 1]}}));
    assert!(mixed.contains("on its own"), "{mixed}");
    act(&mut p, json!({"edit_keys": {"picks": pick(0.0), "shift": 0.5}})).unwrap();
    act(
        &mut p,
        json!({"edit_keys": {"picks": [{"pointer": PLACE, "times": [0.5, 2]}], "scale": 0.5, "pivot": 0.5}}),
    )
    .unwrap();
    assert_eq!(third(&p, 0), json!({"out": [0.25, 0.2]}));
}

#[test]
fn split_handles_stretch_and_shorten_part_by_part() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    keyed(&mut p, &[(0.0, json!([0, 0])), (2.0, json!([40, 20])), (4.0, json!([0, 0]))]);
    act(&mut p, json!({"edit_keys": {"picks": pick(0.0), "part": 1, "out_handle": [1.5, 10]}})).unwrap();
    act(&mut p, json!({"edit_keys": {"picks": [{"pointer": PLACE, "times": [0, 2]}], "scale": 0.5}}))
        .unwrap();
    assert_eq!(third(&p, 0), json!({"out": [[0.334, 13.333333], [0.75, 10.0]]}));
    act(&mut p, json!({"edit_keys": {"picks": pick(1.0), "shift": -0.5}})).unwrap();
    assert_eq!(third(&p, 0), json!({"out": [[0.334, 13.333333], [0.5, 10.0]]}), "the long one is cut");
    act(&mut p, json!({"keyframe": {"pointer": PLACE, "time": 0, "out_handle": [0.2, 0.1]}})).unwrap();
    assert_eq!(third(&p, 0), json!({"out": [0.2, 0.1]}));
}
