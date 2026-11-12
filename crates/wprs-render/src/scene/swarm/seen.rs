use super::{PARTICLE, Swarms};
use crate::Renderer;
use crate::scene::paint::Painter;
use crate::scene::pipes::Mode;
use wprs_scene::Content;

type Seen = (wgpu::BindGroup, [[f32; 4]; 2], [f32; 4]);

impl Painter<'_, '_, '_> {
    pub(super) fn seen(&self, stage: usize, far: f32) -> Option<Seen> {
        let frame = self.frame;
        let s = frame.stages.get(stage)?;
        let group = self.solids.get(s.layer)?.seen.clone()?;
        let model = |d: &wprs_scene::Draw| matches!(d.content, Content::Model { stage: t, .. } if t == stage);
        let at = frame.draws.iter().position(model)?;
        if self.lean(at).is_some() {
            return None;
        }
        let (size, _) = self.geometry(at)?;
        let (a, b) = crate::scene::item::rows(self.local(at, size).inverse());
        Some((group, [a, b], [s.eye.near, s.eye.far, far, 1.0]))
    }
}

impl Swarms {
    pub(super) fn seen_pipeline(&mut self, r: &Renderer, mode: Mode) {
        if self.seens.contains_key(&mode) {
            return;
        }
        let sprites = wgpu::vertex_attr_array![0 => Float32x4, 1 => Float32x4, 2 => Float32x4, 3 => Float32x4, 4 => Float32x4, 5 => Float32x4];
        let layouts = [&self.layouts[1], &self.layouts[2], &r.solids().seen];
        let shader = (&self.module, ("vs_particle", "fs_seen"));
        let made = self.build(r, mode, shader, &layouts, &[(PARTICLE, &sprites)]);
        self.seens.insert(mode, made);
        self.created += 1;
    }
}
