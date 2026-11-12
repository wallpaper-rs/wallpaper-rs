use super::gizmo::{Seen, px, world};
use serde_json::{Value, json};
use wprs_scene::{MODEL_UNIT, Mat, Placed, Rigged};

fn point(w: &Mat, [x, y, z]: [f32; 3]) -> [f32; 3] {
    std::array::from_fn(|r| w[r * 4] * x + w[r * 4 + 1] * y + w[r * 4 + 2] * z + w[r * 4 + 3])
}

fn add(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    std::array::from_fn(|i| a[i] + b[i])
}

fn gap(a: [f32; 2], b: [f32; 2]) -> f32 {
    (a[0] - b[0]).hypot(a[1] - b[1])
}

pub(super) fn colliders(
    seen: &Seen,
    p: &Placed,
    (rig, worlds): (&Rigged, &[Mat]),
    raw: &Value,
) -> Vec<Value> {
    let shown = |q: [f32; 3]| seen.at(world(&p.matrix, q));
    let mut out = Vec::new();
    for (i, c) in raw["colliders"].as_array().into_iter().flatten().enumerate() {
        let Some(n) = c["bone"].as_str().and_then(|b| rig.joint(b)) else { continue };
        let num = |v: &Value| v.as_f64().unwrap_or(0.0) as f32;
        let offset = [0, 1, 2].map(|k| num(&c["offset"][k]));
        let (r, sphere) = (num(&c["radius"]), c["shape"] == "sphere");
        let a = point(&worlds[n], offset);
        let b = if sphere { a } else { point(&worlds[n], add(rig.tip(n), offset)) };
        let (Some(pa), Some(pb)) = (shown(a), shown(b)) else { continue };
        let per = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
            .iter()
            .filter_map(|e| shown(add(a, *e)).map(|q| gap(q, pa)))
            .fold(0.0f32, f32::max);
        out.push(json!({ "index": i, "bone": c["bone"], "shape": if sphere { "sphere" } else { "capsule" },
            "at": px(pa), "to": px(pb), "radius": r, "per": per, "rim": px([pa[0] + r * per, pa[1]]) }));
    }
    out
}

pub(super) fn slides(seen: &Seen, p: &Placed, worlds: &[Mat], rig: &Rigged, raw: &Value) -> Vec<Value> {
    let shown = |q: [f32; 3]| seen.at(world(&p.matrix, q));
    let mut out = Vec::new();
    for e in raw["physics"].as_array().into_iter().flatten() {
        let Some(n) = e["bone"].as_str().and_then(|b| rig.joint(b)) else { continue };
        let [lo, hi] = [0, 1].map(|k| e["translation"][k].as_f64().map(|v| v as f32));
        let (Some(lo), Some(hi)) = (lo, hi) else { continue };
        let rest = rig.joints[n].rest.t;
        let head = rig.joints[n].parent.map_or(rest, |q| point(&worlds[q as usize], rest));
        let (Some(at), Some(unit)) = (shown(head), shown(add(head, [1.0, 0.0, 0.0]))) else { continue };
        let corner = |k: usize| {
            let pick = |bit: usize, a: f32, b: f32| if k >> bit & 1 == 1 { b } else { a };
            let d = [pick(0, lo, hi), pick(1, -hi, -lo), pick(2, lo, hi)].map(|v| v * MODEL_UNIT);
            shown(add(head, d))
        };
        let corners: Vec<Option<[f32; 2]>> = (0..8).map(corner).collect();
        let edges = (0..8usize)
            .flat_map(|a| [1, 2, 4].map(|bit| (a, a | bit)).into_iter().filter(move |(x, y)| x != y));
        let lines: Vec<Value> =
            edges.filter_map(|(a, b)| Some(json!([px(corners[a]?), px(corners[b]?)]))).collect();
        let knob = |v: f32| shown(add(head, [v * MODEL_UNIT, 0.0, 0.0])).map(px);
        out.push(json!({ "bone": e["bone"], "at": px(at), "unit": px(unit), "translation": [lo, hi],
            "lines": lines, "ends": [knob(lo), knob(hi)] }));
    }
    out
}

pub(super) fn reach(seen: &Seen, raw: &Value) -> Vec<Value> {
    let edge = seen.size;
    let frame = [seen.to.apply([0.0, 0.0]), seen.to.apply(edge)];
    let mut out = Vec::new();
    for (i, c) in raw["ik"].as_array().into_iter().flatten().enumerate() {
        let Some(t) = c["target"].as_array().filter(|t| t.len() == 2) else { continue };
        let at = [0, 1].map(|k| t[k].as_f64().unwrap_or(0.0) as f32);
        out.push(json!({ "index": i, "chain": c["chain"], "at": px(seen.to.apply(at)), "point": at,
            "frame": [px(frame[0]), px(frame[1])], "size": edge }));
    }
    out
}
