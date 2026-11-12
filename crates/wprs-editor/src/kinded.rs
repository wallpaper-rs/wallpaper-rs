use crate::schema::{at, deref, root_of};
use serde_json::Value;

pub fn kind(doc: &Value, list: &str) -> Option<String> {
    let (root, _) = root_of(list)?;
    let item = at(doc, &format!("{list}/0"))?;
    deref(root, &item).1.strip_prefix("Key_for_").map(String::from)
}

pub fn sample_in(doc: &Value, list: &str, keys: &[Value], t: f64) -> Option<Value> {
    let kind = kind(doc, list);
    wprs_scene::sample_json_as(keys, t, kind.as_deref()).or_else(|| crate::keys::sample(keys, t))
}

pub fn shown(doc: &Value, pointer: &str, value: &Value, t: f64) -> Value {
    match value.get("keys").and_then(Value::as_array) {
        Some(keys) => sample_in(doc, &format!("{pointer}/keys"), keys, t).unwrap_or(Value::Null),
        None => value.clone(),
    }
}
