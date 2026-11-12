use crate::layers::asset_kind;
use crate::ops::{Op, escape, set};
use anyhow::{Result, ensure};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::SystemTime;
use wprs_core::message::said;

pub const RATE: [f64; 2] = [0.0, 4.0];
const GAP: f64 = 0.05;
const KEPT: usize = 32;

type Probed = (PathBuf, Option<SystemTime>, Option<(u32, u32)>);

static SIZES: Mutex<Vec<Probed>> = Mutex::new(Vec::new());

fn stamp(path: &Path) -> Option<SystemTime> {
    std::fs::metadata(path).and_then(|m| m.modified()).ok()
}

pub fn size(path: &Path) -> Option<(u32, u32)> {
    let when = stamp(path);
    let held = |kept: &[Probed]| kept.iter().find(|(p, w, _)| p == path && *w == when).map(|k| k.2);
    let lock = || SIZES.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(found) = held(&lock()) {
        return found;
    }
    let found = wprs_video::probe(path).ok();
    let mut kept = lock();
    kept.retain(|(p, ..)| p != path);
    if kept.len() >= KEPT {
        kept.remove(0);
    }
    kept.push((path.to_path_buf(), when, found));
    found
}

fn names(layer: &Value) -> Vec<&str> {
    let key = if layer["type"] == "video" { "video" } else { "image" };
    match &layer[key] {
        Value::String(v) => vec![v.as_str()],
        v => v["map"].as_object().map(|m| m.values().filter_map(Value::as_str).collect()).unwrap_or_default(),
    }
}

pub fn shown<'a>(doc: &'a Value, layer: &'a Value) -> Option<(&'a str, &'a str)> {
    names(layer).into_iter().find_map(|id| {
        let path = doc["scene"]["assets"][id]["path"].as_str()?;
        (asset_kind(path) == "video").then_some((id, path))
    })
}

pub fn natural(dir: &Path, doc: &Value, layer: &Value) -> Option<[f64; 2]> {
    let (_, path) = shown(doc, layer)?;
    size(&dir.join(path)).map(|(w, h)| [f64::from(w), f64::from(h)])
}

pub fn takes(pointer: &str) -> bool {
    pointer.ends_with("/texture") && !pointer.starts_with("/scene/pools/")
}

pub fn pictures(doc: &Value) -> Vec<Value> {
    ["image", "video"].iter().flat_map(|k| crate::layers::assets_of(doc, k)).map(|a| json!(a)).collect()
}

pub fn narrow(doc: &Value, out: &mut [crate::inspect::Field]) {
    for f in out.iter_mut().filter(|f| f.key == "rate" && f.pointer.ends_with("/playback/rate")) {
        let owner = f.pointer.strip_suffix("/playback/rate").and_then(|p| doc.pointer(p));
        if owner.is_some_and(|l| shown(doc, l).is_some()) {
            f.range = json!(RATE);
        }
    }
}

pub fn film(dir: &Path, doc: &Value, id: &str, strip: usize) -> Value {
    let Ok((pointer, layer)) = crate::actions::layer(doc, id) else { return Value::Null };
    let Some((asset, path)) = shown(doc, layer) else { return Value::Null };
    let file = dir.join(path);
    let length = wprs_scene::film_file(&file);
    let p = &layer["playback"];
    let frames = (strip > 0).then(|| crate::strips::strip(&file, strip));
    let end = p["end"].as_f64().or(length.map(f64::from));
    json!({ "layer": id, "pointer": format!("{pointer}/playback"), "asset": asset, "file": file,
        "size": size(&file), "length": length, "start": p["start"].as_f64().unwrap_or(0.0), "end": end,
        "rate": crate::keys::value_at(&p["rate"], 0.0).as_f64().unwrap_or(1.0),
        "loop": p["loop"].as_bool().unwrap_or(true), "keyed": p["rate"].get("keys").is_some(),
        "strip": frames })
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum FilmEdit {
    Trim {
        layer: String,
        start: f64,
        #[serde(default)]
        end: Option<f64>,
    },
    Rate {
        layer: String,
        rate: f64,
    },
    Fit {
        layer: String,
    },
}

pub fn act(dir: &Path, doc: &Value, edit: &FilmEdit) -> Result<Vec<Op>> {
    let layer = match edit {
        FilmEdit::Trim { layer, .. } | FilmEdit::Rate { layer, .. } | FilmEdit::Fit { layer } => layer,
    };
    let (pointer, found) = crate::actions::layer(doc, layer)?;
    ensure!(
        shown(doc, found).is_some(),
        said("editor.no_video", "the layer \"%1\" does not show a video", &[&layer])
    );
    let base = format!("{pointer}/playback");
    match edit {
        FilmEdit::Rate { rate, .. } => {
            ensure!(
                (RATE[0]..=RATE[1]).contains(rate),
                said("editor.video_rate", "a video plays from 0 to 4 times its speed", &[])
            );
            Ok(set(doc, &format!("{base}/rate"), json!(rate)))
        }
        FilmEdit::Fit { .. } => {
            let size = natural(dir, doc, found).ok_or_else(|| {
                said("editor.video_size", "the size of the video of \"%1\" cannot be read", &[&layer])
            })?;
            Ok(set(doc, &format!("{pointer}/{}", escape("size")), json!(size)))
        }
        FilmEdit::Trim { start, end, .. } => {
            let (_, path) = shown(doc, found).unwrap_or_default();
            let length = wprs_scene::film_file(&dir.join(path)).map(f64::from);
            let top = length.unwrap_or(f64::INFINITY);
            ensure!(
                *start >= 0.0 && *start + GAP <= top && end.is_none_or(|e| e >= start + GAP && e <= top),
                said(
                    "editor.video_trim",
                    "the part of the video must lie inside it and last 0.05 s or more",
                    &[]
                )
            );
            let mut ops = set(doc, &format!("{base}/start"), json!(start));
            match end {
                Some(e) => ops.extend(set(doc, &format!("{base}/end"), json!(e))),
                None if doc.pointer(&format!("{base}/end")).is_some() => {
                    ops.push(Op::Remove { path: format!("{base}/end") })
                }
                None => {}
            }
            Ok(ops)
        }
    }
}
