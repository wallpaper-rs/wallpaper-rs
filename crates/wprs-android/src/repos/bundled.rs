use super::{add_url, dir, fetch};
use crate::library::now;
use crate::{Error, store};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

const NAME: &str = "Example wallpapers";
const FILE: &str = "examples-repo.json";

#[derive(Default, Serialize, Deserialize)]
struct Record {
    repo_id: String,
    declined: bool,
    stamp: i64,
}

fn record() -> Record {
    let read = dir().ok().and_then(|d| std::fs::read(d.join(FILE)).ok());
    read.and_then(|bytes| serde_json::from_slice(&bytes).ok()).unwrap_or_default()
}

fn keep(record: &Record) -> Result<(), Error> {
    std::fs::create_dir_all(dir()?)?;
    store::write(&dir()?.join(FILE), &serde_json::to_vec(record)?)
}

fn places() -> Result<(PathBuf, PathBuf), Error> {
    Ok((dir()?.join("examples-repo"), dir()?.join("examples-keys")))
}

pub(crate) fn is_bundled(repo_id: &str) -> bool {
    !repo_id.is_empty() && record().repo_id == repo_id
}

pub(crate) fn declined(repo_id: &str) {
    let current = record();
    if current.repo_id == repo_id {
        drop(keep(&Record { declined: true, ..current }));
    }
}

pub(crate) fn due(stamp: i64) -> Result<bool, Error> {
    let current = record();
    if current.declined {
        return Ok(false);
    }
    let (repo, keys) = places()?;
    if current.repo_id.is_empty() || current.stamp != stamp {
        return Ok(true);
    }
    let renewed = wprs_format::bundled(&repo, &keys, NAME, &[], now())?;
    if renewed.changed {
        wprs_format::update_repo(&dir()?, &fetch()?, &renewed.repo_id, now())?;
    }
    Ok(false)
}

fn built(source: &Path, into: &Path) -> Result<Vec<PathBuf>, Error> {
    let mut folders: Vec<PathBuf> = std::fs::read_dir(source)?.flatten().map(|e| e.path()).collect();
    folders.retain(|folder| folder.join(wprs_format::MANIFEST).is_file());
    folders.sort();
    let mut made = Vec::new();
    for folder in folders {
        let name = folder.file_name().and_then(|n| n.to_str()).unwrap_or("scene");
        let wpak = into.join(format!("{name}.wpak"));
        wprs_format::build(&folder, &wpak, &wprs_format::Limits::default())?;
        made.push(wpak);
    }
    Ok(made)
}

pub(crate) fn offer(source: &Path, stamp: i64, force: bool) -> Result<Value, Error> {
    let current = record();
    if current.declined && !force {
        return Ok(Value::Null);
    }
    let work = tempfile::tempdir_in(store::dir()?)?;
    let wpaks = built(source, work.path())?;
    let (repo, keys) = places()?;
    std::fs::create_dir_all(dir()?)?;
    let bundle = wprs_format::bundled(&repo, &keys, NAME, &wpaks, now())?;
    keep(&Record { repo_id: bundle.repo_id.clone(), declined: false, stamp })?;
    if super::added().contains(&bundle.repo_id) {
        wprs_format::update_repo(&dir()?, &fetch()?, &bundle.repo_id, now())?;
    } else {
        add_url(&bundle.url)?;
    }
    Ok(json!({"repo_id": bundle.repo_id}))
}
