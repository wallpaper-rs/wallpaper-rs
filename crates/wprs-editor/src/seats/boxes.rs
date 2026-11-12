use super::Seat;
use crate::keys::value_at;
use serde_json::Value;
use std::path::Path;

static NULL: Value = Value::Null;

pub(crate) fn pair(value: &Value, fallback: [f64; 2]) -> [f64; 2] {
    match value {
        Value::Number(n) => [n.as_f64().unwrap_or(0.0); 2],
        Value::Array(list) if list.len() == 2 => [0, 1].map(|i| list[i].as_f64().unwrap_or(fallback[i])),
        _ => fallback,
    }
}

pub(super) fn size(dir: &Path, doc: &Value, layer: &Value, canvas: [f64; 2], t: f64) -> [f64; 2] {
    if layer.get("size").is_some_and(Value::is_array) {
        return pair(&layer["size"], canvas);
    }
    let file =
        |id: &Value| doc.pointer("/scene/assets")?.get(id.as_str()?)?["path"].as_str().map(|p| dir.join(p));
    match layer["type"].as_str().unwrap_or_default() {
        "image" => file(&value_at(&layer["image"], t))
            .and_then(|f| crate::pictures::dimensions(f).ok())
            .map_or([512.0; 2], |(w, h)| [f64::from(w), f64::from(h)]),
        "video" => [640.0, 360.0],
        "text" => {
            let text = value_at(&layer["text"], t);
            let chars = text.as_str().or(layer["text"]["clock"].as_str()).map_or(4, |s| s.chars().count());
            let em = layer["size"].as_f64().unwrap_or(48.0);
            [em * 0.6 * chars as f64, em * 1.2]
        }
        "shape" => {
            let first = layer["shape"].as_object().and_then(|o| o.iter().next());
            let (kind, shape) = first.map_or(("", &NULL), |(k, v)| (k.as_str(), v));
            match kind {
                "polygon" | "star" => [shape["size"].as_f64().unwrap_or(100.0) * 2.0; 2],
                "line" => {
                    pair(&shape["to"], [100.0, 0.0]).map(|v| v.abs() + shape["width"].as_f64().unwrap_or(1.0))
                }
                _ => pair(&shape["size"], [100.0; 2]),
            }
        }
        _ => canvas,
    }
}

pub(super) fn groups(seats: &mut [Seat]) {
    for i in (0..seats.len()).rev() {
        if seats[i].kind != "group" {
            continue;
        }
        let inside = format!("{}/layers/", seats[i].pointer);
        let back = seats[i].world.inverse();
        let points: Vec<[f32; 2]> = seats
            .iter()
            .filter(|s| s.pointer.strip_prefix(&inside).is_some_and(|rest| !rest.contains('/')))
            .flat_map(|s| s.outline())
            .map(|p| back.apply(p.map(|v| v as f32)))
            .collect();
        let (lo, hi) = points.iter().fold(([f32::MAX; 2], [f32::MIN; 2]), |(lo, hi), p| {
            ([lo[0].min(p[0]), lo[1].min(p[1])], [hi[0].max(p[0]), hi[1].max(p[1])])
        });
        let size = [0, 1].map(|k| f64::from(hi[k] - lo[k]));
        let group = &mut seats[i];
        (group.size, group.origin) = match points.is_empty() || size.iter().any(|s| *s <= 0.0) {
            true => ([0.0; 2], [0.5; 2]),
            false => (size, [0, 1].map(|k| -f64::from(lo[k]) / size[k])),
        };
    }
}
