use crate::Map;
use anyhow::{Context, Result, ensure};
use image::codecs::png::PngEncoder;
use image::{ExtendedColorType, ImageEncoder, ImageReader, Limits, RgbImage};
use sha2::{Digest, Sha256};
use std::io::{Read, Write};
use std::os::unix::fs::DirBuilderExt;
use std::path::{Path, PathBuf};
use std::time::Duration;
use wprs_core::message::said;

const MAX_EDGE: u32 = 8192;
const PICTURE_EDGE: u32 = 2048;
const MAX_BYTES: u64 = 256 << 20;
const STALE: Duration = Duration::from_secs(3600);

pub fn beside(image: &Path) -> PathBuf {
    image.with_added_extension("depth.png")
}

pub(crate) fn folder(dir: &Path) -> Result<()> {
    Ok(std::fs::DirBuilder::new().recursive(true).mode(0o700).create(dir)?)
}

pub(crate) fn sweep(dir: &Path, prefixes: &[&str]) {
    let ours = |name: &str| {
        let rest = prefixes.iter().find_map(|p| name.strip_prefix(p));
        rest.is_some_and(|r| r.len() == 6 && r.bytes().all(|b| b.is_ascii_alphanumeric()))
    };
    let stale = |m: std::fs::Metadata| {
        m.is_file() && m.modified().is_ok_and(|t| t.elapsed().is_ok_and(|a| a > STALE))
    };
    for entry in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        if ours(&entry.file_name().to_string_lossy()) && entry.metadata().is_ok_and(stale) {
            std::fs::remove_file(entry.path()).ok();
        }
    }
}

pub(crate) fn hashed(sha256: &str) -> Option<&str> {
    (sha256.len() == 64 && sha256.bytes().all(|b| b.is_ascii_hexdigit())).then_some(sha256)
}

pub fn read(path: &Path) -> Result<Map> {
    Ok(read_png(path)?.into_luma16())
}

pub(crate) fn read_png(path: &Path) -> Result<image::DynamicImage> {
    let mut reader = ImageReader::open(path)
        .with_context(|| said("depth.cannot_open", "cannot open %1", &[&path.display()]).local())?
        .with_guessed_format()?;
    let mut limits = Limits::default();
    (limits.max_image_width, limits.max_image_height, limits.max_alloc) =
        (Some(MAX_EDGE), Some(MAX_EDGE), Some(MAX_BYTES));
    reader.limits(limits);
    reader
        .decode()
        .with_context(|| said("depth.is_not_a_picture", "%1 is not a picture", &[&path.display()]).local())
}

pub fn shape(image: &Path) -> Result<(u32, u32)> {
    wprs_render::image_size(image)
        .with_context(|| said("depth.is_not_a_picture", "%1 is not a picture", &[&image.display()]).local())
}

pub fn write(map: &Map, path: &Path) -> Result<()> {
    ensure!(
        map.width() > 0 && map.height() > 0,
        "{}",
        said("depth.the_depth_map_is_empty", "the depth map is empty", &[]).local()
    );
    let dir = path.parent().filter(|d| !d.as_os_str().is_empty()).unwrap_or(Path::new("."));
    folder(dir)?;
    sweep(dir, &[".depth-"]);
    let mut file = tempfile::Builder::new().prefix(".depth-").tempfile_in(dir)?;
    let bytes: Vec<u8> = map.as_raw().iter().flat_map(|v| v.to_ne_bytes()).collect();
    let encoder = PngEncoder::new(std::io::BufWriter::new(file.as_file_mut()));
    encoder.write_image(&bytes, map.width(), map.height(), ExtendedColorType::L16)?;
    file.as_file_mut().flush()?;
    file.as_file().sync_all()?;
    file.persist(path).map_err(|e| e.error)?;
    std::fs::File::open(dir).and_then(|d| d.sync_all()).ok();
    Ok(())
}

pub(crate) fn feed(hash: &mut Sha256, mut reader: impl Read) -> std::io::Result<()> {
    let mut chunk = vec![0; 1 << 16];
    loop {
        match reader.read(&mut chunk)? {
            0 => return Ok(()),
            n => hash.update(&chunk[..n]),
        }
    }
}

pub fn sha256(path: &Path) -> std::io::Result<String> {
    let mut hash = Sha256::new();
    feed(&mut hash, std::fs::File::open(path)?)?;
    Ok(hash.finalize().iter().map(|b| format!("{b:02x}")).collect())
}

pub fn decode(path: &Path) -> Result<RgbImage> {
    decode_edge(path, PICTURE_EDGE)
}

pub fn decode_edge(path: &Path, edge: u32) -> Result<RgbImage> {
    let decoded = wprs_render::decode_unoriented(ImageReader::open(path)?, wprs_render::picture_limits());
    let refused = || said("depth.is_not_a_picture", "%1 is not a picture", &[&path.display()]).local();
    let Ok((mut image, orientation)) = decoded else {
        let image = wprs_render::decode_image(path, edge).with_context(refused)?;
        return Ok(image::DynamicImage::ImageRgba8(image).into_rgb8());
    };
    if image.width().max(image.height()) > edge {
        image = image.thumbnail(edge, edge);
    }
    image.apply_orientation(orientation);
    Ok(image.into_rgb8())
}
