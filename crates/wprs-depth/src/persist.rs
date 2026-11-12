use crate::files::{folder, read_png, sweep};
use crate::stack::{FORMAT, Info, Layers, fields_of, json};
use anyhow::{Context, Result, ensure};
use image::ExtendedColorType;
use image::ImageEncoder;
use image::codecs::png::PngEncoder;
use std::io::Write;
use std::path::Path;
use wprs_core::message::said;

pub fn save(path: &Path, layers: &Layers) -> Result<()> {
    let Layers { info, images, fields } = layers;
    let (w, h) = (info.size[0], info.size[1]);
    ensure!(
        images.iter().all(|i| i.dimensions() == (w, h)),
        "{}",
        said("depth.the_layers_differ_in_size", "the layers differ in size", &[]).local()
    );
    let (fw, fh) = (info.field[0], info.field[1]);
    ensure!(
        fields.len() == images.len(),
        "{}",
        said("depth.each_layer_needs_its_depth", "each layer needs its depth field", &[]).local()
    );
    ensure!(
        fields.iter().all(|f| f.dimensions() == (fw, fh)),
        "{}",
        said("depth.the_depth_fields_differ_in", "the depth fields differ in size", &[]).local()
    );
    let dir = path
        .parent()
        .context(said("depth.no_folder_for_the_layers", "no folder for the layers", &[]).local())?;
    folder(dir)?;
    sweep(dir, &[".layers-"]);
    let publish = |name: &Path, write: &mut dyn FnMut(&mut std::fs::File) -> Result<()>| -> Result<()> {
        let mut file = tempfile::Builder::new().prefix(".layers-").tempfile_in(dir)?;
        write(file.as_file_mut())?;
        file.as_file().sync_all()?;
        file.persist(name).map(drop).map_err(|e| e.error.into())
    };
    let deep: Vec<u8> = fields.iter().flat_map(|f| f.as_raw().iter().flat_map(|v| v.to_ne_bytes())).collect();
    publish(&fields_of(path), &mut |f| {
        let encoder = PngEncoder::new(std::io::BufWriter::new(&mut *f));
        Ok(encoder.write_image(&deep, fw, fh * fields.len() as u32, ExtendedColorType::L16)?)
    })?;
    publish(&json(path), &mut |f| Ok(f.write_all(&serde_json::to_vec(info)?)?))?;
    let atlas: Vec<u8> = images.iter().flat_map(|i| i.as_raw().iter().copied()).collect();
    let rows = h * images.len() as u32;
    publish(path, &mut |f| {
        let encoder = PngEncoder::new(std::io::BufWriter::new(&mut *f));
        Ok(encoder.write_image(&atlas, w, rows, ExtendedColorType::Rgba8)?)
    })
}

pub fn load(path: &Path) -> Result<Option<Layers>> {
    let Ok(text) = std::fs::read(json(path)) else { return Ok(None) };
    if !path.is_file() {
        return Ok(None);
    }
    let info: Info = serde_json::from_slice(&text)?;
    ensure!(
        info.format == FORMAT && !info.depths.is_empty(),
        "{}",
        said("depth.the_layer_set_is_not", "the layer set is not understood", &[]).local()
    );
    let atlas = read_png(path)?.into_rgba8();
    let (w, h) = (info.size[0], info.size[1]);
    ensure!(
        atlas.width() == w && atlas.height() == h * info.depths.len() as u32,
        "{}",
        said("depth.the_layer_set_is_torn", "the layer set is torn", &[]).local()
    );
    let cut = |k: u32| image::imageops::crop_imm(&atlas, 0, k * h, w, h).to_image();
    let deep = read_png(&fields_of(path))?.into_luma16();
    let (fw, fh, n) = (info.field[0], info.field[1], info.depths.len() as u32);
    ensure!(
        deep.width() == fw && deep.height() == fh * n,
        "{}",
        said("depth.the_depth_fields_are_torn", "the depth fields are torn", &[]).local()
    );
    let field = |k: u32| image::imageops::crop_imm(&deep, 0, k * fh, fw, fh).to_image();
    let (images, fields) = ((0..n).map(cut).collect(), (0..n).map(field).collect());
    Ok(Some(Layers { images, fields, info }))
}
