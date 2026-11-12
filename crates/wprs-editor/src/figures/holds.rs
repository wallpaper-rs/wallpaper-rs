use crate::seats::boxes::pair;
use serde_json::Value;
use std::f64::consts::{FRAC_PI_2, TAU};

pub(crate) const INSET: f64 = 0.8;

pub(crate) struct Hold {
    pub name: &'static str,
    pub kind: &'static str,
    pub u: [f64; 2],
}

pub(crate) fn number(v: &Value, fallback: f64) -> f64 {
    v.as_f64().unwrap_or(fallback)
}

pub(crate) fn first(layer: &Value) -> Option<(&str, &Value)> {
    layer["shape"].as_object()?.iter().next().map(|(k, v)| (k.as_str(), v))
}

pub(crate) fn extent(kind: &str, s: &Value) -> [f64; 2] {
    match kind {
        "polygon" | "star" => [number(&s["size"], 100.0); 2],
        "line" => {
            let w = number(&s["width"], 1.0);
            pair(&s["to"], [100.0, 0.0]).map(|v| v.abs() + w)
        }
        _ => pair(&s["size"], [100.0; 2]),
    }
}

pub(crate) fn line_ends(s: &Value) -> ([f64; 2], [f64; 2]) {
    let (to, w) = (pair(&s["to"], [100.0, 0.0]), number(&s["width"], 1.0));
    let size = extent("line", s);
    let place = [0, 1].map(|i| (w / 2.0 - to[i].min(0.0)) / size[i]);
    (place, [0, 1].map(|i| place[i] + to[i] / size[i]))
}

pub(crate) fn spoke(index: usize, steps: usize, radius: f64, size: f64) -> [f64; 2] {
    let a = -FRAC_PI_2 + index as f64 * TAU / steps as f64;
    [0.5 + a.cos() * radius / size, 0.5 + a.sin() * radius / size]
}

fn on(name: &'static str, kind: &'static str, u: [f64; 2]) -> Hold {
    Hold { name, kind, u }
}

pub(crate) fn holds(kind: &str, s: &Value, layer: &Value) -> Vec<Hold> {
    let [w, h] = extent(kind, s);
    let mut out = match kind {
        "rect" => {
            let r = number(&s["radius"], 0.0).clamp(0.0, w.min(h) / 2.0);
            let far = 0.5 + INSET / 2.0;
            vec![
                on("width", "edge", [far, 0.5]),
                on("height", "edge", [0.5, far]),
                on("size", "corner", [far, far]),
                on("radius", "round", [1.0 - r / w, 1.0 - far]),
            ]
        }
        "ellipse" | "ring" => {
            let k = 0.5 + 0.5 * std::f64::consts::FRAC_1_SQRT_2;
            let mut list = vec![
                on("width", "edge", [1.0, 0.5]),
                on("height", "edge", [0.5, 1.0]),
                on("size", "corner", [k, k]),
            ];
            if kind == "ring" {
                list.push(on("band", "round", [0.5, number(&s["width"], 1.0) / h]));
            }
            list
        }
        "polygon" => {
            let n = number(&s["sides"], 3.0).max(3.0) as usize;
            vec![
                on("size", "corner", spoke(0, 1, w / 2.0 * INSET, w)),
                on("sides", "count", spoke(1, n, w / 2.0, w)),
            ]
        }
        "star" => {
            let n = number(&s["points"], 5.0).max(2.0) as usize;
            let inner = w / 2.0 * number(&s["inner"], 0.5);
            vec![
                on("size", "corner", spoke(0, 1, w / 2.0 * INSET, w)),
                on("points", "count", spoke(2, n * 2, w / 2.0, w)),
                on("inner", "depth", spoke(1, n * 2, inner, w)),
            ]
        }
        "line" => {
            let (a, b) = line_ends(s);
            let to = pair(&s["to"], [100.0, 0.0]);
            let long = to[0].hypot(to[1]).max(1e-9);
            let half = number(&s["width"], 1.0) / 2.0;
            let n = [-to[1] / long * half / w, to[0] / long * half / h];
            let mid = [(a[0] + b[0]) / 2.0 + n[0], (a[1] + b[1]) / 2.0 + n[1]];
            vec![on("start", "end", a), on("end", "end", b), on("width", "edge", mid)]
        }
        _ => Vec::new(),
    };
    if let Some(width) = layer["stroke"]["width"].as_f64().filter(|_| kind != "line") {
        out.push(on("stroke", "edge", [-width / (2.0 * w), 0.8]));
    }
    if layer["gradient"].is_object() {
        let g = &layer["gradient"];
        out.push(on("gradient_from", "point", pair(&g["from"], [0.0, 0.0])));
        out.push(on("gradient_to", "point", pair(&g["to"], [0.0, 1.0])));
    }
    out
}
