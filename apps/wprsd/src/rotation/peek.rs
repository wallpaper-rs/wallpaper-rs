use super::newest;
use crate::compositor::lock;
use crate::daemon::Daemon;
use crate::error::{Result, failed, not_found};
use crate::timing::{Turn, rotate};
use crate::zone::Zone;
use chrono::Utc;

pub struct Peeked {
    pub playlist: String,
    pub name: String,
    pub item: Option<String>,
    pub position: u32,
    pub count: u32,
}

impl Daemon {
    pub fn peek(&self, output: &str, steps: i32) -> Result<Peeked> {
        let (assigned, on_battery) = self.fingerprint();
        let found = assigned.into_iter().find(|(o, ..)| o == output);
        let (output, uid, active) = found.ok_or_else(|| not_found(format!("{output} has no playlist")))?;
        let now = Utc::now();
        let lists = lock(&self.rotations.lists);
        let (stored, mut found) =
            self.read(&[(output.clone(), uid.clone(), active)], on_battery, now.timestamp());
        let read = found.remove(&uid).ok_or_else(|| failed("the playlist cannot be read"))?;
        let (info, resolved) = read.map_err(failed)?;
        let key = if info.rules.synced { format!("@{uid}") } else { output };
        let rows = newest(stored, &lock(&self.rotations.unsaved));
        let row = rows.into_iter().find(|r| r.output == key && r.playlist == uid);
        let mut row = row.ok_or_else(|| failed("the playlist has not started on this screen yet"))?;
        let turn = Turn {
            live: false,
            fresh: false,
            start: false,
            step: (steps != 0).then_some(steps),
            retime: false,
        };
        rotate(&mut row, lists.get(&key).map(Vec::as_slice), &resolved, &info.rules, turn, now, &Zone);
        let (position, count) = (row.state.position(), resolved.uids.len() as u32);
        Ok(Peeked { playlist: uid, name: info.name, item: row.current, position, count })
    }
}
