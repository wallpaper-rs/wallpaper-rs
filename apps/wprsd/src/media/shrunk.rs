use super::covers::decode;
use super::scaled::jpeg;
use image::{DynamicImage, ImageFormat, RgbImage};
use std::io::Cursor;

fn photo(w: u32, h: u32) -> Vec<u8> {
    let mut out = Cursor::new(Vec::new());
    let image = DynamicImage::from(RgbImage::from_fn(w, h, |x, y| {
        image::Rgb([(x % 251) as u8, (y % 241) as u8, 90])
    }));
    image.write_to(&mut out, ImageFormat::Jpeg).unwrap();
    out.into_inner()
}

#[test]
fn a_large_jpeg_is_decoded_at_the_size_the_scene_needs() {
    let bytes = photo(2048, 1024);
    let small = jpeg(&bytes, 4096, 96 << 20).unwrap();
    assert!(small.width() < 2048 && small.width() >= 256, "decoded at {}", small.width());
    assert!(small.height() < 1024 && small.height() >= 128);
    assert_eq!(decode(&bytes).unwrap().pixels.size(), (256, 128));
}

#[test]
fn only_jpegs_within_the_limits_take_the_scaled_path() {
    assert!(jpeg(b"not a picture", 4096, 96 << 20).is_none());
    assert!(jpeg(&photo(64, 64), 32, 96 << 20).is_none(), "a side over the limit is refused");
    let mut png = Cursor::new(Vec::new());
    DynamicImage::new_rgb8(8, 8).write_to(&mut png, ImageFormat::Png).unwrap();
    assert!(jpeg(png.get_ref(), 4096, 96 << 20).is_none());
    assert_eq!(decode(png.get_ref()).unwrap().pixels.size(), (8, 8));
}
