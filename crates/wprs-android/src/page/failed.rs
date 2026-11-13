use std::collections::BTreeMap;
use std::sync::{Mutex, PoisonError};

const SHOWN: usize = 300;

static FAILED: Mutex<BTreeMap<String, String>> = Mutex::new(BTreeMap::new());

pub(crate) fn note(pack: &str, line: &str) {
    let line: String = line.chars().filter(|c| !c.is_control()).take(SHOWN).collect();
    let mut failed = FAILED.lock().unwrap_or_else(PoisonError::into_inner);
    if failed.get(pack) != Some(&line) {
        failed.insert(pack.to_owned(), line);
        crate::store::changed();
    }
}

pub(crate) fn clear(pack: &str) {
    let mut failed = FAILED.lock().unwrap_or_else(PoisonError::into_inner);
    if failed.remove(pack).is_some() {
        crate::store::changed();
    }
}

pub(crate) fn of(pack: &str) -> Option<String> {
    FAILED.lock().unwrap_or_else(PoisonError::into_inner).get(pack).cloned()
}
