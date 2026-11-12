use super::{B, LOOK, Look, Swarms, buffer};
use crate::Renderer;
use crate::scene::paint::Painter;
use wgpu::{BindGroup, Buffer, TextureView};
use wprs_scene::Content;

type Seen = (TextureView, [[f32; 4]; 2], [f32; 4]);

impl Painter<'_, '_, '_> {
    pub(super) fn seen(&self, stage: usize, far: f32) -> Option<Seen> {
        let frame = self.frame;
        let s = frame.stages.get(stage)?;
        let view = self.solids.get(s.layer)?.seen_view.clone()?;
        let model = |d: &wprs_scene::Draw| matches!(d.content, Content::Model { stage: t, .. } if t == stage);
        let at = frame.draws.iter().position(model)?;
        if self.lean(at).is_some() {
            return None;
        }
        let (size, _) = self.geometry(at)?;
        let (a, b) = crate::scene::item::rows(self.local(at, size).inverse());
        Some((view, [a, b], [s.eye.near, s.eye.far, far, 1.0]))
    }
}

pub(super) fn blank(r: &Renderer) -> TextureView {
    let usage = wgpu::TextureUsages::TEXTURE_BINDING;
    let format = wgpu::TextureFormat::Rgba8Unorm;
    crate::texture::create_texture(&r.device, "wprs-scene no depth", (1, 1), format, usage)
        .create_view(&Default::default())
}

impl Swarms {
    fn grouped(&self, r: &Renderer, buffer: &Buffer, view: &TextureView) -> BindGroup {
        let size = wgpu::BufferSize::new(size_of::<Look>() as u64);
        let look = wgpu::BindingResource::Buffer(wgpu::BufferBinding { buffer, offset: 0, size });
        let entries = [
            wgpu::BindGroupEntry { binding: 0, resource: look },
            wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&self.sampler) },
            wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::TextureView(view) },
        ];
        let desc = wgpu::BindGroupDescriptor { label: None, layout: &self.layouts[1], entries: &entries };
        r.device.create_bind_group(&desc)
    }

    pub(super) fn looked(&mut self, r: &Renderer, needed: u64) {
        if self.looked.as_ref().is_none_or(|(b, _)| b.size() < needed) {
            let buffer = buffer(r, needed.next_power_of_two().max(4 * LOOK), B::UNIFORM | B::COPY_DST);
            let group = self.grouped(r, &buffer, &self.blank);
            (self.looked, self.peeks) = (Some((buffer, group)), Vec::new());
            self.created += 2;
        }
        let seen = std::mem::take(&mut self.seen);
        self.peeks.retain(|(v, _)| seen.iter().flatten().any(|s| s == v));
        for view in seen.iter().flatten() {
            if self.peeks.iter().any(|(v, _)| v == view) {
                continue;
            }
            let Some((buffer, _)) = &self.looked else { break };
            let group = self.grouped(r, buffer, view);
            self.peeks.push((view.clone(), group));
            self.created += 1;
        }
        self.seen = seen;
    }

    pub(super) fn look_group(&self, at: usize) -> Option<&BindGroup> {
        let seen = self.seen.get(at).and_then(Option::as_ref);
        let peek = seen.and_then(|s| self.peeks.iter().find(|(v, _)| v == s)).map(|(_, g)| g);
        peek.or(self.looked.as_ref().map(|(_, g)| g))
    }
}
