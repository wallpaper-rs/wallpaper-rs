use super::layers::transform;
use crate::action::{Grip, LayerEdit as L};
use crate::actions::layer;
use crate::ops::{Op, apply, set};
use crate::seats::grips::{Moved, decompose, fit, grip, tidy};
use crate::seats::{canvas_of, parents, seats};
use anyhow::{Result, anyhow, ensure};
use serde_json::{Value, json};
use std::path::Path;
use wprs_core::message::said;

fn written(doc: &Value, path: &str, m: Moved, at: Option<f64>, replace: bool) -> Result<Vec<Op>> {
    let pair = |v: Option<[f64; 2]>| v.map(|v| json!(v));
    let fields = [
        ("position", pair(m.position)),
        ("rotation", m.rotation.map(|r| json!(r))),
        ("scale", pair(m.scale)),
        ("skew", pair(m.skew)),
        ("origin", pair(m.origin)),
        ("tilt", pair(m.tilt)),
        ("z", m.z.map(|z| json!(z))),
    ];
    transform(doc, path, fields, at, replace)
}

fn found<'a>(all: &'a [crate::seats::Seat], id: &str) -> Result<&'a crate::seats::Seat> {
    let missing =
        || anyhow!(said("editor.grip_unplaced", "the layer %1 is not placed on the canvas", &[&id]));
    all.iter().find(|s| s.id == id).ok_or_else(missing)
}

fn reparent(
    dir: &Path,
    doc: &Value,
    id: &str,
    to: Option<&str>,
    (keep, t): (bool, f64),
) -> Result<(Vec<Op>, Moved)> {
    let (path, _) = layer(doc, id)?;
    let to = to.filter(|p| !p.is_empty());
    if let Some(p) = to {
        ensure!(
            parents(doc, id).iter().any(|c| c == p),
            said(
                "editor.parent_refused",
                "%1 cannot follow %2: it is itself, not drawn, follows %1 already or the chain would pass 8 layers",
                &[&id, &p]
            )
        );
    }
    let pointer = format!("{path}/parent");
    let ops = match to {
        Some(p) => set(doc, &pointer, json!(p)),
        None => doc.pointer(&pointer).map(|_| Op::Remove { path: pointer.clone() }).into_iter().collect(),
    };
    if !keep {
        return Ok((ops, Moved::default()));
    }
    let before = found(&seats(dir, doc, t), id)?.world;
    let mut staged = doc.clone();
    apply(&mut staged, &ops)?;
    let after = found(&seats(dir, &staged, t), id)?.frame;
    let local = after.inverse().then(before);
    let [a, b, c, d, e, f] = local.0;
    let (rotation, skew, scale) = decompose([a, b, c, d]);
    let position = [f64::from(e), f64::from(f)];
    Ok((
        ops,
        tidy(Moved {
            position: Some(position),
            rotation: Some(rotation),
            scale: Some(scale),
            skew: Some(skew),
            ..Moved::default()
        }),
    ))
}

pub(super) fn edit(dir: &Path, doc: &Value, e: &L) -> Result<Vec<Op>> {
    match e {
        L::Grip { layer: id, grip: Grip::Size { corner }, to, at, .. } => {
            let (path, _) = layer(doc, id)?;
            let all = seats(dir, doc, at.unwrap_or(0.0));
            let me = found(&all, id)?;
            let size = crate::onscreen::resize(me, canvas_of(doc), *corner, me.unlean(*to))?;
            super::refused(dir, doc, set(doc, &format!("{path}/size"), json!(size)))
        }
        L::Grip { layer: id, grip: g, to, snap, at, replace } => {
            let (path, _) = layer(doc, id)?;
            let t = at.unwrap_or(0.0);
            let all = seats(dir, doc, t);
            let moved = grip(&all, doc, found(&all, id)?, g, *to, *snap, t)?;
            written(doc, &path, moved, *at, *replace)
        }
        L::Fit { layer: id, fit: how, at, replace } => {
            let (path, _) = layer(doc, id)?;
            let t = at.unwrap_or(0.0);
            let all = seats(dir, doc, t);
            let moved = fit(doc, found(&all, id)?, *how, t)?;
            written(doc, &path, moved, *at, *replace)
        }
        L::Parent { layer: id, parent, keep, at, replace } => {
            let (path, _) = layer(doc, id)?;
            let (mut ops, moved) = reparent(dir, doc, id, parent.as_deref(), (*keep, at.unwrap_or(0.0)))?;
            let mut staged = doc.clone();
            apply(&mut staged, &ops)?;
            let more = written(&staged, &path, moved, *at, *replace)?;
            ops.extend(more);
            Ok(ops)
        }
        _ => Ok(Vec::new()),
    }
}
