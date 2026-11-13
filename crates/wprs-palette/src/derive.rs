use crate::oklab::{lab, rgb};
use crate::tone::{Palette, Rgb};
use std::f64::consts::PI;

pub const MAX_PIXELS: usize = 8192;
const BUCKETS: usize = 36;

struct Samples<'a> {
    data: &'a [u8],
    channels: usize,
    step: usize,
    count: usize,
}

impl Samples<'_> {
    fn each(&self, mut f: impl FnMut([f64; 3])) {
        for i in 0..self.count {
            let o = i * self.step * self.channels;
            f(lab([self.data[o], self.data[o + 1], self.data[o + 2]]));
        }
    }
}

fn percentile_half(sorted: &[f64]) -> f64 {
    let at = 0.5 * (sorted.len() - 1) as f64;
    let (lo, hi) = (at.floor() as usize, at.ceil() as usize);
    let t = at - lo as f64;
    let (a, b) = (sorted[lo], sorted[hi]);
    if t >= 0.5 { b - (b - a) * (1.0 - t) } else { a + (b - a) * t }
}

fn bucket(hue: f64) -> usize {
    (((hue + PI) / (2.0 * PI) * BUCKETS as f64).floor() as i64).rem_euclid(BUCKETS as i64) as usize
}

fn accent_hue(samples: &Samples, fallback: f64) -> f64 {
    let weight = |p: [f64; 3]| if p[0] > 0.30 { p[1].hypot(p[2]).powf(1.5) } else { 0.0 };
    let mut mass = [0.0f64; BUCKETS];
    let mut total = 0.0;
    samples.each(|p| {
        let w = weight(p);
        total += w;
        mass[bucket(p[2].atan2(p[1]))] += w;
    });
    if total < 1e-3 {
        return fallback;
    }
    let mut smooth = [0.0f64; BUCKETS];
    for i in 0..BUCKETS {
        smooth[i] = mass[i] + 0.5 * (mass[(i + BUCKETS - 1) % BUCKETS] + mass[(i + 1) % BUCKETS]);
    }
    let mut best = 0;
    for i in 1..BUCKETS {
        if smooth[i] > smooth[best] {
            best = i;
        }
    }
    let (mut sin, mut cos) = (0.0, 0.0);
    samples.each(|p| {
        let k = bucket(p[2].atan2(p[1]));
        let gap = ((k + BUCKETS - best) % BUCKETS).min((best + BUCKETS - k) % BUCKETS);
        if gap <= 1 {
            let (h, w) = (p[2].atan2(p[1]), weight(p));
            sin += h.sin() * w;
            cos += h.cos() * w;
        }
    });
    sin.atan2(cos)
}

pub fn derive(pixels: &[u8], channels: usize, width: usize, height: usize) -> Option<Palette> {
    let total = width.checked_mul(height)?;
    if !(channels == 3 || channels == 4) || total == 0 || pixels.len() < total * channels {
        return None;
    }
    let step = total.div_ceil(MAX_PIXELS);
    let samples = Samples { data: pixels, channels, step, count: total.div_ceil(step) };
    let mut lights = [0.0f64; MAX_PIXELS];
    let lights = &mut lights[..samples.count];
    let (mut at, mut sum_l, mut all_a, mut all_b) = (0, 0.0, 0.0, 0.0);
    samples.each(|p| {
        lights[at] = p[0];
        at += 1;
        sum_l += p[0];
        all_a += p[1];
        all_b += p[2];
    });
    let n = samples.count as f64;
    lights.sort_unstable_by(f64::total_cmp);
    let median = percentile_half(lights);
    let (mut a, mut b, mut dark) = (0.0, 0.0, 0.0);
    samples.each(|p| {
        if p[0] <= median {
            a += p[1];
            b += p[2];
            dark += 1.0;
        }
    });
    let (mut a, mut b) = (a / dark, b / dark);
    if a.hypot(b) < 0.006 {
        (a, b) = (all_a / n, all_b / n);
    }
    let hue = if a.hypot(b) > 1e-4 { b.atan2(a) } else { 258f64.to_radians() };
    let chroma = (a.hypot(b) * 1.3).clamp(0.014, 0.040);
    let accent = accent_hue(&samples, hue);
    let velo = (0.50 + 0.55 * sum_l / n).clamp(0.62, 0.96);
    Some(Palette {
        noche: Rgb(rgb(0.185, chroma, hue)),
        noche_alta: Rgb(rgb(0.235, chroma * 1.1, hue)),
        noche_linea: Rgb(rgb(0.345, chroma * 1.5, hue)),
        bruma: Rgb(rgb(0.81, 0.026, hue)),
        bruma_baja: Rgb(rgb(0.69, 0.034, hue)),
        luna: Rgb(rgb(0.82, 0.085, accent)),
        luna_honda: Rgb(rgb(0.71, 0.10, accent)),
        velo: ((velo * 100.0).round_ties_even() / 100.0) as f32,
    })
}
