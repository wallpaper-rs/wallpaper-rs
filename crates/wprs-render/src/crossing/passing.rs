use crate::{Building, Built, Crossing, Layer, Renderer};
use std::sync::Arc;
use std::time::Instant;
use wprs_core::Fit;
use wprs_scene::{Quality, Ready};

pub struct Passing {
    steps: Vec<(Arc<Ready>, Quality)>,
    pipes: Vec<Pipe>,
    built: Option<(usize, Crossing)>,
    background: [u8; 3],
}

enum Pipe {
    Waiting(Building),
    Done(Built),
}

fn plain(background: [u8; 3]) -> Layer<'static> {
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
    pub fn new(steps: Vec<(Arc<Ready>, Quality)>, background: [u8; 3]) -> Option<Passing> {
        (!steps.is_empty()).then_some(Passing { steps, pipes: Vec::new(), built: None, background })
    }

    pub fn begin(&mut self, renderer: &Renderer, wake: &Arc<dyn Fn() + Send + Sync>) -> Result<(), String> {
        let build = |(ready, quality): &(Arc<Ready>, Quality)| {
            renderer.build_crossing(ready.clone(), *quality, wake.clone()).map(Pipe::Waiting)
        };
        self.pipes = self.steps.iter().map(build).collect::<Result<_, _>>()?;
        Ok(())
    }

    pub fn ready(&mut self, now: Instant) -> Result<bool, String> {
        let mut done = !self.pipes.is_empty();
        for pipe in &mut self.pipes {
            if let Pipe::Waiting(building) = pipe {
                match building.poll(now) {
                    Some(built) => *pipe = Pipe::Done(built?),
                    None => done = false,
                }
            }
        }
        Ok(done)
    }

    pub fn until(&self) -> Option<Instant> {
        self.pipes.iter().filter_map(|p| if let Pipe::Waiting(b) = p { Some(b.until()) } else { None }).min()
    }

    pub fn duration(&self) -> f32 {
        self.steps.iter().map(|(r, _)| r.duration.as_secs_f32()).sum()
    }

    pub fn phase(&self, progress: f32) -> (usize, f32) {
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

    pub fn frame<'a>(
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
            let Some(Pipe::Done(built)) = self.pipes.get(step) else {
                return Err("it was not built yet".into());
            };
            self.built = Some((step, renderer.crossing_of(built, size)?));
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
