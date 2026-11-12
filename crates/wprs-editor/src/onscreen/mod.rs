mod capture;
mod channel;
mod grip;
mod mask;
mod maskgrip;
mod place;
mod shapes;

use serde_json::{Value, json};
use std::path::Path;

pub(crate) use capture::resize;
pub(crate) use grip::grip;
pub(crate) use maskgrip::grip as mask_grip;

pub fn asks(q: &Value) -> bool {
    ["effect_handles", "capture_view", "mask_view", "mask_picture"].iter().any(|k| q.get(k).is_some())
}

pub fn query(dir: &Path, doc: &Value, q: &Value) -> Value {
    if let Some(c) = q.get("capture_view") {
        let id = c["layer"].as_str().unwrap_or_default();
        let t = c["time"].as_f64().unwrap_or(0.0);
        return capture::view(dir, doc, id, t).unwrap_or(Value::Null);
    }
    if let Some(m) = q.get("mask_view") {
        let id = m["layer"].as_str().unwrap_or_default();
        let (index, t) = (m["index"].as_u64().map(|i| i as usize), m["time"].as_f64().unwrap_or(0.0));
        return mask::view(dir, doc, id, index, t).unwrap_or(Value::Null);
    }
    if let Some(m) = q.get("mask_picture") {
        return channel::channel(dir, doc, m).unwrap_or(Value::Null);
    }
    let e = &q["effect_handles"];
    let (id, index) = (e["layer"].as_str().unwrap_or_default(), e["index"].as_u64().unwrap_or(0) as usize);
    match place::handles(dir, doc, id, index, e["time"].as_f64().unwrap_or(0.0)) {
        Ok(found) => found,
        Err(_) => json!({ "effect": null, "handles": [] }),
    }
}
