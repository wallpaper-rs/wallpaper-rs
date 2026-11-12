use super::Swarms;
use super::lamped::{LONG, ROPE, SPRITE, TRAIL, VS, slots};
use crate::Renderer;
use crate::scene::pipes::{Mode, blend};

impl Swarms {
    pub(super) fn pipeline(&mut self, r: &Renderer, mode: Mode) {
        if self.pipelines.contains_key(&mode) {
            return;
        }
        let layouts = [&self.layouts[1], &self.layouts[2]];
        let module = &self.module;
        let (sprites, ribbons) = (slots(SPRITE), slots(TRAIL));
        let sprite = self.build(r, mode, (module, (VS[SPRITE], "fs_particle")), &layouts, &sprites);
        let ribbon = self.build(r, mode, (module, (VS[TRAIL], "fs_trail")), &layouts, &ribbons);
        self.pipelines.insert(mode, sprite);
        self.ribbons.insert(mode, ribbon);
        self.created += 2;
        self.lamped(r, mode);
        if self.kits.iter().any(|k| k.rope.is_some() || k.long) {
            self.corded(r, mode);
        }
        self.staged(r, mode);
    }

    fn corded(&mut self, r: &Renderer, mode: Mode) {
        let layouts = [&self.layouts[1], &self.layouts[2]];
        let module = &self.module;
        let rope = self.build(r, mode, (module, (VS[ROPE], "fs_cord")), &layouts, &slots(ROPE));
        let long = self.build(r, mode, (module, (VS[LONG], "fs_cord")), &layouts, &slots(LONG));
        self.cords.insert(mode, [rope, long]);
        self.created += 2;
    }

    pub(super) fn build(
        &self,
        r: &Renderer,
        mode: Mode,
        shader: (&wgpu::ShaderModule, (&str, &str)),
        groups: &[&wgpu::BindGroupLayout],
        slots: &[(u64, Vec<wgpu::VertexAttribute>)],
    ) -> wgpu::RenderPipeline {
        pipe(&r.device, (self.format, mode), shader, groups, slots)
    }
}

pub(super) fn pipe(
    device: &wgpu::Device,
    (format, mode): (wgpu::TextureFormat, Mode),
    (module, (vs, fs)): (&wgpu::ShaderModule, (&str, &str)),
    groups: &[&wgpu::BindGroupLayout],
    slots: &[(u64, Vec<wgpu::VertexAttribute>)],
) -> wgpu::RenderPipeline {
    let layouts: Vec<_> = groups.iter().map(|g| Some(*g)).collect();
    let desc =
        wgpu::PipelineLayoutDescriptor { label: None, bind_group_layouts: &layouts, immediate_size: 0 };
    let layout = device.create_pipeline_layout(&desc);
    let step_mode = wgpu::VertexStepMode::Instance;
    let buffers: Vec<_> = slots
        .iter()
        .map(|(array_stride, attributes)| {
            Some(wgpu::VertexBufferLayout { array_stride: *array_stride, step_mode, attributes })
        })
        .collect();
    let target = wgpu::ColorTargetState { blend: blend(mode), ..format.into() };
    let compilation_options = wgpu::PipelineCompilationOptions::default();
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("wprs-scene particles"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module,
            entry_point: Some(vs),
            compilation_options: compilation_options.clone(),
            buffers: &buffers,
        },
        fragment: Some(wgpu::FragmentState {
            module,
            entry_point: Some(fs),
            compilation_options,
            targets: &[Some(target)],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleStrip,
            ..Default::default()
        },
        depth_stencil: None,
        multisample: Default::default(),
        multiview_mask: None,
        cache: None,
    })
}
