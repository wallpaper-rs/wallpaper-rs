use super::mask::{open, shape};
use super::shapes::pair;
use crate::action::{EffectsEdit as E, MaskHold};
use crate::ops::Op;
use anyhow::{Result, anyhow, ensure};
use serde_json::{Value, json};
use std::path::Path;
use wprs_core::message::said;

fn whole(v: f64) -> f64 {
    v.round()
}

fn changed(kind: &str, spec: &Value, hold: &MaskHold, p: [f64; 2]) -> Option<(Option<Value>, Option<f64>)> {
    let mut next = spec.clone();
    let size = pair(&spec["size"], [0.0; 2]);
    let (ax, ay) = (p[0].abs(), p[1].abs());
    let round = ["rect", "ellipse", "ring"].contains(&kind);
    let feather = match kind {
        "line" => {
            let to = pair(&spec["to"], [0.0; 2]);
            let long = to[0].hypot(to[1]).max(1e-9);
            ((-to[1] * p[0] + to[0] * p[1]) / long).abs() - spec["width"].as_f64().unwrap_or(1.0) / 2.0
        }
        _ if round => ax - size[0] / 2.0,
        _ => p[0].hypot(p[1]) - spec["size"].as_f64().unwrap_or(0.0) / 2.0,
    };
    match hold {
        MaskHold::Feather => return Some((None, Some(whole(feather).clamp(0.0, 10000.0)))),
        MaskHold::Size if kind == "line" => next["to"] = json!([whole(p[0] * 2.0), whole(p[1] * 2.0)]),
        MaskHold::Size if round => next["size"] = json!([whole(ax * 2.0).max(1.0), whole(ay * 2.0).max(1.0)]),
        MaskHold::Size => next["size"] = json!(whole(ax.max(ay) * 2.0).max(1.0)),
        MaskHold::Width if kind == "ring" => {
            let room = size[0].min(size[1]) / 2.0;
            next["width"] = json!(whole(size[0] / 2.0 - ax).clamp(1.0, room.max(1.0)));
        }
        MaskHold::Radius if kind == "rect" => {
            next["radius"] = json!(whole(size[0] / 2.0 - p[0]).clamp(0.0, size[0].min(size[1]) / 2.0));
        }
        MaskHold::Inner if kind == "star" => {
            let outer = spec["size"].as_f64().unwrap_or(1.0) / 2.0;
            next["inner"] =
                json!(((p[0].hypot(p[1]) / outer.max(1e-9)).clamp(0.01, 1.0) * 1e3).round() / 1e3);
        }
        _ => return None,
    }
    Some((Some(json!({ kind: next })), None))
}

pub(crate) fn grip(
    dir: &Path,
    doc: &Value,
    (id, index): (&str, Option<usize>),
    hold: &MaskHold,
    to: [f64; 2],
    t: f64,
) -> Result<Vec<Op>> {
    let target = open(dir, doc, id, index, t)?;
    let (kind, spec) = shape(&target.mask).ok_or_else(|| {
        anyhow!(said("editor.mask_no_shape", "the mask of the layer %1 is not a shape", &[&id]))
    })?;
    let seat = &target.seat;
    ensure!(
        seat.size.iter().all(|s| s.abs() > 1e-6),
        said("editor.grip_no_size", "the layer %1 has no size to hold by a corner or an edge", &[&id])
    );
    let u = seat.fraction(to);
    let p = [0, 1].map(|i| (u[i] - 0.5) * seat.size[i]);
    let (shape, feather) = changed(&kind, &spec, hold, p).ok_or_else(|| {
        anyhow!(said("editor.mask_no_hold", "the %1 shape of a mask has no such handle", &[&kind]))
    })?;
    let edit = E::Mask {
        layer: id.into(),
        index,
        texture: None,
        from: None,
        shape,
        channel: None,
        invert: None,
        feather,
        strength: None,
        off: false,
    };
    crate::shaping::effects(dir, doc, &edit)
}
