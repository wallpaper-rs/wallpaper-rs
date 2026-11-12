use crate::{Map, Places};
use anyhow::Result;
use image::RgbaImage;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use wprs_core::Fill;
use wprs_core::edit::Depth;
use wprs_core::travel::Reach;

pub use crate::persist::{load, save};

pub const SUFFIX: &str = ".layers.png";
pub const FIELDS: &str = ".fields.png";
pub const FORMAT: u32 = 2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Settings {
    pub fill: Fill,
    pub count: usize,
    pub shift: f32,
    pub edge: u32,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Band {
    pub layer: usize,
    pub reach: f32,
    pub pixels: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Info {
    pub format: u32,
    pub size: [u32; 2],
    pub fill: String,
    pub cuts: Vec<f32>,
    pub depths: Vec<f32>,
    pub bands: Vec<Band>,
    pub field: [u32; 2],
    pub regions: usize,
    pub built: f32,
    pub reach: Reach,
}

#[derive(Clone)]
pub struct Layers {
    pub info: Info,
    pub images: Vec<RgbaImage>,
    pub fields: Vec<Map>,
}

fn renamed(path: &Path, to: &str) -> PathBuf {
    let name = path.file_name().map(|n| n.to_string_lossy().replace(SUFFIX, to));
    path.with_file_name(name.unwrap_or_default())
}

pub(crate) fn json(path: &Path) -> PathBuf {
    renamed(path, ".layers.json")
}

pub fn fields_of(path: &Path) -> PathBuf {
    renamed(path, FIELDS)
}

pub(crate) fn unjson(png: &Path) {
    if png.to_string_lossy().ends_with(SUFFIX) {
        std::fs::remove_file(json(png)).ok();
        std::fs::remove_file(fields_of(png)).ok();
        std::fs::remove_file(crate::failed_of(png)).ok();
    }
}

impl Settings {
    pub fn named(&self) -> String {
        format!("{}-{}-{:.0}-{}", self.fill.as_str(), self.count, self.shift * 1000.0, self.edge)
    }
}

impl Places {
    pub fn layer_set(&self, image: &Path, sha256: &str, depth: &Depth, set: &Settings) -> Option<PathBuf> {
        let map = self.find(image, sha256, depth)?;
        let meta = std::fs::metadata(&map).ok()?;
        let stamp = meta.modified().ok()?.duration_since(std::time::UNIX_EPOCH).ok()?.as_nanos();
        let (near, far, gamma) = (depth.near, depth.far, depth.gamma);
        let used = format!(
            "{FORMAT}|{sha256}|{}|{}|{near:.3}|{far:.3}|{gamma:.3}|{}|{}|{stamp}|{}|{}|{}",
            depth.model,
            depth.input,
            set.named(),
            meta.len(),
            crate::models::LAMA.id,
            crate::build::VERSION,
            serde_json::to_string(&(&depth.cuts, &depth.strokes, &depth.fills, &depth.joins, &depth.splits))
                .unwrap_or_default()
        );
        let hash: String =
            Sha256::digest(used.as_bytes()).iter().take(8).map(|b| format!("{b:02x}")).collect();
        let name = format!("{}-{hash}{SUFFIX}", crate::files::hashed(sha256)?);
        Some(self.cache.join("depth").join(name))
    }

    pub fn fill(&self, wanted: Fill) -> Fill {
        let ready = self.installed(&crate::models::LAMA).is_some() && !self.library(None).is_empty();
        if wanted == Fill::Lama && !ready { Fill::Simple } else { wanted }
    }

    pub fn discard(&self, path: &Path) {
        std::fs::remove_file(path).ok();
        unjson(path);
    }

    pub fn layer_files(&self) -> Vec<PathBuf> {
        let entries = std::fs::read_dir(self.cache.join("depth")).into_iter().flatten().flatten();
        entries.map(|e| e.path()).filter(|p| p.to_string_lossy().ends_with(SUFFIX)).collect()
    }

    pub fn layer_bytes(&self) -> u64 {
        let size = |p: &PathBuf| std::fs::metadata(p).map_or(0, |m| m.len());
        self.layer_files().iter().map(|p| size(p) + size(&json(p)) + size(&fields_of(p))).sum()
    }

    pub fn unlayer_all(&self) -> Result<usize> {
        let sets = self.layer_files();
        sets.iter().for_each(|p| self.discard(p));
        Ok(sets.len())
    }

    pub fn layers_of(&self, sha256: &str) -> Vec<PathBuf> {
        let prefix = format!("{sha256}-");
        let named = self.layer_files().into_iter();
        named.filter(|p| p.file_name().is_some_and(|n| n.to_string_lossy().starts_with(&prefix))).collect()
    }

    pub fn unlayer(&self, sha256: &str) -> usize {
        let sets = self.layers_of(sha256);
        let gone = |p: &PathBuf| {
            std::fs::remove_file(fields_of(p)).ok();
            std::fs::remove_file(crate::failed_of(p)).ok();
            std::fs::remove_file(json(p)).is_ok() | std::fs::remove_file(p).is_ok()
        };
        sets.iter().filter(|p| gone(p)).count()
    }
}
