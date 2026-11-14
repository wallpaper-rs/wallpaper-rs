use crate::daemon::Daemon;
use image::imageops::FilterType;
use std::path::{Path, PathBuf};
use wprs_core::model::Tones;
use wprs_palette::{Palette, derive};

const WIDTH: u32 = 96;
const HEIGHT: u32 = 54;
const POSTER: u32 = 256;

fn tones_of(poster: &Path) -> Option<Tones> {
    let small = image::open(poster).ok()?.resize_exact(WIDTH, HEIGHT, FilterType::Triangle).to_rgb8();
    let set = derive(small.as_raw(), 3, WIDTH as usize, HEIGHT as usize)?;
    Some(published(&set))
}

fn published(p: &Palette) -> Tones {
    Tones {
        noche: p.noche.to_string(),
        noche_alta: p.noche_alta.to_string(),
        noche_linea: p.noche_linea.to_string(),
        bruma: p.bruma.to_string(),
        bruma_baja: p.bruma_baja.to_string(),
        luna: p.luna.to_string(),
        luna_honda: p.luna_honda.to_string(),
        velo: f64::from(p.velo),
    }
}

impl Daemon {
    fn poster(&self, shown: &Path) -> Option<PathBuf> {
        if image::ImageFormat::from_path(shown).is_ok() {
            return Some(shown.to_path_buf());
        }
        let item = self.store.read(|lib| lib.item(&lib.resolve_item(&shown.to_string_lossy())?)).ok()?;
        self.thumbs.cached(&item, POSTER)
    }

    pub(crate) fn retone(&self, output: &str, shown: Option<&Path>) {
        let tones = shown.and_then(|path| self.poster(path)).and_then(|poster| tones_of(&poster));
        if let Some(info) = self.state().outputs.get_mut(output) {
            (info.tones, info.shown) = (tones, shown.map(Path::to_path_buf));
        }
    }

    pub(crate) fn retone_waiting(&self) {
        let waiting: Vec<(String, PathBuf)> = {
            let st = self.state();
            let wants = |(name, info): (&String, &crate::streams::OutputInfo)| {
                info.tones.is_none().then(|| info.shown.clone().map(|shown| (name.clone(), shown))).flatten()
            };
            st.outputs.iter().filter_map(wants).collect()
        };
        for (output, shown) in waiting {
            let Some(tones) = self.poster(&shown).and_then(|poster| tones_of(&poster)) else { continue };
            if let Some(info) = self.state().outputs.get_mut(&output) {
                info.tones = Some(tones);
            }
            self.changes.changed("state", vec![output]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_picture_gives_its_tones_and_a_missing_one_gives_none() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("warm.png");
        image::RgbImage::from_fn(192, 108, |x, y| image::Rgb([200 - (y as u8), 90 + (x as u8 / 4), 40]))
            .save(&path)
            .unwrap();
        let tones = tones_of(&path).unwrap();
        assert!(tones.noche.starts_with('#') && tones.noche.len() == 7);
        assert!((0.6..=0.97).contains(&tones.velo));
        assert!(tones_of(&dir.path().join("none.png")).is_none());
    }
}
