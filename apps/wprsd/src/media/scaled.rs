use image::{DynamicImage, GrayImage, RgbImage};
use jpeg_decoder::{Decoder, PixelFormat};
use std::io::Cursor;
use wprs_scene::COVER_SIDE;

pub(super) fn jpeg(bytes: &[u8], max_side: u32, budget: u64) -> Option<DynamicImage> {
    if !bytes.starts_with(&[0xff, 0xd8]) {
        return None;
    }
    let mut decoder = Decoder::new(Cursor::new(bytes));
    decoder.read_info().ok()?;
    let info = decoder.info()?;
    let (w, h) = (u32::from(info.width), u32::from(info.height));
    if w.max(h) > max_side || !matches!(info.pixel_format, PixelFormat::RGB24 | PixelFormat::L8) {
        return None;
    }
    decoder.set_max_decoding_buffer_size(budget as usize);
    let scale = f64::from(COVER_SIDE) / f64::from(w.max(h));
    let wanted = |v: u32| ((f64::from(v) * scale).ceil() as u16).max(1);
    let (sw, sh) = decoder.scale(wanted(w), wanted(h)).ok()?;
    let pixels = decoder.decode().ok()?;
    let (sw, sh) = (u32::from(sw), u32::from(sh));
    Some(match info.pixel_format {
        PixelFormat::L8 => DynamicImage::from(GrayImage::from_raw(sw, sh, pixels)?),
        _ => DynamicImage::from(RgbImage::from_raw(sw, sh, pixels)?),
    })
}
