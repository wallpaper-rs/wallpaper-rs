use crate::oklab::linear;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rgb(pub [u8; 3]);

impl Rgb {
    pub const fn hex(code: u32) -> Self {
        Rgb([(code >> 16) as u8, (code >> 8) as u8, code as u8])
    }

    pub fn luminance(self) -> f64 {
        let [r, g, b] = self.0.map(linear);
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.0[0], self.0[1], self.0[2])
    }
}

pub const ESCARCHA: Rgb = Rgb::hex(0xf4efe6);
pub const AMBAR: Rgb = Rgb::hex(0xf0c26b);
pub const CORAL: Rgb = Rgb::hex(0xff9d8a);
pub const BROTE: Rgb = Rgb::hex(0x8fd6a8);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Palette {
    pub noche: Rgb,
    pub noche_alta: Rgb,
    pub noche_linea: Rgb,
    pub bruma: Rgb,
    pub bruma_baja: Rgb,
    pub luna: Rgb,
    pub luna_honda: Rgb,
    pub velo: f32,
}

pub fn contrast(a: Rgb, b: Rgb) -> f64 {
    let (la, lb) = (a.luminance(), b.luminance());
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

pub const fn fixed() -> Palette {
    Palette {
        noche: Rgb::hex(0x0a1322),
        noche_alta: Rgb::hex(0x121e33),
        noche_linea: Rgb::hex(0x2a3a57),
        bruma: Rgb::hex(0xb9c4d6),
        bruma_baja: Rgb::hex(0x8f9db4),
        luna: Rgb::hex(0x9cc3f5),
        luna_honda: Rgb::hex(0x6f9fdc),
        velo: 0.8,
    }
}
