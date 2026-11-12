use super::surface::Hit;
use crate::ops::{Op, escape};
use anyhow::{Result, anyhow, ensure};
use serde_json::{Value, json};
use std::path::Path;
use wprs_core::message::said;
use wprs_scene::{Color, MapWrap};

fn linear(c: f32) -> f32 {
    if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
}

fn encode(c: f32) -> u8 {
    let c = c.clamp(0.0, 1.0);
    let s = if c <= 0.003_130_8 { c * 12.92 } else { 1.055 * c.powf(1.0 / 2.4) - 0.055 };
    (s * 255.0).round() as u8
}

fn wrapped(x: f32, w: MapWrap) -> f32 {
    match w {
        MapWrap::Repeat => x.rem_euclid(1.0),
        MapWrap::Clamp => x.clamp(0.0, 1.0),
        MapWrap::Mirror => {
            let k = x.rem_euclid(2.0);
            if k > 1.0 { 2.0 - k } else { k }
        }
    }
}

pub(crate) fn albedo(dir: &Path, doc: &Value, h: &Hit) -> Option<[f32; 3]> {
    let (_, layer) = crate::actions::layer(doc, &h.layer).ok()?;
    let (model, _) = super::store::asset(dir, doc, layer["model"].as_str()?).ok()?;
    let index = model.material(&h.material);
    let defaults = wprs_scene::Finish::default();
    let f = index.map_or(&defaults, |i| &model.materials[i]);
    let set = &layer["materials"][&h.material];
    let own = set["maps"]["base"].as_str().and_then(|id| crate::pictures::asset(dir, doc, id).ok());
    let file = || {
        let t = model.textures.get(usize::from(f.maps[0]?))?;
        image::load_from_memory(t.bytes()).ok()
    };
    let picture = own.or_else(file).map(|p| p.into_rgba8());
    let uv = f.uvs[0].apply(if f.uvs[0].set == 1 { h.uv1 } else { h.uv });
    let texel = picture.map_or([1.0; 3], |p| {
        let (w, hh) = p.dimensions();
        let wrap = f.uvs[0].sampling.wrap;
        let x = (wrapped(uv[0], wrap[0]) * w as f32).min(w as f32 - 1.0) as u32;
        let y = (wrapped(uv[1], wrap[1]) * hh as f32).min(hh as f32 - 1.0) as u32;
        let px = p.get_pixel(x, y).0;
        std::array::from_fn(|i| linear(f32::from(px[i]) / 255.0))
    });
    let tint = serde_json::from_value::<Color>(set["color"].clone()).map_or([1.0; 4], |c| c.0);
    Some(std::array::from_fn(|i| linear(f.color[i]) * texel[i] * linear(tint[i])))
}

pub(crate) fn color_at(dir: &Path, doc: &Value, h: &Hit) -> Value {
    albedo(dir, doc, h).map_or(Value::Null, |c| {
        let [r, g, b] = c.map(encode);
        json!(format!("#{r:02x}{g:02x}{b:02x}"))
    })
}

pub(crate) fn key_from(dir: &Path, doc: &Value, k: &crate::action::ChromaPick) -> Result<Vec<Op>> {
    let (path, _) = super::edit::model_layer(doc, &k.layer)?;
    let at = format!("{path}/materials/{}", escape(&k.name));
    ensure!(
        doc.pointer(&format!("{at}/shader")).and_then(Value::as_str) == Some("chroma"),
        said("editor.key_chroma", "only a chroma material takes a key color", &[])
    );
    let hits = super::surface::hits(dir, doc, k.time, &[k.at]);
    let hit = hits.into_iter().flatten().next().filter(|h| h.layer == k.layer && h.material == k.name);
    let hit = hit
        .ok_or_else(|| anyhow!(said("editor.key_missed", "the point does not fall on that material", &[])))?;
    let color = color_at(dir, doc, &hit);
    ensure!(color.is_string(), said("editor.key_missed", "the point does not fall on that material", &[]));
    super::material::material(
        dir,
        doc,
        &crate::action::MaterialEdit {
            layer: k.layer.clone(),
            name: k.name.clone(),
            key: Some(color),
            ..Default::default()
        },
    )
}
