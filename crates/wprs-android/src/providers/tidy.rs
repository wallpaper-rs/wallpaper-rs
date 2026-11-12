use crate::{Error, library, settings};
use wprs_library::Library;

pub(crate) fn tag(id: &str) -> String {
    format!("from-{id}")
}

fn protected(lib: &Library, uid: &str) -> bool {
    let Ok(item) = lib.item(uid) else { return false };
    let shown = |o: &wprs_core::config::OutputConfig| o.image.as_ref() == Some(&item.path);
    let config = settings::config();
    let listed = lib.item_playlists(uid).is_ok_and(|p| !p.is_empty());
    item.favorite || listed || shown(&config.default) || config.outputs.values().any(shown)
}

pub(super) fn evict(id: &str, keep: usize, now: i64) -> Result<(), Error> {
    library::with(|lib| {
        let mut mine = lib.search(&format!("tag:{}", tag(id)), now, 0, u32::MAX)?;
        mine.sort_by(|a, b| (a.added_at, &a.uid).cmp(&(b.added_at, &b.uid)));
        let excess = mine.len().saturating_sub(keep);
        let gone: Vec<String> =
            mine.iter().map(|i| i.uid.clone()).filter(|u| !protected(lib, u)).take(excess).collect();
        library::drop_items(lib, &gone, now).map(drop)
    })
}
