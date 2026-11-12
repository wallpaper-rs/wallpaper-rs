mod capture;
mod grip;
mod place;

use serde_json::{Value, json};
use std::path::Path;

pub(crate) use capture::resize;
pub(crate) use grip::grip;

pub fn asks(q: &Value) -> bool {
    q.get("effect_handles").is_some() || q.get("capture_view").is_some()
}

pub fn query(dir: &Path, doc: &Value, q: &Value) -> Value {
    if let Some(c) = q.get("capture_view") {
        let id = c["layer"].as_str().unwrap_or_default();
        return capture::view(dir, doc, id, c["time"].as_f64().unwrap_or(0.0)).unwrap_or(Value::Null);
    }
    let e = &q["effect_handles"];
    let (id, index) = (e["layer"].as_str().unwrap_or_default(), e["index"].as_u64().unwrap_or(0) as usize);
    match place::handles(dir, doc, id, index, e["time"].as_f64().unwrap_or(0.0)) {
        Ok(found) => found,
        Err(_) => json!({ "effect": null, "handles": [] }),
    }
}
