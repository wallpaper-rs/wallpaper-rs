use crate::{library, repos, state};
use std::collections::HashSet;
use std::sync::{Mutex, PoisonError};
use wprs_core::model::Away;
use wprs_library::{Item, Library};
use wprs_sync::Ids;

static IDS: Mutex<Option<Ids>> = Mutex::new(None);

pub(crate) fn remember(ids: &Ids) {
    *IDS.lock().unwrap_or_else(PoisonError::into_inner) = Some(ids.clone());
}

fn here(ids: &Ids, hashed: Vec<(Item, Vec<u8>)>) -> HashSet<String> {
    let installed = repos::installed();
    let keys = hashed.iter().map(|(item, hash)| {
        installed.get(&item.uid).map_or_else(|| ids.blob(hash), |(repo, id)| ids.library(repo, id))
    });
    keys.chain(repos::offered(ids)).collect()
}

pub(crate) fn current() -> Vec<Away> {
    library::with(|lib| Ok(within(lib))).unwrap_or_default()
}

pub(crate) fn within(lib: &Library) -> Vec<Away> {
    let listed = state::read(|s| s.away.clone());
    let ids = IDS.lock().unwrap_or_else(PoisonError::into_inner).clone();
    let Some(ids) = ids.filter(|_| !listed.is_empty()) else { return listed };
    let here = here(&ids, lib.hashed().unwrap_or_default());
    let kept: Vec<Away> = listed.iter().filter(|a| !here.contains(&a.key)).cloned().collect();
    if kept.len() != listed.len() {
        let _ = state::update(|s| s.away.retain(|a| !here.contains(&a.key)));
    }
    kept
}
