use super::parts::{names, parts, side};
use crate::keys::{sample, time_of};
use serde_json::{Value, json};

fn handle(keys: &[Value], i: usize, name: &str) -> Value {
    let key = &keys[i];
    let j = if name == "in" { i.checked_sub(1) } else { Some(i + 1) };
    let Some(other) = j.and_then(|j| keys.get(j)) else { return Value::Null };
    let (here, there) = (parts(&key[1]), parts(&other[1]));
    let (Some(here), Some(there)) = (here, there) else { return Value::Null };
    let scalar = key[1].is_number() && other[1].is_number();
    let gains: Vec<f64> = match scalar {
        true => vec![1.0],
        false => here.iter().zip(&there).map(|(a, b)| b - a).collect(),
    };
    let span = (time_of(other) - time_of(key)).abs();
    let given = side(key, name);
    let [dt, dv] = given.unwrap_or([span / 3.0, if scalar { (there[0] - here[0]) / 3.0 } else { 1.0 / 3.0 }]);
    let sign = if name == "in" { -1.0 } else { 1.0 };
    let points: Vec<Value> =
        here.iter().zip(&gains).map(|(v, g)| json!([time_of(key) + sign * dt, v + sign * dv * g])).collect();
    json!({ "dt": dt, "dv": dv, "set": given.is_some(), "room": span, "share": !scalar, "gains": gains, "at": points })
}

fn channel(doc: &Value, q: &Value) -> anyhow::Result<(String, f64)> {
    let (layer, clip) = (q["layer"].as_str().unwrap_or_default(), q["clip"].as_str().unwrap_or_default());
    let (path, l) = crate::actions::layer(doc, layer)?;
    let (i, found) = crate::clips::found(l, clip)?;
    let row = q["row"].as_array().map_or(&[][..], Vec::as_slice);
    let names: Vec<String> = row.iter().filter_map(Value::as_str).map(crate::ops::escape).collect();
    Ok((format!("{path}/clips/{i}/{}", names.join("/")), found["from"].as_f64().unwrap_or(0.0)))
}

pub fn curves(doc: &Value, q: &Value) -> Value {
    let (pointer, offset) = match q["pointer"].as_str() {
        Some(p) => (p.to_owned(), 0.0),
        None => match channel(doc, q) {
            Ok(found) => found,
            Err(e) => return json!({ "error": e.to_string() }),
        },
    };
    let pointer = pointer.as_str();
    let (list, keys) = match super::list(doc, pointer) {
        Ok(found) => found,
        Err(e) => return json!({ "error": e.to_string() }),
    };
    let samples = q["samples"].as_u64().unwrap_or(96).clamp(2, 512) as usize;
    let (first, last) = (keys.first().map_or(0.0, time_of), keys.last().map_or(0.0, time_of));
    let from = q["from"].as_f64().unwrap_or(first);
    let to = q["to"].as_f64().filter(|t| *t > from).unwrap_or((last).max(from + 1.0));
    let named = keys.iter().map(|k| names(&k[1])).max_by_key(Vec::len).unwrap_or_default();
    let rows: Vec<Value> = keys
        .iter()
        .enumerate()
        .map(|(i, k)| {
            json!({
                "t": time_of(k),
                "value": k[1],
                "parts": parts(&k[1]),
                "ease": k.get(2).filter(|e| !e.is_object()),
                "in": handle(&keys, i, "in"),
                "out": handle(&keys, i, "out"),
            })
        })
        .collect();
    let at = |i: usize| from + (to - from) * i as f64 / (samples - 1) as f64;
    let sampled: Vec<(f64, Option<Vec<f64>>)> =
        (0..samples).map(|i| (at(i), sample(&keys, at(i)).as_ref().and_then(parts))).collect();
    let tracks: Vec<Value> = named
        .iter()
        .enumerate()
        .map(|(p, name)| {
            let points: Vec<Value> = sampled
                .iter()
                .filter_map(|(t, v)| v.as_ref().and_then(|v| v.get(p)).map(|v| json!([t, v])))
                .collect();
            json!({ "part": p, "name": name, "points": points })
        })
        .collect();
    json!({ "pointer": pointer, "list": list, "offset": offset, "range": [from, to], "keys": rows, "tracks": tracks })
}
