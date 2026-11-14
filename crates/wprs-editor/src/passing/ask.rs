use super::{Way, path, readied, words};
use serde_json::{Value, json};
use std::fs;
use std::path::Path;
use wprs_scene::{Ease, MAX_PARAMS, MAX_TRANSITION_MS, MAX_TRANSITION_UNITS, MIN_TRANSITION_MS, Quality};

const SAMPLES: usize = 48;
const LEVELS: [Quality; 4] = [Quality::Q0, Quality::Q1, Quality::Q2, Quality::Q3];

pub fn query(dir: &Path, doc: &Value) -> Value {
    json!({
        "in": way(dir, doc, Way::In),
        "out": way(dir, doc, Way::Out),
        "effects": effects(dir),
        "curves": wprs_scene::easings(),
        "limits": {
            "min_ms": MIN_TRANSITION_MS, "max_ms": MAX_TRANSITION_MS,
            "params": MAX_PARAMS, "units": MAX_TRANSITION_UNITS,
        },
    })
}

pub fn shape(curve: &Value) -> Vec<f32> {
    let ease: Ease = serde_json::from_value(curve.clone()).unwrap_or_default();
    (0..=SAMPLES).map(|i| ease.apply(i as f32 / SAMPLES as f32).clamp(0.0, 1.0)).collect()
}

fn way(dir: &Path, doc: &Value, way: Way) -> Value {
    let Some(value) = doc.pointer(&path(way)) else { return Value::Null };
    let mut out = value.clone();
    out["duration_ms"] = json!(value["duration_ms"].as_u64().unwrap_or(1000));
    out["curve"] = value.get("curve").cloned().unwrap_or(json!("linear"));
    out["shape"] = json!(shape(&out["curve"]));
    let descriptor = descriptor(dir, value["descriptor"].as_str().unwrap_or_default());
    out["name"] = descriptor["name"].clone();
    out["fields"] = descriptor["params"].clone();
    match readied(dir, way, value) {
        Ok(Ok(ready)) => {
            out["plays"] = json!(LEVELS.map(|q| ready.plays_at(q)));
            out["problem"] = Value::Null;
        }
        Ok(Err(why)) => out["problem"] = json!(words(&why).local()),
        Err(e) => out["problem"] = json!(e.to_string()),
    }
    out
}

fn descriptor(dir: &Path, file: &str) -> Value {
    let read = fs::read(dir.join(file)).ok();
    read.and_then(|b| serde_json::from_slice(&b).ok()).unwrap_or(Value::Null)
}

fn effects(dir: &Path) -> Vec<String> {
    let mut found = Vec::new();
    let mut folders = vec![(dir.to_path_buf(), 0)];
    while let Some((folder, depth)) = folders.pop() {
        for entry in fs::read_dir(&folder).into_iter().flatten().flatten() {
            let file = entry.path();
            if file.is_dir() && depth < 2 {
                folders.push((file, depth + 1));
            } else if file.extension().is_some_and(|e| e == "json") && transition(&file) {
                let name = file.strip_prefix(dir).unwrap_or(&file);
                found.push(name.to_string_lossy().replace('\\', "/"));
            }
        }
    }
    found.sort();
    found
}

fn transition(file: &Path) -> bool {
    let small = fs::metadata(file).is_ok_and(|m| m.len() < 64 * 1024);
    small
        && fs::read(file)
            .ok()
            .and_then(|b| serde_json::from_slice::<Value>(&b).ok())
            .is_some_and(|v| v["stage"] == "transition")
}
