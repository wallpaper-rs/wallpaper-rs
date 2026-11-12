use serde_json::{Value, json};
use wprs_scene::Color;

const CHANNELS: [&str; 4] = ["red", "green", "blue", "alpha"];
const DIMENSIONS: [&str; 3] = ["x", "y", "z"];

pub(crate) fn round(v: f64) -> f64 {
    (v * 1e6).round() / 1e6
}

pub(crate) fn names(v: &Value) -> Vec<String> {
    match v {
        Value::Number(_) => vec!["value".into()],
        Value::String(_) => CHANNELS.map(String::from).to_vec(),
        Value::Array(list) if list.len() <= 3 => {
            DIMENSIONS[..list.len()].iter().map(|a| a.to_string()).collect()
        }
        Value::Array(list) => (0..list.len()).map(|i| i.to_string()).collect(),
        _ => Vec::new(),
    }
}

pub(crate) fn parts(v: &Value) -> Option<Vec<f64>> {
    crate::blend::parts(v)
}

pub(crate) fn changed(v: &Value, part: Option<usize>, f: impl Fn(usize, f64) -> f64) -> Option<Value> {
    let mut all = parts(v)?;
    let color = v.is_string();
    let touched = |i: usize| part.map_or(!(color && i == 3), |p| p == i);
    if part.is_some_and(|p| p >= all.len()) {
        return None;
    }
    all.iter_mut().enumerate().filter(|(i, _)| touched(*i)).for_each(|(i, x)| *x = round(f(i, *x)));
    Some(match v {
        Value::Number(_) => json!(all[0]),
        Value::String(_) => json!(Color(std::array::from_fn(|i| all[i].clamp(0.0, 1.0) as f32)).hex()),
        _ => json!(all),
    })
}
