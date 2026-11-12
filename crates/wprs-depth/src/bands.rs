use crate::build::{fitted, leveled};
use crate::plan::Plan;
use crate::stack::Info;
use crate::{Map, Places, read};
use anyhow::{Context, Result, ensure};
use image::RgbImage;
use std::path::Path;
use wprs_core::edit::Depth;

pub fn band(picture: &RgbImage, map: &Map, depth: &Depth, info: &Info, layer: usize) -> Result<Vec<bool>> {
    ensure!(layer < info.depths.len(), "the set has {} layers", info.depths.len());
    let (size, levels) = leveled(picture, map, depth);
    let fixed = Depth { cuts: Some(info.cuts.clone()), ..depth.clone() };
    let plan = Plan::new(&levels, size, &fixed, (info.depths.len(), info.built));
    ensure!(plan.parts.count() == info.depths.len(), "the layer set does not match the depth map");
    Ok(plan.region(layer).band)
}

impl Places {
    pub fn layer_band(
        &self,
        (image, sha256): (&Path, &str),
        depth: &Depth,
        (info, layer): (&Info, usize),
    ) -> Result<(Vec<bool>, (u32, u32))> {
        let map = read(&self.find(image, sha256, depth).context("the image has no depth map")?)?;
        let edge = info.size[0].max(info.size[1]);
        let picture = fitted(crate::files::decode_edge(image, edge)?, edge);
        let found = band(&picture, &map, depth, info, layer)?;
        Ok((found, picture.dimensions()))
    }
}
