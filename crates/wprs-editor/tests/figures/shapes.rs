use super::{at, handle, handles, near, pull, scene, seat};
use serde_json::json;

fn shape(id: &str, shape: serde_json::Value, extra: serde_json::Value) -> serde_json::Value {
    let mut layer = json!({"id": id, "type": "shape", "shape": shape, "fill": "#ff0000",
        "transform": {"position": [500, 400], "rotation": 25, "scale": [1.4, 0.8]}});
    for (k, v) in extra.as_object().cloned().unwrap_or_default() {
        layer[k] = v;
    }
    layer
}

#[test]
fn a_rect_is_resized_and_rounded_from_the_canvas_where_it_is_drawn() {
    let root = tempfile::tempdir().unwrap();
    let mut p =
        scene(&root, json!([shape("box", json!({"rect": {"size": [200, 100], "radius": 10}}), json!({}))]));
    let me = seat(&p, "box");
    let found = handles(&p, "box");
    assert!(near(handle(&found, "size"), me.at([0.9, 0.9]), 1e-6));
    assert!(near(handle(&found, "radius"), me.at([1.0 - 10.0 / 200.0, 0.1]), 1e-6));
    pull(&mut p, "box", "size", me.at([0.5 + 0.4 * 1.5, 0.5 + 0.4 * 2.0])).unwrap();
    let size = at(&p, "/scene/layers/0/shape/rect/size");
    assert!(near([size[0].as_f64().unwrap(), size[1].as_f64().unwrap()], [300.0, 200.0], 0.2), "{size}");
    let me = seat(&p, "box");
    assert!(near(handle(&handles(&p, "box"), "size"), me.at([0.9, 0.9]), 1e-6));
    pull(&mut p, "box", "radius", me.at([0.9, 0.1])).unwrap();
    let radius = at(&p, "/scene/layers/0/shape/rect/radius").as_f64().unwrap();
    assert!((radius - 30.0).abs() < 0.2, "{radius}");
    pull(&mut p, "box", "radius", me.at([-3.0, 0.1])).unwrap();
    assert_eq!(at(&p, "/scene/layers/0/shape/rect/radius"), json!(100.0));
    let height = at(&p, "/scene/layers/0/shape/rect/size/1");
    pull(&mut p, "box", "width", me.at([1.0, 0.5])).unwrap();
    assert_eq!(at(&p, "/scene/layers/0/shape/rect/size/1"), height);
    assert!(at(&p, "/scene/layers/0/shape/rect/size/0").as_f64().unwrap() > 370.0);
}

#[test]
fn an_even_drag_keeps_the_proportions() {
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(&root, json!([shape("box", json!({"ellipse": {"size": [200, 100]}}), json!({}))]));
    let me = seat(&p, "box");
    let found = handles(&p, "box");
    let to = handle(&found, "size");
    let farther = [to[0] + (to[0] - me.at([0.5, 0.5])[0]), to[1] + (to[1] - me.at([0.5, 0.5])[1])];
    super::act(&mut p, json!({"layer": {"figure": {"shape": {"layer": "box", "handle": "size", "to": farther, "even": true}}}})).unwrap();
    let size = at(&p, "/scene/layers/0/shape/ellipse/size");
    let ratio = size[0].as_f64().unwrap() / size[1].as_f64().unwrap();
    assert!((ratio - 2.0).abs() < 0.05, "{size}");
    assert!(size[0].as_f64().unwrap() > 300.0);
}

#[test]
fn the_points_and_depth_of_a_star_follow_their_handles() {
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(
        &root,
        json!([shape("star", json!({"star": {"points": 5, "size": 200, "inner": 0.5}}), json!({}))]),
    );
    let me = seat(&p, "star");
    let found = handles(&p, "star");
    let tip = |k: usize, steps: usize, r: f64| {
        let a = -std::f64::consts::FRAC_PI_2 + k as f64 * std::f64::consts::TAU / steps as f64;
        me.at([0.5 + a.cos() * r / 200.0, 0.5 + a.sin() * r / 200.0])
    };
    assert!(near(handle(&found, "points"), tip(2, 10, 100.0), 1e-6));
    assert!(near(handle(&found, "inner"), tip(1, 10, 50.0), 1e-6));
    pull(&mut p, "star", "points", tip(2, 14, 100.0)).unwrap();
    assert_eq!(at(&p, "/scene/layers/0/shape/star/points"), json!(7));
    pull(&mut p, "star", "inner", tip(1, 14, 80.0)).unwrap();
    let inner = at(&p, "/scene/layers/0/shape/star/inner").as_f64().unwrap();
    assert!((inner - 0.8).abs() < 0.02, "{inner}");
    pull(&mut p, "star", "size", tip(0, 1, 120.0)).unwrap();
    let size = at(&p, "/scene/layers/0/shape/star/size").as_f64().unwrap();
    assert!((size - 300.0).abs() < 0.5, "{size}");
}

#[test]
fn a_polygon_takes_its_sides_from_the_second_vertex() {
    let root = tempfile::tempdir().unwrap();
    let mut p =
        scene(&root, json!([shape("poly", json!({"polygon": {"sides": 3, "size": 200}}), json!({}))]));
    let me = seat(&p, "poly");
    let a = -std::f64::consts::FRAC_PI_2 + std::f64::consts::TAU / 6.0;
    pull(&mut p, "poly", "sides", me.at([0.5 + a.cos() * 0.5, 0.5 + a.sin() * 0.5])).unwrap();
    assert_eq!(at(&p, "/scene/layers/0/shape/polygon/sides"), json!(6));
}

#[test]
fn moving_one_end_of_a_line_leaves_the_other_where_it_is() {
    let root = tempfile::tempdir().unwrap();
    let line = shape("line", json!({"line": {"to": [200, 50], "width": 8}}), json!({}));
    let mut p = scene(&root, json!([line]));
    let found = handles(&p, "line");
    let (start, end) = (handle(&found, "start"), handle(&found, "end"));
    let goal = [end[0] + 60.0, end[1] + 40.0];
    pull(&mut p, "line", "end", goal).unwrap();
    let found = handles(&p, "line");
    assert!(near(handle(&found, "start"), start, 0.3), "{:?} {:?}", handle(&found, "start"), start);
    assert!(near(handle(&found, "end"), goal, 0.6), "{:?} {:?}", handle(&found, "end"), goal);
    let kept = handle(&found, "end");
    pull(&mut p, "line", "start", [start[0] - 30.0, start[1] + 10.0]).unwrap();
    let found = handles(&p, "line");
    assert!(near(handle(&found, "end"), kept, 0.6));
    assert!(near(handle(&found, "start"), [start[0] - 30.0, start[1] + 10.0], 0.6));
}

#[test]
fn a_gradient_has_a_handle_at_each_end_and_the_stroke_a_handle_of_its_width() {
    let root = tempfile::tempdir().unwrap();
    let extra = json!({"gradient": {"from": [0.1, 0.2], "to": [0.9, 0.8], "stops": [[0, "#ff0000"], [1, "#0000ff"]]},
        "stroke": {"color": "#ffffff", "width": 6}});
    let mut p = scene(&root, json!([shape("g", json!({"rect": {"size": [200, 100]}}), extra)]));
    let me = seat(&p, "g");
    let found = handles(&p, "g");
    assert!(near(handle(&found, "gradient_from"), me.at([0.1, 0.2]), 1e-6));
    assert!(near(
        found["gradient"]["to"].as_array().map(|a| [a[0].as_f64().unwrap(), a[1].as_f64().unwrap()]).unwrap(),
        me.at([0.9, 0.8]),
        1e-6
    ));
    pull(&mut p, "g", "gradient_to", me.at([1.5, 0.25])).unwrap();
    let to = at(&p, "/scene/layers/0/gradient/to");
    assert!(near([to[0].as_f64().unwrap(), to[1].as_f64().unwrap()], [1.5, 0.25], 2e-3), "{to}");
    pull(&mut p, "g", "stroke", me.at([-0.05, 0.8])).unwrap();
    let width = at(&p, "/scene/layers/0/stroke/width").as_f64().unwrap();
    assert!((width - 20.0).abs() < 0.5, "{width}");
    assert!(pull(&mut p, "g", "nothing", [0.0, 0.0]).is_err());
}
