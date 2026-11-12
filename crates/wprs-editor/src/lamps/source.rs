use super::lit::picture;
use crate::action::NormalsMake;
use crate::ops::escape;
use anyhow::{Result, anyhow};
use image::RgbaImage;
use serde_json::Value;
use std::path::Path;
use wprs_core::message::said;

pub(super) struct Source {
    pub id: String,
    pub path: Option<String>,
    pub picture: RgbaImage,
}

fn path_of(doc: &Value, id: &str) -> Option<String> {
    doc.pointer(&format!("/scene/assets/{}/path", escape(id))).and_then(Value::as_str).map(String::from)
}

fn video(dir: &Path, doc: &Value, layer: &Value, time: Option<f64>) -> Result<Source> {
    let id = match &layer["video"] {
        Value::String(v) => Some(v.clone()),
        v => v["map"].as_object().and_then(|m| m.values().find_map(Value::as_str)).map(String::from),
    };
    let id =
        id.ok_or_else(|| anyhow!(said("editor.normals_video", "the video layer names no video", &[])))?;
    let path = path_of(doc, &id)
        .ok_or_else(|| anyhow!(said("editor.normals_video", "the video layer names no video", &[])))?;
    let still = wprs_video::still_at(&dir.join(&path), Some(time.unwrap_or(0.0)))
        .ok()
        .and_then(|s| RgbaImage::from_raw(s.width, s.height, s.rgba));
    let picture = still
        .ok_or_else(|| anyhow!(said("editor.normals_frame", "a frame of %1 could not be read", &[&id])))?;
    Ok(Source { id, path: None, picture })
}

fn text(dir: &Path, doc: &Value, layer: &Value) -> Result<Source> {
    let words = match &layer["text"] {
        Value::String(s) => s.clone(),
        Value::Object(m) => m.get("value").and_then(Value::as_str).unwrap_or("00:00").to_string(),
        _ => String::new(),
    };
    let size = layer["size"].as_f64().unwrap_or(48.0) as f32;
    let font = layer["font"].as_str().unwrap_or("");
    let picture = crate::swarms::glyphs::sized(dir, doc, font, &words, size)
        .ok_or_else(|| anyhow!(said("editor.normals_text", "the text layer has no letters to raise", &[])))?;
    let id = layer["id"].as_str().unwrap_or("text").to_string();
    Ok(Source { id, path: None, picture })
}

pub(super) fn source(dir: &Path, doc: &Value, m: &NormalsMake) -> Result<Source> {
    let named = |id: &str| -> Result<Source> {
        picture(doc, id)?;
        let picture = crate::pictures::asset(dir, doc, id)?.into_rgba8();
        Ok(Source { id: id.to_string(), path: path_of(doc, id), picture })
    };
    if let Some(id) = &m.asset {
        return named(id);
    }
    let from = m.layer.as_deref().and_then(|l| crate::actions::layer(doc, l).ok()).map(|(_, l)| l);
    match from {
        Some(l) if l["image"].is_string() => named(l["image"].as_str().unwrap_or_default()),
        Some(l) if l["type"] == "video" => video(dir, doc, l, m.time),
        Some(l) if l["type"] == "text" => text(dir, doc, l),
        _ => {
            Err(anyhow!(said("editor.normals_from", "name a picture or an image, video or text layer", &[])))
        }
    }
}
