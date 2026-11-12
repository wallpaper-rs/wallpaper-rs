use crate::Renderer;
use std::collections::BTreeMap;
use std::sync::PoisonError;

const FALLBACK: &str = "its models are shaded with pbr instead, since ";

const OWN: [&str; 61] = [
    "Light",
    "Scene3",
    "Object",
    "Bones",
    "Draw",
    "Shadows",
    "Finish",
    "In",
    "Bound",
    "Out",
    "Placed",
    "scene",
    "env",
    "env_smp",
    "shadow_a",
    "shadow_b",
    "shadow_cmp",
    "shade",
    "object",
    "bones",
    "draw",
    "morphs",
    "base_map",
    "orm_map",
    "normal_map",
    "occlusion_map",
    "emissive_map",
    "base_smp",
    "orm_smp",
    "normal_smp",
    "occlusion_smp",
    "emissive_smp",
    "uvs",
    "Uvs",
    "map_uv",
    "finish",
    "PI",
    "place",
    "skin",
    "delta",
    "out",
    "posed",
    "vs_solid",
    "vs_skinned",
    "vs_shadow",
    "vs_shadow_skinned",
    "ggx",
    "smith",
    "equirect",
    "falloff",
    "tap",
    "shadowed",
    "hashed",
    "Cut",
    "moved",
    "vs_shadow_cut",
    "vs_shadow_cut_skinned",
    "fs_shadow_cut",
    "shadow_c",
    "shadow_d",
    "round_tap",
];
const MORE: [&str; 3] = ["fs_solid", "shown", "lit"];

fn prefixed(src: &str) -> String {
    let (mut out, mut word) = (String::with_capacity(src.len() + 1024), String::new());
    for c in src.chars().chain([' ']) {
        if c.is_alphanumeric() || c == '_' {
            word.push(c);
            continue;
        }
        if OWN.contains(&word.as_str()) || MORE.contains(&word.as_str()) {
            out.push_str("wprs_");
        }
        out.push_str(&word);
        word.clear();
        out.push(c);
    }
    out.pop();
    out
}

pub(crate) fn solid_source() -> String {
    let own = prefixed(&super::SOURCE.concat());
    format!(
        "@group(0) @binding(3) var<uniform> frame: Frame;\nfn param(i: u32) -> vec4<f32> {{\n    return wprs_finish.params[min(i, 7u)];\n}}\n{own}"
    )
}

impl Renderer {
    pub(super) fn packages(
        &self,
        scene: &wprs_scene::Scene,
        keys: impl Iterator<Item = u64>,
        refused: &mut BTreeMap<String, String>,
    ) {
        let solids = self.solids();
        for key in keys {
            let packages = solids.packages.lock().unwrap_or_else(PoisonError::into_inner);
            if let Some(Err((id, why))) = packages.get(&key) {
                refused.entry(id.clone()).or_insert_with(|| why.clone());
            }
            if packages.contains_key(&key) {
                continue;
            }
            drop(packages);
            let found =
                scene.loaded.values().find(|d| d.key == key && d.stage == wprs_scene::Stage::Material);
            let id = found.map_or_else(|| format!("{key:016x}"), |d| d.id.clone());
            let made = found.ok_or_else(|| "it is not a material shader".to_owned()).and_then(|d| {
                let module = wprs_scene::material_shader(d, &solid_source())?;
                let source = wgpu::ShaderSource::Naga(std::borrow::Cow::Owned(module));
                let label = Some("wprs-scene material shader");
                Ok(self.device.create_shader_module(wgpu::ShaderModuleDescriptor { label, source }))
            });
            let made = made.map_err(|why| (id, format!("{FALLBACK}{why}")));
            if let Err((id, why)) = &made {
                tracing::warn!("the material shader {id} was left out: {why}");
                refused.entry(id.clone()).or_insert_with(|| why.clone());
            }
            solids.packages.lock().unwrap_or_else(PoisonError::into_inner).insert(key, made);
        }
    }
}
