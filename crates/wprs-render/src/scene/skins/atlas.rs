use image::{RgbaImage, imageops};
use wprs_scene::{Rigged, VERTEX_FLOATS};

const PAD: u32 = 2;

pub(crate) struct Atlas {
    pub(crate) rgba: RgbaImage,
    pub(crate) rects: Vec<[f32; 4]>,
}

fn decoded(bytes: &[u8]) -> Option<RgbaImage> {
    let reader = image::ImageReader::new(std::io::Cursor::new(bytes)).with_guessed_format().ok()?;
    Some(crate::decode_limited(reader, crate::picture_limits()).ok()?.into_rgba8())
}

const FLAT: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

pub(crate) fn repeated(rig: &Rigged) -> bool {
    rig.repeats.iter().any(|r| r[2] > 1.0 || r[3] > 1.0)
}

fn repeat_of(rig: &Rigged, sheet: usize) -> [f32; 4] {
    rig.repeats.get(sheet).copied().unwrap_or(FLAT)
}

fn tiled(picture: RgbaImage, [_, _, nu, nv]: [f32; 4]) -> RgbaImage {
    let (nu, nv) = (nu as u32, nv as u32);
    if nu <= 1 && nv <= 1 {
        return picture;
    }
    let (w, h) = picture.dimensions();
    RgbaImage::from_fn(w * nu.max(1), h * nv.max(1), |x, y| *picture.get_pixel(x % w, y % h))
}

pub(crate) fn pictures(rig: &Rigged, repeat: bool) -> Vec<RgbaImage> {
    let each = rig.pictures.iter().enumerate().filter_map(|(i, b)| {
        let picture = decoded(b)?;
        Some(if repeat { tiled(picture, repeat_of(rig, i)) } else { picture })
    });
    each.collect()
}

fn layout(sizes: &[(u32, u32)]) -> (u32, u32, Vec<(u32, u32)>) {
    let area: u64 = sizes.iter().map(|(w, h)| u64::from(w + 2 * PAD) * u64::from(h + 2 * PAD)).sum();
    let widest = sizes.iter().map(|(w, _)| w + 2 * PAD).max().unwrap_or(1);
    let row = widest.max((area as f64).sqrt().ceil() as u32);
    let mut order: Vec<usize> = (0..sizes.len()).collect();
    order.sort_by_key(|&i| std::cmp::Reverse(sizes[i].1));
    let (mut at, mut x, mut y, mut tall, mut wide) = (vec![(0, 0); sizes.len()], 0, 0, 0, 0);
    for i in order {
        let (w, h) = (sizes[i].0 + 2 * PAD, sizes[i].1 + 2 * PAD);
        if x + w > row {
            (x, y, tall) = (0, y + tall, 0);
        }
        at[i] = (x + PAD, y + PAD);
        (x, tall, wide) = (x + w, tall.max(h), wide.max(x + w));
    }
    (wide.max(1), (y + tall).max(1), at)
}

pub(crate) fn pack(mut pictures: Vec<RgbaImage>, edge: u32) -> Option<Atlas> {
    let sizes = |p: &[RgbaImage]| p.iter().map(RgbaImage::dimensions).collect::<Vec<_>>();
    let (w, h, _) = layout(&sizes(&pictures));
    if w.max(h) > edge {
        let k = f64::from(edge.saturating_sub(4 * PAD * pictures.len() as u32).max(1)) / f64::from(w.max(h));
        let shrink = |p: &RgbaImage| {
            let (pw, ph) = p.dimensions();
            let size = |v: u32| ((f64::from(v) * k).floor() as u32).max(1);
            imageops::resize(p, size(pw), size(ph), imageops::FilterType::Triangle)
        };
        pictures = pictures.iter().map(shrink).collect();
    }
    let (w, h, at) = layout(&sizes(&pictures));
    if w.max(h) > edge {
        return None;
    }
    let mut rgba = RgbaImage::new(w, h);
    let mut rects = Vec::with_capacity(pictures.len());
    for (p, (x, y)) in pictures.iter().zip(at) {
        let (pw, ph) = p.dimensions();
        for dy in 0..ph + 2 * PAD {
            for dx in 0..pw + 2 * PAD {
                let sx = dx.saturating_sub(PAD).min(pw - 1);
                let sy = dy.saturating_sub(PAD).min(ph - 1);
                rgba.put_pixel(x + dx - PAD, y + dy - PAD, *p.get_pixel(sx, sy));
            }
        }
        let (fw, fh) = (w as f32, h as f32);
        rects.push([x as f32 / fw, y as f32 / fh, pw as f32 / fw, ph as f32 / fh]);
    }
    Some(Atlas { rgba, rects })
}

pub(crate) fn placed(rig: &Rigged, rects: &[[f32; 4]]) -> Vec<f32> {
    let mut out = rig.vertices.clone();
    for v in out.as_chunks_mut::<VERTEX_FLOATS>().0 {
        let piece = v[VERTEX_FLOATS - 1] as usize;
        let sheet = rig.sheets.get(piece).copied().unwrap_or(0) as usize;
        let Some([x, y, w, h]) = rects.get(sheet).copied() else { continue };
        let [lu, lv, nu, nv] = repeat_of(rig, sheet);
        v[3] = x + ((v[3] - lu) / nu).clamp(0.0, 1.0) * w;
        v[4] = y + ((v[4] - lv) / nv).clamp(0.0, 1.0) * h;
    }
    out
}
