use super::{Held, V3, moved, put, r3, swing};
use crate::action::{Grabbed, Handle, PathEdit, PathKey};
use crate::ops::Op;
use anyhow::{Result, anyhow};
use serde_json::Value;
use std::path::Path;
use std::sync::Arc;
use wprs_core::message::said;
use wprs_scene::grip::{roll, turn};
use wprs_scene::{Eye, Quality, Runtime, Values};

pub(super) fn seen(dir: &Path, doc: &Value, t: f64) -> Option<(super::Seen, Eye)> {
    let scene = super::super::store::scene(dir, doc)?;
    let size = scene.canvas.size.map(|v| v as f32);
    let mut rt = Runtime::new(Arc::new(scene), Values::new());
    rt.advance(t.max(0.0));
    let frame = rt.frame([0.0, 0.0, size[0], size[1]], Quality::Q3);
    let cam = frame.stages.first().map(|s| s.eye)?;
    Some((super::super::gizmo::seen_at(doc, &frame, 0)?, cam))
}

pub(super) fn three(v: &Value) -> Option<V3> {
    let a = v.as_array().filter(|a| a.len() == 3)?;
    Some([a[0].as_f64()? as f32, a[1].as_f64()? as f32, a[2].as_f64()? as f32])
}

fn point(h: &Held, p: V3, other: V3, up: V3) -> Result<V3> {
    match h.g.handle {
        Handle::Orbit | Handle::Dolly => Ok(swing(h, p, other, up)),
        _ => moved(h, p),
    }
}

pub(super) fn camera(doc: &Value, h: &Held) -> Result<Vec<Op>> {
    let raw = &doc["scene"]["camera3d"];
    let cam = h.cam;
    let (eye, center) = (three(&raw["eye"]).unwrap_or(cam.at), three(&raw["center"]).unwrap_or(cam.center));
    let up = three(&raw["up"]).unwrap_or(cam.up);
    if h.g.handle == Handle::Roll {
        let d = turn(&h.seen.eye, h.seen.size, cam.at, cam.forward(), h.from, h.to);
        return put(doc, "/scene/camera3d/up", r3(roll(up, cam.forward(), d)), h.g);
    }
    let (field, value) = match h.g.what {
        Grabbed::Center => ("center", point(h, center, eye, up)?),
        _ => ("eye", point(h, eye, center, up)?),
    };
    put(doc, &format!("/scene/camera3d/{field}"), r3(value), h.g)
}

pub(super) fn key(dir: &Path, doc: &Value, h: &Held, path: &str, t: f64, aim: bool) -> Result<Vec<Op>> {
    let paths = doc.pointer("/scene/camera3d/paths").and_then(Value::as_array);
    let keys = paths
        .and_then(|l| l.iter().find(|p| p["id"] == path))
        .and_then(|p| p["keys"].as_array())
        .ok_or_else(|| anyhow!(said("editor.no_path", "there is no camera path \"%1\"", &[&path])))?;
    let k = keys
        .iter()
        .find(|k| (k["t"].as_f64().unwrap_or(f64::NAN) - t).abs() < 1e-3)
        .ok_or_else(|| anyhow!(said("editor.no_path_key", "the path has no key at %1 s", &[&t])))?;
    let (eye, center) = (three(&k["eye"]).unwrap_or(h.cam.at), three(&k["center"]).unwrap_or(h.cam.center));
    let up = three(&k["up"]).unwrap_or(h.cam.up);
    let new = match aim {
        true => point(h, center, eye, up)?,
        false => point(h, eye, center, up)?,
    };
    let f = |v: V3| Some(v.map(f64::from));
    let edit = PathKey {
        path: path.into(),
        time: t,
        from: None,
        eye: if aim { None } else { f(new) },
        center: if aim { f(new) } else { None },
        up: None,
        fov: None,
        from_view: false,
        remove: false,
    };
    super::super::paths::edit(dir, doc, &PathEdit::Key(edit))
}
