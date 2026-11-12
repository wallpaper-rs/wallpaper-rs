use super::{Seat, canvas_of};
use crate::action::{Fit, Grip};
use crate::keys::value_at;
use anyhow::{Result, ensure};
use serde_json::Value;
use wprs_core::message::said;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Moved {
    pub position: Option<[f64; 2]>,
    pub rotation: Option<f64>,
    pub scale: Option<[f64; 2]>,
    pub skew: Option<[f64; 2]>,
    pub origin: Option<[f64; 2]>,
}

struct Own {
    rotation: f64,
    skew: [f64; 2],
    scale: [f64; 2],
}

fn own(layer: &Value, t: f64) -> Own {
    let tr = &layer["transform"];
    let pair = |k: &str, d: f64| super::boxes::pair(&value_at(&tr[k], t), [d; 2]);
    let rotation = value_at(&tr["rotation"], t).as_f64().unwrap_or(0.0);
    Own { rotation, skew: pair("skew", 0.0), scale: pair("scale", 1.0) }
}

fn turn(p: [f64; 2], degrees: f64) -> [f64; 2] {
    let (s, c) = degrees.to_radians().sin_cos();
    [c * p[0] - s * p[1], s * p[0] + c * p[1]]
}

fn sub(a: [f64; 2], b: [f64; 2]) -> [f64; 2] {
    [a[0] - b[0], a[1] - b[1]]
}

fn round(v: f64, step: f64) -> f64 {
    (v / step).round() * step
}

fn skewed(p: [f64; 2], k: [f64; 2]) -> [f64; 2] {
    let [tx, ty] = k.map(|a| a.to_radians().tan());
    [p[0] + tx * p[1], ty * p[0] + p[1]]
}

fn arm(seat: &Seat, u: [f64; 2]) -> [f64; 2] {
    [0, 1].map(|i| (u[i] - seat.origin[i]) * seat.size[i])
}

pub(crate) fn decompose(m: [f32; 4]) -> (f64, [f64; 2], [f64; 2]) {
    let [a, b, c, d] = m.map(f64::from);
    let angle = b.atan2(a);
    let (s, co) = angle.sin_cos();
    let sx = a.hypot(b);
    let (along, sy) = (c * co + d * s, -c * s + d * co);
    let kx = if sy.abs() > 1e-9 { (along / sy).atan().to_degrees() } else { 0.0 };
    (angle.to_degrees(), [kx, 0.0], [sx, sy])
}

pub(crate) fn tidy(m: Moved) -> Moved {
    let two = |p: [f64; 2], step: f64| p.map(|v| round(v, step));
    Moved {
        position: m.position.map(|p| two(p, 0.01)),
        rotation: m.rotation.map(|r| round(r, 0.01)),
        scale: m.scale.map(|s| two(s, 1e-4)),
        skew: m.skew.map(|k| two(k, 0.01).map(|a| a.clamp(-89.0, 89.0))),
        origin: m.origin.map(|o| two(o, 1e-4)),
    }
}

pub fn grip(
    seats: &[Seat],
    doc: &Value,
    me: &Seat,
    grip: &Grip,
    to: [f64; 2],
    snap: Option<f64>,
    t: f64,
) -> Result<Moved> {
    let layer = doc.pointer(&me.pointer).cloned().unwrap_or_default();
    let o = own(&layer, t);
    let position = me.position();
    let sized = me.size.iter().all(|s| s.abs() > 1e-6);
    let bare =
        said("editor.grip_no_size", "the layer %1 has no size to hold by a corner or an edge", &[&me.id]);
    let moved = match grip {
        Grip::Move => {
            let q = snap.map_or(to, |s| super::snap::snapped(seats, me, to, s, canvas_of(doc)));
            Moved { position: Some(me.local(q)), ..Moved::default() }
        }
        Grip::Nudge => {
            let p = me.pivot();
            Moved { position: Some(me.local([p[0] + to[0], p[1] + to[1]])), ..Moved::default() }
        }
        Grip::Turn => {
            let d = sub(me.local(to), position);
            Moved { rotation: Some(d[0].atan2(-d[1]).to_degrees()), ..Moved::default() }
        }
        Grip::Origin => {
            ensure!(sized, bare);
            let q = me.local(to);
            let back = me.own().inverse().apply(sub(q, position).map(|v| v as f32));
            let origin = [0, 1].map(|i| me.origin[i] + f64::from(back[i]) / me.size[i]);
            Moved { position: Some(q), origin: Some(origin), ..Moved::default() }
        }
        Grip::Scale { corner, even } => {
            ensure!(sized, bare);
            let p = arm(me, *corner);
            let q = skewed_back(turn(sub(me.local(to), position), -o.rotation), o.skew);
            let mut scale = [0, 1].map(|i| if p[i].abs() > 1e-6 { q[i] / p[i] } else { o.scale[i] });
            if *even {
                let base = [p[0] * o.scale[0], p[1] * o.scale[1]];
                let length = base[0] * base[0] + base[1] * base[1];
                let k = if length > 1e-9 { (q[0] * base[0] + q[1] * base[1]) / length } else { 1.0 };
                scale = o.scale.map(|s| s * k);
            }
            Moved {
                scale: Some(scale.map(|s| if s.abs() < 1e-3 { 1e-3f64.copysign(s) } else { s })),
                ..Moved::default()
            }
        }
        Grip::Size { .. } => Moved::default(),
        Grip::Skew { edge } => {
            ensure!(sized, bare);
            let p = arm(me, *edge);
            let p = [p[0] * o.scale[0], p[1] * o.scale[1]];
            let q = turn(sub(me.local(to), position), -o.rotation);
            let upright = (edge[0] - 0.5).abs() >= (edge[1] - 0.5).abs();
            let mut skew = o.skew;
            match upright {
                true if p[0].abs() > 1e-6 => skew[1] = ((q[1] - p[1]) / p[0]).atan().to_degrees(),
                false if p[1].abs() > 1e-6 => skew[0] = ((q[0] - p[0]) / p[1]).atan().to_degrees(),
                _ => {}
            }
            Moved { skew: Some(skew), ..Moved::default() }
        }
    };
    Ok(tidy(moved))
}

fn skewed_back(q: [f64; 2], k: [f64; 2]) -> [f64; 2] {
    let [tx, ty] = k.map(|a| a.to_radians().tan());
    let det = 1.0 - tx * ty;
    if det.abs() < 1e-9 {
        return q;
    }
    [(q[0] - tx * q[1]) / det, (q[1] - ty * q[0]) / det]
}

pub fn fit(doc: &Value, me: &Seat, fit: Fit, t: f64) -> Result<Moved> {
    let layer = doc.pointer(&me.pointer).cloned().unwrap_or_default();
    let o = own(&layer, t);
    ensure!(
        me.size.iter().all(|s| s.abs() > 1e-6),
        said("editor.grip_no_size", "the layer %1 has no size to hold by a corner or an edge", &[&me.id])
    );
    let canvas = canvas_of(doc);
    let center = me.local([canvas[0] / 2.0, canvas[1] / 2.0]);
    let seen: Vec<[f64; 2]> = [[0.0, 0.0], [canvas[0], 0.0], [canvas[0], canvas[1]], [0.0, canvas[1]]]
        .map(|c| skewed_back(turn(sub(me.local(c), center), -o.rotation), o.skew))
        .to_vec();
    let span = [0, 1].map(|i| {
        let values = seen.iter().map(|p| p[i]);
        values.clone().fold(f64::MIN, f64::max) - values.fold(f64::MAX, f64::min)
    });
    let need = [0, 1].map(|i| span[i] / me.size[i]);
    let sign = o.scale.map(|s| if s < 0.0 { -1.0 } else { 1.0 });
    let k = match fit {
        Fit::Cover => [need[0].max(need[1]); 2],
        Fit::Contain => [need[0].min(need[1]); 2],
        Fit::Stretch => need,
    };
    let scale = [k[0] * sign[0], k[1] * sign[1]];
    let middle = arm(me, [0.5, 0.5]);
    let shift = turn(skewed([middle[0] * scale[0], middle[1] * scale[1]], o.skew), o.rotation);
    let position = sub(center, shift);
    Ok(tidy(Moved { position: Some(position), scale: Some(scale), ..Moved::default() }))
}
