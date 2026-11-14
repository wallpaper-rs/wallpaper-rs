use serde_json::{Value, json};
use wprs_palette::{AMBAR, BROTE, CORAL, ESCARCHA, Palette, derive, fixed};

pub(crate) fn tones(argb: &[i32], width: usize, height: usize) -> Value {
    let rgb: Vec<u8> = argb.iter().flat_map(|pixel| pixel.to_be_bytes().into_iter().skip(1)).collect();
    named(derive(&rgb, 3, width, height).unwrap_or_else(fixed))
}

fn named(palette: Palette) -> Value {
    json!({
        "night": palette.noche.to_string(),
        "night_high": palette.noche_alta.to_string(),
        "night_line": palette.noche_linea.to_string(),
        "mist": palette.bruma.to_string(),
        "mist_low": palette.bruma_baja.to_string(),
        "moon": palette.luna.to_string(),
        "moon_deep": palette.luna_honda.to_string(),
        "veil": palette.velo,
        "frost": ESCARCHA.to_string(),
        "amber": AMBAR.to_string(),
        "coral": CORAL.to_string(),
        "sprout": BROTE.to_string(),
    })
}

#[cfg(target_os = "android")]
#[jni::jni_mangle("io.github.wallpaper_rs.Native")]
pub fn tones<'a>(
    mut env: jni::EnvUnowned<'a>,
    _: jni::objects::JObject<'a>,
    pixels: jni::objects::JIntArray<'a>,
    width: jni::sys::jint,
) -> jni::objects::JString<'a> {
    crate::exports::on(&mut env, |env| {
        let mut argb = vec![0; pixels.len(env)?];
        pixels.get_region(env, 0, &mut argb)?;
        let wide = width.unsigned_abs() as usize;
        let tall = argb.len().checked_div(wide).unwrap_or(0);
        Ok(env.new_string(tones(&argb, wide, tall).to_string())?)
    })
}

#[cfg(test)]
mod tests {
    use super::tones;
    use wprs_palette::{ESCARCHA, Rgb, contrast};

    fn hex(value: &serde_json::Value, key: &str) -> Rgb {
        let text = value[key].as_str().unwrap().trim_start_matches('#');
        Rgb::hex(u32::from_str_radix(text, 16).unwrap())
    }

    fn even(argb: i32) -> Vec<i32> {
        vec![argb; 96 * 54]
    }

    #[test]
    fn text_keeps_its_contrast_on_bright_and_dark_wallpapers() {
        for wallpaper in [even(0xFFF8F4E8_u32 as i32), even(0xFF05060A_u32 as i32), even(0xFFE8D040_u32 as i32)] {
            let shown = tones(&wallpaper, 96, 54);
            assert!(contrast(ESCARCHA, hex(&shown, "night")) >= 7.0, "{shown}");
            assert!(contrast(hex(&shown, "mist"), hex(&shown, "night")) >= 4.5, "{shown}");
        }
    }

    #[test]
    fn bright_and_dark_wallpapers_give_different_tones() {
        let bright = tones(&even(0xFF40A0F0_u32 as i32), 96, 54);
        let dark = tones(&even(0xFF601010_u32 as i32), 96, 54);
        assert_ne!(bright["night"], dark["night"]);
    }

    #[test]
    fn no_picture_gives_the_fixed_tones() {
        let none = tones(&[], 0, 0);
        assert_eq!(none["frost"], "#f4efe6");
        assert!(none["night"].as_str().unwrap().starts_with('#'));
    }
}
