use super::place::{Placed, open};
use crate::action::EffectsEdit as E;
use crate::ops::Op;
use anyhow::{Result, anyhow};
use serde_json::{Map, Value, json};
use std::path::Path;
use wprs_core::message::said;
use wprs_scene::{Grab, Hold};

fn round(v: f64, step: f64) -> f64 {
    (v / step).round() * step
}

fn clamp(v: f64, range: [f32; 2]) -> f64 {
    v.clamp(f64::from(range[0]), f64::from(range[1]))
}

fn turned(angle: f64, now: f64, range: [f32; 2]) -> f64 {
    let (lo, hi) = (f64::from(range[0]), f64::from(range[1]));
    let turns = (-6..=6).map(|k| angle + 360.0 * f64::from(k)).filter(|v| (lo..=hi).contains(v));
    turns.min_by(|a, b| (a - now).abs().total_cmp(&(b - now).abs())).unwrap_or_else(|| angle.clamp(lo, hi))
}

fn moved(placed: &Placed, h: &Hold, to: [f64; 2], t: f64) -> Result<Vec<f64>> {
    let u = placed.seat.fraction(to);
    let now = placed.value(h, t);
    let aim = || -> Result<([f64; 2], [f64; 2])> {
        let reach = placed.reach().ok_or_else(|| {
            anyhow!(said(
                "editor.grip_no_size",
                "the layer %1 has no size to hold by a corner or an edge",
                &[&placed.seat.id]
            ))
        })?;
        let anchor = placed.anchor(t);
        let size = placed.seat.size;
        Ok(([(u[0] - anchor[0]) * size[0], (u[1] - anchor[1]) * size[1]], reach))
    };
    Ok(match h.kind {
        Grab::Point | Grab::Corner => u.map(|v| round(clamp(v, h.range), 1e-3)).to_vec(),
        Grab::Vector => {
            let (d, r) = aim()?;
            [0, 1].map(|i| round(clamp(d[i] / r[i], h.range), 1e-3)).to_vec()
        }
        Grab::Angle => {
            let (d, _) = aim()?;
            vec![round(turned(d[1].atan2(d[0]).to_degrees(), now[0], h.range), 0.1)]
        }
    })
}

pub(crate) fn grip(
    dir: &Path,
    doc: &Value,
    (id, index, param): (&str, usize, &str),
    to: [f64; 2],
    both: bool,
    (t, replace): (f64, bool),
) -> Result<Vec<Op>> {
    let placed = open(dir, doc, id, index, t)?;
    let all = placed.handles();
    let missing = || {
        anyhow!(said(
            "editor.no_effect_handle",
            "the parameter %1 of the effect %2 has no handle on the canvas",
            &[&param, &placed.effect.id]
        ))
    };
    let h = all.iter().find(|h| h.param == param).ok_or_else(missing)?;
    let value = moved(&placed, h, to, t)?;
    let mut change = Map::new();
    let put = |h: &Hold, v: Vec<f64>| if h.kind == Grab::Angle { json!(v[0]) } else { json!(v) };
    change.insert(param.into(), put(h, value.clone()));
    if let Some(other) = h.partner.filter(|_| both).and_then(|p| all.iter().find(|o| o.param == p)) {
        let (was, rest) = (placed.value(h, t), placed.value(other, t));
        let next: Vec<f64> =
            (0..2).map(|i| round(clamp(rest[i] + value[i] - was[i], other.range), 1e-3)).collect();
        change.insert(other.param.into(), put(other, next));
    }
    for key in change.keys() {
        crate::shaping::plain(doc, &format!("{}/params/{key}", placed.path), replace)?;
    }
    let set = E::Set { layer: id.into(), index, params: change, enabled: None, name: None, optional: None };
    crate::shaping::effects(dir, doc, &set)
}
