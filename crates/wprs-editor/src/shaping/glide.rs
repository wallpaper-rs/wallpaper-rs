use crate::actions::layer;
use crate::ops::{Op, set};
use anyhow::{Result, ensure};
use serde_json::{Value, json};
use wprs_core::message::said;

pub(super) const PROBE: &str = "/scene/ext/editor/probe";

pub(super) fn depth(doc: &Value, id: &str, depth: [f64; 2], replace: bool) -> Result<Vec<Op>> {
    let (path, _) = layer(doc, id)?;
    super::plain(doc, &format!("{path}/parallax"), replace)?;
    let fits = depth.iter().all(|d| d.is_finite() && (-10.0..=10.0).contains(d));
    ensure!(fits, said("editor.parallax_range", "a parallax depth goes from -10 to 10", &[]));
    let [x, y] = depth.map(|d| (d * 1000.0).round() / 1000.0);
    let pointer = format!("{path}/parallax");
    let mut ops = match (x == y, x == 0.0 && y == 0.0) {
        (_, true) => return Ok(super::effects::put(doc, &pointer, None)),
        (true, _) => set(doc, &pointer, json!(x)),
        _ => set(doc, &pointer, json!([x, y])),
    };
    let mut inputs = doc.pointer("/manifest/inputs").and_then(Value::as_array).cloned().unwrap_or_default();
    let before = inputs.len();
    if !inputs.iter().any(|i| i == "pointer") {
        inputs.push(json!("pointer"));
    }
    if inputs.len() > before {
        ops.extend(set(doc, "/manifest/inputs", json!(inputs)));
    }
    Ok(ops)
}

pub(super) fn probe(
    doc: &Value,
    pointer: Option<[f64; 2]>,
    tilt: Option<[f64; 2]>,
    off: bool,
) -> Result<Vec<Op>> {
    if off {
        return Ok(super::effects::put(doc, PROBE, None));
    }
    let inside = pointer.is_none_or(|p| p.iter().all(|v| (0.0..=1.0).contains(v)));
    ensure!(
        inside,
        said("editor.probe_pointer", "the simulated pointer is a fraction of the canvas, 0 to 1", &[])
    );
    let leaning = tilt.is_none_or(|t| t.iter().all(|v| (-1.0..=1.0).contains(v)));
    ensure!(leaning, said("editor.probe_tilt", "the simulated tilt goes from -1 to 1 on each side", &[]));
    let kept = |k: &str, d: Value| doc.pointer(PROBE).and_then(|p| p.get(k)).cloned().unwrap_or(d);
    let r = |p: [f64; 2]| json!(p.map(|v| (v * 1e4).round() / 1e4));
    let pointer = pointer.map_or_else(|| kept("pointer", json!([0.5, 0.5])), r);
    let tilt = tilt.map_or_else(|| kept("tilt", json!([0, 0])), r);
    Ok(set(doc, PROBE, json!({ "pointer": pointer, "tilt": tilt })))
}
