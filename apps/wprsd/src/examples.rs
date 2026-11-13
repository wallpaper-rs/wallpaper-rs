use crate::daemon::Daemon;
use crate::error::{Result, not_found};
use crate::jobs::now;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};

const NAME: &str = "Example wallpapers";
const EVERY: i64 = 86_400;
static LAST: AtomicI64 = AtomicI64::new(0);

#[derive(Default, Serialize, Deserialize)]
struct Record {
    repo_id: String,
    declined: bool,
}

fn shipped() -> Vec<PathBuf> {
    let system = std::env::var_os("XDG_DATA_DIRS").unwrap_or_else(|| "/usr/local/share:/usr/share".into());
    let dirs = dirs::data_dir().into_iter().chain(std::env::split_paths(&system));
    let entries = dirs.flat_map(|d| std::fs::read_dir(d.join("wallpaper-rs/examples")).into_iter().flatten());
    let mut files: Vec<PathBuf> = entries.flatten().map(|e| e.path()).collect();
    files.retain(|p| p.extension().is_some_and(|e| e == "wpak") && p.is_file());
    files.sort();
    files.dedup_by(|a, b| a.file_name() == b.file_name());
    files
}

impl Daemon {
    fn examples_record(&self) -> Record {
        let read = std::fs::read(self.store.data.join("examples-repo.json")).ok();
        read.and_then(|bytes| serde_json::from_slice(&bytes).ok()).unwrap_or_default()
    }

    fn examples_keep(&self, record: &Record) {
        if let Err(e) = crate::inputs::save(&self.store.data.join("examples-repo.json"), record) {
            tracing::warn!(error = %e.kind(), "cannot record the example repository");
        }
    }

    pub fn is_examples(&self, repo_id: &str) -> bool {
        !repo_id.is_empty() && self.examples_record().repo_id == repo_id
    }

    pub fn examples_removed(&self, repo_id: &str) {
        let record = self.examples_record();
        if record.repo_id == repo_id {
            self.examples_keep(&Record { declined: true, ..record });
        }
    }

    fn examples_bundle(&self) -> Result<wprs_format::Bundled> {
        let files = shipped();
        if files.is_empty() {
            return Err(not_found("this installation carries no example wallpapers"));
        }
        let (repo, keys) = (self.store.data.join("examples-repo"), self.store.data.join("examples-keys"));
        Ok(wprs_format::bundled(&repo, &keys, NAME, &files, now())?)
    }

    pub fn offer_examples(self: &Arc<Self>) {
        let at = now();
        if at - LAST.load(Ordering::Relaxed) < EVERY && LAST.load(Ordering::Relaxed) != 0 {
            return;
        }
        LAST.store(at, Ordering::Relaxed);
        let record = self.examples_record();
        if record.declined {
            return;
        }
        let bundle = match self.examples_bundle() {
            Ok(bundle) => bundle,
            Err(e) => {
                tracing::debug!("{e}");
                return;
            }
        };
        let known = wprs_format::repos(&self.store.data, at).unwrap_or_default();
        let sound = known.iter().any(|r| r.repo_id == bundle.repo_id && r.broken.is_none());
        self.examples_keep(&Record { repo_id: bundle.repo_id.clone(), declined: false });
        if !sound {
            drop(self.add_repo(&bundle.url));
        } else if bundle.changed {
            self.refresh_job(&bundle.repo_id);
        }
    }

    pub fn add_examples(self: &Arc<Self>) -> Result<u64> {
        let bundle = self.examples_bundle()?;
        self.examples_keep(&Record { repo_id: bundle.repo_id, declined: false });
        self.add_repo(&bundle.url)
    }
}
