use super::aimed::three;
use super::{Held, V3, moved, put, r3};
use crate::action::LightPlace;
use crate::ops::{Op, set};
use anyhow::{Result, anyhow, ensure};
use serde_json::Value;
use std::path::Path;
use std::sync::Arc;
use wprs_core::message::said;
use wprs_scene::{Beam, Quality, Runtime, Values, lamp3d};

pub(crate) struct Shown {
    pub at: V3,
    pub aim: V3,
    pub beam: Beam,
}

fn light_layer<'a>(doc: &'a Value, id: &str) -> Result<(String, &'a Value)> {
    let (path, layer) = crate::actions::layer(doc, id)?;
    ensure!(layer["type"] == "light", said("editor.not_light", "%1 is not a light", &[&id]));
    Ok((path, layer))
}

pub(crate) fn shown(dir: &Path, doc: &Value, id: &str, t: f64) -> Option<Shown> {
    let scene = super::super::store::scene(dir, doc)?;
    let size = scene.canvas.size.map(|v| v as f32);
    let mut rt = Runtime::new(Arc::new(scene), Values::new());
    rt.advance(t.max(0.0));
    let frame = rt.frame([0.0, 0.0, size[0], size[1]], Quality::Q3);
    let stage = frame.stages.first()?;
    let lamp = frame.lamps.iter().find(|l| l.layer.id == id)?;
    Some(reach(stage, lamp))
}

pub(crate) fn reach(stage: &wprs_scene::Stage3d, lamp: &wprs_scene::Lamp) -> Shown {
    let (at, d) = lamp3d(stage, &stage.eye, lamp);
    let gap: f32 = (0..3).map(|i| (stage.eye.center[i] - at[i]).powi(2)).sum::<f32>().sqrt().max(1.0);
    let aim = lamp.aim.unwrap_or_else(|| std::array::from_fn(|i| at[i] + d[i] * gap));
    Shown { at, aim, beam: lamp.beam }
}

fn missing(id: &str) -> anyhow::Error {
    anyhow!(said("editor.light_unseen", "%1 does not light a 3D view now", &[&id]))
}

pub(super) fn light(dir: &Path, doc: &Value, h: &Held, id: &str, aim: bool) -> Result<Vec<Op>> {
    let (path, raw) = light_layer(doc, id)?;
    let now = shown(dir, doc, id, h.g.time).ok_or_else(|| missing(id))?;
    let field = if aim { "aim3d" } else { "position3d" };
    let from = three(&raw[field]).unwrap_or(if aim { now.aim } else { now.at });
    put(doc, &format!("{path}/{field}"), r3(moved(h, from)?), h.g)
}

fn point(v: [f64; 3]) -> Result<Value> {
    ensure!(
        v.iter().all(|x| x.is_finite() && x.abs() <= 1e6),
        said("editor.light_point", "a point of a light is within a million units", &[])
    );
    Ok(r3(v.map(|x| x as f32)))
}

pub(crate) fn place(dir: &Path, doc: &Value, e: &LightPlace) -> Result<Vec<Op>> {
    let (path, raw) = light_layer(doc, &e.layer)?;
    let gone = |k: &str| raw.get(k).map(|_| Op::Remove { path: format!("{path}/{k}") });
    if e.flat {
        return Ok(gone("position3d").into_iter().chain(gone("aim3d")).collect());
    }
    let mut ops = Vec::new();
    if e.place {
        let now = shown(dir, doc, &e.layer, 0.0).ok_or_else(|| missing(&e.layer))?;
        ops.extend(set(doc, &format!("{path}/position3d"), r3(now.at)));
        if now.beam != Beam::Point {
            ops.extend(set(doc, &format!("{path}/aim3d"), r3(now.aim)));
        }
    }
    for (k, v) in [("position3d", e.position), ("aim3d", e.aim)] {
        if let Some(v) = v {
            ops.extend(set(doc, &format!("{path}/{k}"), point(v)?));
        }
    }
    Ok(ops)
}

pub(crate) fn listed(doc: &Value, seen: &super::Seen, frame: &wprs_scene::Frame) -> Vec<Value> {
    let Some(stage) = frame.stages.first() else { return Vec::new() };
    let px = |p: V3| seen.at(p).map(super::super::gizmo::px);
    let held = |v: &Value| match v {
        Value::Object(_) if v.get("keys").is_some() && v.get("prop").is_none() => "keyed",
        Value::Object(_) => "bound",
        _ => "fixed",
    };
    frame
        .lamps
        .iter()
        .map(|l| {
            let s = reach(stage, l);
            let pointer = crate::actions::layer(doc, &l.layer.id).map(|(p, _)| p).unwrap_or_default();
            let raw = doc.pointer(&pointer).cloned().unwrap_or_default();
            serde_json::json!({ "id": l.layer.id, "pointer": pointer, "light": l.beam, "space": l.space.is_some(),
                "at": px(s.at), "aim": (l.beam != Beam::Point).then(|| px(s.aim)).flatten(),
                "held": held(&raw["position3d"]), "aim_held": held(&raw["aim3d"]) })
        })
        .collect()
}
