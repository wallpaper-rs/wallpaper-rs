use crate::action::CanvasEdit as C;
use crate::ops::{Op, set};
use crate::seats::lens::{Held, held};
use anyhow::{Result, ensure};
use serde_json::{Value, json};
use std::path::Path;
use wprs_core::message::said;

const CAMERA: &str = "/scene/canvas/camera";

fn written(doc: &Value, h: Held, at: Option<f64>, replace: bool) -> Result<Vec<Op>> {
    let ok = at.is_none_or(|t| t.is_finite() && t >= 0.0);
    ensure!(ok, said("editor.time_must_more", "the time must be 0 or more seconds", &[]));
    if let Some(z) = h.zoom {
        ensure!(
            (0.1..=10.0).contains(&z),
            said("editor.camera_zoom", "the zoom of the camera is 0.1 to 10", &[])
        );
    }
    if let Some(r) = h.rotation {
        ensure!(
            (-360.0..=360.0).contains(&r),
            said("editor.camera_turn", "the camera turns −360 to 360 degrees", &[])
        );
    }
    let mut work = doc.clone();
    if !work.pointer(CAMERA).is_some_and(Value::is_object) {
        work["scene"]["canvas"]["camera"] = json!({});
    }
    let fields = [
        ("center", h.center.map(|c| json!(c))),
        ("zoom", h.zoom.map(|z| json!(z))),
        ("rotation", h.rotation.map(|r| json!(r))),
    ];
    for (k, v) in fields.into_iter().filter_map(|(k, v)| Some((k, v?))) {
        super::keyed::write(&mut work, &format!("{CAMERA}/{k}"), v, at, replace)?;
    }
    Ok(set(doc, CAMERA, work.pointer(CAMERA).cloned().unwrap_or_default()))
}

pub(super) fn edit(dir: &Path, doc: &Value, e: &C) -> Result<Vec<Op>> {
    let ops = match e {
        C::Camera { off: true, .. } => super::effects::put(doc, CAMERA, None),
        C::Camera { center, zoom, rotation, at, replace, .. } => {
            written(doc, Held { center: *center, zoom: *zoom, rotation: *rotation }, *at, *replace)?
        }
        C::CameraGrip { hold, to, at, replace } => {
            written(doc, held(doc, hold, *to, at.unwrap_or(0.0)), *at, *replace)?
        }
        C::CameraView { whole: true } => set(doc, "/scene/ext/editor/camera2d", json!({ "whole": true })),
        C::CameraView { .. } => super::effects::put(doc, "/scene/ext/editor/camera2d", None),
        _ => Vec::new(),
    };
    super::refused(dir, doc, ops)
}
