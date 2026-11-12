mod grip;
mod place;

use serde_json::{Value, json};
use std::path::Path;

pub(crate) use grip::grip;

pub fn asks(q: &Value) -> bool {
    q.get("effect_handles").is_some()
}

pub fn query(dir: &Path, doc: &Value, q: &Value) -> Value {
    let e = &q["effect_handles"];
    let (id, index) = (e["layer"].as_str().unwrap_or_default(), e["index"].as_u64().unwrap_or(0) as usize);
    match place::handles(dir, doc, id, index, e["time"].as_f64().unwrap_or(0.0)) {
        Ok(found) => found,
        Err(_) => json!({ "effect": null, "handles": [] }),
    }
}
