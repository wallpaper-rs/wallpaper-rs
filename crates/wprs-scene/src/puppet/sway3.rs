use super::limb::{range, wrap};
use super::solid::{P3, Solid, arc, head, len, point, sub};
use super::swing::{CATCH_UP, LONGEST_STEP, QUIET, QUIET_TIME, Sim};
use super::{Fig, Puppet};
use crate::rigged::math::{Trs, conjugate, decompose, invert, mul, normalized, qmul};
use crate::{Body as Mode, BonePhysics, Wake};

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
