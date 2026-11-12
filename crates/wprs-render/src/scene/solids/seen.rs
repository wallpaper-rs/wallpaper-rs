use super::pipes::Key;
use super::{DEPTH, GpuModel, Staged};
use crate::Renderer;
use crate::texture::create_texture;
use std::collections::HashMap;
use wgpu::TextureUsages as U;

impl Renderer {
    fn seen_target(&self, staged: &mut Staged) {
        if staged.seen.is_some() {
            return;
        }
        let usage = U::RENDER_ATTACHMENT | U::TEXTURE_BINDING;
        let texture = create_texture(&self.device, "wprs-render seen depth", staged.size, DEPTH, usage);
        let view = texture.create_view(&Default::default());
        let group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("wprs-render seen"),
            layout: &self.solids().seen,
            entries: &[wgpu::BindGroupEntry {
                binding: 7,
                resource: wgpu::BindingResource::TextureView(&view),
            }],
        });
        (staged.seen, staged.seen_view) = (Some(group), Some(view));
    }

    pub(super) fn seen_pass(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        staged: &mut Staged,
        models: &HashMap<u64, GpuModel>,
    ) {
        self.seen_target(staged);
        let (Some(view), Some((_, groups))) = (&staged.seen_view, &staged.groups) else { return };
        let depth = wgpu::RenderPassDepthStencilAttachment {
            view,
            depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(0.0), store: wgpu::StoreOp::Store }),
            stencil_ops: None,
        };
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("wprs-render seen"),
            depth_stencil_attachment: Some(depth),
            ..Default::default()
        });
        pass.set_bind_group(0, &groups[2], &[0]);
        let solid = staged.calls.iter().filter(|c| c.key.blend == 0 && c.key.write && !c.key.prime);
        let keyed = solid.map(|c| (*c, Key { shadow: true, prime: true, package: 0, test: true, ..c.key }));
        self.draw_calls(&mut pass, staged, models, keyed);
    }
}
