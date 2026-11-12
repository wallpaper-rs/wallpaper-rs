use crate::figures::place::{Still, hold};
use crate::ops::{Op, set};
use crate::seats::seat;
use anyhow::{Result, anyhow};
use serde_json::{Value, json};
use std::path::Path;
use wprs_core::message::said;

pub(super) fn drag(
    dir: &Path,
    doc: &Value,
    (id, path, found): (&str, &str, &Value),
    to: [f64; 2],
    (at, replace): (Option<f64>, bool),
) -> Result<Vec<Op>> {
    let t = at.unwrap_or(0.0);
    let me = seat(dir, doc, id, t).ok_or_else(|| {
        anyhow!(said("editor.grip_unplaced", "the layer %1 is not placed on the canvas", &[&id]))
    })?;
    let layout = super::set::layout(found, t);
    let backing = layout.backing.map_or(0.0, |b| f64::from(b.1[0]));
    let margin = f64::from(layout.padding + layout.reach) + 1.0 + backing;
    let block = (me.fraction(to)[0] * me.size[0] - 2.0 * margin).round().clamp(1.0, 16_384.0);
    let mut ops = match found["wrap"].is_object() {
        true => set(doc, &format!("{path}/wrap/width"), json!(block)),
        false => set(doc, &format!("{path}/wrap"), json!({ "width": block })),
    };
    let still = Still { before: me.at([0.0, 0.5]), after: [0.0, 0.5] };
    let more = hold(dir, doc, (id, path), &ops, &still, (at, replace))?;
    ops.extend(more);
    Ok(ops)
}
