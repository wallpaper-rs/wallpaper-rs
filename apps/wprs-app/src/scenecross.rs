use anyhow::{Result, anyhow};
use image::{Rgba, RgbaImage, imageops};
use serde_json::Value;
use std::path::Path;
use wprs_core::Fit;
use wprs_editor::passing::{Way, readied};
use wprs_render::{Layer, Renderer};
use wprs_scene::Quality;

const PLAIN: Rgba<u8> = Rgba([16, 20, 34, 255]);

pub fn wanted(spec: &Value) -> bool {
    spec["crossing"].is_object()
}

fn other(file: &str, size: (u32, u32)) -> RgbaImage {
    let picture = image::open(file).ok().map(|p| p.to_rgba8());
    match picture {
        Some(p) => imageops::resize(&p, size.0, size.1, imageops::FilterType::Triangle),
        None => RgbaImage::from_pixel(size.0, size.1, PLAIN),
    }
}

fn fade(a: &RgbaImage, b: &RgbaImage, t: f32) -> RgbaImage {
    let mut out = a.clone();
    for (o, q) in out.pixels_mut().zip(b.pixels()) {
        for i in 0..4 {
            o[i] = (o[i] as f32 + (q[i] as f32 - o[i] as f32) * t).round() as u8;
        }
    }
    out
}

fn layer(image: &wprs_render::GpuImage) -> Layer<'_> {
    Layer {
        image: Some(image),
        fit: Fit::Stretch,
        background: [0; 3],
        opacity: 1.0,
        uv: None,
        relief: None,
        flip: [false; 2],
        grade: None,
    }
}

pub fn cross(dir: &Path, doc: &Value, out: &Path, spec: &Value) -> Result<String> {
    let c = &spec["crossing"];
    let way = if c["way"] == "out" { Way::Out } else { Way::In };
    let progress = c["progress"].as_f64().unwrap_or(0.5).clamp(0.0, 1.0) as f32;
    let quality: Quality = serde_json::from_value(spec["quality"].clone()).unwrap_or_default();
    let still = image::open(c["still"].as_str().unwrap_or_default())?.to_rgba8();
    let size = still.dimensions();
    let elsewhere = other(c["before"].as_str().unwrap_or_default(), size);
    let (from, to) = match way {
        Way::In => (elsewhere, still),
        Way::Out => (still, elsewhere),
    };
    let value = doc.pointer(&wprs_editor::passing::path(way)).cloned().unwrap_or_default();
    let ready = readied(dir, way, &value)?.map_err(|why| anyhow!(wprs_editor::passing::words(&why)))?;
    let picture = if spec["calm"] == true || !ready.plays_at(quality) {
        fade(&from, &to, progress)
    } else {
        let r = Renderer::shared()?;
        let mut crossing = r.crossing(ready, size, quality).map_err(anyhow::Error::msg)?;
        let (a, b) = (r.upload(&from, Fit::Stretch, size)?, r.upload(&to, Fit::Stretch, size)?);
        r.cross(&mut crossing, &[layer(&a)], &[layer(&b)], progress)?;
        r.read_image(crossing.image())?
    };
    wprs_render::save(&picture, out, 95)?;
    Ok(out.to_string_lossy().into_owned())
}
