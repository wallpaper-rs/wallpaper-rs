use crate::layers::{asset_kind, walk};
use serde_json::{Value, json};
use std::path::Path;

mod waves;

pub use waves::{peaks, wave};

pub const LISTEN: &str = "listen";
const LISTENED: &str = "listen";

fn assets(dir: &Path, doc: &Value) -> Vec<Value> {
    let found = doc.pointer("/scene/assets").and_then(Value::as_object).into_iter().flatten();
    let sounds = found.filter(|(_, a)| asset_kind(a["path"].as_str().unwrap_or_default()) == "sound");
    let row = |(name, a): (&String, &Value)| {
        let path = a["path"].as_str().unwrap_or_default();
        let bytes = std::fs::metadata(dir.join(path)).map(|m| m.len()).unwrap_or(0);
        json!({ "name": name, "path": path, "bytes": bytes })
    };
    sounds.map(row).collect()
}

fn used(layer: &Value) -> Vec<String> {
    let one = layer["sound"].as_str().filter(|s| !s.is_empty()).map(String::from);
    let many = layer["sounds"].as_array().into_iter().flatten().filter_map(|s| s.as_str().map(String::from));
    one.into_iter().chain(many).collect()
}

pub fn list(dir: &Path, doc: &Value) -> Value {
    let layers = walk(doc).into_iter().filter(|(_, l)| l["type"] == "sound").map(|(pointer, l)| {
        json!({
            "id": l["id"],
            "pointer": pointer,
            "sounds": used(l),
            "mode": l["mode"],
            "loop": l["loop"] != false,
            "stream": l["stream"] == true,
            "play_on": l["play_on"],
            "fade_in": l["fade_in"].as_f64().unwrap_or(0.0),
            "fade_out": l["fade_out"].as_f64().unwrap_or(0.0),
            "volume": l["volume"].as_f64(),
            "muted": l["play"] == false,
        })
    });
    json!({ "layers": layers.collect::<Vec<_>>(), "assets": assets(dir, doc) })
}

fn quiet(layers: &mut Value, id: &str) -> bool {
    let mut found = false;
    for layer in layers.as_array_mut().into_iter().flatten() {
        if layer["type"] == "sound" {
            let mine = layer["id"] == id;
            found |= mine;
            layer["play"] = json!(mine);
            if mine {
                layer["loop"] = json!(false);
                layer.as_object_mut().into_iter().for_each(|o| drop(o.remove("play_on")));
            }
        }
        found |= quiet(&mut layer["layers"], id);
    }
    found
}

pub fn listen(scene: &mut Value, what: &Value) {
    let asset = what["asset"].as_str().filter(|a| !a.is_empty());
    let id = what.as_str().filter(|i| !i.is_empty()).unwrap_or(LISTENED);
    if !scene["layers"].is_array() {
        return;
    }
    let found = quiet(&mut scene["layers"], id);
    if let (Some(asset), false) = (asset, found) {
        let layer = json!({ "id": LISTENED, "type": "sound", "sound": asset, "loop": false });
        if let Some(list) = scene["layers"].as_array_mut() {
            list.push(layer);
        }
    }
}
