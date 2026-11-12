use super::root::{frame, walked};
use super::{Puppet, figure};
use crate::rigged::math::{IDENTITY, Mat, apply, invert, mul};
use crate::{Layer, Loop};

pub const COURSE_STEPS: usize = 48;
pub const MAX_COURSE_SECONDS: f32 = 120.0;

impl Puppet {
    pub fn course(&self, layer: &Layer, laps: f32) -> Vec<[f32; 3]> {
        let (Some(m), Some(root)) = (figure(layer), self.root) else { return Vec::new() };
        if !m.root {
            return Vec::new();
        }
        let (up, down) = frame(&self.rig, root);
        let back = invert(&self.travel());
        let lap = |h: &super::Head| self.clips[h.clip].length / h.speed.max(0.05);
        let longest = self.heads.iter().filter(|h| h.mixed > 0.0).map(lap).fold(0.0f32, f32::max);
        let seconds = (laps * longest).min(MAX_COURSE_SECONDS);
        let mut out = Vec::with_capacity(COURSE_STEPS + 1);
        for k in 0..=COURSE_STEPS {
            let ahead = seconds * k as f32 / COURSE_STEPS as f32;
            let mut whole = super::root::Rigid { q: [0.0, 0.0, 0.0, 1.0], t: [0.0; 3] };
            for additive in [false, true] {
                for h in &self.heads {
                    let a = h.layer.map(|i| &m.rig.animations[i]);
                    let open = h.mask.is_empty() || h.mask[root as usize];
                    if a.is_some_and(|a| a.additive) != additive || h.mixed <= 0.0 || !open {
                        continue;
                    }
                    let (clip, map) = (&m.rig.clips[h.clip], &self.clips[h.clip]);
                    let len = map.length.max(1e-6);
                    let moving = h.held.unwrap_or(true) && !h.ended;
                    let t = h.t as f32 + if moving { ahead * h.speed } else { 0.0 };
                    let (t, laps) = match clip.repeat {
                        Loop::Repeat => (t.rem_euclid(len), h.laps + (t / len).floor().max(0.0) as u32),
                        Loop::Once => (t.min(len), h.laps),
                        Loop::Pingpong => (t, h.laps),
                    };
                    let Some((_, w)) = walked(&self.rig, map, (clip.from, clip.repeat), root, (t, laps))
                    else {
                        continue;
                    };
                    whole.mix(&w, h.mixed, additive);
                }
            }
            let travel: Mat = mul(&up, &mul(&whole.matrix(), &down));
            out.push(apply(&mul(&back, &travel), [0.0; 3]));
        }
        out
    }

    pub fn travel(&self) -> Mat {
        if self.posed.generation == 0 { IDENTITY } else { self.posed.travel }
    }
}

pub fn clip_course(rig: &crate::Rigged, clip: &crate::AnimClip, laps: f32) -> Vec<[f32; 3]> {
    let Some(root) = super::root::bone(rig) else { return Vec::new() };
    let map = super::heads::map(rig, clip);
    let (up, down) = frame(rig, root);
    let len = map.length.max(1e-6);
    let mut out = Vec::with_capacity(COURSE_STEPS + 1);
    for k in 0..=COURSE_STEPS {
        let t = len * laps * k as f32 / COURSE_STEPS as f32;
        let (t, laps) = match clip.repeat {
            Loop::Repeat => (t.rem_euclid(len), (t / len).floor() as u32),
            Loop::Once => (t.min(len), 0),
            Loop::Pingpong => (t, 0),
        };
        let Some((_, w)) = walked(rig, &map, (clip.from, clip.repeat), root, (t, laps)) else {
            return Vec::new();
        };
        out.push(apply(&mul(&up, &mul(&w.matrix(), &down)), [0.0; 3]));
    }
    out
}
