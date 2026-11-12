use super::{act, at, project};
use serde_json::json;
use wprs_scene::{Color, Key, Vec3, sample};

const COLOR: &str = "/scene/layers/0/color";

#[test]
fn a_color_written_as_a_list_mixes_as_a_color() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    for (t, v) in [(0.0, json!([1, 0, 0])), (2.0, json!([0, 0, 1]))] {
        act(&mut p, json!({"keyframe": {"pointer": COLOR, "time": t, "value": v}})).unwrap();
    }
    assert_eq!(wprs_editor::kinded::kind(&p.doc, &format!("{COLOR}/keys")).as_deref(), Some("Color"));
    let keys = at(&p, &format!("{COLOR}/keys"));
    let drawn: Vec<Key<Color>> = serde_json::from_value(keys.clone()).unwrap();
    let plain: Vec<Key<Vec3>> = serde_json::from_value(keys.clone()).unwrap();
    let curves = wprs_editor::graph::curves(&p.doc, &json!({"pointer": COLOR, "samples": 5}));
    let middle: Vec<f64> = (0..3).map(|i| curves["tracks"][i]["points"][2][1].as_f64().unwrap()).collect();
    let render = sample(&drawn, 1.0).0;
    assert!((0..3).all(|i| (middle[i] - f64::from(render[i])).abs() < 1e-3), "{middle:?} {render:?}");
    let linear = sample(&plain, 1.0).0;
    assert!((middle[0] - f64::from(linear[0])).abs() > 0.05, "not a plain list: {middle:?} {linear:?}");
    act(&mut p, json!({"keyframe": {"pointer": COLOR, "time": 1}})).unwrap();
    let added = at(&p, &format!("{COLOR}/keys/1/1"));
    assert_eq!(added.as_array().map(Vec::len), Some(3), "a new key keeps the spelling: {added}");
    assert!((added[0].as_f64().unwrap() - f64::from(render[0])).abs() < 1e-3, "{added}");
    let shown = wprs_editor::inspect::fields_at(&p.doc, "/scene/layers/0", 0.5);
    let field = shown.iter().find(|f| f.key == "color").unwrap();
    let now: Vec<Key<Color>> = serde_json::from_value(at(&p, &format!("{COLOR}/keys"))).unwrap();
    let half = sample(&now, 0.5).0;
    assert!((field.shown[2].as_f64().unwrap() - f64::from(half[2])).abs() < 1e-3, "{}", field.shown);
}
