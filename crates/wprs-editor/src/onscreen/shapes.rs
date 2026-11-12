use serde_json::Value;
use std::f64::consts::{FRAC_PI_2, TAU};

pub(super) type Loop = Vec<[f64; 2]>;

const ROUND: usize = 48;

pub(super) fn pair(v: &Value, fallback: [f64; 2]) -> [f64; 2] {
    crate::seats::boxes::pair(v, fallback)
}

fn number(v: &Value, fallback: f64) -> f64 {
    v.as_f64().unwrap_or(fallback)
}

fn ellipse(w: f64, h: f64) -> Loop {
    (0..ROUND)
        .map(|i| {
            let a = i as f64 * TAU / ROUND as f64;
            [a.cos() * w / 2.0, a.sin() * h / 2.0]
        })
        .collect()
}

fn rounded(w: f64, h: f64, r: f64) -> Loop {
    let r = r.clamp(0.0, w.min(h).abs() / 2.0);
    if r < 0.5 {
        return vec![[-w / 2.0, -h / 2.0], [w / 2.0, -h / 2.0], [w / 2.0, h / 2.0], [-w / 2.0, h / 2.0]];
    }
    let centers = [
        [w / 2.0 - r, h / 2.0 - r],
        [-w / 2.0 + r, h / 2.0 - r],
        [-w / 2.0 + r, -h / 2.0 + r],
        [w / 2.0 - r, -h / 2.0 + r],
    ];
    let mut out = Vec::new();
    for (k, c) in centers.iter().enumerate() {
        for s in 0..=8 {
            let a = (k as f64 + s as f64 / 8.0) * FRAC_PI_2;
            out.push([c[0] + a.cos() * r, c[1] + a.sin() * r]);
        }
    }
    out
}

fn spokes(count: usize, outer: f64, inner: Option<f64>) -> Loop {
    let steps = if inner.is_some() { count * 2 } else { count };
    (0..steps)
        .map(|i| {
            let a = -FRAC_PI_2 + i as f64 * TAU / steps as f64;
            let r = if i % 2 == 1 { inner.unwrap_or(outer) } else { outer };
            [a.cos() * r, a.sin() * r]
        })
        .collect()
}

pub(super) fn loops(kind: &str, s: &Value, grow: f64) -> Vec<Loop> {
    let size = pair(&s["size"], [0.0; 2]);
    let (w, h) = (size[0] + 2.0 * grow, size[1] + 2.0 * grow);
    match kind {
        "rect" => vec![rounded(w, h, number(&s["radius"], 0.0) + grow)],
        "ellipse" => vec![ellipse(w, h)],
        "ring" => {
            let band = number(&s["width"], 0.0) * 2.0;
            let hole = [(size[0] - band - 2.0 * grow).max(0.0), (size[1] - band - 2.0 * grow).max(0.0)];
            vec![ellipse(w, h), ellipse(hole[0], hole[1])]
        }
        "polygon" => vec![spokes(
            number(&s["sides"], 3.0).max(3.0) as usize,
            number(&s["size"], 0.0) / 2.0 + grow,
            None,
        )],
        "star" => {
            let outer = number(&s["size"], 0.0) / 2.0;
            let inner = outer * number(&s["inner"], 0.5);
            vec![spokes(number(&s["points"], 5.0).max(2.0) as usize, outer + grow, Some(inner + grow))]
        }
        "line" => {
            let to = pair(&s["to"], [0.0; 2]);
            let long = to[0].hypot(to[1]);
            let (d, half) =
                if long < 1e-9 { ([1.0, 0.0], 0.0) } else { ([to[0] / long, to[1] / long], long / 2.0) };
            let reach = half + number(&s["width"], 1.0) / 2.0 + grow;
            let wide = (number(&s["width"], 1.0) / 2.0 + grow).max(0.0);
            let n = [-d[1] * wide, d[0] * wide];
            let (a, b) = ([-d[0] * reach, -d[1] * reach], [d[0] * reach, d[1] * reach]);
            vec![vec![
                [a[0] + n[0], a[1] + n[1]],
                [b[0] + n[0], b[1] + n[1]],
                [b[0] - n[0], b[1] - n[1]],
                [a[0] - n[0], a[1] - n[1]],
            ]]
        }
        _ => Vec::new(),
    }
}

pub(super) fn holds(kind: &str, s: &Value, feather: f64) -> Vec<(&'static str, [f64; 2])> {
    let size = pair(&s["size"], [0.0; 2]);
    match kind {
        "rect" => {
            let r = number(&s["radius"], 0.0);
            vec![
                ("size", [size[0] / 2.0, size[1] / 2.0]),
                ("feather", [size[0] / 2.0 + feather, 0.0]),
                ("radius", [size[0] / 2.0 - r, -size[1] / 2.0]),
            ]
        }
        "ellipse" => {
            vec![("size", [size[0] / 2.0, size[1] / 2.0]), ("feather", [size[0] / 2.0 + feather, 0.0])]
        }
        "ring" => vec![
            ("size", [size[0] / 2.0, size[1] / 2.0]),
            ("feather", [size[0] / 2.0 + feather, 0.0]),
            ("width", [size[0] / 2.0 - number(&s["width"], 0.0), 0.0]),
        ],
        "polygon" => {
            let r = number(&s["size"], 0.0) / 2.0;
            vec![("size", [r, r]), ("feather", [r + feather, 0.0])]
        }
        "star" => {
            let r = number(&s["size"], 0.0) / 2.0;
            let n = number(&s["points"], 5.0).max(2.0);
            let a = -FRAC_PI_2 + TAU / (n * 2.0);
            let k = r * number(&s["inner"], 0.5);
            vec![("size", [r, r]), ("feather", [r + feather, 0.0]), ("inner", [a.cos() * k, a.sin() * k])]
        }
        "line" => {
            let to = pair(&s["to"], [0.0; 2]);
            let long = to[0].hypot(to[1]).max(1e-9);
            let reach = number(&s["width"], 1.0) / 2.0 + feather;
            vec![
                ("size", [to[0] / 2.0, to[1] / 2.0]),
                ("feather", [-to[1] / long * reach, to[0] / long * reach]),
            ]
        }
        _ => Vec::new(),
    }
}
