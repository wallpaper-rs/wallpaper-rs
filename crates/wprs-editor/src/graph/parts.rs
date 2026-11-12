use serde_json::{Map, Value, json};
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

pub(crate) fn changed(v: &Value, part: Option<usize>, f: impl Fn(f64) -> f64) -> Option<Value> {
    let mut all = parts(v)?;
    let color = v.is_string();
    let touched = |i: usize| part.map_or(!(color && i == 3), |p| p == i);
    if part.is_some_and(|p| p >= all.len()) {
        return None;
    }
    all.iter_mut().enumerate().filter(|(i, _)| touched(*i)).for_each(|(_, x)| *x = round(f(*x)));
    Some(match v {
        Value::Number(_) => json!(all[0]),
        Value::String(_) => json!(Color(std::array::from_fn(|i| all[i].clamp(0.0, 1.0) as f32)).hex()),
        _ => json!(all),
    })
}

pub(crate) fn side(key: &Value, side: &str) -> Option<[f64; 2]> {
    let pair = key.get(2)?.get(side)?.as_array()?;
    Some([pair.first()?.as_f64()?, pair.get(1)?.as_f64()?])
}

fn put(key: &mut Value, side: &str, pair: Option<[f64; 2]>) {
    let mut sides = key.get(2).and_then(Value::as_object).cloned().unwrap_or_else(Map::new);
    match pair {
        Some([dt, dv]) => drop(sides.insert(side.into(), json!([(dt * 1000.0).round() / 1000.0, round(dv)]))),
        None => drop(sides.remove(side)),
    }
    let plain = json!([key[0], key[1]]);
    *key = match (sides.is_empty(), key.get(2).is_some_and(Value::is_object)) {
        (true, true) => plain,
        (false, _) => json!([key[0], key[1], sides]),
        (true, false) => key.clone(),
    };
}

pub(crate) fn stretch(key: &mut Value, scale: f64) {
    for name in ["in", "out"] {
        if let Some([dt, dv]) = side(key, name) {
            put(key, name, Some([dt * scale, dv]));
        }
    }
}

pub(crate) fn clamp(keys: &mut [Value]) {
    let at = |k: &Value| k[0].as_f64().unwrap_or(0.0);
    for i in 0..keys.len() {
        let before = i.checked_sub(1).map_or(0.0, |j| at(&keys[i]) - at(&keys[j]));
        let after = keys.get(i + 1).map_or(0.0, |n| at(n) - at(&keys[i]));
        for (name, room) in [("in", before), ("out", after)] {
            match side(&keys[i], name) {
                Some(_) if room <= 1e-6 => put(&mut keys[i], name, None),
                Some([dt, dv]) if dt > room => put(&mut keys[i], name, Some([room, dv])),
                _ => {}
            }
        }
    }
}

pub(crate) fn aligned(given: [f64; 2], partner: Option<[f64; 2]>, room: f64) -> Option<[f64; 2]> {
    let dt = partner.map_or(room / 3.0, |p| p[0]).min(room);
    (room > 1e-6 && given[0] > 1e-6).then(|| [dt, given[1] * dt / given[0]])
}
