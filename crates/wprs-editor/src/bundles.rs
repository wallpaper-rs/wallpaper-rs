use serde_json::{Value, json};

pub const BUNDLE_STARTERS: [&str; 1] = ["assets"];

pub(crate) fn document(name: &str, base: Value) -> Option<Value> {
    if name != "assets" {
        return None;
    }
    let mut manifest = base;
    manifest["kind"] = json!("assets");
    manifest["entry"] = json!("assets.json");
    manifest["requires"] = json!({ "features": ["assets"] });
    let scene = json!({"schema": wprs_scene::SCHEMA,
        "canvas": {"size": [1920, 1080], "fit": "fill", "background": "#000000"}, "layers": []});
    Some(json!({ "manifest": manifest, "scene": scene }))
}
