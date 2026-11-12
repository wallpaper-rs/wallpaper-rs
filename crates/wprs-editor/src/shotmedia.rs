use anyhow::{Result, anyhow, bail, ensure};
use image::RgbaImage;
use image::imageops::FilterType;
use serde_json::{Value, json};
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
            "a web page is drawn by the web engine of the daemon, not here: use wprs-ctl scene thumbnail with --capture, or the capture button of the app",
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
    match package.manifest.kind {
        wprs_format::Kind::Scene => {}
        wprs_format::Kind::Image | wprs_format::Kind::Video => {
            return media_base(doc, &mut package, (edge, times), each);
        }
        _ => bail!(said(
            "editor.shot_base_scene",
            "only a preset of a scene, a picture or a video can be drawn here; a web page is drawn with wprs-ctl scene thumbnail --capture",
            &[]
        )),
    }
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

fn number(doc: &Value, key: &str) -> Option<f64> {
    doc["preset"]["values"][key].as_f64()
}

pub(crate) fn framed(image: RgbaImage, zoom: f64, (x, y): (f64, f64)) -> RgbaImage {
    if zoom <= 1.0 {
        return image;
    }
    let (w, h) = (f64::from(image.width()), f64::from(image.height()));
    let (cw, ch) = ((w / zoom).max(1.0), (h / zoom).max(1.0));
    let center = |size: f64, window: f64, at: f64| {
        (size / 2.0 + at.clamp(-1.0, 1.0) * (zoom - 1.0) * size / (2.0 * zoom) - window / 2.0)
            .clamp(0.0, size - window)
    };
    let (left, top) = (center(w, cw, x), center(h, ch, y));
    image::imageops::crop_imm(&image, left as u32, top as u32, cw as u32, ch as u32).to_image()
}

fn media_base<R: std::io::Read + std::io::Seek>(
    doc: &Value,
    package: &mut wprs_format::Package<R>,
    (edge, times): (u32, Option<&[f64]>),
    mut each: impl FnMut(RgbaImage),
) -> Result<()> {
    let dir = std::env::temp_dir().join(format!("wprs-base-{}-{}", std::process::id(), package.manifest.id));
    std::fs::remove_dir_all(&dir).ok();
    let drawn = (|| {
        wprs_format::extract(package, &dir)?;
        let entry = json!(package.manifest.entry);
        let base = json!({"manifest": {"entry": entry}});
        let zoom = number(doc, "zoom").unwrap_or(1.0);
        let at = (number(doc, "x").unwrap_or(0.0), number(doc, "y").unwrap_or(0.0));
        if package.manifest.kind != wprs_format::Kind::Video {
            let one = times.is_none_or(|t| t.len() == 1);
            ensure!(one, said("editor.shot_still", "a picture does not move: keep a frame instead", &[]));
            each(fitted(framed(picture(&dir, &base, u32::MAX)?, zoom, at), edge));
            return Ok(());
        }
        let start = number(doc, "start").unwrap_or(0.0);
        let moments: Vec<Option<f64>> = match times {
            Some(list) => list.iter().map(|t| Some(start + t)).collect(),
            None => vec![number(doc, "start")],
        };
        for moment in moments {
            each(fitted(framed(video(&dir, &base, u32::MAX, moment)?, zoom, at), edge));
        }
        Ok(())
    })();
    std::fs::remove_dir_all(&dir).ok();
    drawn
}
