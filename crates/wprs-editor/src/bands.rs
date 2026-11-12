use anyhow::{Result, bail, ensure};
use serde_json::{Map, Value, json};
use wprs_core::message::said;
use wprs_scene::{BANDS, SPLITS, center, edge};

const NAMES: [&str; 5] = ["bass", "mid", "treble", "level", "beat"];
const DIRECTIONS: [&str; 4] = ["x", "y", "inside", "speed"];
const SOURCES: [&str; 15] = [
    "audio", "of", "reduce", "channel", "cursor", "prop", "map", "keys", "timeline", "loop", "media",
    "script", "fn", "params", "clock",
];

fn resolution(q: &Value) -> Result<usize> {
    let of = q["of"].as_u64().unwrap_or(BANDS as u64);
    ensure!(
        [16, 32, 64].contains(&of),
        said("editor.bands_of", "bands are counted out of 16, 32 or 64", &[])
    );
    Ok(of as usize)
}

fn hertz(of: usize, lo: usize, hi: usize) -> [f64; 2] {
    let fine = |i: usize| (i * BANDS / of).min(BANDS);
    [edge(fine(lo)), edge(fine(hi + 1))].map(|hz| (f64::from(hz) * 10.0).round() / 10.0)
}

fn named(of: usize) -> Value {
    let step = BANDS / of;
    let below = |limit: f32| (0..BANDS).filter(|b| center(*b) < limit).count().div_ceil(step);
    let (bass, mid) = (below(SPLITS[0]), below(SPLITS[1]));
    let last = |n: usize| n.saturating_sub(1);
    let from = bass.min(of - 1);
    json!({
        "bass": [0, last(bass)],
        "mid": [from, last(mid).max(from)],
        "treble": [mid.min(of - 1), of - 1],
    })
}

pub fn layout(q: &Value) -> Result<Value> {
    let of = resolution(q)?;
    let bands: Vec<Value> = (0..of)
        .map(|i| {
            let [from, to] = hertz(of, i, i);
            json!({ "index": i, "from": from, "to": to })
        })
        .collect();
    Ok(json!({ "of": of, "bands": bands, "named": named(of), "levels": NAMES, "pointer": DIRECTIONS }))
}

fn bare(base: &Value) -> Map<String, Value> {
    let mut map = base.as_object().cloned().unwrap_or_default();
    SOURCES.iter().for_each(|k| drop(map.remove(*k)));
    map
}

fn index(v: &Value, of: usize) -> Result<usize> {
    let found = v.as_u64().map(|n| n as usize).filter(|n| *n < of);
    found.ok_or_else(|| {
        let top = of - 1;
        anyhow::anyhow!(said("editor.band_index", "a band goes from 0 to %1", &[&top]))
    })
}

fn pick(q: &Value, of: usize, mut map: Map<String, Value>) -> Result<(Value, String)> {
    let band = &q["pick"];
    let label = match band {
        Value::String(name) => {
            ensure!(
                NAMES.contains(&name.as_str()),
                said("editor.band_name", "%1 is not bass, mid, treble, level or beat", &[name])
            );
            map.insert("audio".into(), json!(name));
            name.clone()
        }
        Value::Array(pair) if pair.len() == 2 => {
            let (lo, hi) = (index(&pair[0], of)?, index(&pair[1], of)?);
            ensure!(lo <= hi, said("editor.band_order", "the first band is not above the last", &[]));
            map.insert("audio".into(), json!([lo, hi]));
            let reduce = q["reduce"].as_str().unwrap_or("mean");
            ensure!(
                ["mean", "max"].contains(&reduce),
                said("editor.band_reduce", "bands are reduced by mean or max", &[])
            );
            if reduce == "max" {
                map.insert("reduce".into(), json!("max"));
            }
            format!("{lo}-{hi}")
        }
        n if n.is_number() => {
            let i = index(n, of)?;
            map.insert("audio".into(), json!(i));
            i.to_string()
        }
        _ => bail!(said("editor.band_what", "give a name, a band or a range of two bands", &[])),
    };
    if !band.is_string() && of != BANDS {
        map.insert("of".into(), json!(of));
    }
    match q["channel"].as_str().unwrap_or("average") {
        "average" => {}
        side @ ("left" | "right") => drop(map.insert("channel".into(), json!(side))),
        _ => bail!(said("editor.band_channel", "the channel is average, left or right", &[])),
    }
    Ok((Value::Object(map), label))
}

pub fn source(q: &Value) -> Result<Value> {
    let mut map = bare(&q["base"]);
    if let Some(direction) = q["direction"].as_str() {
        ensure!(
            DIRECTIONS.contains(&direction),
            said("editor.pointer_direction", "%1 is not x, y, inside or speed", &[&direction])
        );
        map.insert("cursor".into(), json!(direction));
        return Ok(json!({ "source": map, "label": direction }));
    }
    let of = resolution(q)?;
    let (source, label) = pick(q, of, map)?;
    let range = match &source["audio"] {
        Value::Array(p) => Some((p[0].as_u64().unwrap_or(0), p[1].as_u64().unwrap_or(0))),
        n if n.is_u64() => n.as_u64().map(|i| (i, i)),
        _ => None,
    };
    let hz = range.map(|(lo, hi)| hertz(of, lo as usize, hi as usize));
    Ok(json!({ "source": source, "label": label, "hertz": hz }))
}

pub fn query(q: &Value) -> Value {
    let asked = q.get("pick").is_some() || q.get("direction").is_some();
    let found = if asked { source(q) } else { layout(q) };
    found.unwrap_or_else(|e| json!({ "error": e.to_string() }))
}
