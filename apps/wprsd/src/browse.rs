use crate::compositor::lock;
use crate::daemon::Daemon;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};
use wprs_core::model::{BrowseEnd, BrowseSource, ShortcutsState};

mod list;
mod reply;
mod session;
pub(crate) mod shortcuts;
#[cfg(test)]
mod tests;
mod transition;

pub use reply::told;

pub const IDLE: Duration = Duration::from_secs(30);
const HELD: Duration = Duration::from_secs(5);

#[derive(Debug, Clone, PartialEq)]
pub struct Session {
    pub source: BrowseSource,
    pub playlist: Option<(String, String)>,
    pub items: Vec<String>,
    pub index: Option<usize>,
    pub steps: i32,
    pub candidate: Option<String>,
    pub shown: Option<String>,
    pub position: u32,
    pub count: u32,
    pub transition: String,
    pub until: Instant,
    pub serial: u64,
}

pub struct Told {
    pub output: String,
    pub json: String,
    pub fresh: Option<u64>,
}

#[derive(Default)]
pub struct Browsing {
    sessions: Mutex<HashMap<String, Session>>,
    held: Mutex<HashMap<String, (Duration, Instant)>>,
    shortcuts: Mutex<Option<ShortcutsState>>,
    made: AtomicU64,
}

pub enum Expiry {
    Gone,
    Until(Instant),
    Ended(Told),
}

impl Browsing {
    fn next_serial(&self) -> u64 {
        self.made.fetch_add(1, Ordering::Relaxed) + 1
    }

    pub fn shortcuts(&self) -> ShortcutsState {
        lock(&self.shortcuts).unwrap_or(ShortcutsState::Off)
    }

    pub fn set_shortcuts(&self, state: ShortcutsState) {
        *lock(&self.shortcuts) = Some(state);
    }

    pub fn hold(&self, outputs: &[String], transition: Duration) {
        let mut held = lock(&self.held);
        held.retain(|_, (_, at)| at.elapsed() < HELD);
        held.extend(outputs.iter().map(|o| (o.clone(), (transition, Instant::now()))));
    }

    pub fn held(&self, output: &str) -> Option<Duration> {
        let mut held = lock(&self.held);
        if held.is_empty() {
            return None;
        }
        held.remove(output).filter(|(_, at)| at.elapsed() < HELD).map(|(d, _)| d)
    }
}

pub fn source(word: &str) -> crate::error::Result<Option<BrowseSource>> {
    match word.trim() {
        "" => Ok(None),
        "playlist" => Ok(Some(BrowseSource::Playlist)),
        "library" => Ok(Some(BrowseSource::Library)),
        _ => Err(crate::error::invalid("the source is playlist, library or empty to choose by itself")),
    }
}

impl Daemon {
    pub fn browse_output(&self, output: &str) -> String {
        if !output.is_empty() {
            return output.to_owned();
        }
        let st = self.state();
        let mut real = st.outputs.keys().filter(|o| !crate::drafts::virtual_output(o));
        match (real.next(), real.next()) {
            (Some(only), None) => only.clone(),
            _ => String::new(),
        }
    }

    pub fn expiry(&self, output: &str, serial: u64) -> Expiry {
        let mut sessions = lock(&self.rotations.browse.sessions);
        let until = match sessions.get(output) {
            Some(s) if s.serial == serial => s.until,
            _ => return Expiry::Gone,
        };
        if until > Instant::now() {
            return Expiry::Until(until);
        }
        sessions.remove(output);
        drop(sessions);
        let json = self.browse_reply(output, None, Some(BrowseEnd::Expired)).unwrap_or_default();
        Expiry::Ended(Told { output: output.to_owned(), json, fresh: None })
    }
}
