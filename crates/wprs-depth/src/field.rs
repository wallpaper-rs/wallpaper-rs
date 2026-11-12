use crate::guided::{Size, boxed};

pub const FIELD: usize = 4;
pub const BEND: f32 = 0.04;
pub const SMOOTH: usize = 2;
pub const BLUR: usize = 6;

pub fn field_size((w, h): Size) -> Size {
    (w.div_ceil(FIELD).max(1), h.div_ceil(FIELD).max(1))
}

pub fn slope(shift: f32, width: usize) -> f32 {
    if shift > 0.0 { BEND / (shift * width as f32) } else { 1.0 }
}

fn blocks(levels: &[f32], own: &[bool], (w, h): Size) -> (Vec<f32>, Vec<f32>) {
    let (fw, fh) = field_size((w, h));
    let (mut sum, mut weight) = (vec![0.0f32; fw * fh], vec![0.0f32; fw * fh]);
    for y in 0..h {
        for x in 0..w {
            let k = y * w + x;
            if own[k] {
                let q = (y / FIELD) * fw + x / FIELD;
                (sum[q], weight[q]) = (sum[q] + levels[k], weight[q] + 1.0);
            }
        }
    }
    let full = (FIELD * FIELD) as f32;
    let d = sum.iter().zip(&weight).map(|(s, n)| if *n > 0.0 { s / n } else { 0.0 }).collect();
    (d, weight.iter().map(|n| n / full).collect())
}

fn envelope(seed: &[Option<f32>], (w, h): Size, g: f32, upper: bool) -> Vec<f32> {
    let far = if upper { f32::MIN } else { f32::MAX };
    let mut d: Vec<f32> = seed.iter().map(|s| s.unwrap_or(far)).collect();
    let diagonal = g * std::f32::consts::SQRT_2;
    let pick = |a: f32, b: f32| if upper { a.max(b) } else { a.min(b) };
    let cost = |v: f32, c: f32| if upper { v - c } else { v + c };
    let relax = |d: &mut Vec<f32>, x: usize, y: usize, steps: &[(isize, isize, f32)]| {
        for &(dx, dy, c) in steps {
            let (nx, ny) = (x as isize + dx, y as isize + dy);
            if nx >= 0 && ny >= 0 && (nx as usize) < w && (ny as usize) < h {
                let via = cost(d[ny as usize * w + nx as usize], c);
                d[y * w + x] = pick(d[y * w + x], via);
            }
        }
    };
    let ahead = [(-1, 0, g), (0, -1, g), (-1, -1, diagonal), (1, -1, diagonal)];
    let behind = [(1, 0, g), (0, 1, g), (1, 1, diagonal), (-1, 1, diagonal)];
    (0..h).for_each(|y| (0..w).for_each(|x| relax(&mut d, x, y, &ahead)));
    (0..h).rev().for_each(|y| (0..w).rev().for_each(|x| relax(&mut d, x, y, &behind)));
    d
}

pub fn field(levels: &[f32], own: &[bool], size: Size, slope: f32) -> Vec<f32> {
    let small = field_size(size);
    let (depth, weight) = blocks(levels, own, size);
    let spread = boxed(&depth.iter().zip(&weight).map(|(d, w)| d * w).collect::<Vec<_>>(), small, SMOOTH);
    let cover = boxed(&weight, small, SMOOTH);
    let enough = if weight.iter().any(|w| *w >= 0.5) { 0.5 } else { f32::MIN_POSITIVE };
    let seed: Vec<Option<f32>> = (0..weight.len())
        .map(|q| (weight[q] >= enough && cover[q] > 0.0).then(|| spread[q] / cover[q]))
        .collect();
    if seed.iter().all(Option::is_none) {
        return vec![0.0; weight.len()];
    }
    let g = slope * FIELD as f32;
    let (upper, lower) = (envelope(&seed, small, g, true), envelope(&seed, small, g, false));
    let mid: Vec<f32> = upper.iter().zip(&lower).map(|(u, l)| (u + l) / 2.0).collect();
    boxed(&boxed(&mid, small, BLUR), small, BLUR)
}

pub fn sample(field: &[f32], (fw, fh): Size, (w, h): Size, x: usize, y: usize) -> f32 {
    let fx = ((x as f32 + 0.5) / w as f32 * fw as f32 - 0.5).clamp(0.0, (fw - 1) as f32);
    let fy = ((y as f32 + 0.5) / h as f32 * fh as f32 - 0.5).clamp(0.0, (fh - 1) as f32);
    let (x0, y0) = (fx.floor() as usize, fy.floor() as usize);
    let (x1, y1) = ((x0 + 1).min(fw - 1), (y0 + 1).min(fh - 1));
    let (tx, ty) = (fx - x0 as f32, fy - y0 as f32);
    let at = |x: usize, y: usize| field[y * fw + x];
    let top = at(x0, y0) * (1.0 - tx) + at(x1, y0) * tx;
    let bottom = at(x0, y1) * (1.0 - tx) + at(x1, y1) * tx;
    top * (1.0 - ty) + bottom * ty
}
