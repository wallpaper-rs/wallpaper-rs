use crate::Renderer;
use std::sync::PoisonError;
use wgpu::BindingResource as R;
use wprs_scene::{Finish, MapWrap, Sampling};

pub(crate) const UV_ROWS: usize = 10;

pub(super) fn uv_rows(f: &Finish) -> [[f32; 4]; UV_ROWS] {
    let mut rows = [[0.0; 4]; UV_ROWS];
    for (k, uv) in f.uvs.iter().enumerate() {
        let m = &uv.matrix;
        rows[k * 2] = [m[0], m[1], m[2], f32::from(uv.set)];
        rows[k * 2 + 1] = [m[3], m[4], m[5], 0.0];
    }
    rows
}

fn address(w: MapWrap) -> wgpu::AddressMode {
    match w {
        MapWrap::Repeat => wgpu::AddressMode::Repeat,
        MapWrap::Clamp => wgpu::AddressMode::ClampToEdge,
        MapWrap::Mirror => wgpu::AddressMode::MirrorRepeat,
    }
}

fn filter(sharp: bool) -> wgpu::FilterMode {
    if sharp { wgpu::FilterMode::Nearest } else { wgpu::FilterMode::Linear }
}

impl Renderer {
    pub(super) fn map_sampler(&self, s: Sampling) -> wgpu::Sampler {
        let solids = self.solids();
        let mut kept = solids.samplers.lock().unwrap_or_else(PoisonError::into_inner);
        let made = kept.entry(s).or_insert_with(|| {
            self.device.create_sampler(&wgpu::SamplerDescriptor {
                label: Some("wprs-render solid maps"),
                address_mode_u: address(s.wrap[0]),
                address_mode_v: address(s.wrap[1]),
                mag_filter: filter(s.sharp),
                min_filter: filter(s.shrink_sharp),
                mipmap_filter: match s.mips {
                    Some(true) => wgpu::MipmapFilterMode::Nearest,
                    _ => wgpu::MipmapFilterMode::Linear,
                },
                lod_max_clamp: if s.mips.is_some() { 32.0 } else { 0.0 },
                ..Default::default()
            })
        });
        made.clone()
    }

    pub(super) fn maps_group(
        &self,
        views: [&wgpu::TextureView; 5],
        f: &Finish,
        uvs: &wgpu::Buffer,
    ) -> wgpu::BindGroup {
        let samplers = f.uvs.map(|u| self.map_sampler(u.sampling));
        let mut entries: Vec<wgpu::BindGroupEntry> = views
            .iter()
            .enumerate()
            .map(|(k, v)| wgpu::BindGroupEntry { binding: k as u32, resource: R::TextureView(v) })
            .collect();
        entries.extend(
            samplers
                .iter()
                .enumerate()
                .map(|(k, s)| wgpu::BindGroupEntry { binding: 5 + k as u32, resource: R::Sampler(s) }),
        );
        entries.push(wgpu::BindGroupEntry { binding: 10, resource: uvs.as_entire_binding() });
        self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("wprs-render solid maps"),
            layout: &self.solids().layouts[2],
            entries: &entries,
        })
    }
}
