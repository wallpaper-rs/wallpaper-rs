use super::surface::{Hit, dot};
use super::turns::{apply, invert, of_mat4};
use crate::action::CombStroke;
use crate::ops::{Op, escape, set};
use crate::paint::{POINTS, Stroke, stroke};
use anyhow::{Result, ensure};
use image::{Rgba, RgbaImage};
use serde_json::{Value, json};
use std::path::Path;
use wprs_core::message::said;

const SIDE: u32 = 512;
const SEAM: f32 = 0.25;

fn combing(a: &Hit, b: &Hit, strength: f32) -> Option<[f32; 3]> {
    let d: [f32; 3] = std::array::from_fn(|i| b.point[i] - a.point[i]);
    let n = a.normal;
    let along: [f32; 3] = std::array::from_fn(|i| d[i] - n[i] * dot(d, n));
    let long = dot(along, along).sqrt();
    let m = of_mat4(&a.matrix);
    let grow = (0..3).map(|c| (0..3).map(|r| m[r][c] * m[r][c]).sum::<f32>().sqrt()).fold(0.0, f32::max);
    let local = apply(&invert(&m)?, along.map(|x| x / long.max(1e-9) * strength * grow));
    (long > 1e-6).then(|| local.map(|x| x.clamp(-1.0, 1.0)))
}

pub(crate) fn comb(dir: &Path, doc: &Value, c: &CombStroke) -> Result<Vec<Op>> {
    let (path, layer) = super::edit::model_layer(doc, &c.layer)?;
    let at = format!("{path}/materials/{}", escape(&c.name));
    ensure!(
        layer["materials"][&c.name]["shader"] == "fur",
        said("editor.comb_fur", "only a fur material is combed", &[])
    );
    let fits = (2..=POINTS).contains(&c.points.len()) && c.points.iter().flatten().all(|p| p.is_finite());
    ensure!(
        fits && c.radius.is_finite() && c.radius > 0.0 && (0.0..=1.0).contains(&c.strength),
        said(
            "editor.comb_stroke",
            "a comb stroke has 2 to %1 points, a radius in pixels and a strength from 0 to 1",
            &[&POINTS]
        )
    );
    let hits = super::surface::hits(dir, doc, c.time, &c.points);
    let mine = |h: &Option<Hit>| h.as_ref().filter(|h| h.layer == c.layer && h.material == c.name).cloned();
    let on: Vec<Option<Hit>> = hits.iter().map(mine).collect();
    let pairs: Vec<(Hit, Hit)> = on
        .windows(2)
        .filter_map(|w| Some((w[0].clone()?, w[1].clone()?)))
        .filter(|(a, b)| (a.uv[0] - b.uv[0]).abs().max((a.uv[1] - b.uv[1]).abs()) < SEAM)
        .collect();
    ensure!(!pairs.is_empty(), said("editor.comb_missed", "the stroke does not cross that material", &[]));
    let asset = |id: &str| format!("/scene/assets/{}", escape(id));
    let block = doc.pointer(&at).cloned().unwrap_or_else(|| json!({}));
    let own = block["maps"]["comb"].as_str().filter(|id| doc.pointer(&asset(id)).is_some()).map(String::from);
    let old = own.as_deref().and_then(|id| crate::pictures::asset(dir, doc, id).ok());
    let mut picture =
        old.map_or_else(|| RgbaImage::from_pixel(SIDE, SIDE, Rgba([128, 128, 128, 0])), |p| p.into_rgba8());
    for (a, b) in &pairs {
        let Some(d) = combing(a, b, c.strength) else { continue };
        let start = a.uv.map(|x| x.rem_euclid(1.0));
        let end = [start[0] + b.uv[0] - a.uv[0], start[1] + b.uv[1] - a.uv[1]];
        let radius = c.radius * a.unit * a.uv_scale;
        let values = [d[0] * 0.5 + 0.5, d[1] * 0.5 + 0.5, d[2] * 0.5 + 0.5, if c.erase { 0.0 } else { 1.0 }];
        let channels = if c.erase { 3..4 } else { 0..4 };
        for k in channels {
            let one = Stroke {
                pointer: &at,
                channel: Some(k),
                radius,
                hardness: c.hardness,
                value: values[k],
                points: &[start, end],
            };
            stroke(&mut picture, &one);
        }
    }
    let id = own.unwrap_or_else(|| crate::layers::unique(doc, &format!("{}_comb", c.name)));
    let like = doc.pointer(&format!("{}/path", asset(&id))).and_then(Value::as_str).map(|p| dir.join(p));
    let saved = crate::pictures::store(dir, ("maps", &id), &picture, like.as_deref())?;
    let mut ops = set(doc, &asset(&id), json!({ "path": saved, "linear": true }));
    let mut block = block;
    block["maps"]["comb"] = json!(id);
    ops.extend(set(doc, &at, block));
    Ok(ops)
}
