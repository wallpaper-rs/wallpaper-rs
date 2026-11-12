use crate::action::MaterialStroke;
use crate::ops::{Op, escape, set};
use crate::paint::{POINTS, Stroke, stroke};
use anyhow::{Result, anyhow, ensure};
use image::{Rgba, RgbaImage};
use serde_json::{Value, json};
use std::path::Path;
use wprs_core::message::said;
use wprs_scene::{MAP_SLOTS, Model};

const SIDE: u32 = 512;

fn start(dir: &Path, doc: &Value, model: &Model, (name, k): (&str, usize), own: Option<&str>) -> RgbaImage {
    if let Some(picture) = own.and_then(|id| crate::pictures::asset(dir, doc, id).ok()) {
        return picture.into_rgba8();
    }
    let file = model.material(name).and_then(|m| model.materials[m].maps[k]);
    let bytes = file.and_then(|t| model.textures.get(usize::from(t))).map(|t| t.bytes());
    if let Some(picture) = bytes.and_then(|b| image::load_from_memory(b).ok()) {
        return picture.into_rgba8();
    }
    let blank = if k == 2 { [128, 128, 255, 255] } else { [255; 4] };
    RgbaImage::from_pixel(SIDE, SIDE, Rgba(blank))
}

pub(crate) fn paint(dir: &Path, doc: &Value, s: &MaterialStroke) -> Result<Vec<Op>> {
    let (path, layer) = super::edit::model_layer(doc, &s.layer)?;
    let (model, _) = super::store::asset(dir, doc, layer["model"].as_str().unwrap_or_default())?;
    ensure!(
        model.material(&s.name).is_some(),
        said("editor.material_name", "the model has no material named %1", &[&s.name])
    );
    let k = MAP_SLOTS.iter().position(|m| *m == s.slot).ok_or_else(|| {
        anyhow!(said(
            "editor.map_slot",
            "%1 is not a map of a material; use base, orm, normal, occlusion or emissive",
            &[&s.slot]
        ))
    })?;
    let fits = (1..=POINTS).contains(&s.points.len()) && s.color.iter().all(|c| c.is_finite());
    ensure!(
        fits && s.radius.is_finite() && s.points.iter().flatten().all(|p| p.is_finite()),
        said("editor.map_stroke", "a stroke of a map has 1 to %1 points and a color of numbers", &[&POINTS])
    );
    let at = format!("{path}/materials/{}", escape(&s.name));
    let mut block = doc.pointer(&at).cloned().unwrap_or_else(|| json!({}));
    let asset = |id: &str| format!("/scene/assets/{}", escape(id));
    let own =
        block["maps"][&s.slot].as_str().filter(|id| doc.pointer(&asset(id)).is_some()).map(String::from);
    let mut picture = start(dir, doc, &model, (&s.name, k), own.as_deref());
    for (c, value) in s.color.iter().enumerate() {
        let one = Stroke {
            pointer: &at,
            channel: Some(c),
            radius: s.radius,
            hardness: s.hardness,
            value: *value,
            points: &s.points,
            direction: false,
        };
        stroke(&mut picture, &one);
    }
    let id = own.unwrap_or_else(|| crate::layers::unique(doc, &format!("{}_{}", s.name, s.slot)));
    let like = doc.pointer(&format!("{}/path", asset(&id))).and_then(Value::as_str).map(|p| dir.join(p));
    let saved = crate::pictures::store(dir, ("maps", &id), &picture, like.as_deref())?;
    let mut ops = set(doc, &asset(&id), json!({ "path": saved, "linear": super::maps::data_slot(&s.slot) }));
    block["maps"][&s.slot] = json!(id);
    ops.extend(set(doc, &at, block));
    Ok(ops)
}
