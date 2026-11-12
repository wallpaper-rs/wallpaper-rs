use image::RgbaImage;
use serde_json::{Value, json};
use std::path::Path;

const SIDE: u32 = 128;
const EDGE: u32 = 512;

fn path_of<'a>(doc: &'a Value, id: &str) -> Option<&'a str> {
    doc.pointer(&format!("/scene/assets/{}/path", crate::ops::escape(id))).and_then(Value::as_str)
}

fn small(image: image::DynamicImage) -> RgbaImage {
    image.thumbnail(SIDE, SIDE).into_rgba8()
}

fn picture(dir: &Path, doc: &Value, id: &str) -> Option<RgbaImage> {
    Some(small(crate::pictures::asset(dir, doc, id).ok()?))
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
    let plain = l["effects"].as_array().is_none_or(Vec::is_empty) && l.get("mask").is_none();
    match l["type"].as_str()? {
        _ if !plain => drawn(dir, doc, id, t),
        "image" => picture(dir, doc, l["image"].as_str()?),
        "video" => frame(dir, doc, first_video(l)?, t),
        "text" => crate::lettering::picture(dir, doc, l, t).map(|i| small(i.into())),
        _ => drawn(dir, doc, id, t),
    }
}

fn seen_box(image: &RgbaImage) -> Option<[u32; 4]> {
    let solid = image.enumerate_pixels().filter(|(_, _, p)| p.0[3] > 0);
    solid.fold(None, |b: Option<[u32; 4]>, (x, y, _)| {
        Some(b.map_or([x, y, x, y], |[x0, y0, x1, y1]| [x0.min(x), y0.min(y), x1.max(x), y1.max(y)]))
    })
}

fn over(dir: &Path, doc: &Value, id: &str, (t, back): (f64, f64)) -> Option<RgbaImage> {
    let mut alone = doc.clone();
    alone["scene"]["ext"]["editor"]["solo"] = json!(id);
    alone["scene"]["canvas"]["background"] = json!([back, back, back, 1.0]);
    crate::shots::still(dir, &alone, EDGE, t).ok()
}

fn drawn(dir: &Path, doc: &Value, id: &str, t: f64) -> Option<RgbaImage> {
    let (dark, light) = (over(dir, doc, id, (t, 0.0))?, over(dir, doc, id, (t, 1.0))?);
    let mut image = RgbaImage::new(dark.width(), dark.height());
    for ((d, l), out) in dark.pixels().zip(light.pixels()).zip(image.pixels_mut()) {
        let gap = (0..3).map(|k| i32::from(l.0[k]) - i32::from(d.0[k])).max().unwrap_or(255);
        let a = (255 - gap).clamp(0, 255) as u8;
        let color = |k: usize| (u32::from(d.0[k]) * 255 / u32::from(a.max(1))).min(255) as u8;
        *out = if a == 0 { image::Rgba([0; 4]) } else { image::Rgba([color(0), color(1), color(2), a]) };
    }
    let [x0, y0, x1, y1] = seen_box(&image)?;
    let cut = image::imageops::crop_imm(&image, x0, y0, x1 - x0 + 1, y1 - y0 + 1).to_image();
    Some(small(cut.into()))
}

pub(super) fn cast(dir: &Path, doc: &Value, texture: &Value, kind: Option<&str>, t: f64) -> Option<String> {
    let image = match (texture, kind) {
        (Value::String(id), Some("picture")) => picture(dir, doc, id),
        (Value::String(id), Some("video")) => frame(dir, doc, id, t),
        (v, _) => layer(dir, doc, v["layer"].as_str()?, t),
    };
    image.map(|i| crate::rigs::url(&i))
}
