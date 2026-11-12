use crate::action::KeyframeEdit;
use crate::blend::parts;
use crate::ops::{Op, set};
use anyhow::{Result, bail, ensure};
use serde_json::{Map, Value, json};
use wprs_core::message::said;

const SPAN: f64 = 1e6;

pub(crate) fn spans(ease: Option<&Value>, handles: [Option<[f64; 2]>; 2]) -> Result<()> {
    let fine = |[dt, dv]: [f64; 2]| dt.is_finite() && dt >= 0.0 && (-SPAN..=SPAN).contains(&dv);
    ensure!(
        handles.iter().flatten().all(|h| fine(*h)),
        said(
            "editor.handle_span",
            "a Bézier handle reaches from 0 s up to the neighbouring keyframe and by -1000000 to 1000000 in value",
            &[]
        )
    );
    let handled = handles.iter().any(Option::is_some);
    ensure!(
        !(handled && ease.is_some()),
        said("editor.ease_or_handles", "a keyframe eases or has Bézier handles, not both", &[])
    );
    Ok(())
}

pub(crate) fn shaped(e: &KeyframeEdit) -> Result<()> {
    spans(e.ease.as_ref(), [e.in_handle, e.out_handle])?;
    ensure!(
        !(e.base.is_some() && e.absolute),
        said("editor.base_or_absolute", "give a base or make the value absolute, not both", &[])
    );
    Ok(())
}

fn strays(keys: &[Value]) -> Vec<(f64, &'static str, f32)> {
    let at = |k: &Value| k[0].as_f64().unwrap_or(0.0);
    let mut out = Vec::new();
    for (i, k) in keys.iter().enumerate() {
        let before = i.checked_sub(1).map_or(0.0, |j| at(k) as f32 - at(&keys[j]) as f32);
        let after = keys.get(i + 1).map_or(0.0, |n| at(n) as f32 - at(k) as f32);
        for (side, room) in [("in", before.max(0.0)), ("out", after.max(0.0))] {
            if crate::graph::hands::longest(k, side).is_some_and(|dt| dt as f32 > room) {
                out.push((at(k), side, room));
            }
        }
    }
    out
}

pub(crate) fn reach(pointer: &str, old: &[Value], new: &[Value]) -> Result<()> {
    let known = strays(old);
    let fresh = strays(new).into_iter().find(|s| !known.iter().any(|k| k.0 == s.0 && k.1 == s.1));
    if let Some((time, _, room)) = fresh {
        let room = (f64::from(room) * 1000.0).round() / 1000.0;
        bail!(said(
            "editor.handle_past_neighbour",
            "a handle of the keyframe of %1 at %2 s reaches past the neighbouring keyframe; it reaches up to %3 s",
            &[&pointer, &time, &room]
        ));
    }
    Ok(())
}

pub(crate) fn end(
    old: Option<&Value>,
    ease: Option<&Value>,
    handles: [Option<[f64; 2]>; 2],
    value: &Value,
) -> Result<Option<Value>> {
    if ease.is_some() {
        return Ok(ease.cloned());
    }
    if handles.iter().all(Option::is_none) {
        return Ok(old.cloned());
    }
    ensure!(
        parts(value).is_some(),
        said(
            "editor.handles_numbers",
            "Bézier handles shape keyframes of numbers, lists of numbers and colors",
            &[]
        )
    );
    let mut sides = old.and_then(Value::as_object).cloned().unwrap_or_else(Map::new);
    let round = |[dt, dv]: [f64; 2]| json!([(dt * 1000.0).round() / 1000.0, (dv * 1e6).round() / 1e6]);
    for (side, h) in ["in", "out"].into_iter().zip(handles) {
        if let Some(h) = h {
            sides.insert(side.into(), round(h));
        }
    }
    Ok(Some(Value::Object(sides)))
}

pub(crate) fn key(time: f64, value: Value, third: Option<Value>) -> Value {
    third.map_or_else(|| json!([time, value]), |t| json!([time, value, t]))
}

fn kind(v: &Value) -> Option<usize> {
    match v {
        Value::Number(_) => Some(1),
        Value::Array(list) if list.iter().all(Value::is_number) => Some(10 + list.len()),
        Value::String(text) => wprs_scene::Color::parse(text).map(|_| 0),
        _ => None,
    }
}

pub(crate) fn based(staged: &Value, e: &KeyframeEdit) -> Result<Vec<Op>> {
    let pointer = e.pointer.as_str();
    let at = format!("{pointer}/base");
    if e.absolute {
        return Ok(staged.pointer(&at).map(|_| Op::Remove { path: at.clone() }).into_iter().collect());
    }
    let Some(base) = &e.base else { return Ok(Vec::new()) };
    let first = staged.pointer(&format!("{pointer}/keys/0/1"));
    ensure!(
        !e.remove && first.is_some(),
        said("editor.base_keys", "only a value with keyframes adds them to a base", &[])
    );
    let numbers = base.as_array().map_or_else(|| vec![base], |l| l.iter().collect());
    let within = numbers.iter().all(|n| n.as_f64().is_none_or(|n| (-SPAN..=SPAN).contains(&n)));
    ensure!(
        within,
        said("editor.base_span", "each number of the base of %1 is from -1000000 to 1000000", &[&pointer])
    );
    let fits = kind(base).is_some() && kind(base) == first.and_then(kind);
    ensure!(
        fits,
        said(
            "editor.base_kind",
            "the base of %1 is a number, a list of numbers or a color like its keyframes",
            &[&pointer]
        )
    );
    Ok(set(staged, &at, base.clone()))
}
