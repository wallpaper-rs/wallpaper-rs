use super::{ClipMap, Fig, Head, Puppet};
use crate::Loop;
use crate::rigged::math::{IDENTITY, Mat, Quat, Trs, conjugate, invert, mul, nlerp, normalized, qmul};
use crate::rigged::{Path, Rigged};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Rigid {
    pub q: Quat,
    pub t: [f32; 3],
}

const STILL: Rigid = Rigid { q: [0.0, 0.0, 0.0, 1.0], t: [0.0; 3] };

fn rotate(q: Quat, v: [f32; 3]) -> [f32; 3] {
    let p = qmul(qmul(q, [v[0], v[1], v[2], 0.0]), conjugate(q));
    [p[0], p[1], p[2]]
}

impl Rigid {
    fn then(&self, b: &Rigid) -> Rigid {
        let r = rotate(self.q, b.t);
        Rigid { q: normalized(qmul(self.q, b.q)), t: std::array::from_fn(|i| r[i] + self.t[i]) }
    }

    fn back(&self) -> Rigid {
        let q = conjugate(self.q);
        Rigid { q, t: rotate(q, self.t).map(|v| -v) }
    }

    fn times(&self, mut n: u32) -> Rigid {
        let (mut out, mut base) = (STILL, *self);
        while n > 0 {
            if n & 1 == 1 {
                out = out.then(&base);
            }
            base = base.then(&base);
            n >>= 1;
        }
        out
    }

    pub(super) fn mix(&mut self, b: &Rigid, w: f32, additive: bool) {
        match additive {
            true => {
                self.t = std::array::from_fn(|i| self.t[i] + b.t[i] * w);
                self.q = normalized(qmul(nlerp(STILL.q, b.q, w), self.q));
            }
            false => {
                self.t = std::array::from_fn(|i| self.t[i] + (b.t[i] - self.t[i]) * w);
                self.q = nlerp(self.q, b.q, w);
            }
        }
    }

    pub(super) fn matrix(&self) -> Mat {
        Trs { t: self.t, r: self.q, s: [1.0; 3] }.matrix()
    }
}

pub(crate) fn bone(rig: &Rigged) -> Option<u16> {
    let moves = |n: u16| {
        rig.motions.iter().flat_map(|m| &m.channels).any(|c| c.node == n && c.path == Path::Translation)
    };
    rig.order.iter().copied().find(|n| moves(*n))
}

fn pose(rig: &Rigged, map: &ClipMap, from: f32, root: u16, t: f32) -> Option<Rigid> {
    let motion = &rig.motions[map.motion?];
    let rest = rig.joints[root as usize].rest;
    let mut out = Rigid { q: rest.r, t: rest.t };
    let mut at = [0.0; 4];
    for c in motion.channels.iter().filter(|c| c.node == root) {
        c.sample(from + t, &mut at);
        match c.path {
            Path::Translation => out.t = [at[0], at[1], at[2]],
            Path::Rotation => out.q = normalized(at),
            _ => {}
        }
    }
    Some(out)
}

pub(crate) fn walked(
    rig: &Rigged,
    map: &ClipMap,
    (from, repeat): (f32, Loop),
    root: u16,
    (t, laps): (f32, u32),
) -> Option<(Rigid, Rigid)> {
    let len = map.length.max(1e-6);
    let start = pose(rig, map, from, root, 0.0)?;
    let now = pose(rig, map, from, root, crate::local(t.max(0.0), len, repeat))?;
    let part = now.then(&start.back());
    let laps = if repeat == Loop::Repeat { laps } else { 0 };
    if laps == 0 {
        return Some((part, part));
    }
    let lap = pose(rig, map, from, root, len)?.then(&start.back());
    Some((part, lap.times(laps).then(&part)))
}

pub(crate) fn weight(cx: &crate::resolve::Context, m: &Fig, h: &Head) -> f32 {
    let a = h.layer.map(|i| &m.rig.animations[i]);
    h.weight.unwrap_or_else(|| a.map_or(1.0, |a| a.weight.get(cx))).clamp(0.0, 1.0) * h.fade
}

pub(crate) fn frame(rig: &Rigged, root: u16) -> (Mat, Mat) {
    let (mut up, mut n) = (IDENTITY, rig.joints[root as usize].parent);
    while let Some(p) = n {
        up = mul(&rig.joints[p as usize].rest.matrix(), &up);
        n = rig.joints[p as usize].parent;
    }
    (up, invert(&up))
}

impl Puppet {
    pub(crate) fn root(&mut self, cx: &crate::resolve::Context, m: &Fig) {
        self.next.travel = IDENTITY;
        let Some(root) = self.root else { return };
        let (mut part, mut whole) = (STILL, STILL);
        for additive in [false, true] {
            for h in self.heads.iter_mut() {
                let a = h.layer.map(|i| &m.rig.animations[i]);
                if a.is_some_and(|a| a.additive) != additive {
                    continue;
                }
                h.mixed = weight(cx, m, h);
                let open = h.mask.is_empty() || h.mask[root as usize];
                let clip = &m.rig.clips[h.clip];
                let at = (clip.from, clip.repeat);
                let Some((p, w)) = walked(&self.rig, &self.clips[h.clip], at, root, (h.t as f32, h.laps))
                    .filter(|_| open && h.mixed > 0.0)
                else {
                    continue;
                };
                part.mix(&p, h.mixed, additive);
                whole.mix(&w, h.mixed, additive);
            }
        }
        let n = root as usize;
        let local = &mut self.locals[n];
        let kept = part.back().then(&Rigid { q: local.r, t: local.t });
        (local.r, local.t) = (kept.q, kept.t);
        let (up, down) = frame(&self.rig, root);
        self.next.travel = mul(&up, &mul(&whole.matrix(), &down));
    }
}
