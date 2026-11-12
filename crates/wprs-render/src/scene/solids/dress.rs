use super::super::Image;
use super::GpuModel;
use crate::Renderer;
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hasher};
use wprs_scene::{Kind, Placed};

pub(crate) fn drawn(p: &Placed) -> u64 {
    p.model.digest ^ p.maps.rotate_left(17)
}

fn picks<'a>(p: &'a Placed, m: usize) -> [Option<&'a str>; 6] {
    let Kind::Model(layer) = &p.layer.kind else { return [None; 6] };
    let name = p.model.materials.get(m).map(|f| f.name.as_str());
    name.and_then(|n| layer.materials.get(n)).map_or([None; 6], |mat| {
        let [a, b, c, d, e] = mat.maps.slots();
        [a, b, c, d, e, mat.maps.comb.as_deref()]
    })
}

fn ids(p: &Placed, images: &HashMap<String, Image>) -> u64 {
    let mut h = DefaultHasher::new();
    for m in 0..p.model.materials.len() {
        picks(p, m).iter().for_each(|id| h.write_u64(id.and_then(|i| images.get(i)).map_or(0, |i| i.0.id)));
    }
    h.finish()
}

impl Renderer {
    pub(super) fn dress(
        &self,
        models: &mut HashMap<u64, GpuModel>,
        p: &Placed,
        images: &HashMap<String, Image>,
    ) {
        let key = drawn(p);
        if p.maps == 0 {
            return;
        }
        let seen = ids(p, images);
        if models.get(&key).is_some_and(|g| g.dressed == seen) {
            return;
        }
        let Some(base) = models.get(&p.model.digest) else { return };
        let mut made = base.clone();
        made.dressed = seen;
        let defaults = wprs_scene::Finish::default();
        for (m, f) in p.model.materials.iter().chain([&defaults]).enumerate() {
            let picked = picks(p, m);
            if picked.iter().all(Option::is_none) {
                continue;
            }
            let views: [&wgpu::TextureView; 5] = std::array::from_fn(|k| {
                let over = picked[k].and_then(|i| images.get(i)).map(|i| &i.0.view);
                over.unwrap_or(&base.views[m][k])
            });
            let comb =
                picked[5].and_then(|i| images.get(i)).map_or(&self.solids().no_comb.view, |i| &i.0.view);
            let white = &self.solids().white.view;
            made.maps[m] = self.maps_group((views, comb, [white; 4]), f, &base.uvs[m]);
            made.views[m] = views.map(Clone::clone);
            made.combs[m] = comb.clone();
        }
        models.insert(key, made);
    }
}
