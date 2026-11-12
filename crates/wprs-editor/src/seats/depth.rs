use super::Seat;
use serde::Serialize;

const RING: usize = 32;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Depth {
    pub pivot: [f64; 2],
    pub ring: Vec<[f64; 2]>,
    pub width: [f64; 2],
    pub height: [f64; 2],
    pub foot: [f64; 2],
    pub lift: [f64; 2],
    pub reach: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Handles {
    pub corners: [[f64; 2]; 4],
    pub edges: [[f64; 2]; 4],
    pub pivot: [f64; 2],
}

fn f(p: [f64; 2]) -> [f32; 2] {
    p.map(|v| v as f32)
}

fn d(p: [f32; 2]) -> [f64; 2] {
    p.map(f64::from)
}

fn gap(a: [f64; 2], b: [f64; 2]) -> f64 {
    (a[0] - b[0]).hypot(a[1] - b[1])
}

pub fn handles(seat: &Seat) -> Handles {
    let edges = [[0.5, 0.0], [1.0, 0.5], [0.5, 1.0], [0.0, 0.5]].map(|u| seat.at(u));
    let pivot = seat.lean.map_or(seat.pivot(), |l| d(l.apply(f(seat.pivot()))));
    Handles { corners: seat.corners(), edges, pivot }
}

pub fn reach(seat: &Seat) -> f64 {
    let o = seat.outline();
    (0.35 * gap(o[0], o[1]).min(gap(o[0], o[3]))).clamp(40.0, 400.0)
}

fn along(seat: &Seat, by: [f64; 2]) -> [f64; 2] {
    let camera = seat.plane.camera;
    let back = camera.inverse().apply(f(seat.pivot()));
    d(camera.apply([back[0] + by[0] as f32, back[1] + by[1] as f32]))
}

fn foot(seat: &Seat) -> [f64; 2] {
    let sized = seat.size.iter().all(|s| s.abs() > 1e-6);
    if sized { seat.flat([0.5, 1.0]) } else { along(seat, [0.0, reach(seat)]) }
}

fn shown(seat: &Seat, tilt: [f32; 2], z: f32, p: [f64; 2]) -> [f64; 2] {
    d(seat.plane.lean(f(seat.pivot()), tilt, z).apply(f(p)))
}

pub fn depth(seat: &Seat) -> Depth {
    let l = reach(seat);
    let (tilt, z) = (seat.tilt, seat.z);
    let ring = (0..RING)
        .map(|k| {
            let a = k as f64 / RING as f64 * std::f64::consts::TAU;
            shown(seat, tilt, z, along(seat, [l * a.cos(), l * a.sin()]))
        })
        .collect();
    let base = foot(seat);
    Depth {
        pivot: shown(seat, tilt, z, seat.pivot()),
        ring,
        width: shown(seat, tilt, z, along(seat, [l, 0.0])),
        height: shown(seat, tilt, z, along(seat, [0.0, -l])),
        foot: shown(seat, tilt, z, base),
        lift: shown(seat, tilt, z + l as f32, base),
        reach: l,
    }
}

fn golden(mut lo: f64, mut hi: f64, cost: impl Fn(f64) -> f64) -> f64 {
    let k = (5f64.sqrt() - 1.0) / 2.0;
    for _ in 0..80 {
        let (a, b) = (hi - k * (hi - lo), lo + k * (hi - lo));
        if cost(a) < cost(b) { hi = b } else { lo = a }
    }
    (lo + hi) / 2.0
}

pub fn tilted(seat: &Seat, axis: usize, to: [f64; 2]) -> [f64; 2] {
    let l = reach(seat);
    let end = if axis == 0 { along(seat, [0.0, -l]) } else { along(seat, [l, 0.0]) };
    let now = f64::from(seat.tilt[axis]);
    let cost = |a: f64| {
        let mut tilt = seat.tilt;
        tilt[axis] = a as f32;
        let miss = gap(shown(seat, tilt, seat.z, end), to);
        miss * miss + (l * 5e-4 * (a - now)).powi(2)
    };
    let best = (-89..=89).map(f64::from).min_by(|a, b| cost(*a).total_cmp(&cost(*b))).unwrap_or(now);
    let a = golden((best - 1.0).max(-89.0), (best + 1.0).min(89.0), cost);
    let mut out = seat.tilt.map(f64::from);
    out[axis] = (a * 10.0).round() / 10.0;
    out
}

pub fn lifted(seat: &Seat, to: [f64; 2]) -> f64 {
    let (l, far) = (reach(seat), f64::from(seat.plane.view.1));
    let base = foot(seat);
    let z = |s: f64| far * (1.0 - 1.0 / s) - l;
    let cost = |s: f64| gap(shown(seat, seat.tilt, (z(s) + l) as f32, base), to);
    let top = far / (far - (0.95 * far).min(1e5)).max(1e-3);
    let s = golden(far / (far + 1e5), top, cost);
    (z(s).clamp(-1e5, 1e5) * 10.0).round() / 10.0
}
