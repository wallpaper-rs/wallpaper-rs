use super::dress::drawn;
use super::{GpuModel, Staged};
use crate::Renderer;
use crate::texture::{gpu_image, texture};
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use wgpu::TextureView;
use wprs_scene::{Frame, Look, Stage3d};

pub(super) fn noise(r: &Renderer) -> crate::texture::GpuImage {
    let pixels = super::super::state::generic("noise");
    let (w, h) = pixels.dimensions();
    gpu_image(texture(r, pixels.as_raw(), (w, h), w * 4, wgpu::TextureFormat::Rgba8Unorm))
}

pub(super) fn key(look: Option<&Look<'_>>) -> u64 {
    let Some(images) = look.map(|l| &l.images).filter(|i| i.iter().any(Option::is_some)) else { return 0 };
    let mut h = DefaultHasher::new();
    images.hash(&mut h);
    h.finish() | 1
}

impl Renderer {
    fn picture<'v>(&'v self, images: &'v HashMap<String, super::super::Image>, id: &str) -> &'v TextureView {
        let solids = self.solids();
        match images.get(id) {
            Some(found) => &found.0.view,
            None => match id {
                "black" => &solids.black.view,
                "normal_flat" => &solids.flat.view,
                "noise" => &solids.noise.view,
                _ => &solids.white.view,
            },
        }
    }

    pub(super) fn pictured(
        &self,
        staged: &mut Staged,
        (models, images): (&HashMap<u64, GpuModel>, &HashMap<String, super::super::Image>),
        frame: &Frame<'_>,
        stage: &Stage3d,
    ) {
        let placed = &frame.placed[stage.models.clone()];
        let defaults = wprs_scene::Finish::default();
        for p in placed {
            let Some(gpu) = models.get(&drawn(p)) else { continue };
            for look in &frame.looks[p.looks.clone()] {
                let at = (drawn(p), usize::from(look.material), key(Some(look)));
                if at.2 == 0 || staged.pictured.get(&at).is_some_and(|(seen, _)| *seen == gpu.dressed) {
                    continue;
                }
                let (Some(own), Some(comb), Some(uvs)) =
                    (gpu.views.get(at.1), gpu.combs.get(at.1), gpu.uvs.get(at.1))
                else {
                    continue;
                };
                let finish = p.model.materials.get(at.1).unwrap_or(&defaults);
                let shown = |k: usize| self.picture(images, look.images[k].unwrap_or("white"));
                let views: [&TextureView; 5] = std::array::from_fn(|k| &own[k]);
                let group =
                    self.maps_group((views, comb, [shown(0), shown(1), shown(2), shown(3)]), finish, uvs);
                staged.pictured.insert(at, (gpu.dressed, group));
            }
        }
        let used = |at: &(u64, usize, u64)| {
            placed.iter().any(|p| {
                let looks = &frame.looks[p.looks.clone()];
                drawn(p) == at.0
                    && looks.iter().any(|l| usize::from(l.material) == at.1 && key(Some(l)) == at.2)
            })
        };
        staged.pictured.retain(|at, _| used(at));
    }
}
