use anyhow::{Result, anyhow, ensure};
use image::codecs::gif::{GifEncoder, Repeat};
use image::{Delay, Frame, RgbaImage};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::{Component, Path};
use wprs_core::message::said;
use wprs_render::{Renderer, SceneAssets};
use wprs_scene::Scene;

pub const MOST_SECONDS: f64 = 15.0;
pub const MOST_EDGE: u32 = 640;
const MOST_BYTES: usize = 8 << 20;

fn inside(path: &str) -> bool {
    Path::new(path).components().all(|c| matches!(c, Component::Normal(_)))
}

pub fn scened(doc: &Value) -> Result<()> {
    ensure!(
        crate::pages::scened(doc),
        said("editor.shot_scene", "only a scene can be drawn here; choose a picture for other packages", &[])
    );
    Ok(())
}

pub fn poster_time(doc: &Value) -> f64 {
    doc["scene"]["clock"]["poster_time"].as_f64().unwrap_or(0.0)
}

pub fn frames(
    dir: &Path,
    doc: &Value,
    (edge, times): (u32, &[f64]),
    each: impl FnMut(RgbaImage),
) -> Result<()> {
    scened(doc)?;
    let scene = Scene::from_json(&crate::actions::visible(doc).to_string())?;
    let read = |path: &str| match inside(path) {
        true => std::fs::read(dir.join(path)).map_err(|e| format!("{path}: {e}")),
        false => Err(format!("{path} is outside the scene folder")),
    };
    let assets = SceneAssets::read(&scene, read).map_err(|e| anyhow!(e))?;
    let properties: BTreeMap<String, wprs_format::Property> =
        serde_json::from_value(doc["manifest"]["properties"].clone()).unwrap_or_default();
    let props = wprs_scene::effective(&properties, &[]);
    let language = doc["manifest"]["default_locale"].as_str().unwrap_or("en");
    let renderer = Renderer::shared().map_err(|e| {
        said("editor.no_graphics", "there is no graphics device to draw the scene: %1", &[&e])
    })?;
    renderer.stills(&scene, &assets, &props, (0.0, language), (edge, times), each)?;
    Ok(())
}

pub fn still(dir: &Path, doc: &Value, edge: u32, time: f64) -> Result<RgbaImage> {
    let mut found = None;
    frames(dir, doc, (edge, &[time]), |image| found = Some(image))?;
    found.ok_or_else(|| anyhow!(said("editor.shot_none", "the scene drew no picture", &[])))
}

fn encode(images: &[RgbaImage], fps: u32) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    let mut encoder = GifEncoder::new_with_speed(&mut out, 20);
    encoder.set_repeat(Repeat::Infinite)?;
    let delay = Delay::from_numer_denom_ms(1000, fps.max(1));
    for image in images {
        encoder.encode_frame(Frame::from_parts(image.clone(), 0, 0, delay))?;
    }
    drop(encoder);
    Ok(out)
}

pub fn animation(
    dir: &Path,
    doc: &Value,
    (from, seconds, fps): (f64, f64, u32),
    edge: u32,
) -> Result<Vec<u8>> {
    ensure!(
        seconds > 0.0 && seconds <= MOST_SECONDS && (1..=30).contains(&fps),
        said("editor.shot_span", "record from 0.1 to 15 s at 1 to 30 frames a second", &[])
    );
    let count = (seconds * f64::from(fps)).round().max(1.0) as usize;
    let times: Vec<f64> = (0..count).map(|n| from + n as f64 / f64::from(fps)).collect();
    let mut edge = edge.clamp(16, MOST_EDGE);
    loop {
        let mut images = Vec::new();
        frames(dir, doc, (edge, &times), |image| images.push(image))?;
        let gif = encode(&images, fps)?;
        if gif.len() <= MOST_BYTES || edge <= 160 {
            ensure!(
                gif.len() <= MOST_BYTES,
                said("editor.shot_big", "the recording is over 8 MiB; record fewer seconds", &[])
            );
            return Ok(gif);
        }
        edge = edge * 3 / 4;
    }
}
