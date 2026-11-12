use anyhow::{Result, anyhow, ensure};
use image::RgbaImage;
use image::codecs::webp::WebPEncoder;

fn put(out: &mut Vec<u8>, name: &[u8; 4], body: &[u8]) {
    out.extend_from_slice(name);
    out.extend_from_slice(&(body.len() as u32).to_le_bytes());
    out.extend_from_slice(body);
    if body.len() % 2 == 1 {
        out.push(0);
    }
}

fn three(value: u32) -> [u8; 3] {
    let b = value.to_le_bytes();
    [b[0], b[1], b[2]]
}

fn lossless(image: &RgbaImage) -> Result<Vec<u8>> {
    let mut file = Vec::new();
    WebPEncoder::new_lossless(&mut file).encode(
        image.as_raw(),
        image.width(),
        image.height(),
        image::ExtendedColorType::Rgba8,
    )?;
    let mut at = 12;
    while let (Some(name), Some(size)) = (file.get(at..at + 4), file.get(at + 4..at + 8)) {
        let size = u32::from_le_bytes([size[0], size[1], size[2], size[3]]) as usize;
        if name == b"VP8L" {
            return file
                .get(at..at + 8 + size + (size & 1))
                .map(<[u8]>::to_vec)
                .ok_or_else(|| anyhow!("cut"));
        }
        at += 8 + size + (size & 1);
    }
    Err(anyhow!("the encoder wrote no lossless picture"))
}

pub fn animated(images: &[RgbaImage], fps: u32) -> Result<Vec<u8>> {
    let first = images.first().ok_or_else(|| anyhow!("no frames"))?;
    let (width, height) = first.dimensions();
    ensure!(images.iter().all(|i| i.dimensions() == (width, height)), "the frames differ in size");
    ensure!(width <= 1 << 24 && height <= 1 << 24, "the frames are too large");
    let mut body = b"WEBP".to_vec();
    let mut header = vec![0x12, 0, 0, 0];
    header.extend(three(width - 1));
    header.extend(three(height - 1));
    put(&mut body, b"VP8X", &header);
    put(&mut body, b"ANIM", &[0, 0, 0, 0, 0, 0]);
    let delay = (1000.0 / f64::from(fps.max(1))).round() as u32;
    for image in images {
        let mut frame = vec![0; 6];
        frame.extend(three(width - 1));
        frame.extend(three(height - 1));
        frame.extend(three(delay));
        frame.push(2);
        frame.extend(lossless(image)?);
        put(&mut body, b"ANMF", &frame);
    }
    let mut out = b"RIFF".to_vec();
    out.extend_from_slice(&(body.len() as u32).to_le_bytes());
    out.extend(body);
    Ok(out)
}
