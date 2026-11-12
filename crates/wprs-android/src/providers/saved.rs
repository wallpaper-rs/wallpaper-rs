use super::kinds::Options;
use crate::{Error, said, store, vault};
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, MutexGuard, PoisonError};

const FILE: &str = "providers.json";
const SEEN: usize = 2000;

static LOCK: Mutex<()> = Mutex::new(());

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub(super) struct Credit {
    pub(super) uid: String,
    pub(super) title: String,
    pub(super) attribution: Option<String>,
    pub(super) license: Option<String>,
    pub(super) link: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub(super) struct Entry {
    pub(super) id: String,
    pub(super) kind: String,
    pub(super) options: Options,
    pub(super) on: bool,
    pub(super) last_sync: Option<i64>,
    pub(super) last_error: Option<serde_json::Value>,
    pub(super) seen: Vec<String>,
    pub(super) credits: Vec<Credit>,
}

impl Entry {
    pub(super) fn remember(&mut self, id: String) {
        self.seen.push(id);
        let extra = self.seen.len().saturating_sub(SEEN);
        self.seen.drain(..extra);
    }
}

pub(super) fn held() -> MutexGuard<'static, ()> {
    LOCK.lock().unwrap_or_else(PoisonError::into_inner)
}

pub(super) fn load() -> Vec<Entry> {
    let bytes = store::dir().ok().and_then(|dir| std::fs::read(dir.join(FILE)).ok());
    bytes.and_then(|bytes| serde_json::from_slice(&bytes).ok()).unwrap_or_default()
}

pub(super) fn save(entries: &[Entry]) -> Result<(), Error> {
    store::write(&store::dir()?.join(FILE), &serde_json::to_vec_pretty(entries)?)
}

pub(super) fn find<'a>(entries: &'a mut [Entry], id: &str) -> Result<&'a mut Entry, Error> {
    let found = entries.iter_mut().find(|e| e.id == id);
    found.ok_or_else(|| said("android.provider_missing", "there is no picture source %1", &[&id]))
}

fn name(id: &str) -> String {
    format!("provider/{id}")
}

pub(super) fn secret(id: &str) -> Option<String> {
    vault::kept().remove(&name(id)).filter(|s| !s.is_empty())
}

pub(super) fn keep(id: &str, secret: &str) -> Result<(), Error> {
    vault::keep(&name(id), secret)
}
