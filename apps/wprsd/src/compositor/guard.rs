use std::time::Instant;
use wprs_render::{RenderError, Renderer, SceneView};
use wprs_scene::{Calming, FlashGuard};

pub(super) struct Guard {
    guard: FlashGuard,
    luma: Vec<f32>,
    since: Instant,
}

impl Guard {
    pub(super) fn new(now: Instant) -> Box<Guard> {
        Box::new(Guard { guard: FlashGuard::default(), luma: Vec::new(), since: now })
    }

    pub(super) fn watch(
        &mut self,
        r: &Renderer,
        view: &mut SceneView,
        now: Instant,
        scene: &str,
    ) -> Result<(), RenderError> {
        if !r.luma(view, Some(&mut self.luma))? {
            return Ok(());
        }
        let before = self.guard.calming();
        let after = self.guard.watch(now.duration_since(self.since).as_secs_f64(), &self.luma);
        match after {
            _ if after == before => {}
            Calming::Slowed(speed) => {
                tracing::warn!(
                    scene,
                    speed,
                    "the scene flashed more than three times a second and plays slower"
                )
            }
            Calming::Paused => {
                tracing::warn!(scene, "the scene kept flashing more than three times a second and stopped")
            }
            Calming::Steady => {}
        }
        Ok(())
    }

    pub(super) fn speed(&self) -> f64 {
        self.guard.speed()
    }

    pub(super) fn label(&self) -> &'static str {
        match self.guard.calming() {
            Calming::Steady => "",
            Calming::Slowed(_) => "slowed",
            Calming::Paused => "paused",
        }
    }
}

#[cfg(test)]
mod tests;
