use super::shapes::{Loop, holds, loops};
use crate::actions::layer;
use crate::seats::{Seat, seat};
use anyhow::{Result, anyhow, ensure};
use serde_json::{Value, json};
use std::path::Path;
use wprs_core::message::said;

pub(super) struct Target {
    pub seat: Seat,
    pub mask: Value,
}

pub(super) fn open(dir: &Path, doc: &Value, id: &str, index: Option<usize>, t: f64) -> Result<Target> {
    let (_, found) = layer(doc, id)?;
    let owner = match index {
        Some(i) => found["effects"].get(i).cloned().ok_or_else(|| {
            anyhow!(said("editor.no_effect_at", "the layer %1 has no effect %2", &[&id, &i]))
        })?,
        None => found.clone(),
    };
    let none = || anyhow!(said("editor.no_mask", "the layer %1 has no mask there", &[&id]));
    let mask = match &owner["mask"] {
        Value::String(texture) => json!({ "texture": texture }),
        m @ Value::Object(_) => m.clone(),
        _ => return Err(none()),
    };
    let seat = seat(dir, doc, id, t).ok_or_else(|| {
        anyhow!(said("editor.grip_unplaced", "the layer %1 is not placed on the canvas", &[&id]))
    })?;
    Ok(Target { seat, mask })
}

pub(super) fn shape(mask: &Value) -> Option<(String, Value)> {
    let (kind, spec) = mask["shape"].as_object()?.iter().next()?;
    Some((kind.clone(), spec.clone()))
}

fn canvas(seat: &Seat, p: [f64; 2]) -> [f64; 2] {
    seat.at([0, 1].map(|i| 0.5 + p[i] / seat.size[i]))
}

fn fractions(seat: &Seat, loops: Vec<Loop>) -> Vec<Vec<[f64; 2]>> {
    let at = |p: [f64; 2]| [0, 1].map(|i| 0.5 + p[i] / seat.size[i]);
    loops.into_iter().map(|l| l.into_iter().map(at).collect()).collect()
}

fn shown(seat: &Seat, loops: Vec<Loop>) -> Vec<Vec<[f64; 2]>> {
    loops.into_iter().map(|l| l.into_iter().map(|p| canvas(seat, p)).collect()).collect()
}

pub(super) fn view(dir: &Path, doc: &Value, id: &str, index: Option<usize>, t: f64) -> Result<Value> {
    let Target { seat, mask, .. } = open(dir, doc, id, index, t)?;
    let feather = mask["feather"].as_f64().unwrap_or(0.0).max(0.0);
    let source = ["texture", "layer", "shape"].into_iter().find(|k| mask.get(k).is_some());
    let sized = seat.size.iter().all(|s| s.abs() > 1e-6);
    let mut out = json!({
        "source": source,
        "invert": mask["invert"] == true, "channel": mask["channel"],
        "strength": mask["strength"].as_f64(), "feather": feather,
        "box": seat.corners(), "unit": seat.unit().0.map(f64::from),
        "texture": mask["texture"], "layer": mask["layer"],
        "size": seat.size, "area": [], "area_u": [], "outer": [], "inner": [], "holds": [], "kind": null,
    });
    if let (Some((kind, spec)), true) = (shape(&mask), sized) {
        out["kind"] = json!(kind);
        out["area"] = json!(shown(&seat, loops(&kind, &spec, 0.0)));
        out["area_u"] = json!(fractions(&seat, loops(&kind, &spec, 0.0)));
        if feather > 0.0 {
            out["outer"] = json!(shown(&seat, loops(&kind, &spec, feather)));
            out["inner"] = json!(shown(&seat, loops(&kind, &spec, -feather)));
        }
        let at = |(grip, p): (&str, [f64; 2])| json!({ "grip": grip, "at": canvas(&seat, p) });
        out["holds"] = json!(holds(&kind, &spec, feather).into_iter().map(at).collect::<Vec<_>>());
    }
    if let Some(from) = mask["layer"].as_str().and_then(|l| crate::seats::seat(dir, doc, l, t)) {
        out["from_box"] = json!(from.corners());
        out["from_unit"] = json!(from.unit().0.map(f64::from));
    }
    ensure!(out["source"].is_string(), "no source");
    Ok(out)
}
