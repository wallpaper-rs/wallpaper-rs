use serde_json::{Map, Value, json};
use std::io::Read;
use std::path::Path;

pub fn media(doc: &Value) -> bool {
    matches!(crate::pages::kind(doc), "image" | "video")
}

fn size(path: &Path) -> Option<[u32; 2]> {
    let mut head = Vec::new();
    std::fs::File::open(path).ok()?.take(1 << 16).read_to_end(&mut head).ok()?;
    wprs_format::media::probe(&head).map(|m| [m.width, m.height])
}

fn proportion(path: &Path, video: bool) -> Option<f64> {
    let [w, h] = match video {
        false => size(path)?,
        true => {
            let strip = crate::strips::strip(path, 1);
            size(Path::new(strip[0]["file"].as_str()?))?
        }
    };
    Some(f64::from(w) / f64::from(h.max(1)))
}

pub fn info(dir: &Path, doc: &Value) -> Value {
    if !media(doc) {
        return Value::Null;
    }
    let entry = doc["manifest"]["entry"].as_str().unwrap_or_default();
    let path = dir.join(entry);
    let bytes = std::fs::metadata(&path).ok().filter(|m| m.is_file()).map(|m| m.len());
    let video = crate::pages::kind(doc) == "video";
    let length = bytes.and(video.then(|| wprs_scene::film_file(&path)).flatten());
    json!({ "kind": crate::pages::kind(doc), "entry": entry, "present": bytes.is_some(), "bytes": bytes,
        "file": bytes.map(|_| path.clone()), "length": length,
        "size": bytes.filter(|_| !video).and_then(|_| size(&path)),
        "aspect": bytes.and_then(|_| proportion(&path, video)) })
}

fn put(into: &mut Map<String, Value>, key: &str, value: &Value) {
    if !value.is_null() {
        into.insert(key.into(), value.clone());
    }
}

pub fn preview_document(dir: &Path, doc: &Value, look: &Value) -> Value {
    let m = &doc["manifest"];
    let video = crate::pages::kind(doc) == "video";
    let shown = &m["display"];
    let entry = m["entry"].as_str().unwrap_or_default();
    let found = info(dir, doc);
    let size = match found["size"].as_array() {
        Some(exact) => exact.clone(),
        None => {
            let aspect = found["aspect"].as_f64().unwrap_or(16.0 / 9.0);
            vec![json!((1080.0 * aspect).round() as u32), json!(1080)]
        }
    };
    let mut canvas = json!({ "size": size, "fit": shown["fit"].as_str().unwrap_or("fill") });
    canvas["background"] = shown["background"].as_str().map_or(json!("#000000"), |b| json!(b));
    let flip = shown["flip"].as_str().unwrap_or("none");
    let scale = [if flip == "horizontal" { -1.0 } else { 1.0 }, if flip == "vertical" { -1.0 } else { 1.0 }];
    let zoom = shown["zoom"].as_f64().filter(|_| video).unwrap_or(1.0);
    let wide = |i: usize| f64::from(size[i].as_u64().unwrap_or(0) as u32);
    let shift = |i: usize| {
        let free = (zoom - 1.0) * wide(i) / 2.0;
        let at = shown["position"][i].as_f64().filter(|_| video).unwrap_or(0.0);
        wide(i) / 2.0 - at * free
    };
    let kind = if video { "video" } else { "image" };
    let mut layer = json!({ "id": "media", "type": kind, "transform": {
        "scale": [scale[0] * zoom, scale[1] * zoom], "position": [shift(0), shift(1)] } });
    layer[kind] = json!("media");
    layer["size"] = json!(size);
    let play = &m["playback"];
    if video {
        let mut rules = Map::new();
        put(&mut rules, "rate", &play["speed"]);
        put(&mut rules, "start", &play["start"]);
        put(&mut rules, "end", &play["end"]);
        put(&mut rules, "loop", &play["loop"]);
        layer["playback"] = Value::Object(rules);
    }
    let mut manifest = m.clone();
    if let Some(fields) = manifest.as_object_mut() {
        for key in ["display", "presets", "requires", "playback", "previews", "preview"] {
            fields.remove(key);
        }
        fields.insert("kind".into(), json!("scene"));
        fields.insert("entry".into(), json!("scene.json"));
    }
    let mut keep = Map::new();
    for key in ["max_fps", "volume", "muted"] {
        put(&mut keep, key, &play[key]);
    }
    if !keep.is_empty() {
        manifest["playback"] = Value::Object(keep);
    }
    let scene = json!({ "schema": wprs_scene::SCHEMA, "canvas": canvas, "layers": [layer],
        "assets": { "media": { "path": entry } } });
    let mut document = json!({ "manifest": manifest, "scene": scene });
    crate::rigs::staged(look).into_iter().for_each(|s| document["preview"] = s);
    document
}
