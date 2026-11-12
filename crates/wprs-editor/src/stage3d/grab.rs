use super::gizmo::{Seen, seen_at};
use super::turns::{angles, around, euler, mul, of_mat4, pure};
use crate::action::{Grab, Grabbed, Handle, KeyframeEdit};
use crate::ops::{Op, set};
use anyhow::{Result, anyhow, ensure};
use serde_json::{Value, json};
use std::path::Path;
use std::sync::Arc;
use wprs_core::message::said;
use wprs_scene::grip::{along, deepen, dolly, orbit, slide, turn};
use wprs_scene::{Quality, Runtime, Values};

mod aimed;
mod lit;

pub(crate) use lit::{listed, place};

pub(super) type V3 = [f32; 3];

pub(super) struct Held<'a> {
    pub g: &'a Grab,
    pub seen: Seen,
    pub cam: wprs_scene::Eye,
    pub from: [f32; 2],
    pub to: [f32; 2],
}

pub(super) fn r3(v: V3) -> Value {
    json!(v.map(|x| (f64::from(x) * 1000.0).round() / 1000.0))
}

pub(super) fn put(doc: &Value, pointer: &str, value: Value, g: &Grab) -> Result<Vec<Op>> {
    let held = doc.pointer(pointer).and_then(Value::as_object);
    let keyed = held.is_some_and(|o| {
        o.contains_key("keys") && o.keys().all(|k| ["keys", "timeline", "loop"].contains(&k.as_str()))
    });
    if held.is_none() || g.replace {
        return Ok(set(doc, pointer, value));
    }
    ensure!(
        keyed,
        said(
            "editor.grab_bound",
            "%1 follows a binding: change the binding or replace it with the value",
            &[&pointer]
        )
    );
    let k = KeyframeEdit { value: Some(value), ..KeyframeEdit::at(pointer, g.time) };
    crate::keys::keyframe(doc, &k)
}

pub(super) fn moved(h: &Held, p: V3) -> Result<V3> {
    let (eye, size, g) = (&h.seen.eye, h.seen.size, h.g);
    let axis = |k: usize| std::array::from_fn(|i| f32::from(u8::from(i == k)));
    Ok(match g.handle {
        Handle::X | Handle::Y | Handle::Z => {
            let k = g.handle as usize;
            let s = along(eye, size, p, axis(k), h.from, h.to);
            std::array::from_fn(|i| p[i] + if i == k { s } else { 0.0 })
        }
        Handle::Plane => slide(eye, size, p, h.from, h.to),
        Handle::Depth => deepen(eye, size, p, h.from, h.to),
        _ => return Err(anyhow!(said("editor.grab_handle", "this handle does not move a point", &[]))),
    })
}

fn model(dir: &Path, doc: &Value, g: &Grab, layer: &str) -> Result<Vec<Op>> {
    let (path, raw) = super::edit::model_layer(doc, layer)?;
    let scene = super::store::scene(dir, doc).ok_or_else(|| anyhow!("no scene"))?;
    let size = scene.canvas.size.map(|v| v as f32);
    let mut rt = Runtime::new(Arc::new(scene), Values::new());
    rt.advance(g.time.max(0.0));
    let frame = rt.frame([0.0, 0.0, size[0], size[1]], Quality::Q3);
    let p =
        frame.placed.iter().find(|p| p.layer.id == layer).ok_or_else(|| {
            anyhow!(said("editor.view_frame", "%1 is not a model that is shown", &[&layer]))
        })?;
    let seen = seen_at(doc, &frame, p.stage).ok_or_else(|| anyhow!("no stage"))?;
    let now = super::turns::now(p);
    let three = |k: &str| -> V3 {
        let v = raw[k].as_array().filter(|a| a.len() == 3).map(|a| a.iter().map(|x| x.as_f64()).collect());
        match v.and_then(|v: Option<Vec<f64>>| v) {
            Some(v) => [v[0] as f32, v[1] as f32, v[2] as f32],
            None => serde_json::from_value(now[k].clone()).unwrap_or([0.0; 3]),
        }
    };
    let c = super::gizmo::world(&p.matrix, p.model.middle());
    let px = |p: [f64; 2]| seen.back(p.map(|v| v as f32));
    let cam = frame.stages.get(p.stage).map_or(seen.eye, |s| s.eye);
    let held = Held { from: px(g.from), to: px(g.to), seen, g, cam };
    let (field, value) = match g.handle {
        Handle::TurnX | Handle::TurnY | Handle::TurnZ => {
            let k = g.handle as usize - Handle::TurnX as usize;
            let a: V3 = std::array::from_fn(|i| f32::from(u8::from(i == k)));
            let d = turn(&held.seen.eye, held.seen.size, c, a, held.from, held.to);
            let base = match raw["rotation"].is_array() {
                true => euler(three("rotation")),
                false => pure(&of_mat4(&p.matrix)),
            };
            ("rotation", json!(angles(&mul(&around(a, d), &base))))
        }
        _ => {
            let at = three("position");
            let to = moved(&held, c)?;
            ("position", r3(std::array::from_fn(|i| at[i] + to[i] - c[i])))
        }
    };
    put(doc, &format!("{path}/{field}"), value, g)
}

fn held<'a>(dir: &Path, doc: &Value, g: &'a Grab) -> Result<Held<'a>> {
    let (seen, cam) = aimed::seen(dir, doc, g.time)
        .ok_or_else(|| anyhow!(said("editor.grab_stage", "the 3D view shows nothing to drag on", &[])))?;
    let px = |p: [f64; 2]| seen.back(p.map(|v| v as f32));
    Ok(Held { g, from: px(g.from), to: px(g.to), seen, cam })
}

pub(crate) fn grab(dir: &Path, doc: &Value, g: &Grab) -> Result<Vec<Op>> {
    ensure!(
        g.from.iter().chain(&g.to).all(|v| v.is_finite()) && g.time.is_finite(),
        said("editor.grab_points", "a drag goes between two points on the view", &[])
    );
    match &g.what {
        Grabbed::Model { layer } => model(dir, doc, g, layer),
        Grabbed::Eye | Grabbed::Center => aimed::camera(doc, &held(dir, doc, g)?),
        Grabbed::Key { path, key, aim } => aimed::key(dir, doc, &held(dir, doc, g)?, path, *key, *aim),
        Grabbed::Light { layer, aim } => lit::light(dir, doc, &held(dir, doc, g)?, layer, *aim),
    }
}

pub(super) fn swing(h: &Held, at: V3, center: V3, up: V3) -> V3 {
    let (dx, dy) = (h.to[0] - h.from[0], h.to[1] - h.from[1]);
    match h.g.handle {
        Handle::Orbit => orbit(at, center, up, [-dx * 0.4, -dy * 0.4]),
        _ => dolly(at, center, h.seen.size, h.from, h.to),
    }
}
