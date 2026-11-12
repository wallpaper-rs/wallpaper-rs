use super::{act, card, near, scene, seat};
use serde_json::{Value, json};
use wprs_editor::Project;

fn nested() -> Value {
    json!([
        card("base", json!({"position": [400, 500], "rotation": 30, "scale": [1.5, 0.75]})),
        {"id": "kid", "type": "solid", "color": "#ff0000", "size": [120, 80], "parent": "base",
            "transform": {"position": [60, 20], "origin": [0.2, 0.3], "rotation": -20, "scale": [1.2, 0.9]},
            "effects": [
                {"effect": "spin", "params": {"center": [0.3, 0.6], "angle": 40}},
                {"effect": "shake"},
                {"effect": "light_shafts", "params": {"center": [0.5, 0.1]}}
            ]}
    ])
}

fn handles(p: &Project, index: usize) -> Value {
    let q = json!({"effect_handles": {"layer": "kid", "index": index, "time": 0.0}});
    wprs_editor::rigs::query(&p.dir, &p.doc, "kid", &q)
}

fn find(found: &Value, param: &str) -> Value {
    found["handles"].as_array().unwrap().iter().find(|h| h["param"] == param).cloned().unwrap()
}

fn grip(p: &mut Project, index: usize, param: &str, to: [f64; 2]) -> anyhow::Result<()> {
    act(p, json!({"effects": {"grip": {"layer": "kid", "index": index, "param": param, "to": to}}}))
}

fn param(p: &Project, index: usize, name: &str) -> Value {
    p.doc.pointer(&format!("/scene/layers/1/effects/{index}/params/{name}")).cloned().unwrap_or(Value::Null)
}

fn at(h: &Value) -> [f64; 2] {
    [h["at"][0].as_f64().unwrap(), h["at"][1].as_f64().unwrap()]
}

#[test]
fn a_point_sits_where_the_renderer_puts_it_and_follows_the_pointer() {
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(&root, nested());
    let found = handles(&p, 0);
    assert_eq!(found["effect"], "spin");
    assert!(near(at(&find(&found, "center")), seat(&p, "kid").at([0.3, 0.6]), 1e-6));
    let goal = seat(&p, "kid").at([0.8, 0.2]);
    grip(&mut p, 0, "center", goal).unwrap();
    let now = param(&p, 0, "center");
    assert!(near([now[0].as_f64().unwrap(), now[1].as_f64().unwrap()], [0.8, 0.2], 2e-3), "{now}");
    assert!(near(at(&find(&handles(&p, 0), "center")), goal, 0.5));
}

#[test]
fn an_angle_has_a_dial_around_its_point() {
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(&root, nested());
    let found = handles(&p, 0);
    let dial = find(&found, "angle");
    assert_eq!(dial["kind"], "angle");
    let me = seat(&p, "kid");
    let anchor = me.at([0.3, 0.6]);
    let first = at(&dial);
    assert!((first[0] - anchor[0]).hypot(first[1] - anchor[1]) > 1.0);
    let far = me.at([0.3 + 40.0 / 120.0, 0.6]);
    grip(&mut p, 0, "angle", far).unwrap();
    let turned = param(&p, 0, "angle").as_f64().unwrap();
    assert!(turned.abs() < 0.2, "{turned}");
    let down = me.at([0.3, 0.6 + 16.0 / 80.0]);
    grip(&mut p, 0, "angle", down).unwrap();
    assert!((param(&p, 0, "angle").as_f64().unwrap() - 90.0).abs() < 0.2);
    let back = at(&find(&handles(&p, 0), "angle"));
    assert!(near(back, down, 0.5), "{back:?} {down:?}");
}

#[test]
fn a_direction_is_an_arrow_from_the_middle_of_the_layer() {
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(&root, nested());
    let found = handles(&p, 1);
    let arrow = find(&found, "direction");
    assert_eq!(arrow["kind"], "vector");
    assert!(arrow["anchor"].is_array());
    let me = seat(&p, "kid");
    let side = 80.0 * 0.2;
    let goal = me.at([0.5, 0.5 + side / 80.0]);
    grip(&mut p, 1, "direction", goal).unwrap();
    let v = param(&p, 1, "direction");
    assert!(v[0].as_f64().unwrap().abs() < 2e-3 && (v[1].as_f64().unwrap() - 1.0).abs() < 2e-3, "{v}");
}

#[test]
fn a_param_without_a_handle_or_a_driven_one_is_refused() {
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(&root, nested());
    let err = grip(&mut p, 0, "radius", [10.0, 10.0]).unwrap_err().to_string();
    assert!(err.contains("no handle"), "{err}");
    act(
        &mut p,
        json!({"set": {"pointer": "/scene/layers/1/effects/0/params/center", "value": {"keys": [[0, [0.1, 0.1]], [1, [0.9, 0.9]]]}}}),
    )
    .unwrap();
    assert!(grip(&mut p, 0, "center", [100.0, 100.0]).is_err());
}
