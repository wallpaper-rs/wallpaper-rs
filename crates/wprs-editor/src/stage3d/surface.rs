use super::gizmo::{seen_at, world};
use serde_json::{Value, json};
use std::path::Path;
use std::sync::Arc;
use wprs_scene::{MODEL_FLOATS, Mat4, Placed, Quality, Runtime, SKIN_FLOATS, Values, unit3};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Hit {
    pub layer: String,
    pub material: String,
    pub uv: [f32; 2],
    pub uv1: [f32; 2],
    pub point: [f32; 3],
    pub normal: [f32; 3],
    pub matrix: Mat4,
    pub unit: f32,
    pub uv_scale: f32,
}

type Spots<'a> = (&'a Placed<'a>, Vec<[f32; 3]>, ([f32; 3], f32));

fn sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    std::array::from_fn(|i| a[i] - b[i])
}

pub(super) fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

fn spot(p: &Placed, v: usize) -> [f32; 3] {
    let f = &p.model.vertices[v * MODEL_FLOATS..];
    let local = [f[0], f[1], f[2]];
    let palette = p.pose.map_or(&p.model.rest[..], |q| &q.palette[..]);
    if !p.model.posed() || palette.is_empty() {
        return world(&p.matrix, local);
    }
    let s = &p.model.skin[v * SKIN_FLOATS..];
    let mut out = [0.0; 3];
    for k in 0..4 {
        let Some(m) = palette.get(s[k] as usize) else { continue };
        for (r, o) in out.iter_mut().enumerate() {
            *o += (m[r * 4] * local[0] + m[r * 4 + 1] * local[1] + m[r * 4 + 2] * local[2] + m[r * 4 + 3])
                * s[4 + k];
        }
    }
    world(&p.matrix, out)
}

fn texel(p: &Placed, v: usize, at: usize) -> [f32; 2] {
    let f = &p.model.vertices[v * MODEL_FLOATS + at..];
    [f[0], f[1]]
}

fn ball(spots: &[[f32; 3]]) -> ([f32; 3], f32) {
    let n = spots.len().max(1) as f32;
    let c: [f32; 3] = std::array::from_fn(|i| spots.iter().map(|p| p[i]).sum::<f32>() / n);
    (c, spots.iter().map(|p| dot(sub(*p, c), sub(*p, c))).fold(0.0, f32::max).sqrt())
}

fn misses((c, r): ([f32; 3], f32), from: [f32; 3], dir: [f32; 3]) -> bool {
    let to = sub(c, from);
    let along = dot(to, dir);
    dot(to, to) - along * along > r * r * 1.0001 + 1e-6
}

fn cast((p, spots): (&Placed, &[[f32; 3]]), from: [f32; 3], dir: [f32; 3], best: &mut Option<(f32, Hit)>) {
    for prim in &p.model.primitives {
        let ids = &p.model.indices[prim.first as usize..(prim.first + prim.count) as usize];
        for t in ids.as_chunks::<3>().0 {
            let [a, b, c] = t.map(|i| i as usize);
            let [pa, pb, pc] = [a, b, c].map(|i| spots[i]);
            let (e1, e2) = (sub(pb, pa), sub(pc, pa));
            let h = cross(dir, e2);
            let det = dot(e1, h);
            if det.abs() < 1e-9 {
                continue;
            }
            let s = sub(from, pa);
            let u = dot(s, h) / det;
            let q = cross(s, e1);
            let w = dot(dir, q) / det;
            let d = dot(e2, q) / det;
            if u < 0.0 || w < 0.0 || u + w > 1.0 || d <= 0.0 || best.as_ref().is_some_and(|(k, _)| *k <= d) {
                continue;
            }
            let mix = |k: usize| {
                let [ta, tb, tc] = [a, b, c].map(|i| texel(p, i, k));
                [0, 1].map(|i| ta[i] * (1.0 - u - w) + tb[i] * u + tc[i] * w)
            };
            let [ua, ub, uc] = [a, b, c].map(|i| texel(p, i, 6));
            let (f1, f2) = ([ub[0] - ua[0], ub[1] - ua[1]], [uc[0] - ua[0], uc[1] - ua[1]]);
            let flat = (f1[0] * f2[1] - f1[1] * f2[0]).abs();
            let n = cross(e1, e2);
            let area = dot(n, n).sqrt().max(1e-12);
            let n = unit3(n);
            let normal = if dot(n, dir) > 0.0 { n.map(|x| -x) } else { n };
            let material = prim.material.and_then(|m| p.model.materials.get(usize::from(m)));
            let hit = Hit {
                layer: p.layer.id.clone(),
                material: material.map_or_else(|| "default".into(), |m| m.name.clone()),
                uv: mix(6),
                uv1: mix(12),
                point: std::array::from_fn(|i| from[i] + dir[i] * d),
                normal,
                matrix: p.matrix,
                unit: 0.0,
                uv_scale: (flat / area).sqrt(),
            };
            *best = Some((d, hit));
        }
    }
}

pub(crate) fn hits(dir: &Path, doc: &Value, t: f64, points: &[[f32; 2]]) -> Vec<Option<Hit>> {
    let Some(scene) = super::store::scene(dir, doc) else { return vec![None; points.len()] };
    let size = scene.canvas.size.map(|v| v as f32);
    let mut rt = Runtime::new(Arc::new(scene), Values::new());
    rt.advance(t.max(0.0));
    let frame = rt.frame([0.0, 0.0, size[0], size[1]], Quality::Q3);
    let sights: Vec<_> =
        (0..frame.stages.len()).filter_map(|k| Some((k, seen_at(doc, &frame, k)?))).collect();
    let placed: Vec<Vec<Spots>> = frame
        .stages
        .iter()
        .map(|s| {
            let each = frame.placed[s.models.clone()].iter();
            each.map(|p| {
                let spots: Vec<[f32; 3]> = (0..p.model.vertex_count()).map(|v| spot(p, v)).collect();
                let around = ball(&spots);
                (p, spots, around)
            })
            .collect()
        })
        .collect();
    points
        .iter()
        .map(|at| {
            let mut best: Option<(f32, Hit)> = None;
            for (k, seen) in &sights {
                let back = seen.to.inverse();
                let q = back.apply(*at);
                let one = back.apply([at[0] + 1.0, at[1]]);
                let stretch = (one[0] - q[0]).hypot(one[1] - q[1]);
                let eye = seen.eye;
                let ray = unit3(sub(eye.place(q, 0.0, seen.size), eye.at));
                let was = best.as_ref().map(|b| b.0);
                for (p, spots, around) in &placed[*k] {
                    if !misses(*around, eye.at, ray) {
                        cast((p, spots), eye.at, ray, &mut best);
                    }
                }
                if let Some((_, h)) = best.as_mut().filter(|b| Some(b.0) != was) {
                    h.unit =
                        stretch * eye.unit(seen.size[1]) * eye.depth(h.point).max(eye.near) / eye.distance();
                }
            }
            best.map(|(_, h)| h)
        })
        .collect()
}

pub(crate) fn surface(dir: &Path, doc: &Value, q: &Value) -> Value {
    let n = |k: usize| q["at"][k].as_f64().unwrap_or(f64::NAN) as f32;
    let Some(h) = hits(dir, doc, q["time"].as_f64().unwrap_or(0.0), &[[n(0), n(1)]]).pop().flatten() else {
        return Value::Null;
    };
    let r = crate::rigs::round;
    json!({ "layer": h.layer, "material": h.material, "uv": h.uv.map(r), "point": h.point.map(r),
        "normal": h.normal.map(r), "unit": r(h.unit), "color": super::picked::color_at(dir, doc, &h) })
}
