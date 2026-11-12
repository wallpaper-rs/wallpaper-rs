use super::Fig;
use super::solid::{P3, dot, point, sub};
use crate::rigged::Rigged;
use crate::rigged::math::Mat;
use crate::{Hull, MAX_BONE_COLLIDERS};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Shape {
    node: u16,
    hull: Hull,
    radius: f32,
    offset: P3,
    tip: P3,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub(crate) struct Shapes {
    list: Vec<Shape>,
    placed: [(P3, P3, f32, u16); MAX_BONE_COLLIDERS],
    count: usize,
}

pub(super) fn shapes(rig: &Rigged, m: &Fig) -> Shapes {
    let shape = |c: &crate::BoneCollider| {
        let node = rig.joint(&c.bone)?;
        let radius = c.radius.clamp(0.0, 100.0);
        Some(Shape { node: node as u16, hull: c.shape, radius, offset: c.offset, tip: rig.tip(node) })
    };
    let list = m.rig.colliders.iter().filter_map(shape).take(MAX_BONE_COLLIDERS).collect();
    Shapes { list, ..Shapes::default() }
}

pub(crate) fn nearest(p: P3, a: P3, b: P3) -> P3 {
    let ab = sub(a, b);
    let f = (dot(sub(a, p), ab) / dot(ab, ab).max(1e-12)).clamp(0.0, 1.0);
    std::array::from_fn(|i| a[i] + ab[i] * f)
}

impl Shapes {
    pub(crate) fn place(&mut self, worlds: &[Mat], budget: usize) {
        self.count = self.list.len().min(budget);
        for (k, s) in self.list[..self.count].iter().enumerate() {
            let w = &worlds[s.node as usize];
            let a = point(w, s.offset);
            let b = match s.hull {
                Hull::Sphere => a,
                Hull::Capsule => point(w, std::array::from_fn(|i| s.tip[i] + s.offset[i])),
            };
            self.placed[k] = (a, b, s.radius, s.node);
        }
    }

    pub(crate) fn out(&self, p: P3, own: &[u16]) -> P3 {
        let mut p = p;
        for &(a, b, r, node) in &self.placed[..self.count] {
            if own.contains(&node) || r <= 0.0 {
                continue;
            }
            let c = nearest(p, a, b);
            let d = sub(c, p);
            let l = dot(d, d).sqrt();
            if l < r {
                let n = if l > 1e-9 { d.map(|v| v / l) } else { [0.0, 1.0, 0.0] };
                p = std::array::from_fn(|i| c[i] + n[i] * r);
            }
        }
        p
    }
}
