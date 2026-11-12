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
            c.restore_content(&output).then_some(()).ok_or_else(crate::words::nothing_before)
        })?;
        tracing::info!(output, "earlier wallpaper brought back");
        self.changes.changed("playlists", Vec::new());
        Ok(())
    }
}
