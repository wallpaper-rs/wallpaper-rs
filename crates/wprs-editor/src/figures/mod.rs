mod drag;
mod holds;
pub(crate) mod place;
mod stops;

use crate::action::{FigureEdit, Stop};
use crate::actions::layer;
use crate::ops::{Op, set};
use crate::seats::seat;
use anyhow::{Result, anyhow};
use serde_json::{Value, json};
use std::path::Path;
use wprs_core::message::said;

pub(crate) use holds::{extent, first};

pub fn asks(q: &Value) -> bool {
    q.get("shape_handles").is_some()
}

pub fn query(dir: &Path, doc: &Value, q: &Value) -> Value {
    let s = &q["shape_handles"];
    let (id, t) = (s["layer"].as_str().unwrap_or_default(), s["time"].as_f64().unwrap_or(0.0));
    let Ok((_, found)) = layer(doc, id) else { return Value::Null };
    let (Some(seat), Some((kind, shape))) = (seat(dir, doc, id, t), first(found)) else { return Value::Null };
    let shown: Vec<Value> = holds::holds(kind, shape, found)
        .iter()
        .map(|h| json!({ "name": h.name, "kind": h.kind, "u": h.u, "at": seat.at(h.u) }))
        .collect();
    let gradient = found.get("gradient").filter(|g| g.is_object()).map(|g| {
        let end = |k: &str, d: [f64; 2]| seat.at(crate::seats::boxes::pair(&g[k], d));
        json!({ "radial": g["radial"].as_bool().unwrap_or(false), "stops": g["stops"],
            "from": end("from", [0.0, 0.0]), "to": end("to", [0.0, 1.0]) })
    });
    json!({ "kind": kind, "handles": shown, "gradient": gradient })
}

pub(crate) struct Moving<'a> {
    pub layer: &'a str,
    pub handle: &'a str,
    pub to: [f64; 2],
    pub even: bool,
    pub at: Option<f64>,
    pub replace: bool,
}

pub(crate) fn grip(dir: &Path, doc: &Value, m: &Moving) -> Result<Vec<Op>> {
    let (path, found) = layer(doc, m.layer)?;
    let t = m.at.unwrap_or(0.0);
    let me = seat(dir, doc, m.layer, t).ok_or_else(|| {
        anyhow!(said("editor.grip_unplaced", "the layer %1 is not placed on the canvas", &[&m.layer]))
    })?;
    let moved = drag::drag(found, m.handle, me.fraction(m.to), m.even)?;
    let mut ops = Vec::new();
    for (key, value) in &moved.fields {
        ops.extend(set(doc, &format!("{path}/{key}"), value.clone()));
    }
    if let Some(keep) = moved.keep {
        let still = place::Still { before: me.at(keep.from), after: keep.to };
        let more = place::hold(dir, doc, (m.layer, &path), &ops, &still, (m.at, m.replace))?;
        ops.extend(more);
    }
    Ok(ops)
}

pub(crate) fn stop(doc: &Value, id: &str, stop: &Stop) -> Result<Vec<Op>> {
    let (path, found) = layer(doc, id)?;
    stops::edit(doc, &path, found, stop)
}

pub(crate) fn edit(dir: &Path, doc: &Value, e: &FigureEdit) -> Result<Vec<Op>> {
    match e {
        FigureEdit::Shape { layer, handle, to, even, at, replace } => {
            grip(dir, doc, &Moving { layer, handle, to: *to, even: *even, at: *at, replace: *replace })
        }
        FigureEdit::Stop { layer, stop: s } => stop(doc, layer, s),
        other => crate::lettering::edit(dir, doc, other),
    }
}
