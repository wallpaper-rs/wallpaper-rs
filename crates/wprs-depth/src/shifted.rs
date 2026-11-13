use crate::guided::Size;
use crate::regions::{Regions, regions};
use std::borrow::Cow;
use wprs_core::edit::Depth;

pub(crate) fn found<'a>(levels: &'a [f32], size: Size, depth: &Depth) -> (Cow<'a, [f32]>, Regions) {
    let mut found = regions(levels, size, depth);
    if depth.moves.is_empty() {
        return (Cow::Borrowed(levels), found);
    }
    let mut by = vec![0.0f32; found.means.len()];
    let mut moved_already = vec![false; by.len()];
    for shift in &depth.moves {
        let l = crate::stale::label_at(&found.label, size, shift.at) as usize;
        if !std::mem::replace(&mut moved_already[l], true) {
            by[l] = shift.by;
        }
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
