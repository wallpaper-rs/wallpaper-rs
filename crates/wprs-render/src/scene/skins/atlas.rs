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

pub(crate) fn pictures(rig: &Rigged) -> Vec<RgbaImage> {
    rig.pictures.iter().filter_map(|b| decoded(b)).collect()
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
        let k = f64::from(edge - 4 * PAD * pictures.len() as u32) / f64::from(w.max(h));
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
        v[3] = x + v[3].clamp(0.0, 1.0) * w;
        v[4] = y + v[4].clamp(0.0, 1.0) * h;
    }
    out
}
