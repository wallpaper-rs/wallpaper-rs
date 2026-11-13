use wprs_scene::{Affine, Quality, Rigged, VERTEX_FLOATS};

const MAX_REACH: f32 = 0.5;
const SLACK: f32 = 0.5;

pub(crate) fn extent(rig: &Rigged) -> [f32; 2] {
    let reach = |k: usize| {
        let each = rig.vertices.as_chunks::<VERTEX_FLOATS>().0.iter().map(|v| v[k]).filter(|v| v.is_finite());
        each.fold(0.0f32, |most, v| most.max(v.abs()))
    };
    [reach(0), reach(1)]
}

pub(crate) fn margins(size: [f32; 2], extent: [f32; 2]) -> [f32; 2] {
    let beyond = |k: usize| match size[k] > 0.0 && extent[k] - size[k] / 2.0 > SLACK {
        true => ((extent[k] - size[k] / 2.0) / size[k]).min(MAX_REACH),
        false => 0.0,
    };
    [beyond(0), beyond(1)]
}

pub(crate) fn expanded(size: [f32; 2], extent: [f32; 2]) -> [f32; 2] {
    let [mx, my] = margins(size, extent);
    [size[0] * (1.0 + 2.0 * mx), size[1] * (1.0 + 2.0 * my)]
}

fn shrink(quality: Quality) -> f32 {
    match quality {
        Quality::Q0 => 0.5,
        Quality::Q1 => 0.75,
        _ => 1.0,
    }
}

pub(crate) fn sized(size: [f32; 2], scale: f32, quality: Quality, edge: u32) -> (u32, u32) {
    let [w, h] = size.map(|v| ((v * scale * shrink(quality)).round() as u32).clamp(1, edge));
    (w, h)
}

pub(super) fn grown(world: Affine, size: [f32; 2], scale: f32, (pw, ph): (u32, u32)) -> f32 {
    let m = world.0;
    let reach = m[0].hypot(m[1]).max(m[2].hypot(m[3]));
    let room = (pw as f32 / (size[0] * scale)).max(ph as f32 / (size[1] * scale));
    let k = reach.min(room);
    if !k.is_finite() || k <= 1.0 { 1.0 } else { 2f32.powf((k.log2() * 2.0).ceil() / 2.0).min(room.max(1.0)) }
}
