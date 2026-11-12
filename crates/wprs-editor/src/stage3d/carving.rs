use super::gizmo::{Seen, world};
use crate::action::{Carve, SculptStroke};
use std::collections::BTreeMap;
use wprs_scene::{Carved, MODEL_FLOATS, Mat, Mat4, Model};

type Moved = BTreeMap<u32, [f32; 3]>;

fn sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

fn unit(v: [f32; 3]) -> [f32; 3] {
    let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if l > 1e-12 { v.map(|x| x / l) } else { [0.0; 3] }
}

fn read(model: &Model, v: u32, k: usize) -> [f32; 3] {
    let at = v as usize * MODEL_FLOATS + k;
    [model.vertices[at], model.vertices[at + 1], model.vertices[at + 2]]
}

fn shaped(model: &Model, moved: &Moved, v: u32) -> [f32; 3] {
    let (p, d) = (read(model, v, 0), moved.get(&v).copied().unwrap_or_default());
    std::array::from_fn(|i| p[i] + d[i])
}

fn skinned(model: &Model, palette: &[Mat], v: u32, p: [f32; 3], w: f32) -> [f32; 3] {
    let s = &model.skin[v as usize * 8..v as usize * 8 + 8];
    let mut out = [0.0; 3];
    for k in 0..4 {
        let Some(m) = palette.get(s[k] as usize).filter(|_| s[4 + k] > 0.0) else { continue };
        (0..3).for_each(|r| {
            out[r] +=
                (m[r * 4] * p[0] + m[r * 4 + 1] * p[1] + m[r * 4 + 2] * p[2] + m[r * 4 + 3] * w) * s[4 + k]
        });
    }
    out
}

pub(super) fn reached(
    model: &Model,
    palette: &[Mat],
    (seen, matrix): (&Seen, &Mat4),
    (stroke, radius, soft): (&[[f32; 2]], f32, f32),
    moved: &Moved,
) -> Vec<(u32, f32)> {
    let eye = seen.eye.at;
    let mut out = Vec::new();
    for v in 0..model.vertex_count() as u32 {
        let at = world(matrix, skinned(model, palette, v, shaped(model, moved, v), 1.0));
        let Some(px) = seen.at(at) else { continue };
        let f = crate::brush::falloff(crate::brush::nearest(stroke, px, radius), soft);
        if f <= 0.0 {
            continue;
        }
        let n = skinned(model, palette, v, read(model, v, 3), 0.0);
        let n = sub(world(matrix, n), world(matrix, [0.0; 3]));
        let toward = sub(eye, at);
        if n[0] * toward[0] + n[1] * toward[1] + n[2] * toward[2] > 0.0 {
            out.push((v, f));
        }
    }
    out
}

fn around(model: &Model, touched: &BTreeMap<u32, f32>) -> BTreeMap<u32, Vec<u32>> {
    let mut out: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
    for t in model.indices.chunks_exact(3) {
        for (a, b) in [(t[0], t[1]), (t[1], t[2]), (t[2], t[0])] {
            for (x, y) in [(a, b), (b, a)] {
                if touched.contains_key(&x) {
                    out.entry(x).or_default().push(y);
                }
            }
        }
    }
    out
}

pub(super) fn apply(model: &Model, s: &SculptStroke, reached: &[(u32, f32)], moved: &mut Moved) {
    let touched: BTreeMap<u32, f32> = reached.iter().copied().collect();
    let near = if s.mode == Carve::Smooth { around(model, &touched) } else { BTreeMap::new() };
    let before = moved.clone();
    for (&v, &f) in &touched {
        let d = moved.entry(v).or_default();
        match s.mode {
            Carve::Push | Carve::Pull => {
                let n = unit(read(model, v, 3));
                let sign = if s.mode == Carve::Push { 1.0 } else { -1.0 };
                (0..3).for_each(|i| d[i] += n[i] * s.strength * f * sign);
            }
            Carve::Smooth => {
                let Some(list) = near.get(&v).filter(|l| !l.is_empty()) else { continue };
                let mean = list.iter().fold([0.0; 3], |m, n| {
                    let q = shaped(model, &before, *n);
                    std::array::from_fn(|i| m[i] + q[i] / list.len() as f32)
                });
                let now = shaped(model, &before, v);
                (0..3).for_each(|i| d[i] += (mean[i] - now[i]) * f * s.strength);
            }
        }
    }
}

fn normals(model: &Model, moved: &Moved, (first, count): (u32, u32), shift: bool) -> BTreeMap<u32, [f32; 3]> {
    let mut sum: BTreeMap<u32, [f32; 3]> = BTreeMap::new();
    let at = |v: u32| if shift { shaped(model, moved, v) } else { read(model, v, 0) };
    for t in model.indices[first as usize..(first + count) as usize].chunks_exact(3) {
        let n = cross(sub(at(t[1]), at(t[0])), sub(at(t[2]), at(t[0])));
        t.iter().for_each(|v| {
            let s = sum.entry(*v).or_default();
            (0..3).for_each(|i| s[i] += n[i]);
        });
    }
    sum.into_iter().map(|(v, n)| (v, unit(n))).collect()
}

pub(super) fn entries(
    model: &Model,
    moved: &Moved,
    source: impl Fn(u32) -> Option<(u16, u16, u32)>,
) -> Vec<Carved> {
    let mut out = Vec::new();
    for prim in &model.primitives {
        let mine =
            moved.range(prim.base..prim.base + prim.vertices).any(|(_, d)| d.iter().any(|x| x.abs() > 1e-7));
        if !mine {
            continue;
        }
        let (was, now) = (
            normals(model, moved, (prim.first, prim.count), false),
            normals(model, moved, (prim.first, prim.count), true),
        );
        for v in prim.base..prim.base + prim.vertices {
            let at = moved.get(&v).copied().unwrap_or_default();
            let n = match (was.get(&v), now.get(&v)) {
                (Some(a), Some(b)) => sub(*b, *a),
                _ => [0.0; 3],
            };
            let Some((node, primitive, vertex)) = source(v) else { continue };
            if at.iter().chain(&n).any(|x| x.abs() > 1e-6) {
                out.push(Carved { node, primitive, vertex, at, normal: n });
            }
        }
    }
    out
}
