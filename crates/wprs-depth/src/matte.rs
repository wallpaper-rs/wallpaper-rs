use crate::guided::{Size, boxed};
use crate::split::distance;
use image::RgbImage;

pub const CONTRAST: f32 = 24.0;
pub const SURE: f32 = 0.25;

pub struct Matte {
    pub alpha: Vec<f32>,
    pub colour: Vec<[u8; 3]>,
}

pub fn width(size: Size) -> usize {
    (4.0 * crate::regions::scale(size)).round().max(2.0) as usize
}

fn means(picture: &RgbImage, core: &[bool], size: Size, r: usize) -> ([Vec<f32>; 3], Vec<f32>) {
    let weight: Vec<f32> = core.iter().map(|c| f32::from(u8::from(*c))).collect();
    let cover = boxed(&weight, size, r);
    let plane = |c: usize| {
        let v: Vec<f32> =
            picture.pixels().zip(core).map(|(p, k)| if *k { f32::from(p[c]) } else { 0.0 }).collect();
        boxed(&v, size, r)
    };
    ([plane(0), plane(1), plane(2)], cover)
}

pub fn matte(picture: &RgbImage, own: &[bool], below: &[bool], size: Size) -> Matte {
    let e = width(size);
    let (inside, outside) = (distance(below, size), distance(own, size));
    let zone = |k: usize| (own[k] && inside[k] <= e as f32) || (below[k] && outside[k] <= e as f32);
    let front: Vec<bool> = (0..own.len()).map(|k| own[k] && inside[k] > e as f32).collect();
    let back: Vec<bool> = (0..own.len()).map(|k| below[k] && outside[k] > e as f32).collect();
    let (f, fw) = means(picture, &front, size, 3 * e);
    let (b, bw) = means(picture, &back, size, 3 * e);
    let raw = picture.as_raw();
    let mut alpha: Vec<f32> = own.iter().map(|o| f32::from(u8::from(*o))).collect();
    let mut colour: Vec<[u8; 3]> = raw.as_chunks::<3>().0.to_vec();
    for k in (0..own.len()).filter(|k| zone(*k)) {
        if fw[k] <= 0.0 || bw[k] <= 0.0 {
            alpha[k] = if own[k] { 1.0 } else { 0.0 };
            continue;
        }
        let fg = [0, 1, 2].map(|c| f[c][k] / fw[k]);
        let bg = [0, 1, 2].map(|c| b[c][k] / bw[k]);
        let c = [0, 1, 2].map(|i| f32::from(raw[k * 3 + i]));
        let span = [0, 1, 2].map(|i| fg[i] - bg[i]);
        let length: f32 = span.iter().map(|s| s * s).sum();
        if length < CONTRAST * CONTRAST {
            alpha[k] = if own[k] { if inside[k] <= 1.0 { 0.5 } else { 1.0 } } else { 0.0 };
            continue;
        }
        let a = ([0, 1, 2].map(|i| (c[i] - bg[i]) * span[i]).iter().sum::<f32>() / length).clamp(0.0, 1.0);
        alpha[k] = a;
        colour[k] = match a {
            a if a >= 0.98 => colour[k],
            a if a < SURE => fg.map(|v| v.round().clamp(0.0, 255.0) as u8),
            a => [0, 1, 2].map(|i| ((c[i] - (1.0 - a) * bg[i]) / a).round().clamp(0.0, 255.0) as u8),
        };
    }
    Matte { alpha, colour }
}
