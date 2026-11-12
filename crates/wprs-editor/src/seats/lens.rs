use super::{canvas_of, parsed};
use crate::action::CameraHold;
use serde::Serialize;
use serde_json::Value;
use std::sync::Arc;
use wprs_scene::{Affine, Quality, Runtime, Values};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Frame {
    pub center: [f64; 2],
    pub zoom: f64,
    pub rotation: f64,
    pub corners: [[f64; 2]; 4],
    pub top: [f64; 2],
    pub whole: bool,
}

pub fn whole(scene: &Value) -> bool {
    scene.pointer("/ext/editor/camera2d/whole") == Some(&Value::Bool(true))
}

pub fn lens(doc: &Value, t: f64) -> Option<Affine> {
    doc.pointer("/scene/canvas/camera")?;
    let scene = parsed(&doc["scene"])?;
    let mut rt = Runtime::new(Arc::new(scene), Values::new());
    rt.advance(t.max(0.0));
    let [w, h] = canvas_of(doc).map(|v| v as f32);
    Some(rt.lens([0.0, 0.0, w, h], Quality::Q0))
}

fn back(lens: Affine, p: [f64; 2]) -> [f64; 2] {
    lens.inverse().apply(p.map(|v| v as f32)).map(f64::from)
}

pub fn frame(doc: &Value, t: f64) -> Option<Frame> {
    let lens = lens(doc, t)?;
    let [w, h] = canvas_of(doc);
    let [a, b, ..] = lens.0.map(f64::from);
    let corners = [[0.0, 0.0], [w, 0.0], [w, h], [0.0, h]].map(|c| back(lens, c));
    Some(Frame {
        center: back(lens, [w / 2.0, h / 2.0]),
        zoom: a.hypot(b),
        rotation: b.atan2(a).to_degrees(),
        top: back(lens, [w / 2.0, -h * 0.08]),
        corners,
        whole: whole(&doc["scene"]),
    })
}

fn near(angle: f64, to: f64) -> f64 {
    let turned = angle + ((to - angle) / 360.0).round() * 360.0;
    if turned.abs() > 360.0 { turned - 360.0 * turned.signum() } else { turned }
}

pub struct Held {
    pub center: Option<[f64; 2]>,
    pub zoom: Option<f64>,
    pub rotation: Option<f64>,
}

pub fn held(doc: &Value, hold: &CameraHold, to: [f64; 2], t: f64) -> Held {
    let found = frame(doc, t);
    let [w, h] = canvas_of(doc);
    let (center, rotation) = found.as_ref().map_or(([w / 2.0, h / 2.0], 0.0), |f| (f.center, f.rotation));
    let r = |v: f64, step: f64| (v / step).round() * step;
    let d = [to[0] - center[0], to[1] - center[1]];
    match hold {
        CameraHold::Move => Held { center: Some(to.map(|v| r(v, 0.01))), zoom: None, rotation: None },
        CameraHold::Zoom { corner } => {
            let s = [(corner[0] - 0.5) * w, (corner[1] - 0.5) * h];
            let far = d[0].hypot(d[1]).max(1e-6);
            let zoom = (s[0].hypot(s[1]) / far).clamp(0.1, 10.0);
            Held { center: None, zoom: Some(r(zoom, 1e-4)), rotation: None }
        }
        CameraHold::Turn => {
            let angle = -90.0 - d[1].atan2(d[0]).to_degrees();
            Held { center: None, zoom: None, rotation: Some(r(near(angle, rotation), 0.01)) }
        }
    }
}

pub fn probed(scene: &Value) -> Option<([f32; 2], [f32; 2])> {
    let p = scene.pointer("/ext/editor/probe")?;
    let pair = |k: &str, d: f32| {
        let v = &p[k];
        [0, 1].map(|i| v.get(i).and_then(Value::as_f64).map_or(d, |x| x as f32))
    };
    Some((pair("pointer", 0.5), pair("tilt", 0.0)))
}
