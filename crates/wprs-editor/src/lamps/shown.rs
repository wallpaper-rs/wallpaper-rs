use image::RgbaImage;
use serde_json::Value;
use std::path::Path;

const SIDE: u32 = 128;

fn path_of<'a>(doc: &'a Value, id: &str) -> Option<&'a str> {
    doc.pointer(&format!("/scene/assets/{}/path", crate::ops::escape(id))).and_then(Value::as_str)
}

fn picture(dir: &Path, doc: &Value, id: &str) -> Option<RgbaImage> {
    Some(crate::pictures::asset(dir, doc, id).ok()?.thumbnail(SIDE, SIDE).into_rgba8())
}

pub(super) fn frame(dir: &Path, doc: &Value, id: &str, t: f64) -> Option<RgbaImage> {
    let strip = crate::strips::strip(&dir.join(path_of(doc, id)?), wprs_video::MAX_STRIP);
    let frames = strip.as_array()?;
    let at = |f: &Value| f["at"].as_f64().unwrap_or(0.0);
    let last = frames.iter().map(at).fold(0.0, f64::max);
    let step = last / (frames.len().max(2) - 1) as f64;
    let span = last + step;
    let t = if span > 0.0 { t.max(0.0) % span } else { 0.0 };
    let near = frames.iter().min_by(|a, b| (at(a) - t).abs().total_cmp(&(at(b) - t).abs()))?;
    let image = image::open(near["file"].as_str()?).ok()?;
    Some(image.thumbnail(SIDE, SIDE).into_rgba8())
}

fn first_video(layer: &Value) -> Option<&str> {
    match &layer["video"] {
        Value::String(v) => Some(v),
        v => v["map"].as_object()?.values().find_map(Value::as_str),
    }
}

fn layer(dir: &Path, doc: &Value, id: &str, t: f64) -> Option<RgbaImage> {
    let (_, l) = crate::actions::layer(doc, id).ok()?;
    match l["type"].as_str()? {
        "image" => picture(dir, doc, l["image"].as_str()?),
        "video" => frame(dir, doc, first_video(l)?, t),
        "text" => crate::lettering::picture(dir, doc, l, t)
            .map(|i| image::DynamicImage::from(i).thumbnail(SIDE, SIDE).into_rgba8()),
        _ => None,
    }
}

pub(super) fn cast(dir: &Path, doc: &Value, texture: &Value, kind: Option<&str>, t: f64) -> Option<String> {
    let image = match (texture, kind) {
        (Value::String(id), Some("picture")) => picture(dir, doc, id),
        (Value::String(id), Some("video")) => frame(dir, doc, id, t),
        (v, _) => layer(dir, doc, v["layer"].as_str()?, t),
    };
    image.map(|i| crate::rigs::url(&i))
}
