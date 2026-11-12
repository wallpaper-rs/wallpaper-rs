use crate::actions::layer;
use crate::keys::value_at;
use crate::seats::{Seat, seat};
use anyhow::{Result, anyhow};
use serde_json::{Value, json};
use std::path::Path;
use wprs_core::message::said;
use wprs_scene::{Descriptor, Grab, Hold};

pub(super) const REACH: f64 = 0.2;

pub(super) struct Placed {
    pub seat: Seat,
    pub path: String,
    pub used: Value,
    pub effect: Descriptor,
}

pub(super) fn open(dir: &Path, doc: &Value, id: &str, index: usize, t: f64) -> Result<Placed> {
    let (path, found) = layer(doc, id)?;
    let used = found["effects"].get(index).cloned();
    let used = used.ok_or_else(|| {
        anyhow!(said("editor.no_effect_at", "the layer %1 has no effect %2", &[&id, &index]))
    })?;
    let effect = crate::shaping::known(dir, doc, used["effect"].as_str().unwrap_or_default())?;
    let seat = seat(dir, doc, id, t).ok_or_else(|| {
        anyhow!(said("editor.grip_unplaced", "the layer %1 is not placed on the canvas", &[&id]))
    })?;
    Ok(Placed { seat, path: format!("{path}/effects/{index}"), used, effect })
}

pub(super) fn pair(v: &Value) -> Option<[f64; 2]> {
    match v {
        Value::Array(a) if a.len() >= 2 => Some([a[0].as_f64()?, a[1].as_f64()?]),
        Value::Number(n) => n.as_f64().map(|n| [n, n]),
        _ => None,
    }
}

impl Placed {
    pub fn value(&self, h: &Hold, t: f64) -> [f64; 2] {
        let given = value_at(&self.used["params"][h.param], t);
        pair(&given).unwrap_or(h.default.map(f64::from))
    }

    pub fn handles(&self) -> Vec<Hold<'_>> {
        self.effect.handles()
    }

    pub fn anchor(&self, t: f64) -> [f64; 2] {
        let first = self.handles().into_iter().find(|h| h.kind == Grab::Point);
        first.map_or([0.5, 0.5], |h| self.value(&h, t))
    }

    pub fn reach(&self) -> Option<[f64; 2]> {
        let side = self.seat.size[0].abs().min(self.seat.size[1].abs()) * REACH;
        self.seat.size.iter().all(|s| s.abs() > 1e-6).then_some([side, side])
    }

    pub fn at(&self, h: &Hold, t: f64) -> Option<[f64; 2]> {
        let value = self.value(h, t);
        let u = match h.kind {
            Grab::Point | Grab::Corner => value,
            Grab::Vector | Grab::Angle => {
                let [rx, ry] = self.reach()?;
                let [sx, sy] = self.seat.size;
                let anchor = self.anchor(t);
                let to = if h.kind == Grab::Vector {
                    [value[0] * rx, value[1] * ry]
                } else {
                    let (s, c) = value[0].to_radians().sin_cos();
                    [c * rx, s * ry]
                };
                [anchor[0] + to[0] / sx, anchor[1] + to[1] / sy]
            }
        };
        Some(self.seat.at(u))
    }
}

pub(super) fn handles(dir: &Path, doc: &Value, id: &str, index: usize, t: f64) -> Result<Value> {
    let placed = open(dir, doc, id, index, t)?;
    let anchor = placed.seat.at(placed.anchor(t));
    let kind = |k: Grab| match k {
        Grab::Point => "point",
        Grab::Corner => "corner",
        Grab::Vector => "vector",
        Grab::Angle => "angle",
    };
    let shown: Vec<Value> = placed
        .handles()
        .iter()
        .filter_map(|h| {
            let at = placed.at(h, t)?;
            let aimed = matches!(h.kind, Grab::Vector | Grab::Angle);
            Some(json!({ "param": h.param, "kind": kind(h.kind), "at": at, "partner": h.partner,
                "anchor": aimed.then_some(anchor), "value": placed.value(h, t), "range": h.range }))
        })
        .collect();
    Ok(json!({ "effect": placed.effect.id, "quad": placed.effect.quad(), "handles": shown }))
}
