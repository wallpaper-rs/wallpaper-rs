use super::Swarms;
use super::lamped::{SPRITE, VS, slots};
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
        let layouts = [&self.layouts[1], &self.layouts[2], &r.solids().seen];
        let shader = (&self.module, (VS[SPRITE], "fs_seen"));
        let made = self.build(r, mode, shader, &layouts, &slots(SPRITE));
        self.seens.insert(mode, made);
        self.created += 1;
    }
}
