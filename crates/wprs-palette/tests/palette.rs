mod common;

use common::{H, W, pictures};
use wprs_palette::{ESCARCHA, Palette, Rgb, contrast, derive, fixed};

type Expected = (&'static str, [u32; 7], f32);

const EXPECTED: &[Expected] = &[
    ("black", [0x0f1319, 0x1a1e25, 0x333a44, 0xb7c2d2, 0x8f9cb1, 0xa2c6fb, 0x7aa3e0], 0.62),
    ("dark", [0x0d1124, 0x171c33, 0x2f3758, 0xbbc0d2, 0x949ab1, 0xb1c1fc, 0x8d9de0], 0.62),
    ("dark_grey", [0x0f140f, 0x1a201a, 0x333c33, 0xb7c6b7, 0x8fa18f, 0xa1d3a3, 0x78b27b], 0.63),
    ("flat_blue", [0x091224, 0x131e33, 0x293958, 0xb8c1d2, 0x919cb1, 0xa8c4fc, 0x81a1e0], 0.76),
    ("flat_green", [0x061707, 0x102311, 0x244125, 0xb7c6b7, 0x8fa18f, 0xa2d3a3, 0x79b27b], 0.86),
    ("flat_red", [0x220b08, 0x301512, 0x542c28, 0xd1bbb8, 0xb09490, 0xf6afa6, 0xd9897f], 0.79),
    ("forest", [0x061708, 0x0f2411, 0x234226, 0xb7c6b7, 0x8ea18f, 0xa5d3a0, 0x7cb278], 0.75),
    ("gradient_h", [0x1b0c1d, 0x27172b, 0x472e4c, 0xc9bccb, 0xa595a9, 0xecb98a, 0xce935d], 0.78),
    ("gradient_v", [0x121219, 0x1d1d25, 0x383844, 0xbfbfd1, 0x9999b0, 0xbfbdf9, 0x9c98dd], 0.81),
    ("grey", [0x0f1319, 0x1a1e25, 0x333a44, 0xb7c2d2, 0x8f9cb1, 0xa2c6fb, 0x7aa3e0], 0.83),
    ("light", [0x19100e, 0x251c19, 0x443532, 0xd1bbb6, 0xaf948d, 0xf3b397, 0xd68d6c], 0.96),
    ("night_city", [0x071324, 0x111e33, 0x263958, 0xb7c2d2, 0x8f9cb1, 0xe0bf84, 0xc19b54], 0.62),
    ("noise", [0x1a0d1e, 0x26172c, 0x462f4e, 0xc8bccc, 0xa495aa, 0xa4d3a1, 0x7bb278], 0.85),
    ("noise_dark", [0x160f1e, 0x221a2b, 0x40334d, 0xc5bdce, 0xa196ac, 0xd3b6f0, 0xb291d2], 0.62),
    ("noise_grey", [0x0f1319, 0x1a1e25, 0x333a44, 0xb7c2d2, 0x8f9cb1, 0xa2c6fb, 0x7aa3e0], 0.82),
    ("saturated", [0x220a0c, 0x301517, 0x542b2e, 0xd1bbbb, 0xb09394, 0xf6aeb1, 0xd9878b], 0.90),
    ("sunset", [0x220a0b, 0x301516, 0x542c2d, 0xd1bbba, 0xb09393, 0xf4b397, 0xd68d6d], 0.84),
    ("white", [0x0f1319, 0x1a1e25, 0x333a44, 0xb7c2d2, 0x8f9cb1, 0xa2c6fb, 0x7aa3e0], 0.96),
];

fn tones(p: &Palette) -> [Rgb; 7] {
    [p.noche, p.noche_alta, p.noche_linea, p.bruma, p.bruma_baja, p.luna, p.luna_honda]
}

fn derived(name: &str) -> Palette {
    let (_, px) = pictures().into_iter().find(|(n, _)| *n == name).unwrap();
    derive(&px, 3, W, H).unwrap()
}

#[test]
fn matches_the_reference_within_one_step() {
    assert!(EXPECTED.len() >= 12);
    for (name, want, velo) in EXPECTED {
        let got = derived(name);
        for (g, w) in tones(&got).iter().zip(want) {
            let w = Rgb::hex(*w);
            for c in 0..3 {
                assert!(g.0[c].abs_diff(w.0[c]) <= 1, "{name}: {g} vs {w}");
            }
        }
        assert!((got.velo - velo).abs() < 0.0101, "{name}: veil {} vs {velo}", got.velo);
    }
}

#[test]
fn guards_hold_for_every_picture() {
    for (name, px) in pictures() {
        let p = derive(&px, 3, W, H).unwrap();
        assert!(contrast(ESCARCHA, p.noche) >= 12.0, "{name}");
        assert!(contrast(p.bruma, p.noche) >= 8.0, "{name}");
        assert!(contrast(p.bruma_baja, p.noche) >= 4.5, "{name}");
        assert!(contrast(ESCARCHA, p.noche_alta) >= 10.0, "{name}");
        assert!(p.noche.luminance() < 0.02, "{name}: light deep tone");
        assert!((0.62..=0.96).contains(&p.velo), "{name}");
    }
}

#[test]
fn fixed_set_has_the_same_guards() {
    let p = fixed();
    assert_eq!(p.noche.to_string(), "#0a1322");
    assert!(contrast(ESCARCHA, p.noche) >= 12.0);
    assert!(contrast(p.bruma, p.noche) >= 8.0);
    assert!((p.velo - 0.8).abs() < 1e-6);
}

#[test]
fn rgba_ignores_alpha_and_bad_input_is_refused() {
    let (_, px) = pictures().into_iter().find(|(n, _)| *n == "sunset").unwrap();
    let rgba: Vec<u8> = px.chunks(3).flat_map(|c| [c[0], c[1], c[2], 7]).collect();
    assert_eq!(derive(&rgba, 4, W, H), derive(&px, 3, W, H));
    assert!(derive(&px[..10], 3, W, H).is_none());
    assert!(derive(&px, 3, 0, H).is_none());
    assert!(derive(&px, 2, W, H).is_none());
}

#[test]
fn large_pictures_are_sampled_not_refused() {
    let px: Vec<u8> = (0..400 * 300).flat_map(|i| [(i % 251) as u8, 90, 160]).collect();
    let p = derive(&px, 3, 400, 300).unwrap();
    assert!(contrast(ESCARCHA, p.noche) >= 12.0);
}

#[test]
fn hex_display_is_lowercase_with_hash() {
    assert_eq!(Rgb::hex(0x0a1b2c).to_string(), "#0a1b2c");
}
