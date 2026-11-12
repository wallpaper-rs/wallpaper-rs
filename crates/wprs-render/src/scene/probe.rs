use crate::Renderer;
use wgpu::Backends;

pub(crate) struct Probe {
    pub(crate) name: String,
    pub(crate) features: wgpu::Features,
    pub(crate) limits: wgpu::Limits,
}

impl Renderer {
    pub(crate) fn probe(&self) -> Option<Probe> {
        let (features, limits) = (self.device.features(), self.device.limits());
        self.gl().then(|| Probe { name: self.gpu.get_info().name, features, limits })
    }
}

impl Probe {
    pub(crate) fn open(&self) -> Option<wgpu::Device> {
        let instance = crate::adapter::instance(Backends::GL, None);
        let adapters = pollster::block_on(instance.enumerate_adapters(Backends::GL));
        let Some(adapter) = adapters.into_iter().find(|a| a.get_info().name == self.name) else {
            tracing::debug!("no second context of {} to try package shaders in", self.name);
            return None;
        };
        let descriptor = wgpu::DeviceDescriptor {
            label: Some("wprs-render trial"),
            required_features: self.features,
            required_limits: self.limits.clone(),
            ..Default::default()
        };
        let opened = pollster::block_on(adapter.request_device(&descriptor));
        opened
            .inspect_err(|e| tracing::debug!("cannot open a context to try package shaders in: {e}"))
            .ok()
            .map(|o| o.0)
    }
}
