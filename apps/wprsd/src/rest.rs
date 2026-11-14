use crate::daemon::Daemon;
use crate::streams::SURFACE;
use wprs_wayland::{Control, Placed};

pub(crate) static CONTROL: std::sync::OnceLock<Control> = std::sync::OnceLock::new();

impl Daemon {
    pub(crate) fn wants_surface(&self, output: &str) -> bool {
        let st = self.state();
        !st.config.is_blank(output) || crate::calibration::calibrating(&st) || st.drafts.contains_key(output)
    }

    pub(crate) fn surface_rests(&self, output: &str) -> bool {
        let Some(control) = CONTROL.get() else { return false };
        let idle = {
            let st = self.state();
            match st.outputs.get(output).filter(|info| info.claims.contains_key(SURFACE)) {
                Some(info) => info.idle,
                None => return false,
            }
        };
        match (self.wants_surface(output), idle) {
            (false, true) => true,
            (false, false) => {
                self.compositor.remove(output);
                crate::inputs::window(output, None);
                self.compositor.settled();
                if let Some(info) = self.state().outputs.get_mut(output) {
                    info.idle = true;
                }
                control.refresh();
                tracing::info!(output, "no wallpaper: the desktop's own background is shown");
                true
            }
            (true, true) => {
                control.refresh();
                true
            }
            (true, false) => false,
        }
    }

    pub(crate) fn rest_output(&self, output: &str, placed: &Placed) {
        let mut st = self.state();
        let info = st.outputs.entry(output.to_owned()).or_default();
        let (width, mm) = (placed.size().0, placed.mm.0);
        let px_per_mm = if mm > 0 { f64::from(width) / f64::from(mm) } else { 0.0 };
        info.screen = Some(wprs_core::Screen {
            x: placed.position.0,
            y: placed.position.1,
            width: placed.logical.0,
            height: placed.logical.1,
            scale: f64::from(placed.scale) / 120.0,
            px_per_mm,
            identity: placed.identity.clone(),
        });
        (info.size, info.refresh, info.idle) =
            (placed.size(), placed.refresh.saturating_add(500) / 1000, true);
        info.claims.entry(SURFACE.into()).or_default();
        info.uploaded = None;
        (info.scene, info.video, info.accent, info.spread) = (None, None, None, false);
        (info.tones, info.shown) = (None, None);
        drop(st);
        self.apply_activity();
        self.changes.changed("state", vec![output.to_owned()]);
    }
}
