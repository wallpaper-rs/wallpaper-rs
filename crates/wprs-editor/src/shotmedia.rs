use anyhow::{Result, anyhow, bail, ensure};
use image::RgbaImage;
use image::imageops::FilterType;
use serde_json::Value;
use std::path::{Path, PathBuf};
use wprs_core::message::said;

fn entry(dir: &Path, doc: &Value) -> Result<PathBuf> {
    let name = doc["manifest"]["entry"].as_str().unwrap_or_default();
    wprs_format::check_path(name)?;
    let path = dir.join(name);
    ensure!(path.is_file(), said("editor.not_in_folder", "%1 is not in the folder", &[&name]));
    Ok(path)
}

fn fitted(image: RgbaImage, edge: u32) -> RgbaImage {
    match image.width().max(image.height()) > edge {
        true => image::imageops::resize(
            &image,
            (u64::from(image.width()) * u64::from(edge) / u64::from(image.width().max(image.height()))).max(1)
                as u32,
            (u64::from(image.height()) * u64::from(edge) / u64::from(image.width().max(image.height())))
                .max(1) as u32,
            FilterType::Triangle,
        ),
        false => image,
    }
}

pub fn unsupported(doc: &Value) -> Result<()> {
    if crate::pages::kind(doc) == "web" {
        bail!(said(
            "editor.shot_web",
            "a web page is drawn by the web engine of the daemon, not here; choose a picture for it",
            &[]
        ));
    }
    Ok(())
}

pub fn picture(dir: &Path, doc: &Value, edge: u32) -> Result<RgbaImage> {
    let path = entry(dir, doc)?;
    let image = image::open(&path)
        .map_err(|e| said("editor.unreadable_picture", "the picture cannot be read: %1", &[&e]))?;
    Ok(fitted(image.to_rgba8(), edge))
}

pub fn video(dir: &Path, doc: &Value, edge: u32, time: Option<f64>) -> Result<RgbaImage> {
    let path = entry(dir, doc)?;
    let still = wprs_video::still_at(&path, time)
        .map_err(|e| anyhow!(said("editor.shot_video", "the video cannot be read: %1", &[&e])))?;
    let image = RgbaImage::from_raw(still.width, still.height, still.rgba)
        .ok_or_else(|| anyhow!(said("editor.shot_none", "the scene drew no picture", &[])))?;
    Ok(fitted(image, edge))
}

pub fn preset(
    doc: &Value,
    base: Option<&Path>,
    (edge, times): (u32, Option<&[f64]>),
    each: impl FnMut(RgbaImage),
) -> Result<()> {
    let base = base.ok_or_else(|| {
        anyhow!(said(
            "editor.shot_base",
            "name the installed file of the base package to draw the preset",
            &[]
        ))
    })?;
    let mut package = wprs_format::open(base, &wprs_format::Limits::default())?;
    ensure!(
        package.manifest.kind == wprs_format::Kind::Scene,
        said("editor.shot_base_scene", "only a preset of a scene can be drawn here", &[])
    );
    let scene = package.scene()?;
    let properties = package.manifest.properties.clone();
    let values: wprs_scene::Values = doc["preset"]["values"]
        .as_object()
        .into_iter()
        .flatten()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    let poster = serde_json::to_value(&scene).ok().and_then(|s| s["clock"]["poster_time"].as_f64());
    let second = poster.unwrap_or(0.0);
    let times = times.map_or_else(|| vec![second], <[f64]>::to_vec);
    let package = std::cell::RefCell::new(package);
    let read = |path: &str| package.borrow_mut().read_file(path).map_err(|e| e.to_string());
    let language = doc["manifest"]["default_locale"].as_str().unwrap_or("en");
    let wanted = crate::shots::Wanted { properties: &properties, values: &values, language };
    crate::shots::draw(&scene, wanted, (&read, &read), (edge, &times), each)
}
