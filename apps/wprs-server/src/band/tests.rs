use super::sheet::{LEAST_VEIL, NEEDED, brightest, css, of, tones};
use image::{Rgb as Pixel, RgbImage};
use wprs_palette::{ESCARCHA, contrast};

const STYLE: &str = include_str!("../../assets/style.css");

fn white() -> RgbImage {
    RgbImage::from_pixel(640, 360, Pixel([255, 255, 255]))
}

fn black() -> RgbImage {
    RgbImage::from_pixel(640, 360, Pixel([0, 0, 0]))
}

fn busy() -> RgbImage {
    let colours = [[255, 255, 255], [0, 0, 0], [255, 220, 0], [0, 200, 255], [255, 40, 120], [240, 240, 200]];
    RgbImage::from_fn(640, 360, |x, y| Pixel(colours[((x / 7 + y / 5 * 3) % 6) as usize]))
}

fn factors() -> Vec<f32> {
    STYLE
        .split("calc(var(--velo) * ")
        .skip(1)
        .map(|rest| rest.split(')').next().unwrap().trim().parse().unwrap())
        .collect()
}

#[test]
fn the_sheet_never_veils_less_than_the_band_counts_on() {
    let found = factors();
    assert!(found.len() >= 7, "{found:?}");
    assert!(found.iter().all(|f| *f >= LEAST_VEIL), "{found:?}");
    assert!(found.contains(&LEAST_VEIL), "{found:?}");
}

#[test]
fn title_menu_and_status_line_stay_readable_over_any_cover() {
    for (name, cover) in [("white", white()), ("black", black()), ("busy", busy())] {
        let (palette, velo) = tones(&cover).unwrap();
        for factor in factors() {
            let ground = brightest(palette.noche, (velo * factor).min(1.0), &cover);
            for text in [ESCARCHA, palette.bruma] {
                let ratio = contrast(text, ground);
                assert!(ratio >= NEEDED, "{name}: {text} over {ground} at {factor} gives {ratio:.2}");
            }
        }
    }
}

#[test]
fn the_veil_only_grows_when_the_cover_needs_it() {
    let (dark, velo) = tones(&black()).unwrap();
    assert!((velo - dark.velo).abs() < 1e-6, "{velo} vs {}", dark.velo);
    let (light, velo) = tones(&busy()).unwrap();
    assert!(velo > light.velo, "{velo} vs {}", light.velo);
}

#[test]
fn the_stylesheet_sets_every_tone_the_sheet_uses() {
    let (palette, velo) = tones(&busy()).unwrap();
    let text = css(&palette, velo);
    for name in [
        "--noche:",
        "--noche-alta:",
        "--noche-linea:",
        "--bruma:",
        "--bruma-baja:",
        "--acento:",
        "--acento-hondo:",
        "--velo:",
        "--noche-rgb:",
    ] {
        assert!(
            text.contains(name) && STYLE.contains(&format!("var({})", name.trim_end_matches(':'))),
            "{name}"
        );
    }
    assert!(text.starts_with(":root{") && !text.contains("url("), "{text}");
}

#[test]
fn a_cover_that_cannot_be_read_gives_no_sheet() {
    let dir = tempfile::tempdir().unwrap();
    let broken = dir.path().join("broken.jpg");
    std::fs::write(&broken, b"\xff\xd8\xff not a picture").unwrap();
    assert!(of(&broken).is_none() && of(&dir.path().join("missing.jpg")).is_none());
    let good = dir.path().join("good.jpg");
    busy().save(&good).unwrap();
    assert!(of(&good).is_some());
}
