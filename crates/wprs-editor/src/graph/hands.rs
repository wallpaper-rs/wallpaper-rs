use super::parts::{parts, round};
use crate::keys::time_of;
use serde_json::{Map, Value, json};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Hand {
    pub pairs: Vec<[f64; 2]>,
    pub split: bool,
}

fn pair(v: &Value) -> Option<[f64; 2]> {
    let list = v.as_array().filter(|l| l.len() == 2)?;
    Some([list[0].as_f64()?, list[1].as_f64()?])
}

pub(crate) fn hand(key: &Value, side: &str) -> Option<Hand> {
    let given = key.get(2)?.get(side)?;
    if let Some(one) = pair(given) {
        return Some(Hand { pairs: vec![one], split: false });
    }
    let pairs = given.as_array()?.iter().map(pair).collect::<Option<Vec<_>>>()?;
    (!pairs.is_empty()).then_some(Hand { pairs, split: true })
}

pub(crate) fn longest(key: &Value, side: &str) -> Option<f64> {
    hand(key, side).map(|h| h.pairs.iter().map(|p| p[0]).fold(0.0, f64::max))
}

pub(crate) fn written(h: &Hand) -> Value {
    let one = |[dt, dv]: [f64; 2]| json!([(dt * 1000.0).round() / 1000.0, round(dv)]);
    match h.split {
        true => Value::Array(h.pairs.iter().copied().map(one).collect()),
        false => one(h.pairs[0]),
    }
}

pub(crate) fn put(key: &mut Value, side: &str, h: Option<Hand>) {
    let mut sides = key.get(2).and_then(Value::as_object).cloned().unwrap_or_else(Map::new);
    match h {
        Some(h) => drop(sides.insert(side.into(), written(&h))),
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
        if let Some(mut h) = hand(key, name) {
            h.pairs.iter_mut().for_each(|p| p[0] *= scale);
            put(key, name, Some(h));
        }
    }
}

pub(crate) fn clamp(keys: &mut [Value]) {
    for i in 0..keys.len() {
        let before = i.checked_sub(1).map_or(0.0, |j| time_of(&keys[i]) - time_of(&keys[j]));
        let after = keys.get(i + 1).map_or(0.0, |n| time_of(n) - time_of(&keys[i]));
        for (name, room) in [("in", before), ("out", after)] {
            match hand(&keys[i], name) {
                Some(_) if room <= 1e-6 => put(&mut keys[i], name, None),
                Some(mut h) if h.pairs.iter().any(|p| p[0] > room) => {
                    h.pairs.iter_mut().for_each(|p| p[0] = p[0].min(room));
                    put(&mut keys[i], name, Some(h));
                }
                _ => {}
            }
        }
    }
}

pub(crate) fn aligned(given: [f64; 2], partner: Option<[f64; 2]>, room: f64) -> Option<[f64; 2]> {
    let dt = partner.map_or(room / 3.0, |p| p[0]).min(room);
    (room > 1e-6 && given[0] > 1e-6).then(|| [dt, given[1] * dt / given[0]])
}

pub(crate) struct Segment {
    pub room: f64,
    pub ways: Vec<f64>,
    pub units: bool,
}

pub(crate) fn segment(keys: &[Value], i: usize, side: &str) -> Option<Segment> {
    let j = if side == "in" { i.checked_sub(1)? } else { i + 1 };
    let (key, other) = (keys.get(i)?, keys.get(j)?);
    let (here, there) = (parts(&key[1])?, parts(&other[1])?);
    let units = key[1].is_number() && other[1].is_number();
    let sign = if side == "in" { -1.0 } else { 1.0 };
    let ways = here.iter().zip(&there).map(|(a, b)| sign * (b - a)).collect();
    Some(Segment { room: (time_of(other) - time_of(key)).abs(), ways, units })
}

pub(crate) fn each(keys: &[Value], i: usize, side: &str) -> Option<(Vec<[f64; 2]>, Segment)> {
    let seg = segment(keys, i, side)?;
    let given = hand(&keys[i], side);
    let pairs = seg
        .ways
        .iter()
        .enumerate()
        .map(|(p, way)| {
            let gain = if seg.units { 1.0 } else { *way };
            match &given {
                None => [seg.room / 3.0, way / 3.0],
                Some(Hand { pairs, split: false }) => [pairs[0][0], pairs[0][1] * gain],
                Some(Hand { pairs, split: true }) => {
                    pairs.get(p).copied().unwrap_or([seg.room / 3.0, way / 3.0])
                }
            }
        })
        .collect();
    Some((pairs, seg))
}

pub(crate) fn linked(pairs: &[[f64; 2]], seg: &Segment, part: usize) -> [f64; 2] {
    let p = part.min(pairs.len().saturating_sub(1));
    let moving = |q: usize| seg.ways.get(q).is_some_and(|w| w.abs() > 1e-9) && q < pairs.len();
    let q = if moving(p) { Some(p) } else { (0..pairs.len()).find(|q| moving(*q)) };
    match (seg.units, q) {
        (true, _) => pairs[p],
        (false, Some(q)) => [pairs[p][0], pairs[q][1] / seg.ways[q]],
        (false, None) => [pairs[p][0], 1.0 / 3.0],
    }
}
