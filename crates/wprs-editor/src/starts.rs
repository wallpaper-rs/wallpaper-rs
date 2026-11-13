use crate::action::{Action, Forever, StartEdit};
use crate::actions::{layer, plan};
use crate::ops::{Op, set};
use anyhow::{Result, ensure};
use serde_json::{Value, json};
use wprs_core::message::said;

pub const CLOCK: &str = "HH:mm";
pub const DATE: &str = "EEEE d MMMM yyyy";
const TIME_SIZE: f64 = 160.0;
const LONGEST: f64 = 86_400.0;
const SPIN: f64 = 20.0;
const DRIFT: f64 = 20.0;
const REACH: [f64; 2] = [200.0, 0.0];

pub(crate) fn edit(doc: &Value, e: &StartEdit) -> Result<Vec<Op>> {
    match e {
        StartEdit::Clock { format, date, id, into, size } => {
            clock(doc, (format.as_deref(), *date), (id, into), *size)
        }
        StartEdit::Forever { layer, motion, seconds, reverse, by, back } => {
            forever(doc, layer, (*motion, *reverse), *seconds, (*by, *back))
        }
    }
}

fn clock(
    doc: &Value,
    (format, date): (Option<&str>, bool),
    (id, into): (&Option<String>, &Option<String>),
    size: Option<f64>,
) -> Result<Vec<Op>> {
    let pattern = format.map(str::trim).filter(|f| !f.is_empty()).unwrap_or(if date { DATE } else { CLOCK });
    ensure!(
        pattern.chars().count() <= 64,
        said("editor.clock_format_long", "a time or date format has at most 64 characters", &[])
    );
    let size = size.unwrap_or(TIME_SIZE);
    ensure!(
        size.is_finite() && (1.0..=4000.0).contains(&size),
        said("editor.clock_size", "the size of the text goes from 1 to 4000 pixels", &[])
    );
    let (kind, name) = if date { ("date", "Date") } else { ("clock", "Clock") };
    let add = Action::AddLayer {
        kind: "text".into(),
        id: id.clone(),
        name: Some(name.into()),
        source: None,
        into: into.clone(),
        at: None,
        file: None,
    };
    let mut ops = plan(doc, &add)?;
    if let Some(Op::Add { value, .. }) = ops.last_mut() {
        value["text"] = json!({ kind: pattern });
        value["size"] = json!(size);
    }
    Ok(ops)
}

fn plain(found: &Value, pointer: &str) -> Result<Option<Value>> {
    let current = found.pointer(pointer).cloned();
    ensure!(
        current.as_ref().is_none_or(|v| !v.is_object()),
        said(
            "editor.follows_another_source",
            "%1 follows another source; set a plain value first",
            &[&pointer]
        )
    );
    Ok(current)
}

fn forever(
    doc: &Value,
    id: &str,
    (motion, reverse): (Forever, bool),
    seconds: Option<f64>,
    (by, back): (Option<[f64; 2]>, Option<bool>),
) -> Result<Vec<Op>> {
    let (path, found) = layer(doc, id)?;
    let seconds = seconds.unwrap_or(if motion == Forever::Spin { SPIN } else { DRIFT });
    ensure!(
        seconds.is_finite() && (0.5..=LONGEST).contains(&seconds),
        said("editor.forever_seconds", "a loop lasts from 0.5 to 86400 seconds", &[])
    );
    if motion == Forever::Spin {
        ensure!(
            by.is_none() && back.is_none(),
            said("editor.spin_options", "a spin takes only seconds and reverse", &[])
        );
        let from = plain(found, "/transform/rotation")?.and_then(|v| v.as_f64()).unwrap_or(0.0);
        let to = from + if reverse { -360.0 } else { 360.0 };
        let keys = json!([[0, from, "linear"], [seconds, to, "linear"]]);
        return Ok(set(doc, &format!("{path}/transform/rotation"), json!({"keys": keys, "loop": "repeat"})));
    }
    let reach = by.unwrap_or(REACH);
    ensure!(
        reach.iter().all(|v| v.is_finite() && v.abs() <= 1e5),
        said("editor.drift_reach", "a drift goes at most 100000 pixels each way", &[])
    );
    let sign = if reverse { -1.0 } else { 1.0 };
    let center = |axis: usize| crate::kits::size(doc)[axis] / 2.0;
    let from = match plain(found, "/transform/position")? {
        Some(Value::Array(p)) if p.len() == 2 => [p[0].as_f64().unwrap_or(0.0), p[1].as_f64().unwrap_or(0.0)],
        _ if found["type"] == "group" => [0.0, 0.0],
        _ => [center(0), center(1)],
    };
    let to = [from[0] + sign * reach[0], from[1] + sign * reach[1]];
    let (ease, repeat) =
        if back.unwrap_or(true) { ("in_out_sine", "pingpong") } else { ("linear", "repeat") };
    let keys = json!([[0, from, ease], [seconds, to, ease]]);
    Ok(set(doc, &format!("{path}/transform/position"), json!({"keys": keys, "loop": repeat})))
}
