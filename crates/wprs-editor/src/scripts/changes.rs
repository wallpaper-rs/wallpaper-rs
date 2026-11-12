use serde_json::{Value, json};
use wprs_core::message::{Message, said};

fn names(entry: &Value, key: &str) -> Vec<String> {
    entry[key].as_array().into_iter().flatten().filter_map(Value::as_str).map(String::from).collect()
}

fn count(entry: &Value) -> usize {
    entry["params"].as_object().map_or(0, |p| p.len())
}

fn moved(path: &str, old: &Value, new: &Value, out: &mut Vec<Message>) {
    let delta = |key: &str| {
        let (before, after) = (names(old, key), names(new, key));
        let plus: Vec<String> = after.iter().filter(|n| !before.contains(n)).cloned().collect();
        let minus: Vec<String> = before.iter().filter(|n| !after.contains(n)).cloned().collect();
        (plus.join(", "), minus.join(", "))
    };
    let ((handled, unhandled), (used, unused)) = (delta("events"), delta("uses"));
    if !handled.is_empty() {
        out.push(said("editor.script_handles", "%1 now handles %2", &[&path, &handled]));
    }
    if !unhandled.is_empty() {
        out.push(said("editor.script_stops", "%1 no longer handles %2", &[&path, &unhandled]));
    }
    if !used.is_empty() {
        out.push(said("editor.script_uses", "%1 now uses %2", &[&path, &used]));
    }
    if !unused.is_empty() {
        out.push(said("editor.script_drops", "%1 no longer uses %2", &[&path, &unused]));
    }
}

pub fn changes(before: &Value, after: &Value) -> Vec<Message> {
    let list = |v: &Value| v.as_array().cloned().unwrap_or_default();
    let (before, after) = (list(before), list(after));
    let mut out = Vec::new();
    for new in &after {
        let path = new["path"].as_str().unwrap_or_default();
        let old = before.iter().find(|s| s["path"] == new["path"]);
        let first = out.len();
        if old.is_none() {
            out.push(said("editor.script_added", "%1 was added", &[&path]));
        }
        let old = old.cloned().unwrap_or_else(|| json!({}));
        moved(path, &old, new, &mut out);
        if count(new) > 0 && count(&old) > 0 {
            out.push(said("editor.script_kept", "%1 kept its %2 settings", &[&path, &count(new)]));
        }
        if out.len() == first {
            out.push(said("editor.script_same", "%1 has the same declaration", &[&path]));
        }
    }
    let gone = before.iter().filter(|s| !after.iter().any(|a| a["path"] == s["path"]));
    out.extend(gone.map(|s| {
        said("editor.script_removed", "%1 was removed", &[&s["path"].as_str().unwrap_or_default()])
    }));
    out
}
