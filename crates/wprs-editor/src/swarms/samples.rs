use image::RgbaImage;
use serde_json::{Value, json};
use std::path::Path;

fn solid(image: &RgbaImage, threshold: f64, place: impl Fn(f64, f64) -> [f64; 2]) -> Value {
    let (w, h) = (f64::from(image.width()), f64::from(image.height()));
    json!(
        image
            .enumerate_pixels()
            .filter(|(_, _, p)| f64::from(p.0[3]) / 255.0 > threshold)
            .map(|(x, y, _)| place((f64::from(x) + 0.5) / w, (f64::from(y) + 0.5) / h))
            .collect::<Vec<_>>()
    )
}

fn pixels(dir: &Path, doc: &Value, layer: &Value, field: &str, t: f64) -> Option<RgbaImage> {
    let asset = crate::keys::value_at(&layer[field], t);
    let path = asset.as_str().and_then(|a| doc["scene"]["assets"][a]["path"].as_str())?;
    crate::pictures::open(dir.join(path)).ok().map(|p| p.thumbnail(48, 48).to_rgba8())
}

pub(super) fn samples(dir: &Path, doc: &Value, v: &Value, t: f64) -> Value {
    let id = v["id"].as_str().unwrap_or_default();
    let threshold = v["threshold"].as_f64().unwrap_or(0.5);
    let mut out = json!({ "kind": "layer", "id": id, "threshold": threshold, "color": v["color"], "inherit": v["inherit"],
        "jitter": v["jitter"], "refresh": v.get("refresh").unwrap_or(&json!("static")), "samples": [], "outline": [] });
    let Some(g) = crate::canvas::gizmos(dir, doc, t).into_iter().find(|g| g.id == id) else { return out };
    let (s, c) = g.rotation.to_radians().sin_cos();
    let at = [g.offset[0] + g.position[0], g.offset[1] + g.position[1]];
    let [w, h] = [g.size[0] * g.scale[0], g.size[1] * g.scale[1]];
    let place = |u: f64, v: f64| {
        let [x, y] = [(u - 0.5) * w, (v - 0.5) * h];
        [((at[0] + x * c - y * s) * 100.0).round() / 100.0, ((at[1] + x * s + y * c) * 100.0).round() / 100.0]
    };
    out["outline"] = json!([place(0.0, 0.0), place(1.0, 0.0), place(1.0, 1.0), place(0.0, 1.0)]);
    let layer = crate::actions::layer(doc, id).map(|(_, l)| l.clone()).unwrap_or_default();
    if layer["type"] == "group" {
        let kids = doc.pointer(&format!("{}/layers", g.pointer)).and_then(Value::as_array);
        let kids = kids.cloned().unwrap_or_default();
        let mut approximate = kids.is_empty();
        let mut all = Vec::new();
        for kid in &kids {
            let Some(kid_id) = kid["id"].as_str() else { continue };
            let sampled = samples(dir, doc, &json!({ "id": kid_id, "threshold": threshold }), t);
            approximate |= sampled["approximate"] == true;
            all.extend(sampled["samples"].as_array().into_iter().flatten().cloned());
        }
        out["samples"] = json!(all);
        if approximate {
            out["approximate"] = json!(true);
        }
        return out;
    }
    let field = match layer["type"].as_str().unwrap_or_default() {
        "image" => "image",
        "mesh" => "texture",
        _ => "",
    };
    if let Some(image) = (!field.is_empty()).then(|| pixels(dir, doc, &layer, field, t)).flatten() {
        out["samples"] = solid(&image, threshold, place);
        return out;
    }
    let text = (layer["type"] == "text")
        .then(|| crate::texts::words(doc, &crate::keys::value_at(&layer["text"], t)))
        .flatten();
    let font = layer["font"].as_str().unwrap_or("");
    if let Some(image) = text.and_then(|s| super::glyphs::raster(dir, doc, font, &s)) {
        out["samples"] = solid(&image, threshold, place);
        return out;
    }
    out["approximate"] = json!(true);
    out
}
