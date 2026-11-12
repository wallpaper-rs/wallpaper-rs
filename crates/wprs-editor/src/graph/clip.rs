use super::{index, list};
use crate::action::KeyPick;
use anyhow::{Result, ensure};
use serde_json::{Value, json};
use wprs_core::message::said;

pub const MARKER: &str = "wprs-keys:";

fn copied(doc: &Value, picks: &[KeyPick]) -> Result<Vec<Value>> {
    ensure!(
        !picks.is_empty(),
        said("editor.keys_picks", "picks names the keys to change: a pointer and the times of its keys", &[])
    );
    let mut out = Vec::new();
    for pick in picks {
        let (_, keys) = list(doc, &pick.pointer)?;
        let chosen =
            pick.times.iter().map(|t| index(&keys, &pick.pointer, *t)).collect::<Result<Vec<_>>>()?;
        let taken: Vec<Value> = chosen.into_iter().map(|i| keys[i].clone()).collect();
        out.push(json!({ "pointer": pick.pointer, "keys": taken }));
    }
    Ok(out)
}

fn keyed(pick: &Value) -> bool {
    let keys = pick["keys"].as_array().filter(|k| !k.is_empty());
    let fine = |k: &Value| k.as_array().is_some_and(|k| (2..=3).contains(&k.len()) && k[0].is_number());
    pick["pointer"].as_str().is_some_and(|p| p.starts_with('/')) && keys.is_some_and(|k| k.iter().all(fine))
}

pub fn read(text: &str) -> Option<Vec<Value>> {
    let found: Value = serde_json::from_str(text.strip_prefix(MARKER)?).ok()?;
    let picks = found["picks"].as_array().filter(|p| !p.is_empty())?;
    picks.iter().all(keyed).then(|| picks.clone())
}

pub fn key_clip(doc: &Value, q: &Value) -> Value {
    if let Some(text) = q.get("read") {
        return json!({ "picks": text.as_str().and_then(read) });
    }
    let picks: Vec<KeyPick> = serde_json::from_value(q["copy"].clone()).unwrap_or_default();
    match copied(doc, &picks) {
        Ok(found) => json!({ "picks": found, "text": format!("{MARKER}{}", json!({ "picks": found })) }),
        Err(e) => json!({ "error": e.to_string() }),
    }
}
