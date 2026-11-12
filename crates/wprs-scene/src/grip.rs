use crate::eye::{Eye, cross, unit3};

type V3 = [f32; 3];
type Px = [f32; 2];

fn add(a: V3, b: V3, k: f32) -> V3 {
    std::array::from_fn(|i| a[i] + b[i] * k)
}

fn sub(a: V3, b: V3) -> V3 {
    std::array::from_fn(|i| a[i] - b[i])
}

fn dot(a: V3, b: V3) -> f32 {
    (0..3).map(|i| a[i] * b[i]).sum()
}

fn turned(v: V3, axis: V3, degrees: f32) -> V3 {
    let k = unit3(axis);
    let (s, c) = degrees.to_radians().sin_cos();
    let kv = cross(k, v);
    std::array::from_fn(|i| v[i] * c + kv[i] * s + k[i] * dot(k, v) * (1.0 - c))
}

pub fn ray(eye: &Eye, [x, y]: Px, [w, h]: Px) -> V3 {
    let (r, u, f) = eye.basis();
    let t = (eye.fov.clamp(1.0, 179.0).to_radians() * 0.5).tan();
    let (nx, ny) = ((x / w.max(1.0) * 2.0 - 1.0) * t * eye.aspect, (1.0 - y / h.max(1.0) * 2.0) * t);
    unit3(std::array::from_fn(|i| f[i] + r[i] * nx + u[i] * ny))
}

fn meet(eye: &Eye, at: Px, size: Px, point: V3, normal: V3) -> Option<V3> {
    let d = ray(eye, at, size);
    let along = dot(d, normal);
    (along.abs() > 1e-4).then(|| add(eye.at, d, dot(sub(point, eye.at), normal) / along))
}

pub fn pixel(eye: &Eye, p: V3, height: f32) -> f32 {
    eye.unit(height) * eye.depth(p).max(eye.near) / eye.distance()
}

pub fn slide(eye: &Eye, size: Px, p: V3, from: Px, to: Px) -> V3 {
    let f = eye.forward();
    match (meet(eye, from, size, p, f), meet(eye, to, size, p, f)) {
        (Some(a), Some(b)) => add(p, sub(b, a), 1.0),
        _ => p,
    }
}

pub fn along(eye: &Eye, size: Px, p: V3, axis: V3, from: Px, to: Px) -> f32 {
    let a = unit3(axis);
    let on = |at: Px| {
        let d = ray(eye, at, size);
        let w = sub(p, eye.at);
        let (b, dd, e) = (dot(a, d), dot(a, w), dot(d, w));
        let den = 1.0 - b * b;
        (den > 1e-3).then(|| (b * e - dd) / den)
    };
    match (on(from), on(to)) {
        (Some(s0), Some(s1)) => s1 - s0,
        _ => (from[1] - to[1]) * pixel(eye, p, size[1]) * -dot(a, eye.forward()).signum(),
    }
}

pub fn deepen(eye: &Eye, size: Px, p: V3, from: Px, to: Px) -> V3 {
    let gap = sub(p, eye.at);
    let far = dot(gap, gap).sqrt().max(1e-4);
    let next = (far * ((from[1] - to[1]) / size[1].max(1.0) * 2.0).exp()).max(eye.near * 2.0);
    add(eye.at, gap, next / far)
}

pub fn turn(eye: &Eye, size: Px, c: V3, axis: V3, from: Px, to: Px) -> f32 {
    let a = unit3(axis);
    let edge = |at: Px| dot(ray(eye, at, size), a).abs() < 0.08;
    let hits = (meet(eye, from, size, c, a), meet(eye, to, size, c, a));
    if let (Some(h0), Some(h1), false, false) = (hits.0, hits.1, edge(from), edge(to)) {
        let (v0, v1) = (sub(h0, c), sub(h1, c));
        return dot(a, cross(v0, v1)).atan2(dot(v0, v1)).to_degrees();
    }
    let Some([cx, cy, _]) = eye.project(c, size) else { return 0.0 };
    let screen = |[x, y]: Px| (cy - y).atan2(x - cx);
    let mut d = (screen(to) - screen(from)).to_degrees();
    d = (d + 540.0).rem_euclid(360.0) - 180.0;
    if dot(a, eye.forward()) > 0.0 { -d } else { d }
}

pub fn orbit(at: V3, center: V3, up: V3, [yaw, pitch]: Px) -> V3 {
    let arm = sub(at, center);
    let side = unit3(cross(up, arm));
    let swung = turned(turned(arm, up, yaw), side, pitch);
    let lift = dot(unit3(swung), unit3(up));
    let kept = if lift.abs() > 0.995 { turned(arm, up, yaw) } else { swung };
    add(center, kept, 1.0)
}

pub fn dolly(at: V3, center: V3, size: Px, from: Px, to: Px) -> V3 {
    let arm = sub(at, center);
    let k = ((to[1] - from[1]) / size[1].max(1.0) * 2.0).exp();
    let long = dot(arm, arm).sqrt().max(1e-4);
    add(center, arm, (long * k).max(0.01) / long)
}

pub fn roll(up: V3, forward: V3, degrees: f32) -> V3 {
    unit3(turned(up, forward, degrees))
}

pub fn ring(c: V3, axis: V3, radius: f32, steps: usize) -> impl Iterator<Item = V3> {
    let a = unit3(axis);
    let seed = if a[1].abs() < 0.9 { [0.0, 1.0, 0.0] } else { [1.0, 0.0, 0.0] };
    let u = unit3(cross(a, seed));
    let v = cross(a, u);
    (0..=steps).map(move |k| {
        let t = k as f32 / steps.max(1) as f32 * std::f32::consts::TAU;
        std::array::from_fn(|i| c[i] + radius * (t.cos() * u[i] + t.sin() * v[i]))
    })
}
