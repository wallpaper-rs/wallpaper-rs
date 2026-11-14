use crate::validate::picture_limits;
use image::{ImageReader, RgbImage};
use std::path::Path;
use wprs_palette::{ESCARCHA, MAX_PIXELS, Palette, Rgb, contrast, derive};

pub const LEAST_VEIL: f32 = 0.75;
pub const NEEDED: f64 = 4.5;
const MOST: f32 = 1.34;

pub fn read(path: &Path) -> Option<RgbImage> {
    let mut reader = ImageReader::open(path).ok()?.with_guessed_format().ok()?;
    reader.limits(picture_limits());
    Some(reader.decode().ok()?.to_rgb8())
}

pub fn tones(cover: &RgbImage) -> Option<(Palette, f32)> {
    let (width, height) = (cover.width() as usize, cover.height() as usize);
    let palette = derive(cover.as_raw(), 3, width, height)?;
    let mut velo = palette.velo;
    while velo < MOST && !readable(&palette, cover, (velo * LEAST_VEIL).min(1.0)) {
        velo = ((velo + 0.01) * 100.0).round() / 100.0;
    }
    Some((palette, velo.min(MOST)))
}

pub fn over(noche: Rgb, alpha: f32, pixel: [u8; 3]) -> Rgb {
    let mix = |n: u8, p: u8| (alpha * n as f32 + (1.0 - alpha) * p as f32).round() as u8;
    Rgb([mix(noche.0[0], pixel[0]), mix(noche.0[1], pixel[1]), mix(noche.0[2], pixel[2])])
}

pub fn brightest(noche: Rgb, alpha: f32, cover: &RgbImage) -> Rgb {
    let total = cover.as_raw().len() / 3;
    let step = total.div_ceil(MAX_PIXELS).max(1);
    let mut seen: Vec<(f64, Rgb)> = cover
        .as_raw()
        .as_chunks::<3>()
        .0
        .iter()
        .step_by(step)
        .map(|p| over(noche, alpha, *p))
        .map(|c| (c.luminance(), c))
        .collect();
    seen.sort_unstable_by(|a, b| a.0.total_cmp(&b.0));
    seen.get((seen.len().saturating_sub(1) * 95).div_ceil(100)).map_or(noche, |s| s.1)
}

pub fn readable(palette: &Palette, cover: &RgbImage, alpha: f32) -> bool {
    let ground = brightest(palette.noche, alpha, cover);
    [ESCARCHA, palette.bruma].iter().all(|text| contrast(*text, ground) >= NEEDED)
}

pub fn css(palette: &Palette, velo: f32) -> String {
    let [r, g, b] = palette.noche.0;
    format!(
        ":root{{--noche:{};--noche-alta:{};--noche-linea:{};--bruma:{};--bruma-baja:{};--acento:{};\
         --acento-hondo:{};--velo:{velo:.2};--noche-rgb:{r} {g} {b}}}\n",
        palette.noche,
        palette.noche_alta,
        palette.noche_linea,
        palette.bruma,
        palette.bruma_baja,
        palette.luna,
        palette.luna_honda,
    )
}

pub fn of(path: &Path) -> Option<String> {
    let cover = read(path)?;
    let (palette, velo) = tones(&cover)?;
    Some(css(&palette, velo))
}
