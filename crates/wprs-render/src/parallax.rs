use crate::texture::GpuImage;
use half::f16;
use wprs_core::UvRect;

pub type DepthMap = image::ImageBuffer<image::Luma<u16>, Vec<u16>>;

#[derive(Clone, Copy)]
pub struct Relief<'a> {
    pub depth: &'a GpuImage,
    pub shift: [f32; 2],
    pub focus: f32,
    pub steps: u32,
    pub overscan: f32,
}

pub(crate) fn zoom(uv: UvRect, overscan: f32) -> UvRect {
    let z = overscan.max(1.0);
    let offset = uv.offset.map(|o| 0.5 + (o - 0.5) / z);
    UvRect { offset, scale: uv.scale.map(|s| s / z), repeat: false }
}

pub(crate) fn halves(map: &DepthMap) -> Vec<u16> {
    map.as_raw().iter().map(|v| f16::from_f32(f32::from(*v) / 65535.0).to_bits()).collect()
}
