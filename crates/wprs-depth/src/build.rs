use crate::brush::lift;
use crate::field::field_size;
use crate::fill::{push_pull, simple};
use crate::guided::{Size, refine};
use crate::matte::{Matte, matte};
use crate::plan::{Plan, Region};
use crate::stack::{Band, FORMAT, Info, Layers, Settings, save};
use crate::windows::{Inpaint, lama};
use crate::{CACHE_BYTES, Map, Places, adjust, read};
use anyhow::{Context, Result, bail, ensure};
use image::{Luma, RgbImage, Rgba, RgbaImage};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use wprs_core::Fill;
use wprs_core::edit::Depth;
use wprs_core::travel::Reach;

pub const VERSION: u32 = 2;
pub const EDGE: u32 = 1920;
pub const MOST_EDGE: u32 = 3840;
pub const MARGIN: f32 = 2.0;

pub const TIERS: [u32; 4] = [1024, EDGE, 2560, MOST_EDGE];

pub fn edge_for(wanted: u32) -> u32 {
    TIERS.into_iter().find(|t| *t >= wanted).unwrap_or(MOST_EDGE)
}

pub(crate) fn fitted(image: RgbImage, edge: u32) -> RgbImage {
    match image.width().max(image.height()) > edge {
        true => image::DynamicImage::ImageRgb8(image).thumbnail(edge, edge).into_rgb8(),
        false => image,
    }
}

pub(crate) fn leveled(picture: &RgbImage, map: &Map, depth: &Depth) -> (Size, Vec<f32>) {
    let size = (picture.width() as usize, picture.height() as usize);
    let refined = refine(map, picture);
    let steps = refined.iter().map(|v| (v * 65535.0).round() as u16).collect();
    let map = Map::from_raw(size.0 as u32, size.1 as u32, steps).unwrap_or_default();
    let flat = Depth { dilate: 0, smooth: 0, ..depth.clone() };
    let mut levels: Vec<f32> =
        adjust(&map, &flat).pixels().map(|Luma([v])| f32::from(*v) / 65535.0).collect();
    lift(&mut levels, size, &depth.strokes);
    (size, levels)
}

struct Source<'a> {
    picture: &'a RgbImage,
    size: Size,
    fill: Fill,
}

fn one(
    cx: &Source,
    region: &Region,
    edge: Option<&Matte>,
    inpaint: Option<&mut Inpaint<'_>>,
    stop: &AtomicBool,
) -> Result<RgbaImage> {
    let Region { known, band, reached, limit, refill, far, .. } = region;
    let has = known.contains(&true);
    let mut filled = match (has, cx.fill) {
        (false, _) => cx.picture.clone(),
        (true, Fill::Lama) => push_pull(cx.picture, known),
        (true, _) => simple(cx.picture, known),
    };
    if has && cx.fill == Fill::Lama && band.contains(&true) {
        let Some(inpaint) = inpaint else { bail!("LaMa is not loaded") };
        lama(&mut filled, band, inpaint, stop)?;
    }
    let (w, h) = cx.size;
    let mut layer = RgbaImage::new(w as u32, h as u32);
    let raw = cx.picture.as_raw();
    for (k, p) in layer.pixels_mut().enumerate() {
        let feather = ((limit[k] + 2.0 - reached[k]) / 2.0).clamp(0.0, 1.0);
        let from_band = if band[k] { if refill[k] { 1.0 } else { feather } } else { 0.0 };
        let (mut rgb, mut alpha) = match (known[k], edge) {
            (true, Some(m)) => (m.colour[k], m.alpha[k]),
            (true, None) => ([raw[k * 3], raw[k * 3 + 1], raw[k * 3 + 2]], 1.0),
            (false, Some(m)) if m.alpha[k] > 0.0 && !band[k] => (m.colour[k], m.alpha[k]),
            _ => {
                let f = filled.as_raw();
                ([f[k * 3], f[k * 3 + 1], f[k * 3 + 2]], 0.0)
            }
        };
        if band[k] && from_band >= alpha {
            let f = filled.as_raw();
            (rgb, alpha) = ([f[k * 3], f[k * 3 + 1], f[k * 3 + 2]], from_band);
        }
        let alpha = if *far {
            1.0
        } else if has {
            alpha
        } else {
            0.0
        };
        if *far && !known[k] {
            let f = filled.as_raw();
            rgb = [f[k * 3], f[k * 3 + 1], f[k * 3 + 2]];
        }
        *p = Rgba([rgb[0], rgb[1], rgb[2], (alpha * 255.0).round() as u8]);
    }
    Ok(layer)
}

fn quantized(values: &[f32], (w, h): Size) -> Map {
    let steps = values.iter().map(|v| (v.clamp(0.0, 1.0) * 65535.0).round() as u16).collect();
    Map::from_raw(w as u32, h as u32, steps).unwrap_or_default()
}

pub fn layers(
    picture: &RgbImage,
    map: &Map,
    (depth, set): (&Depth, &Settings),
    mut inpaint: Option<&mut Inpaint<'_>>,
    progress: &mut dyn FnMut(u64),
    stop: &AtomicBool,
) -> Result<Layers> {
    ensure!(set.fill != Fill::Off, "the fill mode is off: there are no layers to build");
    let (size, levels) = leveled(picture, map, depth);
    progress(10);
    let plan = Plan::new(&levels, size, depth, (set.count, set.shift));
    let cx = Source { picture, size, fill: set.fill };
    let (mut images, mut bands, mut safe) = (Vec::new(), Vec::new(), set.shift);
    let mut below = vec![false; size.0 * size.1];
    for i in 0..plan.parts.count() {
        ensure!(!stop.load(Relaxed), "cancelled");
        let region = plan.region(i);
        let edge = (i > 0).then(|| matte(picture, &plan.masks[i], &below, size));
        images.push(one(&cx, &region, edge.as_ref(), inpaint.as_deref_mut(), stop)?);
        let pixels = region.band.iter().filter(|b| **b).count();
        bands.push(Band { layer: i, reach: region.reach, pixels });
        safe = safe.min(region.safe);
        below.iter_mut().zip(&plan.masks[i]).for_each(|(b, m)| *b |= *m);
        progress(10 + 90 * (i as u64 + 1) / plan.parts.count() as u64);
    }
    let small = field_size(size);
    let fields: Vec<Map> = plan.fields.iter().map(|f| quantized(f, small)).collect();
    let all = plan.fields.iter().flatten();
    let (low, high) = all.fold((f32::MAX, f32::MIN), |(lo, hi), v| (lo.min(*v), hi.max(*v)));
    let info = Info {
        format: FORMAT,
        size: [size.0 as u32, size.1 as u32],
        fill: set.fill.as_str().into(),
        cuts: plan.parts.cuts,
        depths: plan.parts.depths,
        bands,
        field: [small.0 as u32, small.1 as u32],
        regions: plan.regions,
        built: set.shift,
        reach: Reach { shift: safe.max(0.0), low: low.clamp(0.0, 1.0), high: high.clamp(0.0, 1.0) },
    };
    Ok(Layers { info, images, fields })
}

impl Places {
    pub fn build_layers(
        &self,
        (image, sha256): (&Path, &str),
        (depth, set): (&Depth, &Settings),
        inpaint: Option<&mut Inpaint<'_>>,
        progress: &mut dyn FnMut(u64),
        stop: &AtomicBool,
    ) -> Result<PathBuf> {
        let dest = self.layer_set(image, sha256, depth, set).context("the image has no depth map")?;
        let map = read(&self.find(image, sha256, depth).context("the image has no depth map")?)?;
        progress(2);
        let picture = fitted(crate::files::decode_edge(image, set.edge)?, set.edge);
        let made = layers(&picture, &map, (depth, set), inpaint, progress, stop);
        if let Err(e) = &made
            && !stop.load(Relaxed)
        {
            crate::mark_failed(&dest, e);
        }
        let made = made?;
        std::fs::remove_file(crate::failed_of(&dest)).ok();
        save(&dest, &made)?;
        self.trim(&dest, CACHE_BYTES)?;
        Ok(dest)
    }
}
