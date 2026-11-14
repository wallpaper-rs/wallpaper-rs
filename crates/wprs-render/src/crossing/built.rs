use super::{layout, pipeline};
use crate::Renderer;
use crate::scene::swarm::staging::{Later, later, left};
use std::sync::Arc;
use std::time::Instant;
use wprs_scene::{Quality, Ready};

pub struct Built {
    pub(super) pipeline: wgpu::RenderPipeline,
    pub(super) layout: Arc<wgpu::BindGroupLayout>,
    pub(super) ready: Arc<Ready>,
    pub(super) quality: Quality,
}

pub struct Building {
    later: Later<wgpu::RenderPipeline>,
    layout: Arc<wgpu::BindGroupLayout>,
    ready: Arc<Ready>,
    quality: Quality,
}

impl Renderer {
    pub(super) fn begin(
        &self,
        ready: &Ready,
        quality: Quality,
    ) -> Result<(wgpu::naga::Module, Arc<wgpu::BindGroupLayout>), String> {
        if !ready.plays_at(quality) {
            return Err(format!("it costs too much for quality {}", quality as u8));
        }
        left(ready.effect.key).map_or(Ok(()), Err)?;
        let naga = wprs_scene::transition_shader(&ready.effect)?;
        Ok((naga, Arc::new(layout(&self.device))))
    }

    pub fn build_crossing(
        &self,
        ready: Arc<Ready>,
        quality: Quality,
        wake: Arc<dyn Fn() + Send + Sync>,
    ) -> Result<Building, String> {
        let (naga, layout) = self.begin(&ready, quality)?;
        let shared = layout.clone();
        let later =
            later(ready.effect.key, &self.device, wake, move |device| pipeline(device, &shared, naga))?;
        Ok(Building { later, layout, ready, quality })
    }
}

impl Building {
    pub fn until(&self) -> Instant {
        self.later.until()
    }

    pub fn poll(&self, now: Instant) -> Option<Result<Built, String>> {
        Some(self.later.poll(now)?.map(|pipeline| self.built(pipeline)))
    }

    pub fn wait(self) -> Result<Built, String> {
        let made = self.later.wait();
        made.map(|pipeline| Built { pipeline, layout: self.layout, ready: self.ready, quality: self.quality })
    }

    fn built(&self, pipeline: wgpu::RenderPipeline) -> Built {
        Built { pipeline, layout: self.layout.clone(), ready: self.ready.clone(), quality: self.quality }
    }
}
