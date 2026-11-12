use crate::Renderer;

impl Renderer {
    pub(crate) fn gl(&self) -> bool {
        self.gpu.get_info().backend == wgpu::Backend::Gl
    }

    pub(crate) fn array_layers(&self, layers: u32) -> u32 {
        if self.gl() { layers.max(2) } else { layers }
    }
}
