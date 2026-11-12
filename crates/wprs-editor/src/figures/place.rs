use crate::ops::{Op, apply};
use crate::seats::seat;
use crate::shaping::layers::transform;
use anyhow::{Result, anyhow};
use serde_json::{Value, json};
use std::path::Path;
use wprs_core::message::said;

pub(crate) struct Still {
    pub before: [f64; 2],
    pub after: [f64; 2],
}

pub(crate) fn position(
    doc: &Value,
    path: &str,
    to: [f64; 2],
    at: Option<f64>,
    replace: bool,
) -> Result<Vec<Op>> {
    let rounded = to.map(|v| (v * 100.0).round() / 100.0);
    let fields = [
        ("position", Some(json!(rounded))),
        ("rotation", None),
        ("scale", None),
        ("skew", None),
        ("origin", None),
        ("tilt", None),
        ("z", None),
    ];
    transform(doc, path, fields, at, replace)
}

pub(crate) fn hold(
    dir: &Path,
    doc: &Value,
    (id, path): (&str, &str),
    ops: &[Op],
    still: &Still,
    (at, replace): (Option<f64>, bool),
) -> Result<Vec<Op>> {
    let mut staged = doc.clone();
    apply(&mut staged, ops)?;
    let after = seat(dir, &staged, id, at.unwrap_or(0.0)).ok_or_else(|| {
        anyhow!(said("editor.grip_unplaced", "the layer %1 is not placed on the canvas", &[&id]))
    })?;
    let (was, now) = (after.local(still.before), after.local(after.at(still.after)));
    let place = after.position();
    let to = [place[0] + was[0] - now[0], place[1] + was[1] - now[1]];
    position(&staged, path, to, at, replace)
}
