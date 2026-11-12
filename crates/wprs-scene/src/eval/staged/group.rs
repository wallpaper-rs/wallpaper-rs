use super::{MAX_PLANES, Stage3d};
use crate::eval::{Content, Draw, Frame};
use crate::figure::Space;
use crate::resolve::Context;
use crate::{Affine, Blend, Color, Kind, Layer, Quality};

fn plane(cx: &Context, layer: &Layer, eye: &crate::eye::Eye, height: f32) -> f32 {
    let z = layer.transform.z.as_ref().map_or(0.0, |z| z.get(cx));
    (eye.distance() - z * eye.unit(height)).max(eye.near)
}

fn loose(cx: &Context, d: &Draw) -> bool {
    let tilted = d.layer.transform.tilt.is_some() || d.layer.transform.z.is_some();
    matches!(d.content, Content::Group { clip: false })
        && !d.isolate
        && !d.hidden
        && !tilted
        && d.opacity >= 1.0
        && d.blend == Blend::Normal
        && !cx.mattes.contains(&d.layer.id)
}

fn drawn_as(r: &crate::Render) -> Option<&'static str> {
    match () {
        _ if r.refract.is_some() => Some("refract"),
        _ if r.trail.is_some() => Some("trail"),
        _ if r.rope.is_some() => Some("rope"),
        _ => None,
    }
}

pub fn particle_plane(scene: &crate::Scene, layer: &Layer) -> Option<&'static str> {
    let Kind::Particles { system, render, .. } = &layer.kind else { return Some("kind") };
    let Some(s) = scene.particles.get(system) else { return Some("kind") };
    let mut kids = s.children.iter().filter_map(|c| scene.particles.get(&c.system));
    match () {
        _ if s.perspective.is_none() => Some("perspective"),
        _ => drawn_as(render.as_deref().unwrap_or(&s.render)).or_else(|| {
            kids.any(|k| drawn_as(&k.render).is_some() || !k.children.is_empty()).then_some("children")
        }),
    }
}

fn apart(cx: &Context, layer: &Layer) -> bool {
    particle_plane(cx.scene, layer).is_none()
}

fn holds(outer: &Layer, inner: &Layer) -> bool {
    outer.children().iter().any(|c| std::ptr::eq(c, inner) || holds(c, inner))
}

fn deep(layer: &Layer) -> bool {
    matches!(layer.kind, Kind::Group { space: Some(Space::Depth), .. })
}

pub(super) fn merged(cx: &Context, frame: &Frame, at: usize) -> bool {
    let me = &frame.draws[at];
    if !loose(cx, me) {
        return false;
    }
    for j in (0..at).rev() {
        let d = &frame.draws[j];
        if d.end != j + 1 || !holds(d.layer, me.layer) {
            continue;
        }
        if deep(d.layer) {
            return true;
        }
        if !loose(cx, d) {
            return false;
        }
    }
    false
}

pub(super) fn gather<'a>(
    cx: &Context<'a>,
    layer: &'a Layer,
    (at, placed): (usize, usize),
    frame: &mut Frame<'a>,
    canvas: [f32; 2],
) {
    let models = placed..frame.placed.len();
    if frame.placed[models.clone()].iter().any(|p| p.stage != usize::MAX) {
        return;
    }
    let stage = frame.stages.len();
    let (eye, cross) = super::camera(cx, canvas[0] / canvas[1].max(1e-3), frame.quality);
    let end = frame.draws[at].end;
    let mut blocks: Vec<(std::ops::Range<usize>, Option<f32>)> = Vec::new();
    let (mut i, mut seen) = (at + 1, false);
    while i < end {
        let d = &frame.draws[i];
        if loose(cx, d) && !d.layer.depth_test {
            blocks.push((i..i + 1, None));
            i += 1;
            continue;
        }
        let deep = d.layer.depth_test && !matches!(d.content, Content::Model { .. });
        let (far, next, one) = (deep.then(|| plane(cx, d.layer, &eye, canvas[1])), d.end, apart(cx, d.layer));
        match (far, &mut frame.draws[i].content) {
            (Some(far), Content::Particles { seen: s, .. }) if one && frame.quality != Quality::Q0 => {
                (*s, seen) = (Some((stage, far)), true);
                blocks.push((i..next, None));
            }
            _ => blocks.push((i..next, far)),
        }
        i = next;
    }
    let mut planes: Vec<f32> = blocks.iter().filter_map(|b| b.1).collect();
    planes.sort_by(|a, b| b.total_cmp(a));
    planes.dedup_by(|a, b| (*a - *b).abs() < 1e-5);
    planes.truncate(MAX_PLANES);
    let snap = |d: f32| planes.iter().position(|p| d >= *p - 1e-5).unwrap_or(planes.len().saturating_sub(1));
    let group = frame.draws[at].clone();
    let world = group.world.then(Affine::translate([canvas[0] / 2.0, canvas[1] / 2.0]));
    let dynamic = frame.draws[at + 1..end].iter().any(|d| d.dynamic) || cx.wake.get() == crate::Wake::Frame;
    let none = frame.effects.len()..frame.effects.len();
    let slice = |k: usize| Draw {
        end: 0,
        world,
        origin: [0.5, 0.5],
        opacity: 1.0,
        blend: Blend::Normal,
        mask: None,
        mask_texture: None,
        mask_strength: 1.0,
        tint: Color([1.0; 4]),
        brightness: 1.0,
        effects: none.clone(),
        content: Content::Model { layer: &layer.id, stage, slice: k, size: canvas, key: 0 },
        dynamic,
        isolate: false,
        hidden: false,
        lit: None,
        ..group.clone()
    };
    let old: Vec<Draw<'a>> = frame.draws.drain(at + 1..end).collect();
    let mut moved = vec![usize::MAX; old.len()];
    let mut put = |range: &std::ops::Range<usize>, frame: &mut Frame<'a>| {
        let shift = frame.draws.len() as isize - range.start as isize;
        for k in range.clone() {
            let mut d = old[k - at - 1].clone();
            d.end = ((d.end as isize + shift) as usize).min(range.end.saturating_add_signed(shift));
            moved[k - at - 1] = frame.draws.len();
            frame.draws.push(d);
        }
    };
    for k in 0..=planes.len() {
        let mut s = slice(k);
        s.end = frame.draws.len() + 1;
        frame.draws.push(s);
        for b in blocks.iter().filter(|b| b.1.is_some_and(|d| snap(d) == k)) {
            put(&b.0, frame);
        }
    }
    for b in blocks.iter().filter(|b| b.1.is_none()) {
        put(&b.0, frame);
    }
    frame.draws[at].end = frame.draws.len();
    for p in &mut frame.placed[models.clone()] {
        p.draw = moved.get(p.draw - at - 1).copied().unwrap_or(p.draw);
        frame.draws[p.draw].hidden = true;
        p.stage = stage;
    }
    let mut cut = [0.0; MAX_PLANES];
    cut[..planes.len()].copy_from_slice(&planes);
    let size = canvas;
    let (origin, slices) = ([0.5, 0.5], planes.len() + 1);
    frame.stages.push(Stage3d {
        layer: &layer.id,
        size,
        eye,
        world,
        origin,
        models,
        planes: cut,
        slices,
        key: 0,
        still: 0,
        cross,
        wind: [0.0; 2],
        time: cx.t,
        seen,
    });
}
