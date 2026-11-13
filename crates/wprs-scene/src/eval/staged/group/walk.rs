use super::loose;
use crate::Layer;
use crate::eval::staged::MAX_PLANES;
use crate::eval::{Content, Draw, Frame};
use crate::resolve::Context;

fn plane(cx: &Context, layer: &Layer, eye: &crate::eye::Eye, height: f32) -> f32 {
    let z = layer.transform.z.as_ref().map_or(0.0, |z| z.get(cx));
    (eye.distance() - z * eye.unit(height)).max(eye.near)
}

pub(super) type Lens<'e> = (&'e crate::eye::Eye, f32);

pub(super) fn block(
    cx: &Context,
    d: &Draw,
    i: usize,
    (eye, height): Lens,
    stage: usize,
) -> (usize, Option<f32>, bool) {
    if matches!(d.content, Content::Model { stage: s, .. } if s == stage) {
        return (d.end, None, true);
    }
    if loose(cx, d) && !d.layer.depth_test {
        return (i + 1, None, false);
    }
    let deep = d.layer.depth_test && !matches!(d.content, Content::Model { .. });
    let seen = matches!(d.content, Content::Particles { seen: Some((s, _)), .. } if s == stage);
    (d.end, deep.then(|| plane(cx, d.layer, eye, height)).filter(|_| !seen), false)
}

pub(super) fn cuts(
    cx: &Context,
    frame: &Frame,
    (at, end): (usize, usize),
    lens: Lens,
    stage: usize,
) -> ([f32; MAX_PLANES], usize) {
    let mut planes = [0.0; MAX_PLANES];
    for n in 0..MAX_PLANES {
        let (mut i, mut best) = (at + 1, None::<f32>);
        while i < end {
            let (next, far, _) = block(cx, &frame.draws[i], i, lens, stage);
            if let Some(far) = far.filter(|f| n == 0 || planes[n - 1] - f >= 1e-5) {
                best = Some(best.map_or(far, |b| if far.total_cmp(&b).is_gt() { far } else { b }));
            }
            i = next;
        }
        let Some(best) = best else { return (planes, n) };
        planes[n] = best;
    }
    (planes, MAX_PLANES)
}
