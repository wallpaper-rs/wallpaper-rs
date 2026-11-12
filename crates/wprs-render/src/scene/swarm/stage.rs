use super::kit::Kit;
use super::lamped::{SPRITE, slots};
use super::sprite::pipe;
use super::staging::{Trial, left, within};
use super::{Swarms, binds};
use crate::Renderer;
use crate::scene::pipes::Mode;
use std::sync::Arc;
use wprs_scene::{Scene, Stage};

const ENTRIES: [&str; 3] = ["wprs_fs_particle", "wprs_fs_lit_particle", "wprs_fs_bend_particle"];

const OWN: [&str; 29] = [
    "Look",
    "look",
    "Corner",
    "samp",
    "sprite",
    "rects",
    "curve",
    "decode",
    "encode",
    "cell",
    "vs_particle",
    "fs_particle",
    "painted",
    "Bulb",
    "Lights",
    "lights",
    "reach",
    "cast_at",
    "cast_from",
    "cast_a",
    "cast_b",
    "cast_c",
    "cast_d",
    "normal_map",
    "bend_map",
    "turned",
    "lamp_lit",
    "fs_lit_particle",
    "fs_bend_particle",
];

fn source() -> String {
    let lit = include_str!("lit.wgsl").replace("@group(2)", "@group(3)");
    prefixed(&[include_str!("sprite.wgsl"), include_str!("../lamps.wgsl"), &lit].join("\n"))
}

fn prefixed(src: &str) -> String {
    let (mut out, mut word) = (String::with_capacity(src.len() + 512), String::new());
    for c in src.chars().chain([' ']) {
        if c.is_alphanumeric() || c == '_' {
            word.push(c);
            continue;
        }
        if OWN.contains(&word.as_str()) {
            out.push_str("wprs_");
        }
        out.push_str(&word);
        word.clear();
        out.push(c);
    }
    out.pop();
    out
}

pub(super) fn stage(r: &Renderer, scene: &Scene, kit: &mut Kit, id: Option<&str>) {
    let Some(id) = id else { return };
    let Some(d) = scene.loaded.get(id).filter(|d| d.stage == Stage::Particle) else {
        kit.refused = Some((id.into(), "it is not a particle shader".into()));
        return;
    };
    let made = left(d.key).map_or(Ok(()), Err).and_then(|()| {
        let naga = Arc::new(wprs_scene::particle_shader(d, &source())?);
        within((d.key, r), None, move |device| (shader(device, &naga), naga))
    });
    match made {
        Ok((module, naga)) => kit.stage = Some((d.key, module, naga)),
        Err(why) => kit.refused = Some((id.into(), why)),
    }
}

pub(super) fn leave(kits: &mut [Kit], key: u64, why: &str) {
    for kit in kits.iter_mut().filter(|k| k.stage.as_ref().is_some_and(|s| s.0 == key)) {
        tracing::warn!("the particle shader of {} was left out: {why}", kit.id);
        kit.stage = None;
        kit.refused = Some((kit.shader.clone().unwrap_or_default(), why.to_owned()));
    }
}

impl Swarms {
    pub(super) fn staged(&mut self, r: &Renderer, mode: Mode) {
        for k in 0..self.kits.len() {
            let Some((key, module, naga)) = self.kits[k].stage.clone() else { continue };
            let kit = &self.kits[k];
            let wanted = [true, super::lamped::lit(kit), kit.bend.is_some()];
            for (variant, _) in wanted.iter().enumerate().filter(|(_, w)| **w) {
                let mode = if variant == 2 { Mode::Normal } else { mode };
                if self.stages.contains_key(&(mode, key, variant)) {
                    continue;
                }
                let mut groups = vec![self.layouts[1].clone(), self.layouts[2].clone(), self.framing.clone()];
                groups.extend((variant > 0).then(|| self.layouts[6].clone()));
                let format = if variant == 2 { wgpu::TextureFormat::Rgba16Float } else { self.format };
                let (module, naga) = (module.clone(), naga.clone());
                let trial: Trial = Box::new(move |device: &wgpu::Device| {
                    let (all, framing) = (binds::layouts(device), framing(device));
                    let mut groups = vec![&all[1], &all[2], &framing];
                    groups.extend((variant > 0).then_some(&all[6]));
                    drop(sprite(device, (format, mode), (&shader(device, &naga), variant), &groups));
                });
                let made = within((key, r), Some(trial), move |device| {
                    sprite(device, (format, mode), (&module, variant), &groups.iter().collect::<Vec<_>>())
                });
                match made {
                    Ok(pipeline) => {
                        self.stages.insert((mode, key, variant), pipeline);
                        self.created += 1;
                    }
                    Err(why) => leave(&mut self.kits, key, &why),
                }
                if self.kits[k].stage.is_none() {
                    break;
                }
            }
        }
    }

    pub(super) fn framed(&mut self, r: &Renderer, uniform: &wgpu::Buffer) {
        if self.framed.is_some() || self.kits.iter().all(|k| k.stage.is_none()) {
            return;
        }
        let entries = [wgpu::BindGroupEntry { binding: 0, resource: uniform.as_entire_binding() }];
        let desc = wgpu::BindGroupDescriptor { label: None, layout: &self.framing, entries: &entries };
        self.framed = Some(r.device.create_bind_group(&desc));
    }
}

pub(super) fn framing(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    let entries = [binds::entry(0, wgpu::ShaderStages::VERTEX_FRAGMENT, binds::plain(0))];
    let label = Some("wprs-scene particle frame");
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label, entries: &entries })
}

fn shader(device: &wgpu::Device, naga: &wgpu::naga::Module) -> wgpu::ShaderModule {
    let source = wgpu::ShaderSource::Naga(std::borrow::Cow::Owned(naga.clone()));
    let label = Some("wprs-scene particle shader");
    device.create_shader_module(wgpu::ShaderModuleDescriptor { label, source })
}

fn sprite(
    device: &wgpu::Device,
    target: (wgpu::TextureFormat, Mode),
    (module, variant): (&wgpu::ShaderModule, usize),
    groups: &[&wgpu::BindGroupLayout],
) -> wgpu::RenderPipeline {
    pipe(device, target, (module, ("wprs_vs_particle", ENTRIES[variant])), groups, &slots(SPRITE))
}
