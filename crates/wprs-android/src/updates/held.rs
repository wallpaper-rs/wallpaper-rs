use crate::{Error, library, repos, store};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const FILE: &str = "held.json";

type Held = BTreeMap<String, (String, Vec<String>)>;

fn load() -> Held {
    let file = repos::dir().ok().and_then(|d| fs::read(d.join(FILE)).ok());
    file.and_then(|bytes| serde_json::from_slice(&bytes).ok()).unwrap_or_default()
}

fn save(held: &Held) -> Result<(), Error> {
    store::write(&repos::dir()?.join(FILE), &serde_json::to_vec(held)?)
}

pub(crate) fn mark(updates: Vec<Value>) -> Vec<Value> {
    let held = load();
    let mark = |mut update: Value| {
        let asks = update["uid"].as_str().zip(update["available"].as_str()).and_then(|(uid, version)| {
            held.get(uid).filter(|(v, _)| v == version).map(|(_, asks)| asks.clone())
        });
        if let Some(asks) = asks {
            update["asks"] = json!(asks);
        }
        update
    };
    updates.into_iter().map(mark).collect()
}

pub(crate) fn keep_back(
    uid: &str,
    fresh: &str,
    old: &Path,
    new: &Path,
) -> Result<Option<Vec<String>>, Error> {
    let asks = |path: &Path| wprs_format::peek(path).map(|m| (wprs_format::asks(&m), m.version));
    let before = asks(old).map(|(asks, _)| asks).unwrap_or_default();
    let (after, version) = asks(new)?;
    let added: Vec<String> = after.difference(&before).cloned().collect();
    let mut held = load();
    if added.is_empty() {
        if held.remove(uid).is_some() {
            save(&held)?;
        }
        return Ok(None);
    }
    library::with(|lib| Ok(lib.remove_items(&[fresh])?))?;
    drop(fs::remove_file(new));
    let mut installed = repos::installed();
    installed.remove(fresh);
    store::write(&repos::dir()?.join(repos::ITEMS), &serde_json::to_vec(&installed)?)?;
    held.insert(uid.to_owned(), (version, added.clone()));
    save(&held)?;
    Ok(Some(added))
}
