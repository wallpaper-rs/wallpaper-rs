use super::parts::round;
use serde_json::{Value, json};

type Span = Option<[f64; 2]>;

fn widen(span: &mut Span, v: Option<f64>) {
    if let Some(v) = v.filter(|v| v.is_finite()) {
        *span = Some(span.map_or([v, v], |[lo, hi]| [lo.min(v), hi.max(v)]));
    }
}

fn spans(list: &[Span]) -> Value {
    list.iter().map(|s| s.map_or(Value::Null, |s| json!(s.map(round)))).collect()
}

pub(super) fn bounds(rows: &[Value], tracks: &[Value], picked: &[f64]) -> Value {
    let mut all: Vec<Span> = vec![None; tracks.len()];
    for (p, track) in tracks.iter().enumerate() {
        for point in track["points"].as_array().into_iter().flatten() {
            widen(&mut all[p], point[1].as_f64());
        }
    }
    let (mut times, mut parts): (Span, Vec<Span>) = (None, vec![None; tracks.len()]);
    for row in rows {
        let values = row["parts"].as_array().cloned().unwrap_or_default();
        for (span, v) in all.iter_mut().zip(&values) {
            widen(span, v.as_f64());
        }
        for side in ["in", "out"].into_iter().filter(|side| row[side]["set"] == true) {
            for (span, point) in all.iter_mut().zip(row[side]["at"].as_array().into_iter().flatten()) {
                widen(span, point[1].as_f64());
            }
        }
        let t = row["t"].as_f64().unwrap_or(0.0);
        if !picked.iter().any(|c| (c - t).abs() < 1e-3) {
            continue;
        }
        widen(&mut times, Some(t));
        for (span, v) in parts.iter_mut().zip(&values) {
            widen(span, v.as_f64());
        }
        for side in ["in", "out"] {
            let points = row[side]["at"].as_array().cloned().unwrap_or_default();
            for point in &points {
                widen(&mut times, point[0].as_f64());
            }
            for (span, point) in parts.iter_mut().zip(&points) {
                widen(span, point[1].as_f64());
            }
        }
    }
    let chosen = times.map(|range| json!({ "range": range.map(round), "parts": spans(&parts) }));
    json!({ "all": spans(&all), "picked": chosen })
}
