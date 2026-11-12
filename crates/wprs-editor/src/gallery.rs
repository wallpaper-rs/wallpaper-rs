use crate::ops::escape;
use serde_json::{Value, json};
use std::path::Path;

const QUERIES: [&str; 3] = ["film", "sprites", "uses"];
const SKIPPED: [&str; 4] = ["id", "path", "parent", "layer"];

pub fn asks(q: &Value) -> bool {
    QUERIES.iter().any(|k| q.get(k).is_some())
}

fn owner<'a>(doc: &'a Value, pointer: &str) -> Option<(&'a str, &'a str)> {
    let mut at = pointer.to_owned();
    while let Some((up, _)) = at.rsplit_once('/') {
        at = up.to_owned();
        let found = doc.pointer(&at).and_then(Value::as_object);
        if let Some(l) = found.filter(|l| l.get("type").is_some_and(Value::is_string)) {
            return Some((l.get("id")?.as_str()?, l["type"].as_str()?));
        }
    }
    None
}

fn scan(v: &Value, at: &str, id: &str, out: &mut Vec<String>) {
    match v {
        Value::Object(map) => {
            for (key, inner) in map {
                let here = format!("{at}/{}", escape(key));
                match inner {
                    Value::String(s) if s == id && !SKIPPED.contains(&key.as_str()) => out.extend([here]),
                    _ => scan(inner, &here, id, out),
                }
            }
        }
        Value::Array(list) => {
            list.iter().enumerate().for_each(|(i, x)| scan(x, &format!("{at}/{i}"), id, out))
        }
        Value::String(s) if s == id => out.extend([at.to_owned()]),
        _ => {}
    }
}

pub fn uses(doc: &Value, id: &str) -> Value {
    let mut found = Vec::new();
    let scene = doc["scene"].as_object().into_iter().flatten();
    for (key, inner) in scene.filter(|(k, _)| !["assets", "ext"].contains(&k.as_str())) {
        scan(inner, &format!("/scene/{}", escape(key)), id, &mut found);
    }
    scan(&doc["manifest"]["properties"], "/manifest/properties", id, &mut found);
    let rows: Vec<Value> = found
        .into_iter()
        .map(|pointer| {
            let layer = owner(doc, &pointer).map(|(i, t)| json!({ "id": i, "type": t }));
            json!({ "pointer": pointer, "layer": layer })
        })
        .collect();
    json!({ "asset": id, "uses": rows })
}

pub fn query(dir: &Path, doc: &Value, q: &Value) -> Value {
    if let Some(f) = q.get("film") {
        let strip = f["strip"].as_u64().unwrap_or(0) as usize;
        return crate::films::film(dir, doc, f["layer"].as_str().unwrap_or_default(), strip);
    }
    if let Some(s) = q.get("sprites") {
        return crate::cutter::sheet(dir, doc, s["asset"].as_str().unwrap_or_default());
    }
    uses(doc, q["uses"]["asset"].as_str().unwrap_or_default())
}
