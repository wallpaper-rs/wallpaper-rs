use crate::compositor::lock;
use crate::daemon::Daemon;
use crate::error::Result;
use crate::items::reply;
use crate::jobs::now;
use std::collections::HashSet;
use std::sync::atomic::Ordering;
use wprs_core::model::{Away, AwayReply, MODEL};
use wprs_sync::Ids;

pub fn away_title(away: &[Away], profile: Option<&str>, output: &str) -> Option<String> {
    let on = |a: &&Away| a.profiles.iter().any(|p| Some(p.profile.as_str()) == profile && p.output == output);
    away.iter().find(on).map(|a| a.title.clone())
}

impl Daemon {
    pub fn offered_keys(&self, ids: &Ids) -> HashSet<String> {
        let offers = wprs_format::browse_with(&self.store.data, "", true).unwrap_or_default();
        offers.into_iter().map(|offer| ids.library(&offer.repo_id, &offer.id)).collect()
    }

    pub fn added_repos(&self) -> HashSet<String> {
        let known = wprs_format::repos(&self.store.data, now()).unwrap_or_default();
        known.into_iter().map(|r| r.repo_id).collect()
    }

    fn local_keys(&self, ids: &Ids) -> HashSet<String> {
        let installed = self.state().installed.clone();
        let hashed = self.store.read(|lib| Ok::<_, crate::error::Error>(lib.hashed()?)).unwrap_or_default();
        let key = |(item, hash): &(wprs_library::Item, Vec<u8>)| {
            let package = installed.iter().find(|i| i.path == item.path);
            package.map_or_else(|| ids.blob(hash), |p| ids.library(&p.repo_id, &p.id))
        };
        hashed.iter().map(key).collect()
    }

    fn prune_away(&self) {
        let Some(ids) = lock(&self.account.ids).clone() else { return };
        let mut here = self.local_keys(&ids);
        here.extend(self.offered_keys(&ids));
        let _ = self.update(|s| s.away.retain(|a| !here.contains(&a.key)));
    }

    pub fn away_items(&self) -> Vec<Away> {
        let serial = self.changes.serial();
        if !self.saved().away.is_empty() && self.account.pruned.swap(serial, Ordering::Relaxed) != serial {
            self.prune_away();
        }
        self.saved().away.clone()
    }

    pub fn away_page(&self, query: &str, offset: u32) -> Vec<Away> {
        if query.trim().is_empty() && offset == 0 { self.away_items() } else { Vec::new() }
    }

    pub fn away_in(&self, playlist: &str) -> Vec<Away> {
        let mut found = self.away_items();
        found.retain(|a| a.playlists.iter().any(|p| p == playlist));
        found
    }

    pub fn list_away(&self) -> Result<String> {
        reply(AwayReply { model: MODEL, serial: self.changes.serial(), items: self.away_items() })
    }
}
