use std::path::Path;

use image::imageops::FilterType;
use wprs_palette::{Palette, derive, fixed};

const WIDTH: u32 = 96;
const HEIGHT: u32 = 54;

pub fn of_picture(path: &Path) -> Option<Palette> {
    let picture = image::open(path).ok()?;
    let small = picture.resize_exact(WIDTH, HEIGHT, FilterType::Triangle).to_rgb8();
    derive(small.as_raw(), 3, WIDTH as usize, HEIGHT as usize)
}

pub fn json(palette: &Palette) -> String {
    serde_json::json!({
        "noche": palette.noche.to_string(),
        "nocheAlta": palette.noche_alta.to_string(),
        "nocheLinea": palette.noche_linea.to_string(),
        "bruma": palette.bruma.to_string(),
        "brumaBaja": palette.bruma_baja.to_string(),
        "luna": palette.luna.to_string(),
        "lunaHonda": palette.luna_honda.to_string(),
        "velo": palette.velo,
    })
    .to_string()
}

pub fn for_path(path: &str) -> String {
    let found = if path.is_empty() { None } else { of_picture(Path::new(path)) };
    json(&found.unwrap_or(fixed()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_picture_gives_the_fixed_set() {
        let set: serde_json::Value = serde_json::from_str(&for_path("")).unwrap();
        assert_eq!(set["noche"], "#0a1322");
        assert_eq!(set["luna"], "#9cc3f5");
        assert_eq!(for_path("/nonexistent/picture.png"), for_path(""));
    }

    #[test]
    fn a_picture_gives_its_own_tones() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("warm.png");
        image::RgbImage::from_fn(192, 108, |x, y| image::Rgb([200 - (y as u8), 90 + (x as u8 / 4), 40]))
            .save(&path)
            .unwrap();
        let set: serde_json::Value = serde_json::from_str(&for_path(path.to_str().unwrap())).unwrap();
        assert_ne!(set["noche"], "#0a1322");
        assert!(set["velo"].as_f64().is_some_and(|velo| (0.6..=0.97).contains(&velo)));
    }
}
