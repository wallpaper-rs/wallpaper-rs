use crate::daemon::Daemon;
use crate::error::{Result, invalid};
use std::path::PathBuf;
use std::time::Duration;
use wprs_core::settings::Transition;

impl Daemon {
    pub fn transition_of(&self, word: &str) -> Result<Option<Duration>> {
        match word.trim() {
            "" => Ok(None),
            "none" => Ok(Some(Duration::ZERO)),
            "fade" => {
                let ms = self.state().config.transition.duration_ms.min(Transition::MAX_MS);
                Ok(Some(Duration::from_millis(u64::from(ms))))
            }
            _ => Err(invalid("the transition is fade, none or empty for the usual one")),
        }
    }

    pub fn media_of(
        &self,
        output: &str,
        playlist: Option<String>,
        image: Option<PathBuf>,
    ) -> (Option<PathBuf>, Option<Duration>) {
        let (image, transition) = self.playlist_media(output, playlist).unwrap_or((image, None));
        (image, self.rotations.browse.held(output).or(transition))
    }
}
