use image::{Rgba, RgbaImage, imageops};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

const EDGE: u32 = 512;
const KEPT: usize = 32;

fn cache() -> PathBuf {
    dirs::cache_dir().unwrap_or_else(std::env::temp_dir).join("wallpaper-rs").join("ktx2").join("channels")
}

fn tint(text: &str) -> [u8; 3] {
    let hex = text.trim_start_matches('#');
    let part =
        |i: usize| hex.get(i * 2..i * 2 + 2).and_then(|h| u8::from_str_radix(h, 16).ok()).unwrap_or(255);
    [part(0), part(1), part(2)]
}

fn pruned(dir: &Path) {
    let Ok(list) = std::fs::read_dir(dir) else { return };
    let mut kept: Vec<(std::time::SystemTime, PathBuf)> =
        list.flatten().filter_map(|e| Some((e.metadata().ok()?.modified().ok()?, e.path()))).collect();
    kept.sort_by_key(|a| std::cmp::Reverse(a.0));
    kept.iter().skip(KEPT).for_each(|(_, p)| drop(std::fs::remove_file(p)));
}

fn value(pixel: &Rgba<u8>, channel: &str) -> f32 {
    let [r, g, b, a] = pixel.0.map(f32::from);
    match channel {
        "r" => r,
        "g" => g,
        "b" => b,
        "a" => a,
        _ => 0.2126 * r + 0.7152 * g + 0.0722 * b,
    }
}

pub(super) fn channel(dir: &Path, doc: &Value, q: &Value) -> Option<Value> {
    let id = q["asset"].as_str()?;
    let file = dir.join(doc["scene"]["assets"][id]["path"].as_str()?);
    let meta = std::fs::metadata(&file).ok()?;
    let name = q["channel"].as_str().unwrap_or("luminance");
    let (invert, strength) =
        (q["invert"] == true, q["strength"].as_f64().unwrap_or(1.0).clamp(0.0, 1.0) as f32);
    let color = tint(q["tint"].as_str().unwrap_or("#ffffff"));
    let stamp = meta
        .modified()
        .ok()
        .and_then(|m| m.duration_since(std::time::UNIX_EPOCH).ok())
        .map_or(0, |d| d.as_nanos());
    let key = format!(
        "{}|{}|{stamp}|{name}|{invert}|{strength}|{color:?}|{}",
        file.display(),
        meta.len(),
        doc["scene"]["assets"][id]["linear"]
    );
    let hash = wprs_format::sha256(key.as_bytes()).ok()?;
    let out = cache().join(format!("{hash}.png"));
    if out.is_file() {
        return Some(json!({ "path": out }));
    }
    let mut picture = crate::pictures::asset(dir, doc, id).ok()?.to_rgba8();
    if picture.width().max(picture.height()) > EDGE {
        let k = f64::from(EDGE) / f64::from(picture.width().max(picture.height()));
        let (w, h) = (
            (f64::from(picture.width()) * k).max(1.0) as u32,
            (f64::from(picture.height()) * k).max(1.0) as u32,
        );
        picture = imageops::thumbnail(&picture, w, h);
    }
    let mut shown = RgbaImage::new(picture.width(), picture.height());
    for (to, from) in shown.pixels_mut().zip(picture.pixels()) {
        let v = (value(from, name) / 255.0).clamp(0.0, 1.0);
        let v = if invert { 1.0 - v } else { v };
        *to = Rgba([color[0], color[1], color[2], (v * strength * 0.7 * 255.0).round() as u8]);
    }
    std::fs::create_dir_all(cache()).ok()?;
    let part = cache().join(format!("{hash}.part"));
    shown.save_with_format(&part, image::ImageFormat::Png).ok()?;
    std::fs::rename(&part, &out).ok()?;
    pruned(&cache());
    Some(json!({ "path": out }))
}
