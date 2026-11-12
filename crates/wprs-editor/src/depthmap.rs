use crate::action::DepthMapEdit;
use crate::layers::unique;
use crate::ops::{Op, escape, set};
use anyhow::{Result, anyhow, ensure};
use image::{Rgba, RgbaImage};
use serde_json::{Value, json};
use std::path::Path;
use wprs_core::edit::Depth;
use wprs_core::message::said;
use wprs_depth::{Device, Estimator, MODEL, Map, Places, threads};

const INSTALL: &str = "wprs-ctl models download depth-anything-v2-small";

pub fn state(places: &Places) -> Value {
    let installed = places.installed(&MODEL).is_some();
    let runtime = !places.library(None).is_empty();
    json!({ "installed": installed, "runtime": runtime, "ready": installed && runtime, "install": INSTALL })
}

pub fn grey(map: &Map) -> RgbaImage {
    RgbaImage::from_fn(map.width(), map.height(), |x, y| {
        let v = ((u32::from(map.get_pixel(x, y).0[0]) + 128) / 257) as u8;
        Rgba([v, v, v, 255])
    })
}

fn estimate(places: &Places, picture: &image::RgbImage, input: u32) -> Result<Map> {
    let model = places.installed(&MODEL).ok_or_else(|| {
        anyhow!(said(
            "editor.depth_model_missing",
            "the depth model is not installed: run %1, or install it from the depth dialog of the app",
            &[&INSTALL]
        ))
    })?;
    let libraries = places.library(None);
    ensure!(
        !libraries.is_empty(),
        said(
            "editor.depth_runtime_missing",
            "the library that runs the depth model is not installed: run wprs-ctl models download onnxruntime",
            &[]
        )
    );
    Estimator::open(&model, &libraries, threads(), Device::Auto)?.estimate(picture, input)
}

pub(crate) fn make(dir: &Path, doc: &Value, edit: &DepthMapEdit) -> Result<Vec<Op>> {
    let input = edit.input.unwrap_or_else(|| Depth::default().input);
    ensure!(
        (196..=1036).contains(&input) && input.is_multiple_of(14),
        said("editor.depth_input", "the model input is a multiple of 14 from 196 to 1036", &[])
    );
    ensure!(
        doc.pointer(&format!("/scene/assets/{}", escape(&edit.asset))).is_some(),
        said("editor.depth_asset", "%1 is not a picture of the scene", &[&edit.asset])
    );
    if let Some(pointer) = &edit.pointer {
        ensure!(
            pointer.starts_with("/scene/") && pointer.contains("/effects/") && pointer.contains("/params/"),
            said("editor.ramp_param", "a ramp goes to a parameter of an effect", &[])
        );
    }
    let source = crate::pictures::asset(dir, doc, &edit.asset).map_err(|_| {
        anyhow!(said("editor.depth_asset", "%1 is not a picture of the scene", &[&edit.asset]))
    })?;
    let map = estimate(&Places::default(), &source.into_rgb8(), input)?;
    let id = match &edit.id {
        Some(id) => id.clone(),
        None => unique(doc, &format!("{}_depth", edit.asset)),
    };
    let path = crate::pictures::store(dir, ("maps", &id), &grey(&map), None)?;
    let mut ops =
        set(doc, &format!("/scene/assets/{}", escape(&id)), json!({ "path": path, "linear": true }));
    if let Some(pointer) = &edit.pointer {
        ops.extend(set(doc, pointer, json!(id)));
    }
    Ok(ops)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_map_becomes_a_grey_picture_with_white_near() {
        let map = Map::from_vec(3, 1, vec![0, 32896, 65535]).unwrap();
        let picture = grey(&map);
        let values: Vec<u8> = picture.pixels().map(|p| p.0[0]).collect();
        assert_eq!(values, vec![0, 128, 255]);
        assert!(picture.pixels().all(|p| p.0[0] == p.0[1] && p.0[1] == p.0[2] && p.0[3] == 255));
    }
}
