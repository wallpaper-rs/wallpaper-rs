mod walk;

use super::Stage3d;
use crate::eval::{Content, Draw, Frame};
use crate::figure::Space;
use crate::resolve::Context;
use crate::{Affine, Blend, Color, Kind, Layer, Quality};
use walk::{block, cuts};

pub(super) fn loose(cx: &Context, d: &Draw) -> bool {
    let tilted = d.layer.transform.tilt.is_some() || d.layer.transform.z.is_some();
    matches!(d.content, Content::Group { clip: false })
        && !d.isolate
        && !d.hidden
        && !tilted
        && d.opacity >= 1.0
        && d.blend == Blend::Normal
        && !cx.mattes.contains(&d.layer.id)
}

pub fn particle_plane(scene: &crate::Scene, layer: &Layer) -> Option<&'static str> {
    let Kind::Particles { system, .. } = &layer.kind else { return Some("kind") };
    let Some(s) = scene.particles.get(system) else { return Some("kind") };
    s.perspective.is_none().then_some("perspective")
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
    let lens = (&eye, canvas[1]);
    let end = frame.draws[at].end;
    let (mut i, mut seen) = (at + 1, false);
    while i < end {
        let (next, far, _) = block(cx, &frame.draws[i], i, lens, stage);
        let one = far.is_some() && frame.quality != Quality::Q0 && apart(cx, frame.draws[i].layer);
        if let (true, Content::Particles { seen: s, .. }) = (one, &mut frame.draws[i].content) {
            (*s, seen) = (Some((stage, far.unwrap_or_default())), true);
        }
        i = next;
    }
    let (planes, count) = cuts(cx, frame, (at, end), lens, stage);
    let planes_in = &planes[..count];
    let snap = |d: f32| planes_in.iter().position(|p| d >= *p - 1e-5).unwrap_or(count.saturating_sub(1));
    let (group_layer, lean, group_world) =
        (frame.draws[at].layer, frame.draws[at].lean, frame.draws[at].world);
    let world = group_world.then(Affine::translate([canvas[0] / 2.0, canvas[1] / 2.0]));
    let dynamic = frame.draws[at + 1..end].iter().any(|d| d.dynamic) || cx.wake.get() == crate::Wake::Frame;
    let none = frame.effects.len()..frame.effects.len();
    let slice = |k: usize| Draw {
        layer: group_layer,
        lean,
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
    };
    let filler = slice(0);
    let put = |range: std::ops::Range<usize>, frame: &mut Frame<'a>| {
        let shift = frame.draws.len() as isize - range.start as isize;
        for k in range.clone() {
            let left = Draw { end: range.end, ..filler.clone() };
            let mut d = std::mem::replace(&mut frame.draws[k], left);
            d.end = ((d.end as isize + shift) as usize).min(range.end.saturating_add_signed(shift));
            let to = frame.draws.len();
            frame.placed[models.clone()].iter_mut().filter(|p| p.draw == k).for_each(|p| p.draw = to);
            frame.draws.push(d);
        }
    };
    for k in 0..=count {
        let mut s = slice(k);
        s.end = frame.draws.len() + 1;
        frame.draws.push(s);
        let mut i = at + 1;
        while i < end {
            let (next, far, moved) = block(cx, &frame.draws[i], i, lens, stage);
            if !moved && far.is_some_and(|d| snap(d) == k) {
                put(i..next, frame);
            }
            i = next;
        }
    }
    let mut i = at + 1;
    while i < end {
        let (next, _, moved) = block(cx, &frame.draws[i], i, lens, stage);
        if !moved {
            put(i..next, frame);
        }
        i = next;
    }
    let gone = end - at - 1;
    frame.draws.drain(at + 1..end);
    frame.draws[at + 1..].iter_mut().for_each(|d| d.end -= gone);
    frame.draws[at].end = frame.draws.len();
    for p in &mut frame.placed[models.clone()] {
        p.draw -= gone;
        frame.draws[p.draw].hidden = true;
        p.stage = stage;
    }
    let size = canvas;
    let (origin, slices) = ([0.5, 0.5], count + 1);
    frame.stages.push(Stage3d {
        layer: &layer.id,
        size,
        eye,
        world,
        origin,
        models,
        planes,
        slices,
        key: 0,
        still: 0,
        cross,
        wind: [0.0; 2],
        time: cx.t,
        seen,
    });
}
