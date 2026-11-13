use crate::field::sample;
use crate::stack::Layers;
use image::imageops::{FilterType, resize};
use image::{Rgba, RgbaImage};
use wprs_core::UvRect;
use wprs_core::travel::{Reach, shift, travel};

pub struct Preview {
    size: (u32, u32),
    images: Vec<RgbaImage>,
    fields: Vec<Vec<f32>>,
    field: (usize, usize),
    reach: Reach,
}

fn texel(image: &RgbaImage, x: f32, y: f32) -> [f32; 4] {
    let (w, h) = (image.width() as usize, image.height() as usize);
    let (fx, fy) = ((x - 0.5).clamp(0.0, (w - 1) as f32), (y - 0.5).clamp(0.0, (h - 1) as f32));
    let (x0, y0) = (fx.floor() as usize, fy.floor() as usize);
    let (x1, y1) = ((x0 + 1).min(w - 1), (y0 + 1).min(h - 1));
    let (tx, ty) = (fx - x0 as f32, fy - y0 as f32);
    let mut out = [0.0f32; 4];
    for (px, py, weight) in [
        (x0, y0, (1.0 - tx) * (1.0 - ty)),
        (x1, y0, tx * (1.0 - ty)),
        (x0, y1, (1.0 - tx) * ty),
        (x1, y1, tx * ty),
    ] {
        let p = image.get_pixel(px as u32, py as u32);
        let a = f32::from(p[3]) / 255.0;
        out[0] += f32::from(p[0]) * a * weight;
        out[1] += f32::from(p[1]) * a * weight;
        out[2] += f32::from(p[2]) * a * weight;
        out[3] += a * weight;
    }
    out
}

impl Preview {
    pub fn new(layers: &Layers, edge: u32) -> Preview {
        let [w, h] = layers.info.size;
        let scale = (edge as f32 / w.max(h) as f32).min(1.0);
        let size = (((w as f32 * scale).round() as u32).max(1), ((h as f32 * scale).round() as u32).max(1));
        let images = layers.images.iter().map(|i| resize(i, size.0, size.1, FilterType::Triangle)).collect();
        let fields =
            layers.fields.iter().map(|f| f.as_raw().iter().map(|v| f32::from(*v) / 65535.0).collect());
        let field = (layers.info.field[0] as usize, layers.info.field[1] as usize);
        Preview { size, images, fields: fields.collect(), field, reach: layers.info.reach }
    }

    pub fn reach(&self) -> Reach {
        self.reach
    }

    pub fn frame(&self, pointer: [f32; 2], (strength, focus, overscan): (f32, f32, f32)) -> RgbaImage {
        let (w, h) = self.size;
        let aspect = w as f32 / h as f32;
        let whole = UvRect { offset: [0.0, 0.0], scale: [1.0, 1.0], repeat: false };
        let way = travel(strength, &self.reach, (focus, overscan), (aspect, &whole));
        let moved = shift(pointer, way.shift, aspect);
        let zoom = way.zoom.max(1.0);
        let level = |i: usize, u: f32, v: f32| {
            let (px, py) = (u.clamp(0.0, 1.0 - f32::EPSILON), v.clamp(0.0, 1.0 - f32::EPSILON));
            sample(
                &self.fields[i],
                self.field,
                (w as usize, h as usize),
                (px * w as f32) as usize,
                (py * h as f32) as usize,
            )
        };
        RgbaImage::from_fn(w, h, |x, y| {
            let (u, v) = (
                0.5 + ((x as f32 + 0.5) / w as f32 - 0.5) / zoom,
                0.5 + ((y as f32 + 0.5) / h as f32 - 0.5) / zoom,
            );
            let mut rgb = [0.0f32; 3];
            for (i, image) in self.images.iter().enumerate() {
                let first =
                    (u - moved[0] * (level(i, u, v) - focus), v - moved[1] * (level(i, u, v) - focus));
                let at = (
                    first.0 - moved[0] * (level(i, first.0, first.1) - focus),
                    first.1 - moved[1] * (level(i, first.0, first.1) - focus),
                );
                let c = texel(image, at.0.clamp(0.0, 1.0) * w as f32, at.1.clamp(0.0, 1.0) * h as f32);
                let a = c[3].clamp(0.0, 1.0);
                for k in 0..3 {
                    rgb[k] = rgb[k] * (1.0 - a) + c[k];
                }
            }
            Rgba([
                rgb[0].round().clamp(0.0, 255.0) as u8,
                rgb[1].round().clamp(0.0, 255.0) as u8,
                rgb[2].round().clamp(0.0, 255.0) as u8,
                255,
            ])
        })
    }
}
