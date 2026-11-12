use crate::seats::{Seat, canvas_of};
use anyhow::{Result, ensure};
use serde_json::{Value, json};
use std::path::Path;
use wprs_core::message::said;

const BOX: [[f64; 2]; 4] = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
const HOLDS: [[f64; 2]; 8] =
    [[0.0, 0.0], [0.5, 0.0], [1.0, 0.0], [1.0, 0.5], [1.0, 1.0], [0.5, 1.0], [0.0, 1.0], [0.0, 0.5]];

fn zone_of(seat: &Seat, size: [f64; 2], u: [f64; 2]) -> [f64; 2] {
    let at = seat.position();
    seat.canvas([0, 1].map(|i| at[i] + (u[i] - seat.origin[i]) * size[i]))
}

pub(crate) fn resize(seat: &Seat, canvas: [f64; 2], corner: [f64; 2], to: [f64; 2]) -> Result<[f64; 2]> {
    ensure!(
        seat.kind == "adjustment" && seat.size.iter().all(|s| *s > 0.0),
        said("editor.no_capture", "the layer %1 captures no region", &[&seat.id])
    );
    let (at, q) = (seat.position(), seat.local(to));
    let size = [0, 1].map(|i| {
        let arm = corner[i] - seat.origin[i];
        if arm.abs() < 1e-6 {
            return seat.size[i];
        }
        ((q[i] - at[i]) / arm).clamp(1.0, canvas[i] * 2.0).round()
    });
    Ok(size)
}

pub(crate) fn view(dir: &Path, doc: &Value, id: &str, t: f64) -> Result<Value> {
    let seat = crate::seats::seat(dir, doc, id, t).filter(|s| s.kind == "adjustment");
    let seat = seat.ok_or_else(|| anyhow::anyhow!("no capture"))?;
    let (_, layer) = crate::actions::layer(doc, id)?;
    ensure!(layer["size"].is_array(), "no region");
    let canvas = canvas_of(doc);
    let max = [canvas[0] * 2.0, canvas[1] * 2.0];
    let over = seat.size.iter().zip(max).any(|(s, m)| *s > m + 0.5);
    let redraw = layer["redraw"].as_bool().unwrap_or(false);
    Ok(json!({
        "size": seat.size, "max": max, "over": over, "redraw": redraw,
        "zone": BOX.map(|u| zone_of(&seat, seat.size, u)),
        "limit": BOX.map(|u| zone_of(&seat, max, u)),
        "drawn": redraw.then(|| seat.corners()),
        "handles": HOLDS.map(|u| json!({ "corner": u, "at": zone_of(&seat, seat.size, u) })),
    }))
}
