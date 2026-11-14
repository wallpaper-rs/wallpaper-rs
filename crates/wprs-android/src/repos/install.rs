use super::{ITEMS, dir, installed};
use crate::library::now;
use crate::said;
use crate::state::read;
use crate::{Error, library, store};
use serde_json::{Value, json};
use std::path::PathBuf;
use std::sync::Arc;
use wprs_providers::HttpFetch;

pub(crate) fn install(repo_id: &str, id: &str) -> Result<Value, Error> {
    let (uid, path) = fetched(&format!("{repo_id}/{id}"))?;
    let needs = crate::variant::needed(repo_id, &path, 0);
    store::changed();
    Ok(json!({"item": uid, "failed": needs.failed}))
}

pub(crate) fn fetched(spec: &str) -> Result<(String, PathBuf), Error> {
    let (data, fetch) = (dir()?, fetch()?);
    let done = wprs_format::install(&data, &fetch, spec, now(), false, false)?;
    let (uid, _) = store::store(&mut std::fs::File::open(&done.path)?, &format!("{}.wpak", done.id))?;
    drop(std::fs::remove_file(&done.path));
    let mut items = installed();
    items.insert(uid.clone(), (done.repo_id, done.id));
    store::write(&dir()?.join(ITEMS), &serde_json::to_vec(&items)?)?;
    Ok((uid.clone(), library::with(|lib| Ok(lib.item(&uid)?.path))?))
}

pub(crate) fn fetch() -> Result<HttpFetch, Error> {
    let failed =
        |e: &dyn std::fmt::Display| said("android.web_client", "web connections cannot start: %1", &[&e]);
    let web = HttpFetch::new(Arc::default()).map_err(|e| failed(&e))?;
    let (url, pin) = read(|s| (s.url.clone(), s.pin.clone()));
    match pin.filter(|p| !url.is_empty() && !p.is_empty()) {
        Some(pin) => web.pinned(&url, wprs_sync::pinned_tls(&pin)?).map_err(|e| failed(&e)),
        None => Ok(web),
    }
}

pub(crate) fn add_url(url: &str) -> Result<Value, Error> {
    std::fs::create_dir_all(dir()?)?;
    let info = wprs_format::add_repo(&dir()?, &fetch()?, url, now())?;
    store::changed();
    Ok(json!({"repo_id": info.repo_id, "name": info.name, "packages": info.packages}))
}
