use super::Swarms;
use super::kit::Kit;
use crate::Renderer;
use crate::scene::Image;
use crate::scene::pool::Pool;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use wprs_scene::Render;

pub(crate) struct Lamp<'a> {
    pub(crate) images: &'a HashMap<String, Image>,
    pub(crate) white: &'a wgpu::TextureView,
    pub(crate) layers: bool,
}

pub(crate) struct Lamps<'a> {
    pub(crate) device: &'a wgpu::Device,
    pub(crate) pool: &'a mut Pool,
    pub(crate) cast: [(u64, &'a wgpu::TextureView); 4],
}

static SERIAL: AtomicU64 = AtomicU64::new(1);

pub(super) fn serial() -> u64 {
    SERIAL.fetch_add(1, Ordering::Relaxed)
}

fn flag(b: bool) -> f32 {
    f32::from(u8::from(b))
}

pub(super) fn light(r: &Renderer, kit: &mut Kit, (render, sprite): (&Render, [f32; 2]), lamp: &Lamp<'_>) {
    let [sx, sy] = sprite;
    kit.look.crops = [1.0 / sx, 1.0 / sy, 1.0 / sx, 1.0 / sy];
    let bend = render.refract.as_ref();
    if !render.lit && render.normal.is_none() && bend.is_none() && !lamp.layers {
        return;
    }
    let exact = |id: Option<&String>| {
        let found = id.and_then(|id| lamp.images.get(id)).map(|i| &i.0);
        found.and_then(|i| r.exact(i)).or_else(|| found.cloned())
    };
    let (normal, bent) = (exact(render.normal.as_ref()), exact(bend.map(|b| &b.normal)));
    let view = |i: Option<crate::GpuImage>| i.map_or_else(|| lamp.white.clone(), |i| i.view);
    kit.lit = Some([view(normal), view(bent)]);
    let normal = render.normal.as_ref().is_some_and(|id| lamp.images.contains_key(id));
    kit.look.light = [flag(render.lit), flag(normal), 0.0, 1.0];
    kit.bend = bend.map(|b| b.amount.clamp(0.0, 1.0));
    kit.look.bend = [kit.bend.unwrap_or(0.0), 0.0, 0.0, 0.0];
}

impl Swarms {
    pub(super) fn lamp(&self, k: usize, lamps: &mut Lamps<'_>) -> Option<wgpu::BindGroup> {
        let [normal, bent] = self.kits.get(k)?.lit.as_ref()?;
        let mut key = [6; 12];
        key[1] = 1 << 63 | self.serial << 24 | k as u64;
        lamps.cast.iter().zip(&mut key[2..]).for_each(|((id, _), slot)| *slot = *id);
        let (layout, cast, device) = (&self.layouts[6], lamps.cast, lamps.device);
        Some(lamps.pool.cached(key, |pool| {
            let texture = wgpu::BindingResource::TextureView;
            let first = [pool.glow.buffer.as_entire_binding(), texture(normal), texture(bent)];
            let entries: Vec<_> = first
                .into_iter()
                .chain(cast.iter().map(|(_, v)| texture(v)))
                .enumerate()
                .map(|(i, resource)| wgpu::BindGroupEntry { binding: i as u32, resource })
                .collect();
            device.create_bind_group(&wgpu::BindGroupDescriptor { label: None, layout, entries: &entries })
        }))
    }
}
