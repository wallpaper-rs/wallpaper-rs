use super::Fig;
use super::swing::{LONGEST_STEP, QUIET_TIME};
use crate::rigged::Rigged;
use crate::rigged::math::{Mat, Quat, normalized};

pub(crate) type P3 = [f32; 3];

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Solid {
    pub entry: usize,
    pub nodes: Vec<u16>,
    pub tips: Vec<P3>,
    pub at: Vec<P3>,
    pub was: Vec<P3>,
    pub pins: Vec<P3>,
    pub before: Vec<P3>,
    pub fresh: bool,
    pub quiet: f32,
    pub step: f32,
}

pub(crate) fn sub(a: P3, b: P3) -> P3 {
    [b[0] - a[0], b[1] - a[1], b[2] - a[2]]
}

pub(crate) fn dot(a: P3, b: P3) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

pub(crate) fn len(a: P3) -> f32 {
    dot(a, a).sqrt()
}

pub(crate) fn head(w: &Mat) -> P3 {
    [w[3], w[7], w[11]]
}

pub(crate) fn point(w: &Mat, [x, y, z]: P3) -> P3 {
    std::array::from_fn(|r| w[r * 4] * x + w[r * 4 + 1] * y + w[r * 4 + 2] * z + w[r * 4 + 3])
}

pub(crate) fn arc(from: P3, to: P3) -> Quat {
    let (a, b) = (len(from), len(to));
    if a < 1e-9 || b < 1e-9 {
        return [0.0, 0.0, 0.0, 1.0];
    }
    let (f, t) = (from.map(|v| v / a), to.map(|v| v / b));
    let c = [f[1] * t[2] - f[2] * t[1], f[2] * t[0] - f[0] * t[2], f[0] * t[1] - f[1] * t[0]];
    let w = 1.0 + dot(f, t);
    if w < 1e-6 {
        let side = if f[0].abs() < 0.9 { [0.0, -f[2], f[1]] } else { [-f[1], f[0], 0.0] };
        return normalized([side[0], side[1], side[2], 0.0]);
    }
    normalized([c[0], c[1], c[2], w])
}

pub(super) fn solids(rig: &Rigged, m: &Fig, order: &[u16]) -> Vec<Solid> {
    super::swing::bodies(rig, m, order)
        .into_iter()
        .map(|b| {
            let points = vec![[0.0; 3]; b.nodes.len() + 1];
            let (at, was, pins, before) = (points.clone(), points.clone(), points.clone(), points);
            let (entry, nodes, tips) = (b.entry, b.nodes, b.tips);
            Solid { entry, nodes, tips, at, was, pins, before, fresh: true, quiet: 0.0, step: LONGEST_STEP }
        })
        .collect()
}

impl Solid {
    pub(crate) fn pin(&mut self, worlds: &[Mat]) {
        self.pins[0] = head(&worlds[self.nodes[0] as usize]);
        for (i, &n) in self.nodes.iter().enumerate() {
            self.pins[i + 1] = point(&worlds[n as usize], self.tips[i]);
        }
        if std::mem::take(&mut self.fresh) {
            self.hold();
            (self.quiet, self.step) = (0.0, LONGEST_STEP);
        }
    }

    pub(crate) fn hold(&mut self) {
        self.at.clone_from_slice(&self.pins);
        self.was.clone_from_slice(&self.pins);
        self.before.clone_from_slice(&self.pins);
        (self.quiet, self.step) = (QUIET_TIME, LONGEST_STEP);
    }

    pub(crate) fn still(&self) -> bool {
        self.quiet >= QUIET_TIME && self.before == self.pins
    }

    pub(crate) fn push(&mut self, node: u16, v: [f32; 2]) {
        let Some(i) = self.nodes.iter().position(|n| *n == node) else { return };
        let (p, h) = (&mut self.was[i + 1], self.step);
        (p[0], p[1]) = (p[0] - v[0] * h, p[1] - v[1] * h);
        self.quiet = 0.0;
    }
}
