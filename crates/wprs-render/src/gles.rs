use crate::Renderer;

impl Renderer {
    pub(crate) fn gl(&self) -> bool {
        self.gpu.get_info().backend == wgpu::Backend::Gl
    }

    #[doc(hidden)]
    pub fn reads_srgb_through_views(&self) -> bool {
        self.gpu.get_downlevel_capabilities().flags.contains(wgpu::DownlevelFlags::VIEW_FORMATS)
    }

    pub(crate) fn array_layers(&self, layers: u32) -> u32 {
        if self.gl() { layers.max(2) } else { layers }
    }
}
