use crate::daemon::Daemon;
use crate::error::Result;
use crate::items::reply;
use std::collections::HashSet;
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

    pub fn away_items(&self) -> Vec<Away> {
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
