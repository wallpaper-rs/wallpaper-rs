use super::kit::Kit;
use super::sprite::pipe;
use super::{PARTICLE, Swarms};
use crate::Renderer;
use crate::scene::pipes::Mode;
use wgpu::VertexAttribute;

pub(super) const SPRITE: usize = 0;
pub(super) const TRAIL: usize = 1;
pub(super) const ROPE: usize = 2;
pub(super) const LONG: usize = 3;

pub(super) const VS: [&str; 4] = ["vs_particle", "vs_trail", "vs_rope", "vs_long"];
const LIT: [&str; 4] = ["fs_lit_particle", "fs_lit_trail", "fs_lit_cord", "fs_lit_cord"];
const BEND: [&str; 4] = ["fs_bend_particle", "fs_bend_trail", "fs_bend_cord", "fs_bend_cord"];

pub(super) fn slots(kind: usize) -> Vec<(u64, Vec<VertexAttribute>)> {
    match kind {
        SPRITE => vec![(
            PARTICLE,
            wgpu::vertex_attr_array![0 => Float32x4, 1 => Float32x4, 2 => Float32x4, 3 => Float32x4, 4 => Float32x4, 5 => Float32x4].to_vec(),
        )],
        TRAIL => vec![
            (16, wgpu::vertex_attr_array![0 => Float32x2, 1 => Float16x2, 2 => Unorm8x4].to_vec()),
            (16, wgpu::vertex_attr_array![3 => Float32x2, 4 => Float16x2, 5 => Unorm8x4].to_vec()),
        ],
        ROPE => vec![
            (48, wgpu::vertex_attr_array![0 => Float32x4, 1 => Float32x4, 2 => Float32x4].to_vec()),
            (48, wgpu::vertex_attr_array![3 => Float32x4, 4 => Float32x4, 5 => Float32x4].to_vec()),
        ],
        _ => vec![
            (16, wgpu::vertex_attr_array![0 => Float32x2, 1 => Float16x2].to_vec()),
            (16, wgpu::vertex_attr_array![3 => Float32x2, 4 => Float16x2, 5 => Unorm8x4].to_vec()),
            (16, wgpu::vertex_attr_array![6 => Float32x2, 7 => Float16x2, 8 => Unorm8x4].to_vec()),
            (16, wgpu::vertex_attr_array![9 => Float32x2, 10 => Float16x2].to_vec()),
        ],
    }
}

fn kinds(k: &Kit) -> [bool; 4] {
    [true, k.trail > 0 && !k.long, k.rope.is_some(), k.long]
}

pub(super) fn lit(k: &Kit) -> bool {
    k.lit.is_some() && k.bend.is_none()
}

impl Swarms {
    pub(super) fn lamped(&mut self, r: &Renderer, mode: Mode) {
        let layouts = [&self.layouts[1], &self.layouts[2], &self.layouts[6]];
        for kind in 0..4 {
            let wanted = |k: &Kit| kinds(k)[kind];
            if self.kits.iter().any(|k| lit(k) && wanted(k)) && !self.lits.contains_key(&(mode, kind)) {
                let made = self.build(r, mode, (&self.module, (VS[kind], LIT[kind])), &layouts, &slots(kind));
                self.lits.insert((mode, kind), made);
                self.created += 1;
            }
            if self.kits.iter().any(|k| k.bend.is_some() && wanted(k)) && self.bends[kind].is_none() {
                let shader = (&self.module, (VS[kind], BEND[kind]));
                let format = (wgpu::TextureFormat::Rgba16Float, Mode::Normal);
                self.bends[kind] = Some(pipe(&r.device, format, shader, &layouts, &slots(kind)));
                self.created += 1;
            }
        }
    }
}
