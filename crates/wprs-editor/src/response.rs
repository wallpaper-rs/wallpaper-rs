use serde_json::{Value, json};
use wprs_core::message::said;
use wprs_scene::Smooth;

const POINTS: usize = 96;
const LIMIT: f64 = 60.0;
const SETTLE: f64 = 3.0;
const FLOOR: f64 = 0.05;

pub fn times(smooth: &Value) -> Result<[f64; 2], String> {
    let parsed: Smooth = serde_json::from_value(smooth.clone()).map_err(|e| e.to_string())?;
    let [up, down] = parsed.times().map(f64::from);
    if ![up, down].iter().all(|t| (0.0..=LIMIT).contains(t)) {
        return Err(said("editor.smooth_range", "smoothing goes from 0 to 60 seconds", &[]).to_string());
    }
    Ok([up, down])
}

pub fn settle(tau: f64) -> f64 {
    tau * SETTLE
}

pub fn query(q: &Value) -> Value {
    let [up, down] = match times(&q["smooth"]) {
        Ok(times) => times,
        Err(error) => return json!({ "error": error }),
    };
    let rise = settle(up).max(FLOOR);
    let fall = settle(down).max(FLOOR);
    let lead = (rise + fall) * 0.1;
    let total = lead * 2.0 + rise + fall;
    let (start, turn) = (lead, lead + rise + lead);
    let step = total / (POINTS - 1) as f64;
    let at = |i: usize| {
        let t = i as f64 * step;
        let value = if t < start {
            0.0
        } else if t < turn {
            1.0 - if up > 0.0 { (-(t - start) / up).exp() } else { 0.0 }
        } else {
            let top = 1.0 - if up > 0.0 { (-(turn - start) / up).exp() } else { 0.0 };
            top * if down > 0.0 { (-(t - turn) / down).exp() } else { 0.0 }
        };
        json!([(t * 1000.0).round() / 1000.0, (value * 1000.0).round() / 1000.0])
    };
    json!({
        "up": up,
        "down": down,
        "duration": total,
        "rises": start,
        "falls": turn,
        "settles": [settle(up), settle(down)],
        "points": (0..POINTS).map(at).collect::<Vec<_>>(),
    })
}
