use crate::daemon::Daemon;
use crate::error::{Result, invalid};
use std::path::Path;

impl Daemon {
    pub(crate) fn heard(&self, shown: Option<&Path>) -> f64 {
        let uid = shown.and_then(|path| self.state().items.get(path).cloned());
        let item = uid.and_then(|uid| self.store.read(|lib| lib.item(&uid)).ok());
        match item {
            Some(i) if i.sound_muted == Some(true) => 0.0,
            Some(i) => i.sound_volume.map_or(1.0, |v| f64::from(v.min(100)) / 100.0),
            None => 1.0,
        }
    }

    pub fn set_sound(&self, uids: &[String], muted: &str, volume: &str) -> Result<()> {
        let (muted, volume) = (word::<bool>(muted)?, word::<u8>(volume)?);
        self.edit_items(uids, |lib, uids| lib.set_sound_many(uids, muted, volume))?;
        self.configure_all();
        Ok(())
    }
}

fn word<T: std::str::FromStr>(text: &str) -> Result<Option<Option<T>>> {
    match text.trim() {
        "" => Ok(None),
        "none" => Ok(Some(None)),
        other => {
            let bad = || invalid(format!("{other:?} is not a valid sound value"));
            other.parse().map(|v| Some(Some(v))).map_err(|_| bad())
        }
    }
}
