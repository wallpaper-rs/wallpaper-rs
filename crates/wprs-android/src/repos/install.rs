use super::{ITEMS, dir, fetch, installed};
use crate::library::now;
use crate::{Error, library, store};
use serde_json::{Value, json};
use std::path::PathBuf;

pub(crate) fn install(repo_id: &str, id: &str) -> Result<Value, Error> {
    let (uid, path) = fetched(&format!("{repo_id}/{id}"))?;
    let needs = crate::variant::needed(repo_id, &path, 0);
    store::changed();
    Ok(json!({"item": uid, "failed": needs.failed}))
}

pub(crate) fn fetched(spec: &str) -> Result<(String, PathBuf), Error> {
    let (data, fetch) = (dir()?, fetch()?);
    let mut done = wprs_format::install(&data, &fetch, spec, now(), false, false)?;
    if !done.path.is_file() {
        wprs_format::remove(&data, &done.id, Some(&done.version))?;
        done = wprs_format::install(&data, &fetch, spec, now(), false, false)?;
    }
    let (uid, _) = store::store(&mut std::fs::File::open(&done.path)?, &format!("{}.wpak", done.id))?;
    drop(std::fs::remove_file(&done.path));
    let mut items = installed();
    items.insert(uid.clone(), (done.repo_id, done.id));
    store::write(&dir()?.join(ITEMS), &serde_json::to_vec(&items)?)?;
    Ok((uid.clone(), library::with(|lib| Ok(lib.item(&uid)?.path))?))
}
