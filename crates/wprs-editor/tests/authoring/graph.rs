use super::{act, at, project, refused};
use serde_json::{Value, json};
use wprs_editor::Project;

const PLACE: &str = "/scene/layers/0/transform/position";
const COLOR: &str = "/scene/layers/0/color";

fn keyed(p: &mut Project, pointer: &str, keys: &[(f64, Value)]) {
    for (t, v) in keys {
        act(p, json!({"keyframe": {"pointer": pointer, "time": t, "value": v}})).unwrap();
    }
}

fn keys(p: &Project, pointer: &str) -> Vec<Value> {
    at(p, &format!("{pointer}/keys")).as_array().cloned().unwrap_or_default()
}

fn steps(p: &Project) -> usize {
    p.history.done.len()
}

#[test]
fn picked_keys_move_scale_and_lift_together_as_one_step() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    keyed(&mut p, PLACE, &[(0.0, json!([0, 0])), (1.0, json!([10, 20])), (2.0, json!([30, 40]))]);
    keyed(&mut p, COLOR, &[(0.0, json!("#000000")), (2.0, json!("#ffffff"))]);
    act(
        &mut p,
        json!({"edit_keys": {"picks": [{"pointer": PLACE, "times": [1]}], "out_handle": [0.5, 0.2]}}),
    )
    .unwrap();
    let before = steps(&p);
    let picks = json!([{"pointer": PLACE, "times": [1, 2]}, {"pointer": COLOR, "times": [2]}]);
    act(&mut p, json!({"edit_keys": {"picks": picks, "scale": 2, "pivot": 1, "shift": 0.5}})).unwrap();
    assert_eq!(steps(&p), before + 1, "one step for the whole selection");
    let moved = keys(&p, PLACE);
    assert_eq!(moved.iter().map(|k| k[0].as_f64().unwrap()).collect::<Vec<_>>(), [0.0, 1.5, 3.5]);
    assert_eq!(moved[1][2]["out"], json!([1.0, 0.2]), "handles stretch with the times");
    assert_eq!(keys(&p, COLOR)[1][0], json!(3.5));
    let picks = json!([{"pointer": PLACE, "times": [1.5]}]);
    act(&mut p, json!({"edit_keys": {"picks": picks, "part": 1, "lift": 5}})).unwrap();
    assert_eq!(keys(&p, PLACE)[1][1], json!([10.0, 25.0]));
    let picks = json!([{"pointer": COLOR, "times": [3.5]}]);
    act(&mut p, json!({"edit_keys": {"picks": picks, "part": 0, "value": 0}})).unwrap();
    assert_eq!(keys(&p, COLOR)[1][1], json!("#00ffff"));
    let picks = json!([{"pointer": PLACE, "times": [1.5]}]);
    let met = refused(&mut p, json!({"edit_keys": {"picks": picks, "shift": 2}}));
    assert!(met.contains("would meet at 3.5 s"), "{met}");
    let early = refused(&mut p, json!({"edit_keys": {"picks": picks, "shift": -5}}));
    assert!(early.contains("before 0 s"), "{early}");
    let unpart = refused(&mut p, json!({"edit_keys": {"picks": picks, "value": 3}}));
    assert!(unpart.contains("part names the part"), "{unpart}");
    let missing =
        refused(&mut p, json!({"edit_keys": {"picks": [{"pointer": PLACE, "times": [9]}], "shift": 1}}));
    assert!(missing.contains("no keyframe at 9 s"), "{missing}");
    act(&mut p, json!({"edit_keys": {"picks": [{"pointer": PLACE, "times": [0, 3.5]}], "remove": true}}))
        .unwrap();
    assert_eq!(keys(&p, PLACE).len(), 1);
    let last = refused(
        &mut p,
        json!({"edit_keys": {"picks": [{"pointer": PLACE, "times": [1.5]}], "remove": true}}),
    );
    assert!(last.contains("keep at least one key"), "{last}");
}

#[test]
fn moving_a_key_shortens_handles_that_would_pass_their_neighbour() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    let opacity = "/scene/layers/0/opacity";
    keyed(&mut p, opacity, &[(0.0, json!(0)), (2.0, json!(1)), (4.0, json!(0))]);
    act(
        &mut p,
        json!({"edit_keys": {"picks": [{"pointer": opacity, "times": [0]}], "out_handle": [1.5, 0.5]}}),
    )
    .unwrap();
    act(&mut p, json!({"edit_keys": {"picks": [{"pointer": opacity, "times": [2]}], "shift": -1}})).unwrap();
    assert_eq!(keys(&p, opacity)[0][2]["out"], json!([1.0, 0.5]));
}

#[test]
fn aligned_handles_keep_one_slope_and_straighten_is_one_step() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    let opacity = "/scene/layers/0/opacity";
    keyed(&mut p, opacity, &[(0.0, json!(0)), (1.0, json!(0.5)), (3.0, json!(1))]);
    let pick = json!([{"pointer": opacity, "times": [1]}]);
    act(&mut p, json!({"edit_keys": {"picks": pick, "in_handle": [0.5, 0.1]}})).unwrap();
    act(&mut p, json!({"edit_keys": {"picks": pick, "out_handle": [1, 0.4], "aligned": true}})).unwrap();
    let key = &keys(&p, opacity)[1];
    assert_eq!((key[2]["out"].clone(), key[2]["in"].clone()), (json!([1.0, 0.4]), json!([0.5, 0.2])));
    let broken = json!({"edit_keys": {"picks": pick, "in_handle": [0.25, -0.3]}});
    act(&mut p, broken).unwrap();
    assert_eq!(keys(&p, opacity)[1][2]["out"], json!([1.0, 0.4]), "a broken handle leaves the other alone");
    let past = refused(&mut p, json!({"edit_keys": {"picks": pick, "out_handle": [2.5, 0]}}));
    assert!(past.contains("neighbouring keyframe"), "{past}");
    let two = refused(
        &mut p,
        json!({"edit_keys": {"picks": [{"pointer": opacity, "times": [0, 1]}], "out_handle": [0.2, 0]}}),
    );
    assert!(two.contains("one key at a time"), "{two}");
    let before = steps(&p);
    act(&mut p, json!({"edit_keys": {"picks": pick, "straighten": true}})).unwrap();
    assert_eq!(steps(&p), before + 1);
    assert_eq!(keys(&p, opacity)[1][2], json!({"in": [0.25, -0.3]}));
}

#[test]
fn copied_keys_paste_at_a_time_into_their_tracks_and_plain_values() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    let opacity = "/scene/layers/0/opacity";
    keyed(&mut p, opacity, &[(0.0, json!(0)), (1.0, json!(1))]);
    act(&mut p, json!({"set": {"pointer": "/scene/layers/0/transform/rotation", "value": 0}})).unwrap();
    let copied = json!([[0.0, 0.25, {"out": [0.5, 0.1]}], [1.0, 0.75]]);
    let pick = json!([{"pointer": opacity, "keys": copied}, {"pointer": "/scene/layers/0/transform/rotation", "keys": [[0.5, 90]]}]);
    act(&mut p, json!({"edit_keys": {"picks": pick, "at": 1}})).unwrap();
    let pasted: Vec<f64> = keys(&p, opacity).iter().map(|k| k[0].as_f64().unwrap()).collect();
    assert_eq!(pasted, [0.0, 1.0, 2.0]);
    assert_eq!(keys(&p, opacity)[1][1], json!(0.25), "the key at 1 s is replaced");
    assert_eq!(at(&p, "/scene/layers/0/transform/rotation/keys"), json!([[1.5, 90]]));
    let bad = refused(
        &mut p,
        json!({"edit_keys": {"picks": [{"pointer": opacity, "keys": [[0, "#ff0000"]]}], "at": 4}}),
    );
    assert!(bad.contains("give a number"), "{bad}");
    let shapeless =
        refused(&mut p, json!({"edit_keys": {"picks": [{"pointer": opacity, "keys": [[0]]}], "at": 4}}));
    assert!(shapeless.contains("a pasted key is [time, value]"), "{shapeless}");
}

#[test]
fn the_curves_query_splits_keys_into_parts_with_handles_and_the_sampled_curve() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    keyed(&mut p, PLACE, &[(0.0, json!([0, 0])), (2.0, json!([100, 50]))]);
    act(&mut p, json!({"edit_keys": {"picks": [{"pointer": PLACE, "times": [0]}], "out_handle": [1, 0.5]}}))
        .unwrap();
    let curves = wprs_editor::graph::curves(&p.doc, &json!({"pointer": PLACE, "samples": 5}));
    assert_eq!(curves["list"], json!(format!("{PLACE}/keys")));
    let tracks = curves["tracks"].as_array().unwrap();
    assert_eq!(tracks.iter().map(|t| t["name"].clone()).collect::<Vec<_>>(), [json!("x"), json!("y")]);
    let middle = &tracks[1]["points"][2];
    let shown = wprs_editor::keys::value_at(&at(&p, PLACE), 1.0);
    assert_eq!(middle[1], shown[1], "the curve is the sampled value");
    let out = &curves["keys"][0]["out"];
    assert_eq!(
        (out["set"].clone(), out["share"].clone(), out["at"][0].clone()),
        (json!(true), json!(true), json!([1.0, 50.0]))
    );
    assert_eq!(curves["keys"][1]["in"]["set"], json!(false));
    let back = curves["keys"][1]["in"]["at"][0][1].as_f64().unwrap();
    assert!((back - 66.6667).abs() < 1e-3, "an in handle points back: {back}");
    assert!(
        wprs_editor::graph::curves(&p.doc, &json!({"pointer": "/scene/layers/0/opacity"}))["error"]
            .is_string()
    );
}
