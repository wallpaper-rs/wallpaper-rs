use crate::compositor::lock;
use crate::daemon::Daemon;
use crate::error::{Result, failed};
use crate::repos::newer;
use std::collections::BTreeMap;
use std::sync::Arc;
use wprs_core::model::JobKind;
use wprs_format::{Installed, asks};

pub type Held = BTreeMap<(String, String), (String, Vec<String>)>;

pub fn follow(daemon: &Arc<Daemon>, key: &str, value: &str) {
    if key == "repos.auto_update" && value == "true" {
        let daemon = daemon.clone();
        std::thread::spawn(move || daemon.update_installed());
    }
}

impl Daemon {
    pub fn held_asks(&self, repo: &str, id: &str, version: &str) -> Vec<String> {
        let held = lock(&self.remote.held);
        held.get(&(repo.to_owned(), id.to_owned()))
            .filter(|(v, _)| v == version)
            .map_or(Vec::new(), |h| h.1.clone())
    }

    pub fn update_installed(self: &Arc<Self>) {
        if !self.state().config.repos.auto_update || self.library_ready().is_err() {
            return;
        }
        let (Ok(list), Ok(latest)) = (wprs_format::installed(&self.store.data), self.latest()) else {
            return;
        };
        let newest = |r: &&Installed| {
            r.revoked.is_none()
                && list.iter().filter(|i| i.id == r.id).all(|i| !newer(&i.version, &r.version))
        };
        for record in list.iter().filter(newest) {
            let Some((_, version)) = latest.get(&(record.repo_id.clone(), record.id.clone())) else {
                continue;
            };
            let held = self.held_asks(&record.repo_id, &record.id, version);
            if !newer(version, &record.version) || !held.is_empty() {
                continue;
            }
            let spec = format!("{}/{}", record.repo_id, record.id);
            let target = format!("{}/{}@{version}", record.repo_id, record.id);
            if self.jobs.find(JobKind::Install, &target).is_empty()
                && let Err(e) = self.install_as(&spec, false, false, true)
            {
                tracing::warn!(package = record.id, "cannot update a package: {e}");
            }
        }
    }

    pub fn replace_older(&self, record: &Installed) -> Result<()> {
        let all = wprs_format::installed(&self.store.data)?;
        let shared = |v: &str| all.iter().filter(|i| i.id == record.id && i.version == v).count() > 1;
        let older: Vec<&Installed> = all
            .iter()
            .filter(|i| {
                i.repo_id == record.repo_id && i.id == record.id && newer(&record.version, &i.version)
            })
            .filter(|i| !shared(&i.version) && self.shown_on(&i.path).is_ok())
            .collect();
        for old in older {
            wprs_format::remove(&self.store.data, &old.id, Some(&old.version))?;
        }
        Ok(())
    }

    pub fn keep_back(&self, record: &Installed) -> Result<()> {
        let all = wprs_format::installed(&self.store.data)?;
        let others = all.iter().filter(|i| i.id == record.id && i.version != record.version);
        let old = others.max_by_key(|i| semver::Version::parse(&i.version).ok());
        let before = old.and_then(|o| wprs_format::peek(&o.path).ok()).map(|m| asks(&m)).unwrap_or_default();
        let after = asks(&wprs_format::peek(&record.path)?);
        let new: Vec<String> = after.difference(&before).cloned().collect();
        let key = (record.repo_id.clone(), record.id.clone());
        if new.is_empty() {
            lock(&self.remote.held).remove(&key);
            return Ok(());
        }
        wprs_format::remove(&self.store.data, &record.id, Some(&record.version))?;
        lock(&self.remote.held).insert(key, (record.version.clone(), new.clone()));
        self.changes.changed("repos", Vec::new());
        Err(failed(format!(
            "{} {} asks for new permissions ({}) and was not updated; install it by hand to allow them",
            record.id,
            record.version,
            new.join(", ")
        )))
    }
}

#[cfg(test)]
mod tests;
