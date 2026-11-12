use super::{list, sorted};
use crate::action::KeysEdit;
use crate::keys::time_of;
use crate::ops::{Op, set};
use anyhow::{Result, anyhow, ensure};
use serde_json::{Value, json};
use wprs_core::message::said;

fn readable(key: &Value) -> bool {
    let list = key.as_array();
    list.is_some_and(|k| (2..=3).contains(&k.len()) && k[0].as_f64().is_some_and(|t| t.is_finite()))
}

pub(super) fn paste(doc: &Value, e: &KeysEdit) -> Result<Vec<Op>> {
    let at = e.at.ok_or_else(|| {
        anyhow!(said("editor.keys_at", "at is the second where the first pasted key goes", &[]))
    })?;
    let all = e.picks.iter().flat_map(|p| &p.keys);
    ensure!(
        all.clone().all(readable),
        said(
            "editor.keys_pasted",
            "a pasted key is [time, value], [time, value, easing] or [time, value, {\"in\", \"out\"}]",
            &[]
        )
    );
    let first = all.map(time_of).fold(f64::INFINITY, f64::min);
    let mut ops = Vec::new();
    for pick in e.picks.iter().filter(|p| !p.keys.is_empty()) {
        let pointer = pick.pointer.as_str();
        let (path, mut keys, old) = match list(doc, pointer) {
            Ok((path, keys)) => (path, keys.clone(), keys),
            Err(_) if doc.pointer(pointer).is_some_and(|v| !v.is_object()) => {
                (pointer.to_owned(), Vec::new(), Vec::new())
            }
            Err(e) => return Err(e),
        };
        for key in &pick.keys {
            let t = ((at + time_of(key) - first) * 1000.0).round() / 1000.0;
            ensure!(
                t >= 0.0,
                said("editor.keys_before_zero", "the keys of %1 would move before 0 s", &[&pointer])
            );
            keys.retain(|k| (time_of(k) - t).abs() >= 1e-3);
            let mut key = key.clone();
            key[0] = json!(t);
            keys.push(key);
        }
        sorted(pointer, &mut keys)?;
        crate::handles::reach(pointer, &old, &keys)?;
        let value = match path == pointer && old.is_empty() {
            true => json!({ "keys": keys }),
            false => Value::Array(keys),
        };
        ops.extend(set(doc, &path, value));
    }
    Ok(ops)
}
