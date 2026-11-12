use crate::layers::asset_kind;
use crate::ops::{Op, escape};
use anyhow::{Result, anyhow, ensure};
use serde_json::{Value, json};
use std::path::Path;
use wprs_core::message::said;
use wprs_scene::MAX_FRAMES;

use crate::cells::inside;
pub use crate::cells::{Order, cells};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum SpriteEdit {
    Grid {
        asset: String,
        rows: u32,
        columns: u32,
        #[serde(default)]
        margin: u32,
        #[serde(default)]
        spacing: u32,
        #[serde(default)]
        count: Option<u32>,
        #[serde(default)]
        order: Order,
    },
    Rects {
        asset: String,
        rects: Vec<[u32; 4]>,
    },
    Add {
        asset: String,
        rect: [u32; 4],
    },
    Change {
        asset: String,
        index: usize,
        rect: [u32; 4],
    },
    Remove {
        asset: String,
        index: usize,
    },
    Reorder {
        asset: String,
        from: usize,
        to: usize,
    },
    Play {
        asset: String,
        #[serde(default)]
        fps: Option<f32>,
        #[serde(default)]
        frames: Option<u32>,
    },
    Clear {
        asset: String,
    },
}

type Sheet = (String, Vec<[u32; 4]>, (u32, u32));

fn picture(dir: &Path, doc: &Value, id: &str) -> Result<Sheet> {
    let path = doc["scene"]["assets"][id]["path"].as_str().filter(|p| asset_kind(p) == "image");
    let path =
        path.ok_or_else(|| anyhow!(said("editor.sheet_picture_of", "\"%1\" is not a picture", &[&id])))?;
    let size = crate::pictures::dimensions(dir.join(path))
        .map_err(|e| said("editor.unreadable_picture", "the picture cannot be read: %1", &[&e]))?;
    let rects = serde_json::from_value(doc["scene"]["assets"][id]["rects"].clone()).unwrap_or_default();
    Ok((format!("/scene/assets/{}", escape(id)), rects, size))
}

fn put(doc: &Value, base: &str, rects: Vec<[u32; 4]>, size: (u32, u32)) -> Result<Vec<Op>> {
    ensure!(
        rects.len() <= MAX_FRAMES as usize,
        said("editor.sprite_count", "a sprite sheet has at most %1 frames", &[&MAX_FRAMES])
    );
    rects.iter().try_for_each(|r| inside(*r, size))?;
    let mut ops = Vec::new();
    let dropped: &[&str] =
        if rects.is_empty() { &["atlas", "frames", "fps", "durations"] } else { &["atlas", "frames"] };
    for key in dropped {
        if doc.pointer(&format!("{base}/{key}")).is_some() {
            ops.push(Op::Remove { path: format!("{base}/{key}") });
        }
    }
    let path = format!("{base}/rects");
    let value = json!(rects);
    ops.push(if rects.is_empty() {
        Op::Remove { path }
    } else if doc.pointer(&path).is_some() {
        Op::Replace { path, value }
    } else {
        Op::Add { path, value }
    });
    Ok(ops)
}

pub fn act(dir: &Path, doc: &Value, edit: &SpriteEdit) -> Result<Vec<Op>> {
    let asset = match edit {
        SpriteEdit::Grid { asset, .. }
        | SpriteEdit::Rects { asset, .. }
        | SpriteEdit::Add { asset, .. }
        | SpriteEdit::Change { asset, .. }
        | SpriteEdit::Remove { asset, .. }
        | SpriteEdit::Reorder { asset, .. }
        | SpriteEdit::Play { asset, .. }
        | SpriteEdit::Clear { asset } => asset,
    };
    let (base, mut rects, size) = picture(dir, doc, asset)?;
    let missing = |i: usize| said("editor.sprite_none", "there is no frame %1", &[&i]);
    let ops = match edit {
        SpriteEdit::Grid { rows, columns, margin, spacing, count, order, .. } => {
            let mut all = cells(size, *rows, *columns, *margin, *spacing, *order)?;
            all.truncate(count.map_or(all.len(), |n| (n as usize).clamp(1, all.len())));
            put(doc, &base, all, size)?
        }
        SpriteEdit::Rects { rects, .. } => put(doc, &base, rects.clone(), size)?,
        SpriteEdit::Add { rect, .. } => {
            rects.push(*rect);
            put(doc, &base, rects, size)?
        }
        SpriteEdit::Change { index, rect, .. } => {
            *rects.get_mut(*index).ok_or_else(|| missing(*index))? = *rect;
            put(doc, &base, rects, size)?
        }
        SpriteEdit::Remove { index, .. } => {
            ensure!(*index < rects.len(), missing(*index));
            rects.remove(*index);
            put(doc, &base, rects, size)?
        }
        SpriteEdit::Reorder { from, to, .. } => {
            ensure!(*from < rects.len(), missing(*from));
            let moved = rects.remove(*from);
            rects.insert((*to).min(rects.len()), moved);
            put(doc, &base, rects, size)?
        }
        SpriteEdit::Clear { .. } => put(doc, &base, Vec::new(), size)?,
        SpriteEdit::Play { fps, frames, .. } => play(doc, &base, rects.len(), *fps, *frames)?,
    };
    crate::shaping::refused(dir, doc, ops)
}

fn play(doc: &Value, base: &str, cells: usize, fps: Option<f32>, frames: Option<u32>) -> Result<Vec<Op>> {
    ensure!(
        fps.is_none_or(|f| (0.1..=120.0).contains(&f)),
        said("editor.sprite_fps", "a sprite sheet plays from 0.1 to 120 frames a second", &[])
    );
    ensure!(
        frames.is_none_or(|n| n >= 1 && (cells == 0 || n as usize <= cells)),
        said("editor.sprite_frames", "a sprite sheet plays between 1 frame and all its frames", &[])
    );
    let mut ops = Vec::new();
    for (key, value) in [("fps", fps.map(|f| json!(f))), ("frames", frames.map(|n| json!(n)))] {
        let Some(value) = value else { continue };
        let path = format!("{base}/{key}");
        ops.push(if doc.pointer(&path).is_some() {
            Op::Replace { path, value }
        } else {
            Op::Add { path, value }
        });
    }
    Ok(ops)
}

pub fn sheet(dir: &Path, doc: &Value, id: &str) -> Value {
    let Ok((base, rects, size)) = picture(dir, doc, id) else { return Value::Null };
    let a = doc.pointer(&base).cloned().unwrap_or_default();
    json!({ "asset": id, "file": dir.join(a["path"].as_str().unwrap_or_default()), "size": [size.0, size.1],
        "rects": rects, "atlas": a["atlas"], "fps": a["fps"], "frames": a["frames"], "durations": a["durations"],
        "most": MAX_FRAMES })
}
