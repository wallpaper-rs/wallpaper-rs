use crate::schema::{PROPERTY, SCENE, child, deref};
use serde_json::Value;

pub(super) const ASSETS: [&str; 6] = ["image", "video", "sound", "font", "model", "media"];

pub(super) fn reword(
    root: &Value,
    schema: &Value,
    v: &mut Value,
    kinds: &[&str],
    (old, new): (&str, &str),
    inherited: Option<&str>,
) {
    let named = schema["x-ref"].as_str().or_else(|| deref(root, schema).0["x-ref"].as_str());
    let kind = named.or(inherited);
    match v {
        Value::String(s) if s == old && kind.is_some_and(|k| kinds.contains(&k)) => *s = new.to_owned(),
        Value::Array(_) | Value::Object(_) => {
            let keys: Vec<String> = match &*v {
                Value::Array(list) => (0..list.len()).map(|i| i.to_string()).collect(),
                Value::Object(map) => map.keys().cloned().collect(),
                _ => Vec::new(),
            };
            let inherit = if v.is_array() { kind } else { None };
            let inner: Vec<Option<Value>> = keys.iter().map(|k| child(root, schema, k, v)).collect();
            for (key, inner) in keys.iter().zip(inner) {
                let slot = match &mut *v {
                    Value::Array(list) => key.parse::<usize>().ok().and_then(|i| list.get_mut(i)),
                    other => other.get_mut(key),
                };
                if let (Some(inner), Some(slot)) = (inner, slot) {
                    reword(root, &inner, slot, kinds, (old, new), inherit);
                }
            }
        }
        _ => {}
    }
}

pub(super) fn assets_in(scene: &mut Value, manifest: &mut Value, old: &str, new: &str) {
    reword(&SCENE, &SCENE, scene, &ASSETS, (old, new), None);
    if let Some(list) = manifest.get_mut("properties").and_then(Value::as_object_mut) {
        list.values_mut().for_each(|p| reword(&PROPERTY, &PROPERTY, p, &ASSETS, (old, new), None));
    }
}

pub(super) fn within(section: &str, key: &str, v: &mut Value, words: &[(&[&str], String, String)]) {
    let held =
        child(&SCENE, &SCENE, section, &Value::Null).and_then(|s| child(&SCENE, &s, key, &Value::Null));
    if let Some(schema) = held {
        for (kinds, old, new) in words {
            reword(&SCENE, &schema, v, kinds, (old, new), None);
        }
    }
}
