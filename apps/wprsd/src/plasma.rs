use crate::daemon::Daemon;
use crate::error::{Result, failed, invalid};
use std::sync::atomic::{AtomicBool, Ordering};

pub(crate) static PLASMA: AtomicBool = AtomicBool::new(false);

const OURS: &str = "io.github.wallpaper_rs";
const OWN: &str = "org.kde.image";
const MARKER: &str = "plasma-handed-back";

pub(crate) fn switch_script(from: &str, to: &str) -> String {
    format!(
        "var all = desktops(); var changed = 0; \
         for (var i = 0; i < all.length; i++) {{ \
         if (all[i].wallpaperPlugin == \"{from}\") {{ all[i].wallpaperPlugin = \"{to}\"; changed++; }} }} \
         changed"
    )
}

fn evaluate(script: &str) -> anyhow::Result<u32> {
    crate::outside::block_on(async {
        let connection = wprs_client::bus::session().await?;
        let reply = connection
            .call_method(
                Some("org.kde.plasmashell"),
                "/PlasmaShell",
                Some("org.kde.PlasmaShell"),
                "evaluateScript",
                &(script,),
            )
            .await?;
        let text: String = reply.body().deserialize()?;
        Ok(text.trim().parse().unwrap_or(0))
    })
}

impl Daemon {
    fn marker(&self) -> std::path::PathBuf {
        self.store.data.join(MARKER)
    }

    pub fn hand_back_to_plasma(&self) -> Result<u32> {
        if !PLASMA.load(Ordering::Relaxed) {
            return Err(invalid(crate::words::not_plasma()));
        }
        self.clear_wallpaper("")?;
        let changed = evaluate(&switch_script(OURS, OWN)).map_err(|e| {
            tracing::debug!("{e:#}");
            failed(crate::words::plasma_unreachable())
        })?;
        if changed > 0 {
            std::fs::write(self.marker(), b"").ok();
        }
        tracing::info!(screens = changed, "the screens went back to the Plasma wallpaper");
        Ok(changed)
    }

    pub(crate) fn take_back_from_plasma(&self) {
        let shown = {
            let st = self.state();
            st.outputs.keys().any(|name| !st.config.is_blank(name))
        };
        if !shown || !self.marker().exists() {
            return;
        }
        match evaluate(&switch_script(OWN, OURS)) {
            Ok(changed) => tracing::info!(screens = changed, "the screens use the wallpaper-rs plugin again"),
            Err(e) => return tracing::debug!("cannot switch the Plasma wallpaper type: {e:#}"),
        }
        std::fs::remove_file(self.marker()).ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_script_switches_only_the_screens_on_the_named_plugin() {
        let script = switch_script(OURS, OWN);
        assert!(script.contains("wallpaperPlugin == \"io.github.wallpaper_rs\""));
        assert!(script.contains("wallpaperPlugin = \"org.kde.image\""));
        assert!(script.starts_with("var all = desktops();") && script.ends_with("changed"));
    }
}
