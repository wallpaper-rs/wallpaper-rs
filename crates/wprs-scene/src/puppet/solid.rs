use super::limb::{range, wrap};
use super::swing::{CATCH_UP, LONGEST_STEP, QUIET, QUIET_TIME, Sim};
use super::{Fig, Puppet};
use crate::rigged::Rigged;
use crate::rigged::math::{Mat, Quat, Trs, conjugate, decompose, invert, mul, normalized, qmul};
use crate::{Body as Mode, BonePhysics, Wake};

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

fn arc(from: P3, to: P3) -> Quat {
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
    fn pin(&mut self, worlds: &[Mat]) {
        self.pins[0] = head(&worlds[self.nodes[0] as usize]);
        for (i, &n) in self.nodes.iter().enumerate() {
            self.pins[i + 1] = point(&worlds[n as usize], self.tips[i]);
        }
        if std::mem::take(&mut self.fresh) {
            self.hold();
            (self.quiet, self.step) = (0.0, LONGEST_STEP);
        }
    }

    fn hold(&mut self) {
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

fn mix(a: P3, b: P3, f: f32) -> P3 {
    std::array::from_fn(|i| a[i] + (b[i] - a[i]) * f)
}

fn aim(home: P3, d: P3, limit: Option<[f32; 2]>) -> P3 {
    let flat = |v: P3| v[0].hypot(v[1]);
    let unit = |v: P3| if len(v) > 1e-9 { v.map(|x| x / len(v)) } else { [1.0, 0.0, 0.0] };
    let Some([lo, hi]) = limit else { return unit(if len(d) > 1e-9 { d } else { home }) };
    let (a0, e0) = (home[1].atan2(home[0]), home[2].atan2(flat(home)));
    let a = a0 + wrap(d[1].atan2(d[0]) - a0).clamp(lo, hi);
    let cone = lo.abs().max(hi.abs());
    let e = e0 + (d[2].atan2(flat(d)) - e0).clamp(-cone, cone);
    [e.cos() * a.cos(), e.cos() * a.sin(), e.sin()]
}

fn advance(b: &mut Solid, e: &BonePhysics, sim: &Sim, (f0, f1): (f32, f32), hit: &super::collide::Shapes) {
    let (h, scale) = (sim.h, sim.h / b.step.max(1e-6));
    let lerp = |b: &Solid, i: usize, f: f32| mix(b.before[i], b.pins[i], f);
    let decay = (-e.damping * 10.0 * h).exp();
    let k = match e.mode {
        Mode::Spring => (e.stiffness * 20.0).powi(2),
        Mode::Rigid | Mode::Rope => (e.stiffness * 4.0).powi(2),
    };
    let (pin, from) = (lerp(b, 0, f1), lerp(b, 0, f0));
    let carry = sub(from, pin).map(|v| v * (1.0 - e.inertia));
    let drop = -e.gravity * 98.0 * sim.unit;
    let pull = |at: P3, was: P3, target: P3, i: usize| -> P3 {
        let v = sub(was, at).map(|v| v * decay * scale);
        let gust = sim.gust[0] + sim.gust[1] * crate::camera::noise(sim.t * 0.9 + i as f32 * 0.37, 7);
        let wind = [sim.wind[0], sim.wind[1], 0.0];
        std::array::from_fn(|c| {
            let air = (wind[c] * gust - v[c] / h) * 4.0 * e.wind;
            let a = k * (target[c] - at[c]) + air + if c == 1 { drop } else { 0.0 };
            at[c] + v[c] + a * h * h
        })
    };
    let mut speed = 0.0f32;
    match e.translation {
        None => (b.at[0], b.was[0]) = (pin, pin),
        Some([lo, hi]) => {
            let next = pull(b.at[0], b.was[0], pin, 0);
            let (lo, hi) = (lo * sim.unit, hi * sim.unit);
            let d = sub(pin, next);
            let (dx, dy, dz) = (d[0].clamp(lo, hi), (-d[1]).clamp(lo, hi), d[2].clamp(lo, hi));
            b.was[0] = b.at[0];
            b.at[0] = [pin[0] + dx, pin[1] - dy, pin[2] + dz];
            speed = len(sub(b.was[0], b.at[0]));
        }
    }
    for i in 1..b.at.len() {
        for p in [&mut b.at[i], &mut b.was[i]] {
            *p = std::array::from_fn(|c| p[c] + carry[c]);
        }
        let next = pull(b.at[i], b.was[i], lerp(b, i, f1), i);
        b.was[i] = b.at[i];
        b.at[i] = next;
    }
    let limit = range(e.rotation);
    for i in 1..b.at.len() {
        let home = sub(lerp(b, i - 1, f1), lerp(b, i, f1));
        let dir = aim(home, sub(b.at[i - 1], b.at[i]), limit);
        let p = std::array::from_fn(|c| b.at[i - 1][c] + len(home) * dir[c]);
        b.at[i] = hit.out(p, &b.nodes);
        speed = speed.max(len(sub(b.was[i], b.at[i])));
    }
    b.quiet = if speed < QUIET * sim.unit * h { (b.quiet + h).min(QUIET_TIME) } else { 0.0 };
    b.step = h;
}

impl Puppet {
    pub(crate) fn sway(&mut self, cx: &crate::resolve::Context, m: &Fig, sim: &Sim) {
        let sim = &Sim { wind: sim.wind.map(|w| w * m.unit), unit: m.unit, ..*sim };
        let elapsed = if sim.frozen { 0.0 } else { self.lag.min(CATCH_UP) };
        let steps = if elapsed < 1e-3 { 0 } else { (elapsed / LONGEST_STEP - 1e-3).ceil() as u32 };
        let sim = &Sim { h: elapsed / steps.max(1) as f32, ..*sim };
        let at = |s: u32| s as f32 / steps.max(1) as f32;
        if steps > 0 || sim.frozen {
            self.lag = 0.0;
        }
        for k in 0..self.solids.len() {
            let e = &m.rig.physics[self.solids[k].entry];
            self.shapes.place(&self.worlds, sim.colliders);
            let body = &mut self.solids[k];
            body.pin(&self.worlds);
            if sim.frozen {
                body.hold();
                continue;
            }
            if body.before != body.pins {
                body.quiet = 0.0;
            }
            let windy = e.wind > 0.0 && sim.wind != [0.0; 2];
            if windy || body.quiet < QUIET_TIME {
                for s in 0..steps {
                    let t = sim.t - (steps - 1 - s) as f32 * sim.h;
                    advance(body, e, &Sim { t, ..*sim }, (at(s), at(s + 1)), &self.shapes);
                }
            }
            if steps > 0 {
                body.before.clone_from_slice(&body.pins);
            }
            if windy || body.quiet < QUIET_TIME {
                cx.note(Wake::Frame);
            }
            self.bend(k, e.translation.is_some());
            self.worlds(cx, m);
        }
    }

    fn bend(&mut self, k: usize, moves: bool) {
        let Puppet { solids, locals, worlds, rig, .. } = self;
        let body = &solids[k];
        for (i, &n) in body.nodes.iter().enumerate() {
            let n = n as usize;
            let parent = rig.joints[n].parent.map(|p| worlds[p as usize]);
            let world = |l: &Trs| parent.map_or(l.matrix(), |p| mul(&p, &l.matrix()));
            if i == 0 && moves {
                let d = sub(head(&world(&locals[n])), body.at[0]);
                let back = parent.map_or(crate::rigged::math::IDENTITY, |p| invert(&p));
                (0..3).for_each(|r| locals[n].t[r] += (0..3).map(|c| back[r * 4 + c] * d[c]).sum::<f32>());
            }
            let w = world(&locals[n]);
            let p = head(&w);
            let q = arc(sub(p, point(&w, body.tips[i])), sub(p, body.at[i + 1]));
            let up = parent.map_or([0.0, 0.0, 0.0, 1.0], |p| decompose(&p).r);
            locals[n].r = normalized(qmul(qmul(conjugate(up), qmul(q, up)), locals[n].r));
            worlds[n] = world(&locals[n]);
        }
    }
}
