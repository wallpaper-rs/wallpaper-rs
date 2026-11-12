mod clock;
mod list;
mod set;
mod width;

use crate::action::FigureEdit;
use crate::actions::layer;
use crate::keys::value_at;
use crate::ops::{Op, set as put};
use crate::seats::seat;
use anyhow::{Result, ensure};
use serde_json::{Value, json};
use std::path::Path;
use wprs_core::message::said;
use wprs_type::Measured;

pub fn asks(q: &Value) -> bool {
    ["text_box", "font_list", "clock_sample", "clock_help"].iter().any(|k| q.get(k).is_some())
}

fn shown(layer: &Value, t: f64) -> String {
    let value = value_at(&layer["text"], t);
    if let Some(text) = value.as_str() {
        return text.to_owned();
    }
    let source = &layer["text"];
    if let Some(pattern) = source["clock"].as_str() {
        return wprs_scene::format_time(pattern, clock_now(), "en", false).0;
    }
    if let Some(pattern) = source["date"].as_str() {
        return wprs_scene::format_time(pattern, clock_now(), "en", true).0;
    }
    let first = source["map"].as_object().and_then(|m| m.values().find_map(Value::as_str));
    first.unwrap_or("0000").to_owned()
}

fn clock_now() -> f64 {
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH);
    now.map_or(0.0, |d| d.as_secs_f64())
}

pub(crate) fn measure(dir: &Path, doc: &Value, layer: &Value, t: f64) -> Option<Measured> {
    let chain = set::chain(dir, doc, layer);
    let size = layer["size"].as_f64().unwrap_or(48.0) as f32;
    wprs_type::measure(&chain, &shown(layer, t), size, &set::layout(layer, t))
}

pub fn query(dir: &Path, doc: &Value, q: &Value) -> Value {
    if let Some(c) = q.get("clock_sample") {
        return clock::sample(c);
    }
    if let Some(c) = q.get("clock_help") {
        return clock::help(c);
    }
    let (key, id, t) = match (q.get("text_box"), q.get("font_list")) {
        (Some(b), _) => {
            ("text_box", b["layer"].as_str().unwrap_or_default(), b["time"].as_f64().unwrap_or(0.0))
        }
        (_, Some(f)) => ("font_list", f["layer"].as_str().unwrap_or_default(), 0.0),
        _ => return Value::Null,
    };
    let found = layer(doc, id).ok().map(|(_, l)| l.clone());
    let empty = Value::Null;
    let layer = found.as_ref().unwrap_or(&empty);
    if key == "font_list" {
        return list::fonts(doc, layer);
    }
    let (Some(seat), Some(m)) = (seat(dir, doc, id, t), measure(dir, doc, layer, t)) else {
        return Value::Null;
    };
    json!({
        "size": m.size, "lines": m.lines, "widest": m.widest,
        "wrapped": layer["wrap"].is_object(),
        "width": layer["wrap"]["width"],
        "handle": seat.at([1.0, 0.5]),
        "corners": seat.corners(),
    })
}

pub(crate) fn edit(dir: &Path, doc: &Value, e: &FigureEdit) -> Result<Vec<Op>> {
    match e {
        FigureEdit::TextWidth { layer: id, to, remove, at, replace } => {
            let (path, found) = layer(doc, id)?;
            ensure!(
                found["type"] == "text",
                said("editor.not_text", "the layer %1 is not a text layer", &[id])
            );
            let wrap = format!("{path}/wrap");
            let Some(to) = to.filter(|_| !remove) else {
                return Ok(found.get("wrap").map(|_| Op::Remove { path: wrap }).into_iter().collect());
            };
            width::drag(dir, doc, (id, &path, found), to, (*at, *replace))
        }
        FigureEdit::Fonts { layer: id, font, fallback } => {
            fonts(doc, id, font.as_deref(), fallback.as_deref())
        }
        _ => Ok(Vec::new()),
    }
}

fn fonts(doc: &Value, id: &str, font: Option<&str>, fallback: Option<&[String]>) -> Result<Vec<Op>> {
    let (path, found) = layer(doc, id)?;
    ensure!(found["type"] == "text", said("editor.not_text", "the layer %1 is not a text layer", &[&id]));
    let mut ops = Vec::new();
    if let Some(name) = font {
        let at = format!("{path}/font");
        match name.is_empty() {
            true => ops.extend(found.get("font").map(|_| Op::Remove { path: at })),
            false => ops.extend(put(doc, &at, json!(name))),
        }
    }
    if let Some(list) = fallback {
        let most = wprs_scene::MAX_FALLBACK;
        ensure!(
            list.len() <= most,
            said("editor.fallback_most", "a text has at most %1 fallback fonts", &[&most])
        );
        let at = format!("{path}/fallback");
        match list.is_empty() {
            true => ops.extend(found.get("fallback").map(|_| Op::Remove { path: at })),
            false => ops.extend(put(doc, &at, json!(list))),
        }
    }
    Ok(ops)
}
