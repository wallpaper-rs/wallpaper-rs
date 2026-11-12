use super::audit::audit;
use super::declare::functions;
use crate::ops::escape;
use serde_json::{Value, json};
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;

fn walk(value: &Value, pointer: &str, path: &str, out: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            if map.get("script").is_some_and(|s| s == path) {
                out.push(pointer.to_owned());
            }
            map.iter().for_each(|(k, v)| walk(v, &format!("{pointer}/{}", escape(k)), path, out));
        }
        Value::Array(list) => {
            list.iter().enumerate().for_each(|(i, v)| walk(v, &format!("{pointer}/{i}"), path, out))
        }
        _ => {}
    }
}

pub fn bound(doc: &Value, path: &str) -> Vec<String> {
    let mut out = Vec::new();
    walk(&doc["scene"], "/scene", path, &mut out);
    out
}

fn read(dir: &Path, path: &str) -> Option<String> {
    wprs_format::check_path(path).ok()?;
    fs::read_to_string(dir.join(path)).ok()
}

pub fn listing(dir: &Path, doc: &Value) -> Value {
    let entries = doc.pointer("/scene/scripts").and_then(Value::as_array).into_iter().flatten().enumerate();
    let scripts: Vec<Value> = entries
        .map(|(i, s)| {
            let path = s["path"].as_str().unwrap_or_default();
            let text = read(dir, path);
            let found = audit(dir, doc, path).unwrap_or_else(|_| json!({}));
            json!({"path": path, "pointer": format!("/scene/scripts/{i}"), "size": text.as_ref().map_or(0, String::len),
                "missing": text.is_none(), "events": s["events"], "uses": s["uses"],
                "params": s.get("params").cloned().unwrap_or_else(|| json!({})),
                "functions": text.as_deref().map(functions).unwrap_or_default(),
                "bound": bound(doc, path), "synced": found["synced"], "audit": found})
        })
        .collect();
    let have = |p: &String| scripts.iter().any(|s| s["path"] == *p);
    let free: Vec<String> =
        crate::pages::scripts(dir).into_iter().filter(|p| p.starts_with("scripts/")).collect();
    json!({"scripts": scripts, "free": free.into_iter().filter(|p| !have(p)).collect::<Vec<_>>()})
}

pub fn stamp(dir: &Path, doc: &Value) -> u64 {
    let mut hasher = DefaultHasher::new();
    let list = doc.pointer("/scene/scripts").and_then(Value::as_array).into_iter().flatten();
    for path in list.filter_map(|s| s["path"].as_str()) {
        let meta = wprs_format::check_path(path).ok().and_then(|_| fs::metadata(dir.join(path)).ok());
        let modified = meta.as_ref().and_then(|m| m.modified().ok());
        (path, meta.map(|m| m.len()), modified).hash(&mut hasher);
    }
    hasher.finish()
}
