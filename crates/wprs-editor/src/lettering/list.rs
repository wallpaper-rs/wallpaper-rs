use serde_json::{Value, json};
use std::path::Path;

const GENERIC: [&str; 3] = ["sans-serif", "serif", "monospace"];
const EXTENSIONS: [&str; 5] = ["ttf", "otf", "ttc", "otc", "woff2"];

pub(super) fn packaged(doc: &Value) -> Vec<(String, String)> {
    let assets = doc.pointer("/scene/assets").and_then(Value::as_object);
    assets
        .into_iter()
        .flatten()
        .filter_map(|(id, a)| {
            let path = a["path"].as_str()?;
            let ext = Path::new(path).extension()?.to_str()?.to_lowercase();
            EXTENSIONS.contains(&ext.as_str()).then(|| (id.clone(), path.to_owned()))
        })
        .collect()
}

fn found(doc: &Value, name: &str) -> &'static str {
    match name {
        _ if GENERIC.contains(&name) => "generic",
        _ if packaged(doc).iter().any(|(id, _)| id == name) => "package",
        _ if wprs_type::installed(name) => "system",
        _ => "missing",
    }
}

pub(super) fn fonts(doc: &Value, layer: &Value) -> Value {
    let package: Vec<Value> =
        packaged(doc).into_iter().map(|(id, path)| json!({ "name": id, "path": path })).collect();
    let chain: Vec<Value> = layer["font"]
        .as_str()
        .into_iter()
        .chain(layer["fallback"].as_array().into_iter().flatten().filter_map(Value::as_str))
        .map(|n| json!({ "name": n, "source": found(doc, n) }))
        .collect();
    json!({
        "package": package,
        "generic": GENERIC,
        "system": wprs_type::families(),
        "chain": chain,
        "most": wprs_scene::MAX_FALLBACK,
    })
}
