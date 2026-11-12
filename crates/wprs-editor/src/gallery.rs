use serde_json::Value;
use std::path::Path;

const QUERIES: [&str; 2] = ["film", "sprites"];

pub fn asks(q: &Value) -> bool {
    QUERIES.iter().any(|k| q.get(k).is_some())
}

pub fn query(dir: &Path, doc: &Value, q: &Value) -> Value {
    if let Some(f) = q.get("film") {
        let strip = f["strip"].as_u64().unwrap_or(0) as usize;
        return crate::films::film(dir, doc, f["layer"].as_str().unwrap_or_default(), strip);
    }
    if let Some(s) = q.get("sprites") {
        return crate::cutter::sheet(dir, doc, s["asset"].as_str().unwrap_or_default());
    }
    Value::Null
}
