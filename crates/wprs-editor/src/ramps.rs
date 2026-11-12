use crate::action::{EDITOR, RampEdit, RampFormat, RampStop};
use crate::layers::unique;
use crate::ops::{Op, escape, set};
use anyhow::{Result, ensure};
use image::{Rgba, RgbaImage};
use serde_json::{Value, json};
use std::ops::RangeInclusive;
use std::path::Path;
use wprs_core::message::said;

pub const MOST: usize = 16;
const WIDTHS: RangeInclusive<u32> = 16..=1024;
const SIZE: usize = 17;

type Stops = Vec<(f32, [f32; 4])>;

fn parsed(list: &[RampStop]) -> Result<Stops> {
    ensure!(
        (1..=MOST).contains(&list.len()),
        said("editor.ramp_stops", "a ramp has 1 to %1 stops", &[&MOST])
    );
    let mut out = Stops::new();
    for s in list {
        ensure!(
            s.at.is_finite() && (0.0..=1.0).contains(&s.at),
            said("editor.ramp_at", "a stop sits between 0 and 1", &[])
        );
        let color = serde_json::from_value::<wprs_scene::Color>(json!(s.color))
            .map_err(|_| anyhow::anyhow!(said("editor.ramp_color", "%1 is not a color", &[&s.color])))?;
        out.push((s.at, color.0));
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    Ok(out)
}

pub fn at(stops: &Stops, t: f32) -> [f32; 4] {
    let after = stops.iter().position(|s| s.0 > t);
    match after {
        None => stops.last().map_or([0.0; 4], |s| s.1),
        Some(0) => stops[0].1,
        Some(k) => {
            let ((t0, a), (t1, b)) = (stops[k - 1], stops[k]);
            let u = ((t - t0) / (t1 - t0).max(1e-6)).clamp(0.0, 1.0);
            std::array::from_fn(|c| a[c] + (b[c] - a[c]) * u)
        }
    }
}

fn byte(v: f32) -> u8 {
    (v.clamp(0.0, 1.0) * 255.0).round() as u8
}

pub fn strip(stops: &Stops, width: u32) -> RgbaImage {
    let last = (width - 1).max(1) as f32;
    RgbaImage::from_fn(width, 1, |x, _| Rgba(at(stops, x as f32 / last).map(byte)))
}

pub fn table(stops: &Stops) -> String {
    let mut out = format!("TITLE \"ramp\"\nLUT_3D_SIZE {SIZE}\n");
    let level = |k: usize| k as f32 / (SIZE - 1) as f32;
    for b in 0..SIZE {
        for g in 0..SIZE {
            for r in 0..SIZE {
                let grey = 0.2126 * level(r) + 0.7152 * level(g) + 0.0722 * level(b);
                let [x, y, z, _] = at(stops, grey);
                out.push_str(&format!("{x:.5} {y:.5} {z:.5}\n"));
            }
        }
    }
    out
}

fn written(dir: &Path, id: &str, stops: &Stops, edit: &RampEdit) -> Result<String> {
    match edit.format {
        RampFormat::Png => crate::pictures::store(dir, ("ramps", id), &strip(stops, edit.width), None),
        RampFormat::Cube => {
            let path = (1..)
                .map(|n| format!("luts/{id}-{n}.cube"))
                .find(|p| !dir.join(p).exists())
                .unwrap_or_default();
            std::fs::create_dir_all(dir.join("luts"))?;
            std::fs::write(dir.join(&path), table(stops))?;
            Ok(path)
        }
    }
}

fn ours(doc: &Value, id: &str) -> bool {
    doc.pointer(&format!("{EDITOR}/ramps/{}", escape(id))).is_some_and(Value::is_array)
        && doc.pointer(&format!("/scene/assets/{}", escape(id))).is_some()
}

pub fn stops_of(doc: &Value, id: &str) -> Value {
    let found = doc.pointer(&format!("{EDITOR}/ramps/{}", escape(id)));
    json!({ "asset": id, "stops": found.cloned().unwrap_or(Value::Null) })
}

pub(crate) fn make(dir: &Path, doc: &Value, edit: &RampEdit) -> Result<Vec<Op>> {
    let stops = parsed(&edit.stops)?;
    ensure!(
        WIDTHS.contains(&edit.width),
        said("editor.ramp_width", "a ramp is %1 to %2 pixels wide", &[WIDTHS.start(), WIDTHS.end()])
    );
    if let Some(pointer) = &edit.pointer {
        let param = pointer.starts_with("/scene/") && pointer.contains("/effects/");
        ensure!(
            param && pointer.contains("/params/"),
            said("editor.ramp_param", "a ramp goes to a parameter of an effect", &[])
        );
    }
    if let Some(id) = &edit.asset {
        ensure!(ours(doc, id), said("editor.ramp_own", "%1 is not a ramp made here", &[id]));
    }
    let last = edit.pointer.as_deref().and_then(|p| p.rsplit('/').next()).unwrap_or("ramp");
    let id = edit.asset.clone().unwrap_or_else(|| unique(doc, last));
    let path = written(dir, &id, &stops, edit)?;
    let mut ops = set(doc, &format!("/scene/assets/{}", escape(&id)), json!({ "path": path }));
    ops.extend(set(doc, &format!("{EDITOR}/ramps/{}", escape(&id)), json!(edit.stops)));
    if let Some(pointer) = &edit.pointer {
        ops.extend(set(doc, pointer, json!(id)));
    }
    Ok(ops)
}
