use super::keys::Key;
use super::side::Side;
use crate::animate::Animate;

fn at(p: [f32; 4], s: f32) -> f32 {
    let r = 1.0 - s;
    r * r * r * p[0] + 3.0 * r * r * s * p[1] + 3.0 * r * s * s * p[2] + s * s * s * p[3]
}

fn solve(xs: [f32; 4], ys: [f32; 4], t: f32) -> f32 {
    let (mut lo, mut hi) = (0.0, 1.0);
    for _ in 0..24 {
        let mid = (lo + hi) / 2.0;
        (lo, hi) = if at(xs, mid) < t { (mid, hi) } else { (lo, mid) };
    }
    at(ys, (lo + hi) / 2.0)
}

pub(super) fn curve<T: Animate>(a: &Key<T>, b: &Key<T>, t: f32) -> Option<T> {
    let (out, back) = (a.handles.and_then(|h| h.out), b.handles.and_then(|h| h.before));
    if out.is_none() && back.is_none() {
        return None;
    }
    if out.is_some_and(|s| s.split()) || back.is_some_and(|s| s.split()) {
        return split(a, b, t, [out, back]);
    }
    let (([v0, ..], n), ([v1, ..], _)) = (a.v.comps(), b.v.comps());
    let lone = a.one.zip(b.one).filter(|_| n != 1);
    let (v0, v1) = match lone {
        Some(ends) => ends,
        None if n == 1 => (v0, v1),
        None => (0.0, 1.0),
    };
    let third = [(b.t - a.t) / 3.0, (v1 - v0) / 3.0];
    let pair = |s: Option<Side>| s.map_or(third, |s| s.each()[0]);
    let ([dt0, dv0], [dt1, dv1]) = (pair(out), pair(back));
    let v = solve([a.t, a.t + dt0, b.t - dt1, b.t], [v0, v0 + dv0, v1 - dv1, v1], t);
    Some(match lone {
        Some(_) => T::from_json(&serde_json::Value::from(v)).unwrap_or_else(|| a.v.clone()),
        None if n == 1 => a.v.with_comps([v, 0.0, 0.0, 0.0]),
        None => a.v.lerp(&b.v, v),
    })
}

fn split<T: Animate>(a: &Key<T>, b: &Key<T>, t: f32, [out, back]: [Option<Side>; 2]) -> Option<T> {
    let ((p0, n), (p1, m)) = (a.v.parts(), b.v.parts());
    if n == 0 || n != m {
        return None;
    }
    let units = n == 1 || a.one.zip(b.one).is_some();
    let mut v = p0;
    for i in 0..n {
        let way = p1[i] - p0[i];
        let third = [(b.t - a.t) / 3.0, way / 3.0];
        let pick = |s: Option<Side>| match s {
            None => third,
            Some(Side::All([dt, dv])) => [dt, if units { dv } else { dv * way }],
            Some(side) => side.each().get(i).copied().unwrap_or(third),
        };
        let ([dt0, dv0], [dt1, dv1]) = (pick(out), pick(back));
        v[i] = solve([a.t, a.t + dt0, b.t - dt1, b.t], [p0[i], p0[i] + dv0, p1[i] - dv1, p1[i]], t);
    }
    Some(a.v.with_parts(v))
}
