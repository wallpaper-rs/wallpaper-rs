use super::scripted::Scripted;
use super::{Pixels, SceneAssets};
use crate::{Layer, RenderError, Renderer};
use image::RgbaImage;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use wprs_core::Fit;
use wprs_scene::{Lut, Quality, Runtime, Scene, Values};

const PICTURES: [&str; 9] = ["png", "jpg", "jpeg", "webp", "gif", "bmp", "ktx2", "hdr", "exr"];
const FONTS: [&str; 5] = ["ttf", "otf", "ttc", "otc", "woff2"];
const DECODED: u64 = 512 << 20;

type Failure = Box<dyn std::error::Error + Send + Sync>;

fn still(bytes: Vec<u8>, linear: bool) -> Result<RgbaImage, Failure> {
    let reader = image::ImageReader::new(std::io::Cursor::new(bytes));
    Ok(crate::decode_asset(reader, crate::picture_limits(), linear)?)
}

impl SceneAssets {
    pub fn read(
        scene: &Scene,
        mut read: impl FnMut(&str) -> Result<Vec<u8>, String>,
    ) -> Result<SceneAssets, String> {
        let mut assets = SceneAssets::default();
        let mut decoded = crate::Pictures::default();
        for (id, asset) in &scene.assets {
            let ext = Path::new(&asset.path).extension().and_then(|e| e.to_str()).unwrap_or_default();
            let ext = ext.to_ascii_lowercase();
            if PICTURES.contains(&ext.as_str()) {
                let total = decoded
                    .add::<Failure>((&mut assets, id, asset), || Ok(read(&asset.path)?), still)
                    .map_err(|e| e.to_string())?;
                (total <= DECODED)
                    .then_some(())
                    .ok_or("the images of the scene are too large once decoded")?;
            } else if FONTS.contains(&ext.as_str()) {
                assets.fonts.insert(id.clone(), Arc::new(read(&asset.path)?));
            } else if ext == "cube" {
                let lut = Lut::parse(&read(&asset.path)?).map_err(|e| e.to_string())?;
                assets.luts.insert(id.clone(), Arc::new(lut));
            }
        }
        Ok(assets)
    }

    pub fn with_scripts(
        mut self,
        scene: &Scene,
        mut read: impl FnMut(&str) -> Result<Vec<u8>, String>,
    ) -> Result<SceneAssets, String> {
        for script in &scene.scripts {
            let text =
                String::from_utf8(read(&script.path)?).map_err(|_| format!("{} is not text", script.path))?;
            self.scripts.push((script.path.clone(), text));
        }
        Ok(self)
    }

    pub fn reloadable<R: FnMut(&str) -> Result<Vec<u8>, String>>(
        &mut self,
        scene: &Scene,
        open: impl Fn() -> Option<R> + Send + Sync + 'static,
    ) {
        let (open, mut made) = (Arc::new(open), HashMap::new());
        for (id, pixels) in &mut self.images {
            let Some(asset) = scene.assets.get(id).filter(|_| pixels.packed().is_none()) else { continue };
            let (path, animated, linear, open) =
                (asset.path.clone(), asset.animated, asset.linear, open.clone());
            let reload = move || {
                let bytes = open()?(&path).ok()?;
                if animated {
                    crate::animation(&bytes).ok().map(|(atlas, _)| atlas)
                } else {
                    still(bytes, linear).ok()
                }
            };
            let fresh = made.entry(Arc::as_ptr(pixels)).or_insert_with(|| {
                pixels.image().map_or_else(|| pixels.clone(), |image| Pixels::reloadable(image, reload))
            });
            *pixels = fresh.clone();
        }
    }
}

impl Renderer {
    pub fn poster(
        &self,
        scene: &Scene,
        assets: &SceneAssets,
        props: &Values,
        clock: (f64, &str),
        edge: u32,
    ) -> Result<RgbaImage, RenderError> {
        self.still(scene, assets, props, clock, (edge, f64::from(scene.clock.poster_time)))
    }

    pub fn still(
        &self,
        scene: &Scene,
        assets: &SceneAssets,
        props: &Values,
        clock: (f64, &str),
        (edge, time): (u32, f64),
    ) -> Result<RgbaImage, RenderError> {
        let mut out = Vec::new();
        self.stills(scene, assets, props, clock, (edge, &[time]), |image| out.push(image))?;
        out.pop().ok_or_else(|| RenderError::Readback("no image".into()))
    }

    pub fn stills(
        &self,
        scene: &Scene,
        assets: &SceneAssets,
        props: &Values,
        (wall, lang): (f64, &str),
        (edge, times): (u32, &[f64]),
        mut each: impl FnMut(RgbaImage),
    ) -> Result<(), RenderError> {
        let [w, h] = scene.canvas.size.map(|v| v.max(1) as f32);
        let scale = (edge as f32 / w.max(h)).min(1.0);
        let size = ((w * scale).round().max(1.0) as u32, (h * scale).round().max(1.0) as u32);
        let mut view = self.scene_view(scene, assets, props)?;
        let mut runtime = Runtime::new(Arc::new(scene.clone()), props.clone());
        assets.sheets.iter().for_each(|(id, d)| runtime.set_sheet(id, d.clone()));
        runtime.set_wall(wall);
        runtime.set_language(lang);
        let mut scripts = Scripted::start(scene, assets, &runtime, (lang, None));
        let (mut target, mut pixels) = (self.create_target(size.0, size.1)?, Vec::new());
        for (i, time) in times.iter().enumerate() {
            runtime.seek(time.clamp(0.0, f64::from(wprs_scene::MAX_POSTER_TIME)));
            if let Some(scripts) = scripts.as_mut() {
                let at = (view.view(size), [size.0, size.1], 60, 3);
                (0..if i == 0 { 4 } else { 1 }).for_each(|_| scripts.step(&mut runtime, at));
            }
            let frame = runtime.frame(view.view(size), Quality::Q3);
            let image = self.render_scene(&mut view, &runtime, &frame, size, Quality::Q3, &[])?;
            each(self.read_into(&mut target, &mut pixels, image, size)?);
        }
        Ok(())
    }

    pub(crate) fn read_scene(
        &self,
        image: &crate::GpuImage,
        size: (u32, u32),
    ) -> Result<RgbaImage, RenderError> {
        let (mut target, mut pixels) = (self.create_target(size.0, size.1)?, Vec::new());
        self.read_into(&mut target, &mut pixels, image, size)
    }

    fn read_into(
        &self,
        target: &mut crate::Target,
        pixels: &mut Vec<u8>,
        image: &crate::GpuImage,
        size: (u32, u32),
    ) -> Result<RgbaImage, RenderError> {
        let layer = Layer {
            image: Some(image),
            fit: Fit::Stretch,
            background: [0; 3],
            opacity: 1.0,
            uv: None,
            relief: None,
            flip: [false; 2],
            grade: None,
        };
        self.render_layers(target, &[layer], pixels)?;
        let mut rgba = pixels.clone();
        rgba.as_chunks_mut::<4>().0.iter_mut().for_each(|p| p.swap(0, 2));
        RgbaImage::from_raw(size.0, size.1, rgba).ok_or_else(|| RenderError::Readback("short image".into()))
    }
}
