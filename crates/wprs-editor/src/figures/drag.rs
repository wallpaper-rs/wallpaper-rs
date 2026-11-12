use super::holds::{INSET, extent, first, line_ends, number};
use crate::seats::boxes::pair;
use anyhow::{Result, anyhow};
use serde_json::{Value, json};
use std::f64::consts::{PI, SQRT_2, TAU};
use wprs_core::message::said;

pub(crate) struct Keep {
    pub from: [f64; 2],
    pub to: [f64; 2],
}

#[derive(Default)]
pub(crate) struct Drag {
    pub fields: Vec<(String, Value)>,
    pub keep: Option<Keep>,
}

fn round(v: f64, step: f64) -> f64 {
    (v / step).round() * step
}

fn grown(u: f64, side: f64, k: f64) -> f64 {
    round((2.0 * (u - 0.5).abs() * side * k).max(1.0), 0.1)
}

fn turned(u: [f64; 2]) -> f64 {
    let a = (u[0] - 0.5).atan2(0.5 - u[1]).rem_euclid(TAU);
    if a > PI { TAU - a } else { a }
}

fn count(u: [f64; 2], low: f64) -> u64 {
    let a = turned(u);
    if a < TAU / 64.0 { 64 } else { (TAU / a).round().clamp(low, 64.0) as u64 }
}

fn put(out: &mut Drag, key: String, value: Value) {
    out.fields.push((key, value));
}

pub(crate) fn drag(layer: &Value, name: &str, u: [f64; 2], even: bool) -> Result<Drag> {
    let mut out = Drag::default();
    if let Some(end) = name.strip_prefix("gradient_") {
        let at = u.map(|v| round(v.clamp(-4.0, 5.0), 1e-3));
        put(&mut out, format!("gradient/{}", if end == "from" { "from" } else { "to" }), json!(at));
        return Ok(out);
    }
    let (kind, s) = first(layer).ok_or_else(|| {
        anyhow!(said("editor.no_shape_handle", "the layer %1 has no handle %2", &[&layer["id"], &name]))
    })?;
    let [w, h] = extent(kind, s);
    let base = format!("shape/{kind}");
    let size = pair(&s["size"], [w, h]);
    let center = [(u[0] - 0.5) * w, (u[1] - 0.5) * h];
    match (kind, name) {
        (_, "stroke") => {
            put(&mut out, "stroke/width".into(), json!(round((-2.0 * u[0] * w).clamp(0.1, w.min(h)), 0.1)));
        }
        ("rect" | "ellipse" | "ring", "width" | "height" | "size") => {
            let k = if name == "size" && kind != "rect" { SQRT_2 } else { 1.0 / INSET };
            let (mut nw, mut nh) = (grown(u[0], size[0], k), grown(u[1], size[1], k));
            match name {
                "width" => nh = size[1],
                "height" => nw = size[0],
                _ if even => {
                    let f = (nw / size[0] + nh / size[1]) / 2.0;
                    (nw, nh) = (round(size[0] * f, 0.1), round(size[1] * f, 0.1));
                }
                _ => {}
            }
            let low = nw.min(nh) / 2.0;
            put(&mut out, format!("{base}/size"), json!([nw, nh]));
            for (key, floor) in [("radius", 0.0), ("width", 1e-3)] {
                if s[key].as_f64().is_some_and(|v| v > low) {
                    put(&mut out, format!("{base}/{key}"), json!(round(low.max(floor), 0.1)));
                }
            }
        }
        ("rect", "radius") => {
            let r = ((1.0 - u[0]) * size[0]).clamp(0.0, size[0].min(size[1]) / 2.0);
            put(&mut out, format!("{base}/radius"), json!(round(r, 0.1)));
        }
        ("ring", "band") => {
            let band = (u[1] * size[1]).clamp(1e-3, size[0].min(size[1]) / 2.0);
            put(&mut out, format!("{base}/width"), json!(round(band, 0.1)));
        }
        ("polygon" | "star", "size") => {
            let radius = center[0].hypot(center[1]);
            put(&mut out, format!("{base}/size"), json!(round((2.0 * radius / INSET).max(1.0), 0.1)));
        }
        ("polygon", "sides") => put(&mut out, format!("{base}/sides"), json!(count(u, 3.0))),
        ("star", "points") => put(&mut out, format!("{base}/points"), json!(count(u, 2.0))),
        ("star", "inner") => {
            let inner = center[0].hypot(center[1]) / (number(&s["size"], 100.0) / 2.0);
            put(&mut out, format!("{base}/inner"), json!(round(inner.clamp(0.01, 1.0), 0.01)));
        }
        ("line", "start" | "end") => line_end(s, name, u, [w, h], &mut out),
        ("line", "width") => {
            let (a, b) = line_ends(s);
            let to = pair(&s["to"], [100.0, 0.0]);
            let long = to[0].hypot(to[1]).max(1e-9);
            let mid = [(a[0] + b[0]) / 2.0 * w, (a[1] + b[1]) / 2.0 * h];
            let off = [u[0] * w - mid[0], u[1] * h - mid[1]];
            let across = (off[0] * -to[1] + off[1] * to[0]) / long;
            put(&mut out, format!("{base}/width"), json!(round((2.0 * across).clamp(0.5, 1e4), 0.1)));
        }
        _ => {
            return Err(anyhow!(said(
                "editor.no_shape_handle",
                "the layer %1 has no handle %2",
                &[&layer["id"].as_str().unwrap_or_default(), &name]
            )));
        }
    }
    Ok(out)
}

fn line_end(s: &Value, name: &str, u: [f64; 2], [w, h]: [f64; 2], out: &mut Drag) {
    let (a, b) = line_ends(s);
    let fixed = if name == "start" { b } else { a };
    let at = [u[0] * w, u[1] * h];
    let held = [fixed[0] * w, fixed[1] * h];
    let to =
        if name == "start" { [held[0] - at[0], held[1] - at[1]] } else { [at[0] - held[0], at[1] - held[1]] };
    let to = to.map(|v| round(v.clamp(-1e5, 1e5), 0.1));
    let next = json!({ "to": to, "width": s["width"] });
    let (na, nb) = line_ends(&next);
    out.keep = Some(Keep { from: fixed, to: if name == "start" { nb } else { na } });
    put(out, "shape/line/to".into(), json!(to));
}
