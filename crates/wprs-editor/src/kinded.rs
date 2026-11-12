use crate::keys::{mix, time_of};
use crate::schema::{at, deref, root_of};
use serde_json::{Value, json};
use wprs_scene::Ease;

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

pub(crate) fn unread(keys: &[Value], t: f64) -> Option<Value> {
    let i = keys.partition_point(|k| time_of(k) <= t);
    match (i.checked_sub(1).and_then(|i| keys.get(i)), keys.get(i)) {
        (Some(a), Some(b)) => {
            let ease: Ease =
                serde_json::from_value(a.get(2).cloned().unwrap_or(json!("linear"))).unwrap_or_default();
            let f = ((t - time_of(a)) / (time_of(b) - time_of(a)).max(1e-6)) as f32;
            Some(mix(&a[1], &b[1], ease.apply(f)))
        }
        (Some(k), None) | (None, Some(k)) => Some(k[1].clone()),
        (None, None) => None,
    }
}
