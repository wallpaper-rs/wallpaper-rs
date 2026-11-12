use crate::keys::value_at;
use crate::seats::boxes::pair;
use ab_glyph::FontArc;
use serde_json::Value;
use std::path::Path;
use std::sync::{Mutex, OnceLock};
use wprs_scene::{Color, Layout, MAX_FALLBACK, Wrap};
use wprs_type::Fonts;

fn cache() -> &'static Mutex<Fonts> {
    static CACHE: OnceLock<Mutex<Fonts>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(Fonts::from_faces([])))
}

fn resolve(fonts: &mut Fonts, dir: &Path, doc: &Value, name: &str) -> String {
    let asset = &doc["scene"]["assets"][name];
    let Some(path) = asset["path"].as_str() else { return name.to_owned() };
    let file = dir.join(path);
    let face = asset["face"].as_u64().unwrap_or(0) as u32;
    let stamp = std::fs::metadata(&file).ok().map(|m| (m.len(), m.modified().ok()));
    let key = format!("{path}#{face}#{stamp:?}");
    if !fonts.has(&key) {
        let loaded = std::fs::read(&file).ok().is_some_and(|data| fonts.add(&key, &data, face));
        if !loaded {
            return name.to_owned();
        }
    }
    key
}

pub(super) fn chain(dir: &Path, doc: &Value, layer: &Value) -> Vec<FontArc> {
    let mut fonts = cache().lock().unwrap_or_else(|e| e.into_inner());
    let font = layer["font"].as_str().filter(|f| !f.is_empty()).map(|f| resolve(&mut fonts, dir, doc, f));
    let fallback: Vec<String> = layer["fallback"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .take(MAX_FALLBACK)
        .map(|n| resolve(&mut fonts, dir, doc, n))
        .collect();
    fonts.chain(font.as_deref(), &fallback)
}

fn at(v: &Value, t: f64, fallback: f64) -> f64 {
    value_at(v, t).as_f64().unwrap_or(fallback)
}

fn reach(layer: &Value, t: f64) -> f32 {
    let width = at(&layer["outline"]["width"], t, 0.0).clamp(0.0, 64.0);
    let shadow = &layer["shadow"];
    let cast = if shadow.is_object() {
        let off = pair(&value_at(&shadow["offset"], t), [2.0, 2.0]);
        off[0].abs().max(off[1].abs()).min(1024.0) + at(&shadow["blur"], t, 2.0).clamp(0.0, 64.0) + width
    } else {
        0.0
    };
    let blur = at(&layer["blur"], t, 0.0).clamp(0.0, 64.0);
    let drawn = layer["outline"].is_object() || shadow.is_object() || layer["blur"].is_number();
    if !drawn && layer["render"] != "sdf" {
        return 0.0;
    }
    (2f64.powf((width.max(cast).max(blur) + 2.0).log2().ceil()).max(4.0)) as f32
}

pub(super) fn layout(layer: &Value, t: f64) -> Layout {
    let wrap = layer["wrap"].is_object().then(|| Wrap {
        width: layer["wrap"]["width"].as_f64().unwrap_or(1.0) as f32,
        lines: layer["wrap"]["lines"].as_u64().map(|n| n as u32),
        ellipsis: layer["wrap"]["ellipsis"].as_bool().unwrap_or(false),
    });
    let backing = layer["background"]
        .is_object()
        .then(|| (Color([0.0; 4]), pair(&layer["background"]["padding"], [0.0; 2]).map(|v| v as f32), 0.0));
    Layout {
        wrap,
        line_height: layer["line_height"].as_f64().map(|h| h as f32),
        padding: layer["padding"].as_f64().unwrap_or(0.0) as f32,
        backing,
        spacing: at(&layer["letter_spacing"], t, 0.0).clamp(-100.0, 1000.0) as f32,
        reach: reach(layer, t),
    }
}
