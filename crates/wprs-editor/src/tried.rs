use serde_json::{Map, Value, json};

pub fn values(doc: &Value, look: &Value) -> Option<Value> {
    let overrides = look.get("values").filter(|v| v.is_object());
    let preset = look.get("preset").and_then(Value::as_str).filter(|p| !p.is_empty());
    if overrides.is_none() && preset.is_none() {
        return None;
    }
    let mut tried = Map::new();
    let listed = doc["manifest"]["presets"].as_array();
    let chosen = preset.and_then(|id| listed?.iter().find(|p| p["id"] == id));
    for source in
        chosen.and_then(|p| p["values"].as_object()).into_iter().chain(overrides.and_then(Value::as_object))
    {
        source
            .iter()
            .filter(|(_, v)| !v.is_null())
            .for_each(|(k, v)| drop(tried.insert(k.clone(), v.clone())));
    }
    Some(Value::Object(tried))
}

pub fn stage(doc: &Value, look: &Value, document: &mut Value) {
    let Some(tried) = values(doc, look) else { return };
    if !document["preview"].is_object() {
        document["preview"] = json!({});
    }
    document["preview"]["values"] = tried;
}

pub fn query(doc: &Value, ask: &Value) -> Value {
    let properties = serde_json::from_value(doc["manifest"]["properties"].clone()).unwrap_or_default();
    let presets: Vec<wprs_scene::Preset> =
        serde_json::from_value(doc["manifest"]["presets"].clone()).unwrap_or_default();
    let languages: Vec<String> = serde_json::from_value(ask["languages"].clone()).unwrap_or_default();
    let tried = values(doc, ask).unwrap_or_default();
    let layer: wprs_scene::Values =
        tried.as_object().map(|o| o.clone().into_iter().collect()).unwrap_or_default();
    let shown = wprs_scene::effective(&properties, &[&layer]);
    let (fields, labels) = wprs_scene::form(&properties, &presets, &shown, &languages);
    let fields: Vec<&wprs_core::model::SceneField> =
        fields.iter().filter(|f| !f.secret && f.kind != "shortcut" && f.kind != "button").collect();
    json!({ "fields": fields, "presets": labels, "values": shown })
}
