mod bands;
mod brush;
mod build;
mod chart;
#[cfg(feature = "fetch")]
mod fetch;
mod field;
mod files;
mod fill;
mod grow;
mod guided;
#[cfg(feature = "infer")]
mod infer;
#[cfg(feature = "infer")]
mod inpaint;
#[cfg(feature = "fetch")]
mod install;
#[cfg(feature = "infer")]
mod made;
mod matte;
pub mod models;
mod normals;
mod persist;
mod plan;
mod preview;
mod process;
#[cfg(feature = "infer")]
mod rebuild;
mod regions;
mod runtime;
mod shifted;
mod split;
mod stack;
mod stale;
mod still;
mod store;
mod views;
mod warp;
mod windows;

pub use bands::band;
pub use brush::{lift, lift_map, marks, stamp};
pub use build::{EDGE, MARGIN, MOST_EDGE, TIERS, edge_for, layers};
pub use chart::{Chart, Zone, chart, colour};
#[cfg(feature = "fetch")]
pub use fetch::{AGENT, Cancelled, allowed, download};
pub use field::{BEND, FIELD, field, field_size, sample, slope};
pub use files::{beside, decode, decode_edge, read, sha256, shape, write};
pub use fill::{push_pull, simple};
pub use guided::{EPSILON, RADIUS, filter, planes, refine};
#[cfg(feature = "infer")]
pub use infer::{Device, Estimator, Stage, Stopped, TELEMETRY, background, threads};
#[cfg(feature = "infer")]
pub use inpaint::Inpainter;
#[cfg(feature = "infer")]
pub use made::{Bench, bench, generate, generate_until};
pub use matte::{Matte, matte};
pub use models::{Asset, LAMA, MODEL, Part, RUNTIME};
pub use normals::{MAX_BLUR, MAX_STRENGTH, normals, relief, sphere};
pub use preview::Preview;
pub use process::{adjust, coloured, input_size, normalize, shrink, tensor};
#[cfg(feature = "infer")]
pub use rebuild::{settings, sized};
pub use regions::{JUMP, Regions, regions};
pub use split::{Split, cut, distance, indices, mask, soft, split};
pub use stack::{Band, FIELDS, FORMAT, Info, Layers, SUFFIX, Settings, fields_of, load, save};
pub use stale::{Note, stale};
pub use still::{failed_of, has_failed, lama_size, mark_failed, model_size, still_note};
pub use store::CACHE_BYTES;
pub use views::{band_mask, halves, histogram, hue, outlines, tint};
pub use warp::turned;
pub use windows::{Inpaint, lama, plan};

use std::path::{Path, PathBuf};

pub type Map = image::ImageBuffer<image::Luma<u16>, Vec<u16>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Places {
    pub data: PathBuf,
    pub cache: PathBuf,
}

impl Default for Places {
    fn default() -> Self {
        let root = |dir: Option<PathBuf>| dir.unwrap_or_else(|| PathBuf::from(".")).join("wallpaper-rs");
        Self { data: root(dirs::data_dir()), cache: root(dirs::cache_dir()) }
    }
}

impl Places {
    pub fn installed(&self, asset: &Asset) -> Option<PathBuf> {
        Some(self.file(asset)).filter(|p| p.is_file())
    }

    pub fn file(&self, asset: &Asset) -> PathBuf {
        let kind = if asset.kind == Part::Runtime { "runtime" } else { "models" };
        self.data.join(kind).join(asset.id).join(asset.file)
    }

    pub fn library(&self, explicit: Option<&Path>) -> Vec<PathBuf> {
        let env = std::env::var_os("ORT_DYLIB_PATH").filter(|v| !v.is_empty()).map(PathBuf::from);
        let dirs = ["/app/lib", "/usr/lib64", "/usr/lib", "/usr/local/lib"].map(PathBuf::from);
        let arch = PathBuf::from(format!("/usr/lib/{}-linux-gnu", std::env::consts::ARCH));
        let names = ["libonnxruntime.so", "libonnxruntime.so.1"];
        let system = dirs.into_iter().chain([arch]).flat_map(|d| names.map(|n| d.join(n)));
        let downloaded = RUNTIME.and_then(|r| self.installed(&r));
        let found = explicit.map(Path::to_path_buf).into_iter().chain(env).chain(system).chain(downloaded);
        found.filter(|p| p.is_file()).collect()
    }
}
