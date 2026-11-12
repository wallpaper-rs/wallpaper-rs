use super::declare::{BLIND, declaration, used_params};
use anyhow::{Result, ensure};
use serde_json::{Value, json};
use std::fs;
use std::path::Path;
use wprs_core::message::said;

fn names(value: &Value) -> Vec<String> {
    value.as_array().into_iter().flatten().filter_map(Value::as_str).map(String::from).collect()
}

fn plain(mut events: Vec<String>) -> Vec<String> {
    if events == ["update"] {
        events.clear();
    }
    events
}

fn minus(a: &[String], b: &[String]) -> Vec<String> {
    a.iter().filter(|n| !b.contains(n)).cloned().collect()
}

pub fn audit(dir: &Path, doc: &Value, path: &str) -> Result<Value> {
    let list = doc.pointer("/scene/scripts").and_then(Value::as_array);
    let entry = list.and_then(|l| l.iter().find(|s| s["path"] == path));
    let entry = entry.ok_or_else(|| said("editor.no_script", "the scene has no script %1", &[&path]))?;
    ensure!(
        wprs_format::check_path(path).is_ok(),
        said("editor.script_path", "%1 is not a path of the package", &[&path])
    );
    let text = fs::read_to_string(dir.join(path))
        .map_err(|e| said("editor.cannot_read", "cannot read %1: %2", &[&path, &e]))?;
    let found = declaration(&text);
    let (declared_events, found_events) = (plain(names(&entry["events"])), plain(names(&found["events"])));
    let (declared_uses, found_uses) = (names(&entry["uses"]), names(&found["uses"]));
    let declared_params: Vec<String> =
        entry["params"].as_object().into_iter().flatten().map(|(k, _)| k.clone()).collect();
    let used = used_params(&text);
    let vague = used.is_empty() && text.contains("params");
    let unused_uses: Vec<String> =
        minus(&declared_uses, &found_uses).into_iter().filter(|u| !BLIND.contains(&u.as_str())).collect();
    let undeclared = json!({"events": minus(&found_events, &declared_events),
        "uses": minus(&found_uses, &declared_uses), "params": minus(&used, &declared_params)});
    let unused = json!({"events": minus(&declared_events, &found_events), "uses": unused_uses,
        "params": if vague { Vec::new() } else { minus(&declared_params, &used) }});
    let empty = |v: &Value| {
        v.as_object().is_some_and(|m| m.values().all(|l| l.as_array().is_some_and(Vec::is_empty)))
    };
    Ok(json!({"path": path, "undeclared": undeclared, "unused": unused,
        "synced": empty(&undeclared) && empty(&unused),
        "rig": super::rigcalls::missing(dir, doc, &text)}))
}
