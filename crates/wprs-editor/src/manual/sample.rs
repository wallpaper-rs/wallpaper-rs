use super::shape::resolved;
use serde_json::{Map, Value, json};

fn present(v: &Value) -> Vec<&Value> {
    ["oneOf", "anyOf"]
        .into_iter()
        .filter_map(|k| v[k].as_array())
        .flatten()
        .filter(|a| a["type"] != "null")
        .collect()
}

fn kind(v: &Value) -> &str {
    match &v["type"] {
        Value::String(s) => s,
        Value::Array(a) => a.iter().filter_map(Value::as_str).find(|k| *k != "null").unwrap_or("null"),
        _ => "",
    }
}

pub(super) fn of(v: &Value, defs: &Value, depth: usize) -> Value {
    let v = resolved(v, defs);
    if let Some(c) = v.get("const") {
        return c.clone();
    }
    if let Some(first) = v["enum"].as_array().and_then(|l| l.first()) {
        return first.clone();
    }
    if let Some(first) = present(&v).first() {
        return of(first, defs, depth);
    }
    let low = v["minimum"].as_f64().unwrap_or(0.0).max(0.0);
    match kind(&v) {
        "object" => {
            let mut out = Map::new();
            let need = v["required"].as_array().into_iter().flatten().filter_map(Value::as_str);
            for key in need.filter(|_| depth > 0) {
                out.insert(key.into(), of(&v["properties"][key], defs, depth - 1));
            }
            Value::Object(out)
        }
        "array" => {
            let count = v["minItems"].as_u64().unwrap_or(0) as usize;
            Value::Array((0..count).filter(|_| depth > 0).map(|_| of(&v["items"], defs, depth - 1)).collect())
        }
        "string" => json!("text"),
        "integer" => json!(low as u64),
        "number" => json!(low),
        "boolean" => json!(false),
        _ => Value::Null,
    }
}
