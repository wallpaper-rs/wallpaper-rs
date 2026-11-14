use crate::bind::Slot;
use crate::scene::swarm::staging::within;
use crate::texture::{create_texture, gpu_image};
use crate::{GpuImage, Layer, RenderError, Renderer};
use std::sync::Arc;
use wprs_scene::{MAX_PARAMS, Quality, Ready};

mod built;
mod passing;

pub use built::{Building, Built};
pub use passing::Passing;

const PANE: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;
const WORDS: usize = MAX_PARAMS * 4 + 4;

pub struct Crossing {
    pipeline: wgpu::RenderPipeline,
    group: wgpu::BindGroup,
    uniform: wgpu::Buffer,
    panes: [(wgpu::TextureView, Vec<Slot>); 2],
    out: GpuImage,
    words: [f32; WORDS],
    ready: Arc<Ready>,
}

impl Crossing {
    pub fn image(&self) -> &GpuImage {
        &self.out
    }

    pub fn size(&self) -> (u32, u32) {
        self.out.size()
    }

    pub fn ready(&self) -> &Arc<Ready> {
        &self.ready
    }
}

impl Renderer {
    pub fn crossing(
        &self,
        ready: Arc<Ready>,
        size: (u32, u32),
        quality: Quality,
    ) -> Result<Crossing, String> {
        let (naga, layout) = self.begin(&ready, quality)?;
        let made = within((ready.effect.key, self), None, {
            let layout = layout.clone();
            move |device| pipeline(device, &layout, naga)
        })?;
        self.crossing_of(&Built { pipeline: made, layout, ready, quality }, size)
    }

    pub fn crossing_of(&self, built: &Built, size: (u32, u32)) -> Result<Crossing, String> {
        let (ready, quality, layout) = (built.ready.clone(), built.quality, &built.layout);
        self.check_target(size.0, size.1).map_err(|e| e.to_string())?;
        let usage = wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING;
        let pane = |label| create_texture(&self.device, label, size, PANE, usage);
        let (before, after) = (pane("wprs-render crossing before"), pane("wprs-render crossing after"));
        let out = gpu_image(pane_out(&self.device, size));
        let uniform = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("wprs-render crossing uniform"),
            size: (WORDS * 4) as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let views = [before.create_view(&Default::default()), after.create_view(&Default::default())];
        let group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("wprs-render crossing"),
            layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: uniform.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&self.clamp) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::TextureView(&views[0]) },
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(&views[1]) },
            ],
        });
        let mut words = [0.0; WORDS];
        ready.slots.iter().flatten().zip(&mut words).for_each(|(v, w)| *w = *v);
        words[MAX_PARAMS * 4..].copy_from_slice(&[
            size.0 as f32,
            size.1 as f32,
            0.0,
            f32::from(quality as u8),
        ]);
        let [b, a] = views;
        let panes = [(b, Vec::new()), (a, Vec::new())];
        Ok(Crossing { pipeline: built.pipeline.clone(), group, uniform, panes, out, words, ready })
    }

    pub fn cross(
        &self,
        c: &mut Crossing,
        before: &[Layer<'_>],
        after: &[Layer<'_>],
        progress: f32,
    ) -> Result<(), RenderError> {
        let size = c.out.size();
        c.words[MAX_PARAMS * 4 + 2] = c.ready.at(progress);
        let drawn = self.scoped(|| {
            self.queue.write_buffer(&c.uniform, 0, bytemuck::cast_slice(&c.words));
            let [(bv, bs), (av, as_)] = &mut c.panes;
            let first = self.encode(bv, bs, before, size, PANE, None);
            let second = self.encode(av, as_, after, size, PANE, None);
            let mut encoder = self.device.create_command_encoder(&Default::default());
            let ops = wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                store: wgpu::StoreOp::Store,
            };
            let attachment = wgpu::RenderPassColorAttachment {
                view: &c.out.view,
                depth_slice: None,
                resolve_target: None,
                ops,
            };
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("wprs-render crossing pass"),
                color_attachments: &[Some(attachment)],
                ..Default::default()
            });
            pass.set_pipeline(&c.pipeline);
            pass.set_bind_group(0, &c.group, &[]);
            pass.draw(0..3, 0..1);
            drop(pass);
            self.queue.submit([first.finish(), second.finish(), encoder.finish()]);
        });
        drawn.inspect_err(|_| c.panes.iter_mut().for_each(|p| p.1.clear()))
    }
}

fn pane_out(device: &wgpu::Device, size: (u32, u32)) -> wgpu::Texture {
    let usage = wgpu::TextureUsages::RENDER_ATTACHMENT
        | wgpu::TextureUsages::TEXTURE_BINDING
        | wgpu::TextureUsages::COPY_SRC;
    create_texture(device, "wprs-render crossing out", size, PANE, usage)
}

pub(crate) fn layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    let entry = |binding, ty| wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty,
        count: None,
    };
    let picture = wgpu::BindingType::Texture {
        sample_type: wgpu::TextureSampleType::Float { filterable: true },
        view_dimension: wgpu::TextureViewDimension::D2,
        multisampled: false,
    };
    let uniform = wgpu::BindingType::Buffer {
        ty: wgpu::BufferBindingType::Uniform,
        has_dynamic_offset: false,
        min_binding_size: None,
    };
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("wprs-render crossing"),
        entries: &[
            entry(0, uniform),
            entry(1, wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering)),
            entry(2, picture),
            entry(3, picture),
        ],
    })
}

pub(crate) fn pipeline(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    naga: wgpu::naga::Module,
) -> wgpu::RenderPipeline {
    let source = wgpu::ShaderSource::Naga(std::borrow::Cow::Owned(naga));
    let label = Some("wprs-scene transition");
    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor { label, source });
    let target = wgpu::ColorTargetState { format: PANE, blend: None, write_mask: wgpu::ColorWrites::ALL };
    crate::shader::pipeline(device, layout, &module, "wprs_fs", target)
}
