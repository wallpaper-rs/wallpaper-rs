use crate::admin::{latest, load};
use crate::repo::at;
use crate::store::json;
use crate::{
    Entry, Result, Settings, Signer, add, init, keygen, publish, rotate_root, settings, sign_root, status,
};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

const ENTRY_DAYS: u32 = 90;
const ROOT_DAYS: u32 = 1096;
const RENEW_ENTRY_DAYS: i64 = 45;
const RENEW_ROOT_DAYS: i64 = 90;
const DAY: i64 = 86_400;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bundled {
    pub url: String,
    pub repo_id: String,
    pub changed: bool,
}

fn secret(keys: &Path, name: &str) -> Result<Signer> {
    Signer::new(&fs::read_to_string(keys.join(format!("{name}.key")))?, None)
}

fn make(keys: &Path, name: &str) -> Result<String> {
    fs::create_dir_all(keys)?;
    fs::set_permissions(keys, fs::Permissions::from_mode(0o700))?;
    let pair = keygen(None)?;
    let file = keys.join(format!("{name}.key"));
    fs::write(&file, &pair.secret)?;
    fs::set_permissions(&file, fs::Permissions::from_mode(0o600))?;
    Ok(pair.public)
}

fn create(repo: &Path, keys: &Path, name: &str, now: i64) -> Result<()> {
    let start = Settings {
        name: name.to_owned(),
        root_keys: vec![make(keys, "root")?],
        online_keys: vec![make(keys, "online")?],
        entry_max_days: ENTRY_DAYS,
        root_days: ROOT_DAYS,
        ..Settings::default()
    };
    init(repo, &start, now)?;
    sign_root(repo, &secret(keys, "root")?)?;
    Ok(())
}

fn renew_root(repo: &Path, keys: &Path, now: i64) -> Result<bool> {
    let (_, version) = latest(repo)?;
    if (load(repo, version)?.expires.timestamp() - now) / DAY > RENEW_ROOT_DAYS {
        return Ok(false);
    }
    rotate_root(repo, &settings(repo)?, now)?;
    sign_root(repo, &secret(keys, "root")?)?;
    Ok(true)
}

fn stale(repo: &Path, now: i64) -> bool {
    let last = fs::read_to_string(repo.join("entry/latest")).ok().and_then(|t| t.trim().parse::<u64>().ok());
    let entry: Option<Entry> = last.and_then(|n| json(&repo.join(format!("entry/{n}.json"))).ok());
    entry.is_none_or(|e| (e.expires.timestamp() - now) / DAY < RENEW_ENTRY_DAYS)
}

fn present(repo: &Path, wpak: &Path) -> bool {
    let Ok(manifest) = crate::peek(wpak) else { return false };
    repo.join("packages").join(&manifest.id).join(format!("{}.wpak", manifest.version)).is_file()
}

pub fn bundled(repo: &Path, keys: &Path, name: &str, wpaks: &[PathBuf], now: i64) -> Result<Bundled> {
    at(now)?;
    let fresh = !repo.join("root/1.json").is_file();
    if fresh {
        create(repo, keys, name, now)?;
    }
    let mut changed = fresh || renew_root(repo, keys, now)?;
    for wpak in wpaks.iter().filter(|w| !present(repo, w)) {
        add(repo, wpak, None, now)?;
        changed = true;
    }
    if changed || stale(repo, now) {
        publish(repo, &secret(keys, "online")?, ENTRY_DAYS, now)?;
        changed = true;
    }
    let found = status(repo)?;
    Ok(Bundled { url: found.address, repo_id: found.repo_id, changed })
}
