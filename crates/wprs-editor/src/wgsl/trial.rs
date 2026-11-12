use crate::ops::escape;
use serde_json::{Value, json};
use std::path::Path;
use wprs_scene::{Descriptor, MAX_WGSL};

pub fn located(error: &str) -> Option<(usize, usize)> {
    let at = error.find("wgsl:")? + 5;
    let mut parts = error[at..].split(|c: char| !c.is_ascii_digit()).filter(|p| !p.is_empty());
    let line: usize = parts.next()?.parse().ok()?;
    let column = parts.next().and_then(|c| c.parse().ok()).unwrap_or(1);
    Some((wprs_scene::shader_line(line)?, column))
}

pub(crate) fn files(doc: &Value, id: &str) -> Option<(String, String)> {
    let entry = doc.pointer(&format!("/scene/effects/{}", escape(id)))?;
    let path = |k: &str| entry[k].as_str().filter(|p| wprs_format::check_path(p).is_ok()).map(String::from);
    Some((path("descriptor")?, path("wgsl")?))
}

fn read(dir: &Path, path: &str) -> Option<String> {
    let size = std::fs::metadata(dir.join(path)).ok()?.len();
    (size <= MAX_WGSL as u64).then(|| std::fs::read_to_string(dir.join(path)).ok()).flatten()
}

fn handles(d: &Descriptor) -> Vec<Value> {
    let kind = |g: wprs_scene::Grab| format!("{g:?}").to_lowercase();
    let held = d.handles();
    held.iter().map(|h| json!({ "param": h.param, "handle": kind(h.kind), "partner": h.partner })).collect()
}

pub fn check(id: &str, json: &str, wgsl: &str) -> Value {
    if json.len() > MAX_WGSL || wgsl.len() > MAX_WGSL {
        let error = format!("a descriptor or a shader is at most {} KiB", MAX_WGSL >> 10);
        return json!({ "effect": id, "ok": false, "error": error });
    }
    let stage = serde_json::from_str::<Value>(json).ok().and_then(|v| v["stage"].as_str().map(String::from));
    let result = Descriptor::packaged(id, json, wgsl);
    let error = result.as_ref().err().cloned();
    let at = error.as_deref().and_then(located);
    let built = result.as_ref().ok();
    json!({ "effect": id, "ok": error.is_none(), "stage": stage.as_deref().unwrap_or("layer"),
        "error": error, "line": at.map(|a| a.0), "column": at.map(|a| a.1),
        "units": built.map(Descriptor::units), "uses": built.map(|d| json!(d.uses)),
        "handles": built.map(handles).unwrap_or_default() })
}

pub fn checked(dir: &Path, doc: &Value, q: &Value) -> Value {
    let id = q["effect"].as_str().unwrap_or_default();
    let Some((descriptor, wgsl)) = files(doc, id) else {
        return json!({ "effect": id, "ok": false, "error": "the shader files are missing" });
    };
    let given = |k: &str, path: &str| q[k].as_str().map(String::from).or_else(|| read(dir, path));
    match (given("descriptor", &descriptor), given("wgsl", &wgsl)) {
        (Some(json), Some(code)) => check(id, &json, &code),
        _ => json!({ "effect": id, "ok": false, "error": "the shader files are missing" }),
    }
}

pub fn source(dir: &Path, doc: &Value, id: &str) -> Value {
    let Some((descriptor, wgsl)) = files(doc, id) else { return Value::Null };
    let json = read(dir, &descriptor).unwrap_or_default();
    let parsed: Value = serde_json::from_str(&json).unwrap_or(Value::Null);
    let found: Option<Descriptor> = serde_json::from_value(parsed.clone()).ok();
    let tools = found.iter().flat_map(|d| d.params.iter().map(|p| (p.id.clone(), json!(p.tools(id)))));
    let tools: serde_json::Map<String, Value> = tools.collect();
    json!({ "effect": id, "descriptor_path": descriptor, "wgsl_path": wgsl, "descriptor": json,
        "wgsl": read(dir, &wgsl).unwrap_or_default(), "parsed": parsed, "tools": tools,
        "stage": parsed["stage"].as_str().unwrap_or("layer"), "templates": super::starters::TEMPLATES,
        "kinds": super::form::KINDS, "fields": super::form::FIELDS })
}
