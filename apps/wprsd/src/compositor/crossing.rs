use super::output::Shown;
use std::sync::Arc;
use wprs_core::Fit;
use wprs_render::{Crossing, Layer, Renderer};
use wprs_scene::{Quality, Ready, Way};

pub(super) struct Passing {
    steps: Vec<(Arc<Ready>, Quality)>,
    built: Option<(usize, Crossing)>,
    background: [u8; 3],
}

pub(super) fn choose(old: &Shown, new: &Shown, calm: bool) -> Option<Passing> {
    if calm {
        return None;
    }
    let steps: Vec<_> =
        [(old, Way::Out), (new, Way::In)].into_iter().filter_map(|(s, w)| own(s, w)).collect();
    (!steps.is_empty()).then_some(Passing { steps, built: None, background: new.background })
}

fn own(shown: &Shown, way: Way) -> Option<(Arc<Ready>, Quality)> {
    let scene = shown.scene.as_ref()?;
    match scene.spec.pack.scene.passages.way(way)? {
        Ok(ready) => Some((ready.clone(), scene.drawn_at())),
        Err(why) => {
            tracing::warn!("the transition of the scene was left out and the plain fade plays: {why}");
            None
        }
    }
}

pub(super) fn plain(background: [u8; 3]) -> Layer<'static> {
    Layer {
        image: None,
        fit: Fit::Stretch,
        background,
        opacity: 1.0,
        uv: None,
        relief: None,
        flip: [false; 2],
        grade: None,
    }
}

impl Passing {
    pub(super) fn duration(&self) -> f32 {
        self.steps.iter().map(|(r, _)| r.duration.as_secs_f32()).sum()
    }

    pub(super) fn phase(&self, progress: f32) -> (usize, f32) {
        let mut left = progress.clamp(0.0, 1.0) * self.duration();
        for (i, (ready, _)) in self.steps.iter().enumerate() {
            let length = ready.duration.as_secs_f32().max(1e-3);
            if left < length || i + 1 == self.steps.len() {
                return (i, (left / length).min(1.0));
            }
            left -= length;
        }
        (0, 0.0)
    }

    pub(super) fn frame<'a>(
        &'a mut self,
        renderer: &Renderer,
        (mut from, mut to): (Layer<'_>, Layer<'_>),
        progress: f32,
        size: (u32, u32),
    ) -> Result<Layer<'a>, String> {
        (from.opacity, to.opacity) = (1.0, 1.0);
        let (step, t) = self.phase(progress);
        let fresh = |b: &(usize, Crossing)| b.0 == step && b.1.size() == size;
        if !self.built.as_ref().is_some_and(fresh) {
            self.built = None;
            let (ready, quality) = self.steps[step].clone();
            self.built = Some((step, renderer.crossing(ready, size, quality)?));
        }
        let middle = plain(self.background);
        let (before, after) = match (self.steps.len(), step) {
            (1, _) => (from, to),
            (_, 0) => (from, middle),
            _ => (middle, to),
        };
        let Some((_, crossing)) = self.built.as_mut() else { return Err("it was not built".into()) };
        renderer.cross(crossing, &[before], &[after], t).map_err(|e| e.to_string())?;
        Ok(Layer { image: Some(crossing.image()), ..plain([0; 3]) })
    }
}

#[cfg(test)]
mod tests;
