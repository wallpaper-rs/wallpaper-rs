use crate::guided::Size;
use crate::regions::{Regions, regions};
use std::borrow::Cow;
use wprs_core::edit::Depth;

pub(crate) fn found<'a>(levels: &'a [f32], size: Size, depth: &Depth) -> (Cow<'a, [f32]>, Regions) {
    let mut found = regions(levels, size, depth);
    if depth.moves.is_empty() {
        return (Cow::Borrowed(levels), found);
    }
    let (w, h) = size;
    let mut by = vec![0.0f32; found.means.len()];
    for shift in &depth.moves {
        let (x, y) = ((shift.at[0] * w as f32) as usize, (shift.at[1] * h as f32) as usize);
        by[found.label[y.min(h - 1) * w + x.min(w - 1)] as usize] += shift.by;
    }
    let mut sums = vec![0.0f64; by.len()];
    let moved: Vec<f32> = levels
        .iter()
        .zip(&found.label)
        .map(|(v, l)| {
            let now = (v + by[*l as usize]).clamp(0.0, 1.0);
            sums[*l as usize] += f64::from(now);
            now
        })
        .collect();
    found.means = sums.iter().zip(&found.sizes).map(|(s, n)| (*s / (*n).max(1) as f64) as f32).collect();
    (Cow::Owned(moved), found)
}
