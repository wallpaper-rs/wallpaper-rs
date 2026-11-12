use crate::actions::locked;
use crate::keys::value_at;
use crate::seats::boxes::pair;
use crate::seats::{Seat, seats};
use serde::Serialize;
use serde_json::Value;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Gizmo {
    pub id: String,
    pub pointer: String,
    pub offset: [f64; 2],
    pub position: [f64; 2],
    pub size: [f64; 2],
    pub rotation: f64,
    pub scale: [f64; 2],
    pub skew: [f64; 2],
    pub origin: [f64; 2],
    pub tilt: [f64; 2],
    pub z: f64,
    pub corners: Option<[[f64; 2]; 4]>,
    pub outline: [[f64; 2]; 4],
    pub pivot: [f64; 2],
    pub angle: f64,
    pub extent: [f64; 2],
    pub unit: [f64; 6],
    pub parent: Option<String>,
    pub locked: bool,
    pub kind: String,
}

fn length(a: [f64; 2], b: [f64; 2]) -> f64 {
    (b[0] - a[0]).hypot(b[1] - a[1])
}

pub fn gizmo(seat: &Seat, doc: &Value, t: f64, locked: bool) -> Gizmo {
    let layer = doc.pointer(&seat.pointer).unwrap_or(&Value::Null);
    let transform = &layer["transform"];
    let outline = seat.outline();
    let center = seat.flat([0.5, 0.5]);
    let position = seat.position();
    let leaned = seat.lean.is_some() && seat.corners() != outline;
    Gizmo {
        id: seat.id.clone(),
        pointer: seat.pointer.clone(),
        offset: [center[0] - position[0], center[1] - position[1]],
        position,
        size: seat.size,
        rotation: value_at(&transform["rotation"], t).as_f64().unwrap_or(0.0),
        scale: pair(&value_at(&transform["scale"], t), [1.0; 2]),
        skew: pair(&value_at(&transform["skew"], t), [0.0; 2]),
        origin: seat.origin,
        tilt: pair(&value_at(&transform["tilt"], t), [0.0; 2]),
        z: value_at(&transform["z"], t).as_f64().unwrap_or(0.0),
        corners: leaned.then(|| seat.corners()),
        pivot: seat.pivot(),
        angle: (outline[1][1] - outline[0][1]).atan2(outline[1][0] - outline[0][0]).to_degrees(),
        extent: [length(outline[0], outline[1]), length(outline[0], outline[3])],
        unit: seat.unit().0.map(f64::from),
        parent: seat.parent.clone(),
        outline,
        locked,
        kind: seat.kind.clone(),
    }
}

pub fn gizmos(dir: &Path, doc: &Value, t: f64) -> Vec<Gizmo> {
    let locked = locked(doc);
    let all = seats(dir, doc, t);
    let shown = all.iter().filter(|s| s.kind != "sound");
    shown.map(|s| gizmo(s, doc, t, locked.contains(&s.id))).collect()
}
