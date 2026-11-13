use crate::parallax::DepthMap;
use crate::shader::{Pairs, layout_of, module};
use crate::texture::{GpuImage, next_id};
use crate::{RenderError, Renderer};
use image::RgbaImage;
use std::sync::Arc;
use wgpu::{TextureDimension, TextureFormat, TextureUsages, TextureViewDimension as Dim};
use wprs_core::Edit;

pub const MOST_LAYERS: usize = 8;

fn split(map: &DepthMap) -> Vec<u8> {
    map.as_raw().iter().flat_map(|v| [(v >> 8) as u8, (v & 255) as u8]).collect()
}

impl Renderer {
    pub(crate) fn decks(&self) -> &Pairs {
        self.decks.get_or_init(|| {
            let shader =
                module(&self.device, &[include_str!("grade.wgsl"), include_str!("deck.wgsl")].concat());
            Pairs::new(layout_of(&self.device, &[Dim::D2, Dim::D2Array, Dim::D2Array]), shader, "fs_deck")
        })
    }

    fn blank(&self, (width, height): (u32, u32), format: TextureFormat, count: usize) -> wgpu::Texture {
        let size = wgpu::Extent3d { width, height, depth_or_array_layers: count as u32 };
        let made =
            wgpu::Extent3d { depth_or_array_layers: self.array_layers(size.depth_or_array_layers), ..size };
        self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("wprs-render layers"),
            size: made,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        })
    }

    fn array(
        &self,
        (width, height): (u32, u32),
        format: TextureFormat,
        planes: &[&[u8]],
    ) -> (wgpu::Texture, wgpu::TextureView) {
        let texture = self.blank((width, height), format, planes.len());
        let size = wgpu::Extent3d { width, height, depth_or_array_layers: planes.len() as u32 };
        let texel = format.block_copy_size(None).unwrap_or(4);
        for (z, plane) in planes.iter().enumerate() {
            let mut at = texture.as_image_copy();
            at.origin.z = z as u32;
            let rows = wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(width * texel),
                rows_per_image: Some(height),
            };
            self.queue.write_texture(at, plane, rows, wgpu::Extent3d { depth_or_array_layers: 1, ..size });
        }
        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(Dim::D2Array),
            ..Default::default()
        });
        (texture, view)
    }

    pub fn upload_layers(
        &self,
        layers: &[RgbaImage],
        fields: &[DepthMap],
        levels: &[f32],
    ) -> Result<GpuImage, RenderError> {
        self.layered(layers, fields, levels, None)
    }

    pub fn develop_layers(
        &self,
        (layers, fields, levels): (&[RgbaImage], &[DepthMap], &[f32]),
        edit: &Edit,
        depth: Option<&DepthMap>,
    ) -> Result<GpuImage, RenderError> {
        let flat = Edit { geometry: Default::default(), ..edit.clone() };
        self.layered(layers, fields, levels, Some((&flat, depth)))
    }

    fn developed_array(
        &self,
        layers: &[RgbaImage],
        (edit, depth): (&Edit, Option<&DepthMap>),
    ) -> Result<(wgpu::Texture, wgpu::TextureView), RenderError> {
        let size = layers[0].dimensions();
        let format = TextureFormat::Rgba8UnormSrgb;
        let texture = self.blank(size, format, layers.len());
        let mut encoder = self.device.create_command_encoder(&Default::default());
        for (z, layer) in layers.iter().enumerate() {
            let done = self.developed(layer, edit, depth, (size, true))?;
            let mut to = texture.as_image_copy();
            to.origin.z = z as u32;
            let extent = wgpu::Extent3d { width: size.0, height: size.1, depth_or_array_layers: 1 };
            encoder.copy_texture_to_texture(done.as_image_copy(), to, extent);
        }
        self.queue.submit([encoder.finish()]);
        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(Dim::D2Array),
            ..Default::default()
        });
        Ok((texture, view))
    }

    fn layered(
        &self,
        layers: &[RgbaImage],
        fields: &[DepthMap],
        levels: &[f32],
        develop: Option<(&Edit, Option<&DepthMap>)>,
    ) -> Result<GpuImage, RenderError> {
        let paired = layers.len() == levels.len() && layers.len() == fields.len();
        let first = layers.first().map(RgbaImage::dimensions).filter(|_| paired);
        let (width, height) = first.ok_or(RenderError::InvalidSize(0, 0))?;
        let small = fields.first().map_or((0, 0), DepthMap::dimensions);
        self.check_target(width, height)?;
        let torn = layers.iter().any(|l| l.dimensions() != (width, height))
            || fields.iter().any(|f| f.dimensions() != small || small.0 == 0 || small.1 == 0);
        if layers.len() > MOST_LAYERS || torn {
            return Err(RenderError::InvalidSize(width, height));
        }
        self.scoped(|| {
            let format = TextureFormat::Rgba8UnormSrgb;
            let colours: Vec<&[u8]> = layers.iter().map(|l| l.as_raw().as_slice()).collect();
            let (texture, view) = match develop {
                Some(with) => self.developed_array(layers, with)?,
                None => self.array((width, height), format, &colours),
            };
            let depths: Vec<Vec<u8>> = fields.iter().map(split).collect();
            let planes: Vec<&[u8]> = depths.iter().map(Vec::as_slice).collect();
            let fields = Some(self.array(small, TextureFormat::Rg8Unorm, &planes).1);
            let levels = Some(Arc::from(levels));
            Ok(GpuImage { texture, view, id: next_id(), format, crop: [1.0; 2], levels, fields })
        })?
    }
}
