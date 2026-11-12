use crate::depth::{self, Raise};
use crate::framing::placed;
use crate::gpu::Gpu;
use crate::power::Power;
use crate::store;
use crate::tilt::{self, Motion};
use std::path::Path;
use std::time::Instant;
use wprs_core::edit::Geometry;
use wprs_core::travel::{Reach, shift, travel};
use wprs_core::{Fit, UvRect};
use wprs_render::{GpuImage, Layer, Relief};

const BATTERY_STEPS: u32 = 8;

pub(crate) struct Raised {
    key: (u64, String, Raise, Option<Geometry>),
    depth: GpuImage,
    reach: Reach,
    motion: Motion,
}

#[derive(Default)]
pub(crate) struct Tilted(u64, bool, bool);

#[derive(Clone, Copy)]
pub(crate) struct Look {
    pub(crate) fit: Fit,
    pub(crate) background: [u8; 3],
    pub(crate) pan: f32,
    pub(crate) crop: Option<[f32; 4]>,
    pub(crate) flip: [bool; 2],
    pub(crate) grade: Option<wprs_core::Grade>,
}

impl Tilted {
    pub(crate) fn rate(&mut self, preview: bool, lock: bool) -> u32 {
        let generation = store::generation();
        if self.0 != generation {
            *self = Tilted(generation, tilt::wanted(preview, lock), tilt::scene(preview, lock));
        }
        let power = Power::now();
        tilt::rate(self.1 && !(self.2 && power.still), power, preview)
    }
}

pub(crate) fn raise(
    old: &mut Option<Raised>,
    gpu: &Gpu,
    shown: Option<(&str, &Path)>,
    geometry: Option<&Geometry>,
) -> bool {
    let key = shown
        .and_then(|(uid, path)| Some((gpu.epoch, uid.to_owned(), depth::raised(path)?, geometry.cloned())));
    if old.as_ref().map(|raised| &raised.key) == key.as_ref() {
        return false;
    }
    let motion =
        old.take().filter(|raised| key.as_ref().is_some_and(|k| k.1 == raised.key.1)).map(|r| r.motion);
    *old = key.and_then(|key| {
        let layers = key.2.deck.as_ref().and_then(|path| wprs_depth::load(path).ok().flatten())?;
        let layers = match &key.3 {
            Some(geometry) if !geometry.is_identity() => wprs_depth::turned(&layers, geometry),
            _ => layers,
        };
        let uploaded = gpu.renderer.upload_layers(&layers.images, &layers.fields, &layers.info.depths);
        let depth = uploaded.inspect_err(|e| log::warn!("{e}")).ok()?;
        Some(Raised { key, depth, reach: layers.info.reach, motion: motion.unwrap_or_default() })
    });
    true
}

impl Raised {
    pub(crate) fn step(
        &mut self,
        offset: f32,
        clock: (i64, u32),
        moving: bool,
        size: (u32, u32),
    ) -> (bool, bool) {
        if !moving {
            return (self.motion.rest(), false);
        }
        let parallax = &self.key.2.parallax;
        let goal = tilt::goal(offset, Instant::now());
        self.motion.step(goal, parallax.smoothing_ms, clock, tilt::pixels(parallax, size))
    }

    fn relief(&self, aspect: f32, uv: &UvRect, charging: bool) -> Relief<'_> {
        let p = &self.key.2.parallax;
        let steps = if charging { p.steps } else { p.steps.min(BATTERY_STEPS) };
        tilt::saw(uv.scale[0]);
        let wanted = tilt::wanted_shift(p.strength, uv.scale[0]);
        let way = travel(wanted, &self.reach, (p.focus, p.overscan), (aspect, uv));
        let shift = shift(self.motion.at(), way.shift, aspect);
        Relief { depth: &self.depth, shift, focus: p.focus, steps, overscan: way.zoom }
    }
}

pub(crate) fn layer<'a>(
    image: Option<&'a GpuImage>,
    opacity: f32,
    look: &Look,
    size: (u32, u32),
    raised: Option<(&'a Raised, bool)>,
) -> Layer<'a> {
    let uv = image.map(|image| placed(look.fit, look.crop, image.size(), size, look.pan));
    let aspect = image.map_or(1.0, |i| i.size().0 as f32 / i.size().1.max(1) as f32);
    let relief = raised.zip(uv).filter(|(_, uv)| !uv.repeat);
    let relief = relief.map(|((raised, charging), uv)| raised.relief(aspect, &uv, charging));
    let (fit, background) = (look.fit, look.background);
    Layer { image, fit, background, opacity, uv, relief, flip: look.flip, grade: look.grade }
}
