use super::{Expiry, Session, Told};
use crate::daemon::Daemon;
use crate::error::Result;
use crate::items::reply;
use crate::playlists1::Playlists;
use std::sync::Weak;
use std::time::Instant;
use wprs_core::model::{BrowseEnd, BrowseState, MODEL};
use zbus::object_server::SignalEmitter;

impl Daemon {
    pub fn browse_reply(
        &self,
        output: &str,
        session: Option<&Session>,
        ended: Option<BrowseEnd>,
    ) -> Result<String> {
        let candidate = session.and_then(|s| s.candidate.clone());
        let item = candidate.map(|uid| self.store.read(|lib| lib.item(&uid))).transpose()?;
        let path = item.as_ref().map(|i| i.path.clone());
        let candidate = item.map(|i| self.summary(i)).transpose()?;
        let (playlist, playlist_name) = session.and_then(|s| s.playlist.clone()).unzip();
        let left = session.map(|s| s.until.saturating_duration_since(Instant::now()));
        reply(BrowseState {
            model: MODEL,
            serial: self.changes.serial(),
            output: output.to_owned(),
            active: session.is_some(),
            ended,
            source: session.map(|s| s.source),
            playlist,
            playlist_name: playlist_name.unwrap_or_default(),
            steps: session.map_or(0, |s| s.steps),
            position: session.map_or(0, |s| s.position),
            count: session.map_or(0, |s| s.count),
            shown: session.and_then(|s| s.shown.clone()),
            candidate,
            path,
            transition: session.map(|s| s.transition.clone()).unwrap_or_default(),
            idle_ms: left.map_or(0, |d| d.as_millis().min(u128::from(u32::MAX)) as u32),
            shortcuts: self.rotations.browse.shortcuts(),
        })
    }
}

pub async fn told(daemon: &Weak<Daemon>, emitter: SignalEmitter<'_>, told: Told) -> Result<String> {
    let sent = Playlists::browse_changed(&emitter, &told.output, &told.json).await;
    sent.unwrap_or_else(|e| tracing::debug!("cannot emit a browse signal: {e}"));
    if let Some(serial) = told.fresh {
        tokio::spawn(expire(daemon.clone(), emitter.into_owned(), told.output, serial));
    }
    Ok(told.json)
}

async fn expire(daemon: Weak<Daemon>, emitter: SignalEmitter<'static>, output: String, serial: u64) {
    loop {
        let Some(found) = daemon.upgrade().map(|d| d.expiry(&output, serial)) else { return };
        match found {
            Expiry::Gone => return,
            Expiry::Until(until) => tokio::time::sleep_until(until.into()).await,
            Expiry::Ended(told) => {
                let sent = Playlists::browse_changed(&emitter, &told.output, &told.json).await;
                sent.unwrap_or_else(|e| tracing::debug!("cannot emit a browse signal: {e}"));
                return;
            }
        }
    }
}
