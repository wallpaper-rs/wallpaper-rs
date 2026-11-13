use serde_json::{Value, json};

pub(crate) fn of(doc: &Value, pointer: &str) -> Option<Value> {
    let (head, name) = pointer.rsplit_once('/')?;
    if !head.ends_with("/transform") {
        return None;
    }
    let size = doc.pointer("/scene/canvas/size").and_then(Value::as_array);
    let half = |i: usize| size.and_then(|s| s.get(i)).and_then(Value::as_f64).map_or(0.0, |n| n / 2.0);
    match name {
        "position" => Some(json!([half(0), half(1)])),
        "tilt" => Some(json!([0, 0])),
        "z" => Some(json!(0)),
        _ => None,
    }
}
