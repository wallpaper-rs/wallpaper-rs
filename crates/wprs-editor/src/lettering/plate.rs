use super::set::{chain, layout};
use crate::keys::value_at;
use image::{Rgba, RgbaImage};
use serde_json::Value;
use std::path::Path;
use wprs_scene::{Align, Color};

fn backing(layer: &Value, t: f64) -> Option<(f32, f32)> {
    let back = &layer["background"];
    if !back.is_object() {
        return None;
    }
    let color = serde_json::from_value::<Color>(value_at(&back["color"], t)).map_or(1.0, |c| c.0[3]);
    let radius = value_at(&back["radius"], t).as_f64().unwrap_or(0.0) as f32;
    Some((color.clamp(0.0, 1.0), radius.max(0.0)))
}

fn plate(image: &mut RgbaImage, inset: f32, (alpha, radius): (f32, f32)) {
    let (w, h) = (image.width() as f32, image.height() as f32);
    let inner = [(w - 2.0 * inset).max(1.0), (h - 2.0 * inset).max(1.0)];
    let r = radius.min(inner[0].min(inner[1]) / 2.0);
    let half = inner.map(|v| v / 2.0);
    let center = [w / 2.0, h / 2.0];
    let height = alpha * 0.5 * 255.0;
    for (x, y, p) in image.enumerate_pixels_mut() {
        let d = [0, 1].map(|k| ([x, y][k] as f32 + 0.5 - center[k]).abs() - (half[k] - r));
        let out = d[0].max(0.0).hypot(d[1].max(0.0)) + d[0].max(d[1]).min(0.0) - r;
        let cover = ((0.5 - out).clamp(0.0, 1.0) * height).round() as u8;
        *p = Rgba([255, 255, 255, p.0[3].max(cover)]);
    }
}

pub(crate) fn picture(dir: &Path, doc: &Value, layer: &Value, t: f64) -> Option<RgbaImage> {
    let fonts = chain(dir, doc, layer);
    let size = layer["size"].as_f64().unwrap_or(48.0) as f32;
    let align = serde_json::from_value::<Align>(layer["align"].clone()).unwrap_or_default();
    let set = layout(layer, t);
    let mut image = wprs_type::raster(&fonts, &super::shown(layer, t), size, align, 1.0, &set)?;
    if let Some(back) = backing(layer, t) {
        plate(&mut image, set.padding, back);
    }
    Some(image)
}
