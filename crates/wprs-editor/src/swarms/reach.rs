use super::shapes::Frame;
use image::RgbaImage;
use serde_json::{Value, json};
use std::path::Path;

const SIDE: f64 = 56.0;
const SHOWN: usize = 24;

pub(super) fn field(
    small: &RgbaImage,
    threshold: f32,
    unit: f64,
    place: &dyn Fn(f64, f64) -> [f64; 2],
) -> Vec<Value> {
    let (sw, sh) = (small.width() + 2, small.height() + 2);
    let mut rgba = vec![0u8; (sw * sh * 4) as usize];
    for (x, y, p) in small.enumerate_pixels() {
        rgba[(((y + 1) * sw + x + 1) * 4 + 3) as usize] = p.0[3];
    }
    let (cells, d) = wprs_scene::distance_field(sw, sh, &rgba, threshold, [sw as f32, sh as f32]);
    let step = cells[0].max(cells[1]).div_ceil(SHOWN).max(1);
    let rows = (0..cells[1]).step_by(step).flat_map(|y| (0..cells[0]).step_by(step).map(move |x| (x, y)));
    rows.map(|(x, y)| {
        let (u, v) =
            ((x as f64 - 0.5) / f64::from(small.width()), (y as f64 - 0.5) / f64::from(small.height()));
        let [px, py] = place(u, v);
        json!([px, py, (f64::from(d[y * cells[0] + x]) * unit * 10.0).round() / 10.0])
    })
    .collect()
}

pub(super) fn points(solid: &[[f64; 2]]) -> Vec<Value> {
    let from = solid.iter().fold([f64::MAX; 2], |m, p| [m[0].min(p[0]), m[1].min(p[1])]);
    let to = solid.iter().fold([f64::MIN; 2], |m, p| [m[0].max(p[0]), m[1].max(p[1])]);
    if solid.is_empty() || from[0] > to[0] {
        return Vec::new();
    }
    let cell = ((to[0] - from[0]).max(to[1] - from[1]) / SIDE).max(1e-3);
    let origin = [from[0] - 2.0 * cell, from[1] - 2.0 * cell];
    let size = [0, 1].map(|i| ((to[i] - from[i]) / cell).ceil() as u32 + 5);
    let mut image = RgbaImage::new(size[0], size[1]);
    for p in solid {
        let (cx, cy) = (((p[0] - origin[0]) / cell) as i64, ((p[1] - origin[1]) / cell) as i64);
        for (x, y) in
            (-1..=1).flat_map(|dy| (-1..=1).map(move |dx| (dx, dy))).map(|(dx, dy)| (cx + dx, cy + dy))
        {
            if x >= 0 && y >= 0 && x < i64::from(size[0]) && y < i64::from(size[1]) {
                image.get_pixel_mut(x as u32, y as u32).0 = [255; 4];
            }
        }
    }
    let place = |u: f64, v: f64| {
        let at = |c: usize, t: f64| origin[c] + t * f64::from(size[c]) * cell;
        [(at(0, u) * 100.0).round() / 100.0, (at(1, v) * 100.0).round() / 100.0]
    };
    field(&image, 0.5, cell, &place)
}

pub(super) fn mask(dir: &Path, doc: &Value, v: &Value, f: &Frame, canvas: [f64; 2]) -> Value {
    let threshold = v["threshold"].as_f64().unwrap_or(0.5);
    let corners = [[0.0, 0.0], [canvas[0], 0.0], canvas, [0.0, canvas[1]]].map(|p| f.round(f.apply(p)));
    let grip = f.round([0, 1].map(|i| corners[0][i] + (corners[1][i] - corners[0][i]) * threshold));
    let mut out = json!({ "texture": v["texture"], "threshold": threshold, "corners": corners, "grip": grip, "field": [], "approximate": true });
    let path = v["texture"].as_str().and_then(|a| doc["scene"]["assets"][a]["path"].as_str());
    let Some(picture) = path.and_then(|p| crate::pictures::open(dir.join(p)).ok()) else { return out };
    let small = picture.thumbnail(62, 62).to_rgba8();
    let unit = (canvas[0] / f64::from(small.width())).min(canvas[1] / f64::from(small.height())) * f.scale();
    let place = |u: f64, w: f64| f.round(f.apply([u * canvas[0], w * canvas[1]]));
    out["field"] = json!(field(&small, threshold as f32, unit, &place));
    out["approximate"] = json!(false);
    out
}
