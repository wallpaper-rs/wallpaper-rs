use crate::brush::stamp;
use crate::grow::{grow, merge_small};
use crate::guided::Size;
use wprs_core::edit::Depth;

pub const JUMP: f32 = 0.08;
pub const RADIUS: f32 = 3.0;
pub const LEAST: f32 = 0.001;
pub const UNSET: u32 = u32::MAX;

#[derive(Debug, Clone, PartialEq)]
pub struct Regions {
    pub label: Vec<u32>,
    pub means: Vec<f32>,
    pub sizes: Vec<usize>,
}

pub fn scale((w, h): Size) -> f32 {
    (w.max(h) as f32 / 1920.0).max(0.25)
}

fn spread(levels: &[f32], (w, h): Size, r: usize) -> Vec<bool> {
    let mut rows = vec![(0.0f32, 0.0f32); levels.len()];
    for y in 0..h {
        for x in 0..w {
            let line = &levels[y * w + x.saturating_sub(r)..y * w + (x + r + 1).min(w)];
            rows[y * w + x] = line.iter().fold((f32::MAX, f32::MIN), |(lo, hi), v| (lo.min(*v), hi.max(*v)));
        }
    }
    let mut edge = vec![false; levels.len()];
    for x in 0..w {
        for y in 0..h {
            let (mut lo, mut hi) = (f32::MAX, f32::MIN);
            for yy in y.saturating_sub(r)..(y + r + 1).min(h) {
                let (a, b) = rows[yy * w + x];
                (lo, hi) = (lo.min(a), hi.max(b));
            }
            edge[y * w + x] = hi - lo > JUMP;
        }
    }
    edge
}

fn flood(edge: &[bool], (w, h): Size) -> (Vec<u32>, usize) {
    let mut label = vec![UNSET; edge.len()];
    let (mut next, mut stack) = (0u32, Vec::new());
    for s in 0..edge.len() {
        if edge[s] || label[s] != UNSET {
            continue;
        }
        label[s] = next;
        stack.push(s);
        while let Some(k) = stack.pop() {
            let (x, y) = (k % w, k / w);
            let around = [
                (x > 0, k.wrapping_sub(1)),
                (x + 1 < w, k + 1),
                (y > 0, k.wrapping_sub(w)),
                (y + 1 < h, k + w),
            ];
            for (inside, q) in around {
                if inside && !edge[q] && label[q] == UNSET {
                    label[q] = next;
                    stack.push(q);
                }
            }
        }
        next += 1;
    }
    (label, next as usize)
}

fn joined(label: &mut [u32], (w, h): Size, depth: &Depth) {
    for join in &depth.joins {
        let at = |p: [f32; 2]| {
            let (x, y) = ((p[0] * w as f32) as usize, (p[1] * h as f32) as usize);
            label[y.min(h - 1) * w + x.min(w - 1)]
        };
        let (a, b) = (at(join.a), at(join.b));
        if a != b {
            label.iter_mut().filter(|l| **l == b).for_each(|l| *l = a);
        }
    }
}

fn compact(label: &mut [u32], levels: &[f32]) -> Regions {
    let top = label.iter().copied().max().map_or(0, |m| m as usize + 1);
    let mut map = vec![UNSET; top];
    let mut sums: Vec<(f64, usize)> = Vec::new();
    for (l, d) in label.iter_mut().zip(levels) {
        if map[*l as usize] == UNSET {
            map[*l as usize] = sums.len() as u32;
            sums.push((0.0, 0));
        }
        *l = map[*l as usize];
        let s = &mut sums[*l as usize];
        *s = (s.0 + f64::from(*d), s.1 + 1);
    }
    let means = sums.iter().map(|(s, n)| (*s / (*n).max(1) as f64) as f32).collect();
    Regions { label: label.to_vec(), means, sizes: sums.iter().map(|s| s.1).collect() }
}

pub fn regions(levels: &[f32], size: Size, depth: &Depth) -> Regions {
    let r = (RADIUS * scale(size)).round().max(1.0) as usize;
    let mut edge = spread(levels, size, r);
    for split in &depth.splits {
        stamp(size, split, |k, weight| edge[k] |= weight > 0.0);
    }
    let (mut label, count) = flood(&edge, size);
    if count == 0 {
        return compact(&mut vec![0; levels.len()], levels);
    }
    grow(&mut label, levels, size);
    let least = (levels.len() as f32 * LEAST) as usize;
    merge_small(&mut label, size, least.max(1));
    joined(&mut label, size, depth);
    compact(&mut label, levels)
}
