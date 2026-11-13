use super::Staged;
use super::package::{FALLBACK, solid_source};
use super::pipes::{Key, build};
use crate::Renderer;
use crate::scene::swarm::staging::{Trial, left, within};
use std::collections::BTreeMap;
use std::sync::{Arc, PoisonError};
use wgpu::{BindGroupLayout, RenderPipeline, ShaderModule};
use wprs_scene::{Descriptor, Scene, Stage};

type Made = (ShaderModule, Vec<(Key, RenderPipeline)>);

fn made(
    device: &wgpu::Device,
    layouts: &[BindGroupLayout],
    (d, have): (&Descriptor, Option<ShaderModule>),
    wanted: &[Key],
) -> Result<Made, String> {
    let module = match have {
        Some(module) => module,
        None => {
            let source = wgpu::ShaderSource::Naga(std::borrow::Cow::Owned(wprs_scene::material_shader(
                d,
                &solid_source(),
            )?));
            let label = Some("wprs-scene material shader");
            device.create_shader_module(wgpu::ShaderModuleDescriptor { label, source })
        }
    };
    let pipes = wanted.iter().map(|k| (*k, build(device, layouts, *k, (&module, "wprs_")))).collect();
    Ok((module, pipes))
}

impl Renderer {
    pub(super) fn material_pipes(
        &self,
        scene: &Scene,
        staged: &mut Staged,
        refused: &mut BTreeMap<String, String>,
    ) {
        let solids = self.solids();
        let mut missing: Vec<Key> = Vec::new();
        for c in
            staged.calls.iter().filter(|c| c.key.package != 0 && !staged.refused.contains(&c.key.package))
        {
            let code = c.key.code();
            if solids.pipes.lock().unwrap_or_else(PoisonError::into_inner).contains_key(&code) {
                continue;
            }
            if !missing.iter().any(|k| k.code() == code) {
                missing.push(c.key);
            }
        }
        while let Some(first) = missing.first().map(|k| k.package) {
            let wanted: Vec<Key> = missing.iter().copied().filter(|k| k.package == first).collect();
            missing.retain(|k| k.package != first);
            if !self.material(scene, first, wanted, refused) {
                staged.refused.push(first);
            }
        }
    }

    fn material(
        &self,
        scene: &Scene,
        key: u64,
        wanted: Vec<Key>,
        refused: &mut BTreeMap<String, String>,
    ) -> bool {
        let solids = self.solids();
        let found = scene.loaded.values().find(|d| d.key == key && d.stage == Stage::Material).cloned();
        let id = found.as_ref().map_or_else(|| format!("{key:016x}"), |d| d.id.clone());
        let have = solids.packages.lock().unwrap_or_else(PoisonError::into_inner).get(&key).cloned();
        let built = found.ok_or_else(|| "it is not a material shader".to_owned()).and_then(|d| {
            if let Some(why) = left(key) {
                return Err(why);
            }
            let (tried, keys): (Arc<Descriptor>, Vec<Key>) = (d.clone(), wanted.clone());
            let trial: Trial = Box::new(move |device| {
                let layouts = super::layout::layouts(device);
                drop(made(device, &layouts, (&tried, None), &keys));
            });
            let layouts = solids.layouts.clone();
            within((key, self), Some(trial), move |device| made(device, &layouts, (&d, have), &wanted))
                .and_then(|m| m)
        });
        match built {
            Ok((module, pipes)) => {
                solids.packages.lock().unwrap_or_else(PoisonError::into_inner).insert(key, module);
                let mut cache = solids.pipes.lock().unwrap_or_else(PoisonError::into_inner);
                pipes.into_iter().for_each(|(k, p)| drop(cache.insert(k.code(), p)));
                true
            }
            Err(why) => {
                let why = format!("{FALLBACK}{why}");
                tracing::warn!("the material shader {id} was left out: {why}");
                refused.entry(id).or_insert(why);
                false
            }
        }
    }
}
