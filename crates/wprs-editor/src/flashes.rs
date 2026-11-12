use crate::findings::{Problem, warning};
use image::RgbaImage;
use serde::Serialize;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use wprs_core::message::said;
use wprs_scene::{Calming, FLASH_GRID, FlashGuard, SAFE_FLASHES};

const EDGE: u32 = 160;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Report {
    pub flashing: bool,
    pub rate: f32,
    pub at: Option<f64>,
    pub from: f64,
    pub seconds: f64,
    pub fps: u32,
}

type Memory = BTreeMap<PathBuf, (String, Report)>;

static MEMORY: Mutex<Memory> = Mutex::new(BTreeMap::new());

pub fn grid(image: &RgbaImage) -> Vec<f32> {
    let [columns, rows] = FLASH_GRID;
    let (w, h) = (image.width().max(1), image.height().max(1));
    let mut sums = vec![(0.0f32, 0u32); (columns * rows) as usize];
    for (x, y, pixel) in image.enumerate_pixels() {
        let cell = ((y * rows / h).min(rows - 1) * columns + (x * columns / w).min(columns - 1)) as usize;
        let [r, g, b, _] = pixel.0.map(|v| f32::from(v) / 255.0);
        sums[cell].0 += 0.2126 * r + 0.7152 * g + 0.0722 * b;
        sums[cell].1 += 1;
    }
    sums.into_iter().map(|(sum, n)| sum / n.max(1) as f32).collect()
}

pub fn judge(frames: impl IntoIterator<Item = (f64, Vec<f32>)>, span: (f64, f64, u32)) -> Report {
    let mut guard = FlashGuard::default();
    let (mut peak, mut at) = (0.0f32, None);
    for (time, luma) in frames {
        let calming = guard.watch(time, &luma);
        peak = peak.max(guard.rate());
        if calming != Calming::Steady && at.is_none() {
            (at, peak) = (Some(time), peak.max(SAFE_FLASHES));
        }
    }
    Report { flashing: at.is_some(), rate: peak, at, from: span.0, seconds: span.1, fps: span.2 }
}

fn stamp(doc: &Value) -> String {
    crate::project::stamp(&json!([doc["scene"], doc["manifest"]["properties"]]))
}

pub fn measure(dir: &Path, doc: &Value, (from, seconds, fps): (f64, f64, u32)) -> anyhow::Result<Report> {
    anyhow::ensure!(
        seconds > 0.0 && seconds <= 60.0 && (1..=30).contains(&fps),
        said("editor.flash_span", "measure from 0.1 to 60 s at 1 to 30 frames a second", &[])
    );
    let count = (seconds * f64::from(fps)).round().max(2.0) as usize;
    let times: Vec<f64> = (0..count).map(|n| from + n as f64 / f64::from(fps)).collect();
    let mut lumas = Vec::new();
    crate::shots::frames(dir, doc, (EDGE, &times), |image| lumas.push(grid(&image)))?;
    let report = judge(times.into_iter().zip(lumas), (from, seconds, fps));
    remember(dir, doc, &report);
    Ok(report)
}

pub fn remember(dir: &Path, doc: &Value, report: &Report) {
    if let Ok(mut memory) = MEMORY.lock() {
        memory.insert(dir.to_path_buf(), (stamp(doc), report.clone()));
    }
}

pub fn cached(dir: &Path, doc: &Value) -> Option<Report> {
    let memory = MEMORY.lock().ok()?;
    let (held, report) = memory.get(dir)?;
    (*held == stamp(doc)).then(|| report.clone())
}

pub fn suggestion(dir: &Path, doc: &Value) -> Option<Problem> {
    if doc["manifest"]["photosensitive"] == true || doc["manifest"]["schema"] != wprs_format::SCHEMA {
        return None;
    }
    let report = cached(dir, doc).filter(|r| r.flashing)?;
    let at = report.at.unwrap_or_default();
    let said = said(
        "editor.flashes_measured",
        "the scene flashed more than %1 times a second near second %2 of a render; if people can see that, set photosensitive",
        &[&SAFE_FLASHES, &format!("{at:.1}")],
    );
    Some(warning("/manifest/photosensitive", said))
}

pub fn ask(dir: &Path, doc: &Value, q: &Value) -> Value {
    if q["run"] != true {
        return json!(cached(dir, doc));
    }
    let span = (q["from"].as_f64().unwrap_or(0.0), q["seconds"].as_f64().unwrap_or(8.0));
    match measure(dir, doc, (span.0, span.1, q["fps"].as_u64().map_or(15, |f| f as u32))) {
        Ok(report) => json!(report),
        Err(e) => json!({ "error": crate::findings::told(&e) }),
    }
}
