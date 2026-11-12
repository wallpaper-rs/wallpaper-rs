use crate::settings::properties;
use serde_json::{Value, json};
use wprs_scene::{Leveling, Quality};

const QUALITIES: [Quality; 4] = [Quality::Q0, Quality::Q1, Quality::Q2, Quality::Q3];

pub fn effective(doc: &Value, id: &str) -> Value {
    let props = properties(doc);
    let Some(prop) = props.get(id) else { return Value::Null };
    let leveling = Leveling::of(&props);
    let start = props.iter().map(|(k, p)| (k.clone(), p.default_value())).collect();
    let rows = QUALITIES.map(|q| {
        let value = leveling.tuned(&start, q).remove(id).unwrap_or_default();
        json!({ "quality": q, "value": value, "own": prop.quality.contains_key(&q) })
    });
    json!({ "property": id, "default": prop.default_value(), "levels": rows })
}
