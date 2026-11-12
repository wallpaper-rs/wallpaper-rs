use crate::action::ShotEdit;
use crate::ops::Op;
use anyhow::Result;
use serde_json::Value;
use std::fs;
use std::path::Path;

const STILL: u32 = 1024;
const MOTION: u32 = 480;

pub fn file(dir: &Path, doc: &Value, e: &ShotEdit) -> Result<Option<(String, bool)>> {
    let part = dir.join("assets").join(".shot.part");
    let animated = match e {
        ShotEdit::Flashes { from, seconds, fps } => {
            let span = (from.unwrap_or(0.0), seconds.unwrap_or(8.0), fps.unwrap_or(15));
            crate::flashes::measure(dir, doc, span)?;
            return Ok(None);
        }
        ShotEdit::Still { time } => {
            let image =
                crate::shots::still(dir, doc, STILL, time.unwrap_or_else(|| crate::shots::poster_time(doc)))?;
            fs::create_dir_all(dir.join("assets"))?;
            image.save_with_format(&part, image::ImageFormat::Png)?;
            false
        }
        ShotEdit::Recording { from, seconds, fps } => {
            let span = (from.unwrap_or(0.0), seconds.unwrap_or(4.0), fps.unwrap_or(10));
            let gif = crate::shots::animation(dir, doc, span, MOTION)?;
            fs::create_dir_all(dir.join("assets"))?;
            fs::write(&part, gif)?;
            true
        }
    };
    let stored = crate::previews::store(dir, doc, &part, animated);
    fs::remove_file(&part).ok();
    Ok(Some((stored?, animated)))
}

pub fn act(dir: &Path, doc: &Value, e: &ShotEdit) -> Result<Vec<Op>> {
    match file(dir, doc, e)? {
        Some((path, animated)) => crate::previews::adopt(doc, animated, &path),
        None => Ok(Vec::new()),
    }
}
