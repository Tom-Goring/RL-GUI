// TODO: Rewrite triangle pipeline if I end up needing it
pub struct Pipeline {}

impl Pipeline {
    pub fn new(_device: &wgpu::Device, _format: wgpu::TextureFormat) -> Self {
        Self {}
    }

    pub fn draw(&self, _encoder: &mut wgpu::CommandEncoder, _target: &wgpu::SwapChainFrame) {}
}
