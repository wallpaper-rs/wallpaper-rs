#[cfg(test)]
mod tests;

use crate::error::invalid;
use crate::media::{Frame, covers};
use anyhow::{Result, ensure};
use serde_json::Value;
use std::path::{Component, Path};
use wprs_scene::MadeTrack;

fn inside(path: &str) -> bool {
    let p = Path::new(path);
    !path.is_empty() && p.components().all(|c| matches!(c, Component::Normal(_)))
}

pub(crate) fn split(document: &str) -> (String, Option<Value>) {
    if !document.contains("\"media\"") {
        return (document.to_owned(), None);
    }
    let Ok(mut doc) = serde_json::from_str::<Value>(document) else { return (document.to_owned(), None) };
    let media = doc["preview"].as_object_mut().and_then(|p| p.remove("media"));
    match media {
        Some(media) => (doc.to_string(), Some(media)),
        None => (document.to_owned(), None),
    }
}

pub(crate) fn values(document: String) -> (String, Option<crate::sceneprops::Stored>) {
    if !document.contains("\"values\"") {
        return (document, None);
    }
    let Ok(mut doc) = serde_json::from_str::<Value>(&document) else { return (document, None) };
    let Some(tried) = doc["preview"].as_object_mut().and_then(|p| p.remove("values")) else {
        return (document, None);
    };
    let stored = tried
        .as_object()
        .map(|o| o.iter().filter_map(|(k, v)| Some((k.clone(), crate::sceneprops::stored(v)?))).collect());
    (doc.to_string(), stored)
}

pub(crate) fn frame(dir: &Path, media: Value) -> Result<Frame> {
    let track = MadeTrack::read(media).map_err(invalid)?;
    let cover =
        match track.cover.as_deref().filter(|c| !c.is_empty()) {
            None => None,
            Some(path) => {
                ensure!(inside(path), invalid("media.cover is a picture inside the scene folder"));
                let bytes = std::fs::read(dir.join(path)).ok().filter(|b| b.len() as u64 <= covers::MAX_FILE);
                let decoded = bytes.and_then(|b| covers::decode(&b));
                Some(decoded.ok_or_else(|| {
                    invalid(format!("media.cover {path} is not a picture that can be read"))
                })?)
            }
        };
    made(track, cover)
}

pub(crate) fn simulated(track: &str, cover: &[u8]) -> Result<Frame> {
    let track = MadeTrack::read(
        serde_json::from_str(track).map_err(|e| invalid(format!("the simulated media player: {e}")))?,
    )
    .map_err(invalid)?;
    ensure!(track.cover.is_none(), invalid("send the cover as bytes, not as media.cover"));
    ensure!(cover.len() as u64 <= covers::MAX_FILE, invalid("the cover is larger than 16 MiB"));
    let cover = match cover.is_empty() {
        true => None,
        false => Some(
            covers::decode(cover).ok_or_else(|| invalid("the cover is not a picture that can be read"))?,
        ),
    };
    made(track, cover)
}

fn made(track: MadeTrack, cover: Option<std::sync::Arc<covers::Cover>>) -> Result<Frame> {
    let state = track.state(cover.as_ref().map(|c| (c.serial, c.colors))).map_err(invalid)?;
    Ok(Frame { cover, ..Frame::new(state) })
}
