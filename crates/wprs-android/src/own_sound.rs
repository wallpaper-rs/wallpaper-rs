use crate::library::with;
use std::collections::HashMap;
use std::sync::{Mutex, PoisonError};

type Seen = HashMap<String, f64>;

static SEEN: Mutex<Option<Seen>> = Mutex::new(None);

pub(crate) fn forget() {
    *SEEN.lock().unwrap_or_else(PoisonError::into_inner) = None;
}

pub(crate) fn factor(muted: Option<bool>, volume: Option<u8>) -> f64 {
    if muted == Some(true) { 0.0 } else { volume.map_or(1.0, |v| f64::from(v.min(100)) / 100.0) }
}

fn heard(column: &str, value: &str) -> f64 {
    let known = SEEN.lock().unwrap_or_else(PoisonError::into_inner);
    if let Some(known) = known.as_ref().and_then(|s| s.get(value)) {
        return *known;
    }
    drop(known);
    let found = with(|lib| Ok(lib.sound_where(column, value)?)).map_or(1.0, |(m, v)| factor(m, v));
    SEEN.lock()
        .unwrap_or_else(PoisonError::into_inner)
        .get_or_insert_default()
        .insert(value.to_owned(), found);
    found
}

pub(crate) fn of_package(id: &str) -> f64 {
    heard("pkg_id", id)
}
