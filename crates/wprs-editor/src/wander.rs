use serde_json::{Value, json};

const SPEED: [f64; 2] = [0.7, 1.1];
const REACH: [f64; 2] = [0.42, 0.36];

pub fn at(seconds: f64) -> [f64; 2] {
    let wave = |axis: usize, phase: f64| 0.5 + REACH[axis] * (seconds * SPEED[axis] + phase).sin();
    [wave(0, 0.0), wave(1, 1.3)]
}

pub fn query(q: &Value) -> Value {
    let [x, y] = at(q["time"].as_f64().unwrap_or(0.0));
    json!({ "x": x, "y": y, "buttons": 0 })
}
