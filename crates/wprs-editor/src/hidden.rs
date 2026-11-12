use crate::findings::{Problem, warning};
use serde_json::Value;
use wprs_core::message::said;
use wprs_scene::Color;

const DRAWN_NOT: [&str; 3] = ["light", "sound", "group3d"];

fn number(v: &Value) -> Option<f64> {
    v.as_f64()
}

fn pair(v: &Value) -> Option<[f64; 2]> {
    let list = v.as_array()?;
    Some([number(list.first()?)?, number(list.get(1)?)?])
}

fn static_flag(layer: &Value) -> bool {
    layer.get("visible").is_none_or(|v| v.as_bool() == Some(true))
}

fn opaque(color: &Value) -> bool {
    color.as_str().and_then(Color::parse).is_some_and(|c| c.0[3] >= 0.999)
}

fn filled(layer: &Value) -> bool {
    match (layer.get("gradient"), layer.get("fill"), layer["type"].as_str()) {
        (Some(g), _, _) => {
            let stops = g["stops"].as_array();
            stops.is_some_and(|s| !s.is_empty() && s.iter().all(|stop| opaque(&stop[1])))
        }
        (None, Some(fill), _) => opaque(fill),
        (None, None, Some("solid")) => opaque(&layer["color"]),
        _ => false,
    }
}

fn plain_transform(layer: &Value) -> bool {
    let Some(t) = layer.get("transform") else { return layer.get("anchor").is_none() };
    let scale_ok = t.get("scale").is_none_or(|s| match number(s) {
        Some(n) => n >= 1.0,
        None => pair(s).is_some_and(|p| p[0] >= 1.0 && p[1] >= 1.0),
    });
    let still = |key: &str, zero: f64| t.get(key).is_none_or(|v| number(v) == Some(zero));
    let origin = t.get("origin").is_none_or(|o| pair(o) == Some([0.5, 0.5]));
    let tilted = t.get("tilt").is_some() || t.get("z").is_some();
    t.get("position").is_none()
        && scale_ok
        && still("rotation", 0.0)
        && t.get("skew").is_none_or(|s| pair(s) == Some([0.0, 0.0]))
        && origin
        && !tilted
        && layer.get("anchor").is_none()
}

fn covers(layer: &Value, canvas: [f64; 2]) -> bool {
    let kind = layer["type"].as_str();
    let size = match kind {
        Some("solid") => layer.get("size").map_or(Some(canvas), pair),
        Some("shape") => {
            let rect = &layer["shape"]["rect"];
            let round = rect.get("radius").and_then(number).unwrap_or(0.0) > 0.0;
            if round { None } else { pair(&rect["size"]) }
        }
        _ => None,
    };
    let scale = layer["transform"].get("scale");
    let scales = scale.and_then(pair).or_else(|| scale.and_then(number).map(|n| [n; 2])).unwrap_or([1.0; 2]);
    let big = size.is_some_and(|s| s[0] * scales[0] >= canvas[0] && s[1] * scales[1] >= canvas[1]);
    big && filled(layer)
        && static_flag(layer)
        && layer.get("opacity").is_none_or(|o| number(o).is_some_and(|n| n >= 1.0))
        && layer.get("blend").is_none_or(|b| b == "normal")
        && layer.get("mask").is_none()
        && layer.get("parent").is_none()
        && layer.get("parallax").is_none_or(|p| number(p) == Some(0.0))
        && layer.get("effects").and_then(Value::as_array).is_none_or(Vec::is_empty)
        && plain_transform(layer)
}

fn drawn(layer: &Value) -> bool {
    layer["type"].as_str().is_some_and(|k| !DRAWN_NOT.contains(&k)) && static_flag(layer)
}

fn names(layer: &Value) -> String {
    layer["id"].as_str().unwrap_or_default().to_owned()
}

fn beneath(list: &Value, base: &str, cover: &str, out: &mut Vec<Problem>) {
    for (i, layer) in list.as_array().into_iter().flatten().enumerate() {
        let at = format!("{base}/{i}");
        if layer["type"] == "group" {
            beneath(&layer["layers"], &format!("{at}/layers"), cover, out);
        } else if drawn(layer) {
            let hidden = names(layer);
            out.push(warning(
                at,
                said(
                    "editor.layer_covered",
                    "The layer \"%1\" never shows: \"%2\", above it, covers the whole canvas with no transparency; hide or move \"%2\", or move \"%1\" above it",
                    &[&hidden, &cover],
                ),
            ));
        }
    }
}

fn extent(layer: &Value) -> Option<f64> {
    let shape = layer.get("shape")?;
    let half = |size: &Value| pair(size).map(|p| p[0].hypot(p[1]) / 2.0);
    let radius = ["rect", "ellipse"].iter().find_map(|k| shape.get(*k).and_then(|s| half(&s["size"])));
    let star = shape.get("star").and_then(|s| number(&s["size"]));
    radius.or(star)
}

fn off(layer: &Value, canvas: [f64; 2]) -> bool {
    let t = &layer["transform"];
    let (Some(at), Some(reach)) = (t.get("position").and_then(pair), extent(layer)) else { return false };
    let scales = t.get("scale").map_or(Some([1.0; 2]), |s| pair(s).or_else(|| number(s).map(|n| [n; 2])));
    let Some(scales) = scales else { return false };
    let r = reach * scales[0].abs().max(scales[1].abs());
    let plain = layer.get("anchor").is_none() && layer.get("parent").is_none() && static_flag(layer);
    let beyond = at[0] + r < 0.0 || at[0] - r > canvas[0] || at[1] + r < 0.0 || at[1] - r > canvas[1];
    plain && beyond && t.get("origin").is_none()
}

fn walk(list: &Value, base: &str, canvas: [f64; 2], out: &mut Vec<Problem>) {
    let layers = list.as_array().map(Vec::as_slice).unwrap_or_default();
    for (i, layer) in layers.iter().enumerate() {
        let at = format!("{base}/{i}");
        if covers(layer, canvas) {
            beneath(&Value::Array(layers[..i].to_vec()), base, &names(layer), out);
        }
        if layer["type"] == "group" {
            let moved = layer.get("transform").is_some() || layer.get("anchor").is_some();
            if !moved {
                walk(&layer["layers"], &format!("{at}/layers"), canvas, out);
            }
        } else if off(layer, canvas) {
            out.push(warning(
                at,
                said(
                    "editor.layer_off_canvas",
                    "The layer \"%1\" lies completely outside the canvas, so it never shows; move it inside the canvas",
                    &[&names(layer)],
                ),
            ));
        }
    }
}

pub(crate) fn problems(scene: &Value) -> Vec<Problem> {
    let canvas = pair(&scene["canvas"]["size"]).unwrap_or([1920.0, 1080.0]);
    let mut out = Vec::new();
    walk(&scene["layers"], "/scene/layers", canvas, &mut out);
    let mut seen = Vec::new();
    out.retain(|p| {
        let fresh = !seen.contains(&p.pointer);
        seen.push(p.pointer.clone());
        fresh
    });
    out
}
