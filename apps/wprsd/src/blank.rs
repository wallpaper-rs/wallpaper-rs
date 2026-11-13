use crate::apply::target_output;
use crate::daemon::Daemon;
use crate::error::Result;

impl Daemon {
    pub fn clear_wallpaper(&self, output: &str) -> Result<()> {
        let output = target_output(output)?;
        let mut stopped = Vec::new();
        self.change_output(&output, |c| {
            stopped.extend(c.resolve(&output).playlist);
            c.clear_content(&output);
            Ok(())
        })?;
        tracing::info!(output, "wallpaper removed");
        self.changes.changed("playlists", stopped);
        Ok(())
    }

    pub fn restore_wallpaper(&self, output: &str) -> Result<()> {
        let output = target_output(output)?;
        self.change_output(&output, |c| {
            c.restore_content(&output).then_some(()).ok_or_else(crate::words::nothing_before)?;
            let gone = c.restored_files(&output).iter().any(|file| !file.exists());
            (!gone).then_some(()).ok_or_else(crate::words::earlier_gone)
        })?;
        tracing::info!(output, "earlier wallpaper brought back");
        self.changes.changed("playlists", Vec::new());
        Ok(())
    }

    pub(crate) fn clear_removed(&self, files: &[std::path::PathBuf]) -> Result<()> {
        let playing = |files: &[std::path::PathBuf]| -> Vec<String> {
            let st = self.state();
            let shown = |p: &crate::rotation::Playing| p.path.as_ref().is_some_and(|f| files.contains(f));
            st.playing.iter().filter(|(_, p)| shown(p)).map(|(name, _)| name.clone()).collect()
        };
        let moved = playing(files);
        if !moved.is_empty() {
            self.show(crate::rotation::Act::Step(&moved, 1));
        }
        let known: Vec<String> = self.state().outputs.keys().cloned().collect();
        let emptied = |name: &String| self.state().playing.get(name).is_none_or(|p| p.path.is_none());
        let mut stuck = playing(files);
        stuck.extend(moved.iter().filter(|name| emptied(name)).cloned());
        let shown = self.state().config.shown_among(&known, files);
        if shown.is_empty() && stuck.is_empty() {
            return Ok(());
        }
        for name in &stuck {
            self.clear_wallpaper(name)?;
        }
        if !shown.is_empty() {
            self.change_output("", |c| {
                c.clear_shown(&known, files);
                Ok(())
            })?;
        }
        tracing::info!("the removed item was the wallpaper; the screens are cleared");
        Ok(())
    }
}
