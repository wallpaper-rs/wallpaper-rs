use serde_json::{Value, json};
use wprs_editor::rigs::query;

fn ask(doc: &Value, q: Value) -> Value {
    query(std::path::Path::new("."), doc, "", &q)
}

#[test]
fn smoothing_is_drawn_as_a_rise_and_a_fall() {
    let plot = ask(&Value::Null, json!({ "response": { "smooth": { "up": 0.1, "down": 0.5 } } }));
    let points = plot["points"].as_array().unwrap();
    let at = |t: f64| points.iter().find(|p| p[0].as_f64().unwrap() >= t).unwrap()[1].as_f64().unwrap();
    assert_eq!(at(0.0), 0.0);
    let (rises, falls) = (plot["rises"].as_f64().unwrap(), plot["falls"].as_f64().unwrap());
    assert!(at(falls - 0.001) > 0.9, "{plot}");
    assert!(at(falls + 0.5) < at(falls), "{plot}");
    let settles = plot["settles"].as_array().unwrap();
    assert!(rises < falls && (settles[0].as_f64().unwrap() - 0.3).abs() < 1e-6, "{plot}");
    assert!((settles[1].as_f64().unwrap() - 1.5).abs() < 1e-6, "{plot}");
    let plain = ask(&Value::Null, json!({ "response": { "smooth": 0 } }));
    assert!(plain["points"].as_array().unwrap().iter().any(|p| p[1] == json!(1.0)));
    assert!(ask(&Value::Null, json!({ "response": { "smooth": 61 } }))["error"].is_string());
    assert!(ask(&Value::Null, json!({ "response": { "smooth": "slow" } }))["error"].is_string());
}

#[test]
fn the_made_up_pointer_stays_on_the_screen_and_moves() {
    let first = ask(&Value::Null, json!({ "wander": { "time": 0 } }));
    let later = ask(&Value::Null, json!({ "wander": { "time": 2.5 } }));
    assert_ne!(first["x"], later["x"]);
    for t in 0..200 {
        let [x, y] = wprs_editor::wander::at(f64::from(t) * 0.37);
        assert!((0.0..=1.0).contains(&x) && (0.0..=1.0).contains(&y), "{t}");
    }
}

#[test]
fn the_smooth_of_a_binding_has_a_field_of_its_own() {
    let doc = json!({ "scene": { "layers": [{ "id": "a", "type": "solid", "opacity": { "audio": "bass", "smooth": 0.3 } }] } });
    let fields = wprs_editor::inspect::fields(&doc, "/scene/layers/0/opacity");
    let smooth = fields.iter().find(|f| f.key == "smooth").expect("a binding has smooth");
    assert_eq!((smooth.widget.as_str(), smooth.value.clone()), ("smooth", json!(0.3)));
    let apart = json!({ "scene": { "layers": [{ "id": "a", "type": "solid", "opacity": { "audio": "bass", "smooth": { "up": 0.1, "down": 1 } } }] } });
    let fields = wprs_editor::inspect::fields(&apart, "/scene/layers/0/opacity");
    assert_eq!(fields.iter().find(|f| f.key == "smooth").map(|f| f.widget.as_str()), Some("smooth"));
}
