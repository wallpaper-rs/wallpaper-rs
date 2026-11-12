use super::gizmo::seen_at;
use crate::action::{Carve, SculptStroke};
use crate::ops::{Op, escape, set};
use anyhow::{Result, anyhow, ensure};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;
use wprs_core::message::said;
use wprs_scene::{Carved, MAX_SCULPTS, Model, Quality, Runtime, Values, carve_decode, carve_encode};

pub const MAX_STROKE: usize = 256;
pub const MAX_STRENGTH: f32 = 10.0;

type Moved = BTreeMap<u32, [f32; 3]>;

fn checked(s: &SculptStroke) -> Result<()> {
    let named = !s.shape.is_empty()
        && s.shape.len() <= 64
        && s.shape.chars().all(|c| c.is_alphanumeric() || "_-".contains(c));
    ensure!(
        named,
        said("editor.sculpt_name", "a sculpted shape has a name of 1 to 64 letters, digits, _ or -", &[])
    );
    let drawn =
        (1..=MAX_STROKE).contains(&s.points.len()) && s.points.iter().flatten().all(|v| v.is_finite());
    ensure!(drawn, said("editor.stroke_points", "a stroke has 1 to %1 points", &[&MAX_STROKE]));
    ensure!(
        s.radius.is_finite() && (1.0..=4096.0).contains(&s.radius),
        said("editor.sculpt_radius", "the brush has a radius of 1 to 4096 pixels", &[])
    );
    let most = if s.mode == Carve::Smooth { 1.0 } else { MAX_STRENGTH };
    ensure!(
        s.strength.is_finite() && s.strength > 0.0 && s.strength <= most,
        said("editor.sculpt_strength", "the strength goes above 0 and up to %1", &[&most])
    );
    ensure!(
        s.softness.is_none_or(|f| (0.0..=1.0).contains(&f)),
        said("editor.sculpt_soft", "the softness goes from 0 to 1", &[])
    );
    Ok(())
}

fn source(model: &Model, v: u32) -> Option<(u16, u16, u32)> {
    let rig = model.rig.as_ref()?;
    let piece = rig.pieces.iter().find(|p| (p.base..p.base + p.vertices).contains(&v))?;
    Some((piece.source[0], piece.source[1], v - piece.base))
}

fn global(model: &Model, c: &Carved) -> Option<u32> {
    let piece = model.rig.as_ref()?.pieces.iter().find(|p| p.source == [c.node, c.primitive])?;
    (c.vertex < piece.vertices).then_some(piece.base + c.vertex)
}

pub(crate) fn sculpt(dir: &Path, doc: &Value, s: &SculptStroke) -> Result<Vec<Op>> {
    checked(s)?;
    let (path, layer) = super::edit::model_layer(doc, &s.layer)?;
    let id = layer["model"].as_str().unwrap_or_default().to_owned();
    let (file, _) = super::store::file_as(&super::store::path(dir, doc, &id)?, true)?;
    let (_, _, rig) = crate::skin::rigged(dir, doc, layer)?;
    let carved = super::carved::shapes(dir, doc, &id);
    let own = carved.iter().find(|(n, _)| *n == s.shape).map(|(_, p)| p.clone());
    let taken = rig.shape(&s.shape).is_some() && own.is_none();
    ensure!(!taken, said("editor.sculpt_taken", "the file already has a blend shape named %1", &[&s.shape]));
    ensure!(
        own.is_some() || carved.len() < MAX_SCULPTS,
        said("editor.sculpts_most", "a model has at most %1 sculpted shapes", &[&MAX_SCULPTS])
    );
    let at = format!("{path}/shapes");
    let scene = super::store::scene(dir, doc)
        .ok_or_else(|| anyhow!(said("editor.no_scene", "the scene cannot be read", &[])))?;
    let size = scene.canvas.size.map(|v| v as f32);
    let mut rt = Runtime::new(Arc::new(scene), Values::new());
    rt.advance(s.time.max(0.0));
    let frame = rt.frame([0.0, 0.0, size[0], size[1]], Quality::Q3);
    let p = frame
        .placed
        .iter()
        .find(|p| p.layer.id == s.layer)
        .ok_or_else(|| anyhow!(said("editor.not_shown", "%1 is not shown at this time", &[&s.layer])))?;
    let seen = seen_at(doc, &frame, p.stage)
        .ok_or_else(|| anyhow!(said("editor.not_shown", "%1 is not shown at this time", &[&s.layer])))?;
    let model: &Model = &file;
    let mut moved: Moved = BTreeMap::new();
    if let Some(bytes) = own.as_ref().and_then(|p| std::fs::read(p).ok()) {
        for c in carve_decode(&bytes).map_err(|e| anyhow!("{e}"))? {
            global(model, &c).into_iter().for_each(|v| _ = moved.insert(v, c.at));
        }
    }
    let palette = p.pose.map_or(&model.rest[..], |q| &q.palette[..]);
    let reached = super::carving::reached(
        model,
        palette,
        (&seen, &p.matrix),
        (&s.points, s.radius, s.softness.unwrap_or(0.5)),
        &moved,
    );
    super::carving::apply(model, s, &reached, &mut moved);
    let entries = super::carving::entries(model, &moved, |v| source(model, v));
    let folder = Path::new(
        doc.pointer(&format!("/scene/assets/{}/path", escape(&id)))
            .and_then(Value::as_str)
            .unwrap_or_default(),
    )
    .parent()
    .map(Path::to_path_buf)
    .unwrap_or_default();
    let stem =
        |n: usize| folder.join(format!("{id}-{}-{n}.wpsh", s.shape)).to_string_lossy().replace('\\', "/");
    let saved = (1..).map(stem).find(|f| !dir.join(f).exists()).unwrap_or_default();
    std::fs::write(dir.join(&saved), carve_encode(&entries))?;
    let mut ops =
        set(doc, &format!("/scene/assets/{}/shapes/{}", escape(&id), escape(&s.shape)), json!(saved));
    if layer["shapes"].get(&s.shape).is_none() {
        ops.extend(set(doc, &format!("{at}/{}", escape(&s.shape)), json!(1)));
    }
    Ok(ops)
}
