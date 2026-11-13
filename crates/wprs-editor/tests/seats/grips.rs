use super::{act, card, near, scene, seat};
use serde_json::{Value, json};

fn nested() -> Value {
    json!([
        card("base", json!({"position": [400, 500], "rotation": 30, "scale": [1.5, 0.75], "skew": [12, 0]})),
        {"id": "kid", "type": "solid", "color": "#ff0000", "size": [120, 80], "parent": "base",
            "transform": {"position": [60, 20], "origin": [0.2, 0.3], "rotation": -20, "scale": [1.2, 0.9], "skew": [8, -6]}}
    ])
}

fn grip(p: &mut wprs_editor::Project, grip: Value, to: [f64; 2]) {
    act(p, json!({"layer": {"grip": {"layer": "kid", "grip": grip, "to": to}}})).unwrap();
}

#[test]
fn moving_puts_the_pivot_under_the_pointer_inside_a_parent() {
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(&root, nested());
    grip(&mut p, json!("move"), [610.0, 333.0]);
    assert!(near(seat(&p, "kid").pivot(), [610.0, 333.0], 0.05), "{:?}", seat(&p, "kid").pivot());
    let before = seat(&p, "kid").pivot();
    grip(&mut p, json!("nudge"), [0.0, -10.0]);
    assert!(near(seat(&p, "kid").pivot(), [before[0], before[1] - 10.0], 0.05));
}

#[test]
fn a_corner_follows_the_pointer_and_the_far_corner_stays() {
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(&root, nested());
    let pinned = seat(&p, "kid").flat([0.2, 0.3]);
    let goal = {
        let c = seat(&p, "kid").flat([1.0, 1.0]);
        [c[0] + 25.0, c[1] + 40.0]
    };
    grip(&mut p, json!({"scale": {"corner": [1, 1]}}), goal);
    let kid = seat(&p, "kid");
    assert!(near(kid.flat([1.0, 1.0]), goal, 0.05), "{:?} {goal:?}", kid.flat([1.0, 1.0]));
    assert!(near(kid.flat([0.2, 0.3]), pinned, 0.05));
    let even = kid.flat([0.0, 0.0]);
    let scale = |p: &wprs_editor::Project| p.doc.pointer("/scene/layers/1/transform/scale").cloned().unwrap();
    let before = scale(&p);
    grip(&mut p, json!({"scale": {"corner": [0, 0], "even": true}}), [even[0] - 30.0, even[1] + 5.0]);
    let after = scale(&p);
    let ratio = |v: &Value| v[0].as_f64().unwrap() / v[1].as_f64().unwrap();
    assert!((ratio(&before) - ratio(&after)).abs() < 1e-3, "{before} {after}");
}

#[test]
fn an_edge_skews_the_layer_onto_the_pointer() {
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(&root, nested());
    let kid = seat(&p, "kid");
    let edge = kid.flat([1.0, 0.5]);
    let (sin, cos) = (-20f64).to_radians().sin_cos();
    let (zero, step) = (kid.canvas([0.0, 0.0]), kid.canvas([-sin * 30.0, cos * 30.0]));
    let goal = [edge[0] + step[0] - zero[0], edge[1] + step[1] - zero[1]];
    grip(&mut p, json!({"skew": {"edge": [1, 0.5]}}), goal);
    let now = seat(&p, "kid").flat([1.0, 0.5]);
    assert!(near(now, goal, 0.05), "{now:?} {goal:?}");
    let skew = p.doc.pointer("/scene/layers/1/transform/skew/0").and_then(Value::as_f64).unwrap();
    assert!((skew - 8.0).abs() < 1e-6, "the other skew changed to {skew}");
}

#[test]
fn moving_the_origin_keeps_the_layer_in_place() {
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(&root, nested());
    let outline = seat(&p, "kid").outline();
    let to = seat(&p, "kid").flat([0.9, 0.1]);
    grip(&mut p, json!("origin"), to);
    let kid = seat(&p, "kid");
    assert!(near(kid.pivot(), to, 0.05), "{:?} {to:?}", kid.pivot());
    assert!(kid.origin.iter().zip([0.9, 0.1]).all(|(a, b)| (a - b).abs() < 1e-3), "{:?}", kid.origin);
    assert!(
        kid.outline().iter().zip(outline).all(|(a, b)| near(*a, b, 0.05)),
        "{:?} {outline:?}",
        kid.outline()
    );
}

#[test]
fn turning_points_the_top_at_the_pointer() {
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(&root, json!([card("kid", json!({"position": [500, 500]}))]));
    grip(&mut p, json!("turn"), [600.0, 500.0]);
    let r = p.doc.pointer("/scene/layers/0/transform/rotation").and_then(Value::as_f64).unwrap();
    assert!((r - 90.0).abs() < 1e-6, "{r}");
}

#[test]
fn a_moved_layer_snaps_to_the_middle_and_to_other_layers() {
    let root = tempfile::tempdir().unwrap();
    let layers =
        json!([card("kid", json!({"position": [300, 300]})), card("other", json!({"position": [700, 200]}))]);
    let mut p = scene(&root, layers);
    let snap = |p: &mut wprs_editor::Project, to: [f64; 2]| {
        act(p, json!({"layer": {"grip": {"layer": "kid", "grip": "move", "to": to, "snap": 6}}})).unwrap();
        seat(p, "kid").pivot()
    };
    assert!(near(snap(&mut p, [496.0, 403.0]), [500.0, 403.0], 1e-3));
    assert!(near(snap(&mut p, [404.0, 197.0]), [400.0, 200.0], 1e-3), "{:?}", seat(&p, "kid").pivot());
    assert!(near(snap(&mut p, [440.0, 260.0]), [440.0, 260.0], 1e-3));
}

#[test]
fn fitting_fills_or_holds_the_canvas() {
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(&root, json!([card("kid", json!({"position": [100, 100], "origin": [0, 0]}))]));
    let fit = |p: &mut wprs_editor::Project, how: &str| {
        act(p, json!({"layer": {"fit": {"layer": "kid", "fit": how}}})).unwrap();
        seat(p, "kid").outline()
    };
    let cover = fit(&mut p, "cover");
    assert!(near(cover[0], [-500.0, 0.0], 0.05) && near(cover[2], [1500.0, 1000.0], 0.05), "{cover:?}");
    let contain = fit(&mut p, "contain");
    assert!(near(contain[0], [0.0, 250.0], 0.05) && near(contain[2], [1000.0, 750.0], 0.05), "{contain:?}");
    let stretch = fit(&mut p, "stretch");
    assert!(near(stretch[0], [0.0, 0.0], 0.05) && near(stretch[2], [1000.0, 1000.0], 0.05), "{stretch:?}");
}

#[test]
fn fitting_a_child_of_a_turned_parent_still_covers_the_canvas() {
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(&root, nested());
    act(&mut p, json!({"layer": {"fit": {"layer": "kid", "fit": "stretch"}}})).unwrap();
    let outline = seat(&p, "kid").outline();
    let xs: Vec<f64> = outline.iter().map(|c| c[0]).collect();
    let ys: Vec<f64> = outline.iter().map(|c| c[1]).collect();
    let span =
        |v: &[f64]| v.iter().cloned().fold(f64::MIN, f64::max) - v.iter().cloned().fold(f64::MAX, f64::min);
    assert!(span(&xs) >= 999.0 && span(&ys) >= 999.0, "{outline:?}");
    let middle = seat(&p, "kid").flat([0.5, 0.5]);
    assert!(near(middle, [500.0, 500.0], 0.05), "{middle:?}");
}

fn leaning() -> Value {
    json!([card("kid", json!({"position": [500, 450], "tilt": [25, -30], "z": 120, "rotation": 15}))])
}

#[test]
fn scaling_and_turning_a_tilted_layer_follow_the_pointer() {
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(&root, leaning());
    let goal = {
        let c = seat(&p, "kid").at([1.0, 1.0]);
        [c[0] + 30.0, c[1] + 20.0]
    };
    grip(&mut p, json!({"scale": {"corner": [1, 1]}}), goal);
    let reached = seat(&p, "kid").at([1.0, 1.0]);
    assert!(near(reached, goal, 0.5), "{reached:?} {goal:?}");
    let top = seat(&p, "kid").at([0.5, 0.0]);
    let aim = [top[0] + 40.0, top[1] + 10.0];
    grip(&mut p, json!("turn"), aim);
    let d = seat(&p, "kid");
    let (c, t) = (d.at([0.5, 0.5]), d.at([0.5, 0.0]));
    let want = (aim[0] - d.at([0.5, 0.5])[0]).atan2(-(aim[1] - c[1]));
    let got = (t[0] - c[0]).atan2(-(t[1] - c[1]));
    assert!((want - got).abs() < 0.3, "{want} {got}");
}

fn under_a_leaning_parent() -> Value {
    json!([
        card("base", json!({"position": [500, 450], "tilt": [20, -25], "z": 90, "rotation": 10})),
        {"id": "kid", "type": "solid", "color": "#ff0000", "size": [120, 80], "parent": "base",
            "transform": {"position": [60, 20], "rotation": -20, "tilt": [15, 10], "z": 40}}
    ])
}

#[test]
fn handles_follow_the_pointer_on_a_layer_under_a_leaning_parent() {
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(&root, under_a_leaning_parent());
    let goal = {
        let c = seat(&p, "kid").at([1.0, 1.0]);
        [c[0] + 30.0, c[1] + 20.0]
    };
    grip(&mut p, json!({"scale": {"corner": [1, 1]}}), goal);
    let reached = seat(&p, "kid").at([1.0, 1.0]);
    assert!(near(reached, goal, 0.5), "scale {reached:?} {goal:?}");
    let top = seat(&p, "kid").at([0.5, 0.0]);
    let aim = [top[0] + 40.0, top[1] + 10.0];
    grip(&mut p, json!("turn"), aim);
    let d = seat(&p, "kid");
    let (c, t) = (d.at([0.5, 0.5]), d.at([0.5, 0.0]));
    let want = (aim[0] - c[0]).atan2(-(aim[1] - c[1]));
    let got = (t[0] - c[0]).atan2(-(t[1] - c[1]));
    assert!((want - got).abs() < 0.3, "turn {want} {got}");
}
