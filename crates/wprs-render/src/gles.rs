use crate::Renderer;

impl Renderer {
    pub(crate) fn gl(&self) -> bool {
        self.gpu.get_info().backend == wgpu::Backend::Gl
    }
}
