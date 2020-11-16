pub struct Surface {
    pub swap_chain: wgpu::SwapChain,
    pub surface: wgpu::Surface,
    pub desc: wgpu::SwapChainDescriptor,
}

impl Surface {
    pub fn new(
        device: &wgpu::Device,
        surface: wgpu::Surface,
        width: u32,
        height: u32,
        present_mode: wgpu::PresentMode,
    ) -> Self {
        let desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width,
            height,
            present_mode,
        };

        Self {
            swap_chain: device.create_swap_chain(&surface, &desc),
            surface,
            desc,
        }
    }

    pub fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        self.desc.width = width;
        self.desc.height = height;
        self.swap_chain = device.create_swap_chain(&self.surface, &self.desc);
    }

    pub fn width(&self) -> u32 {
        self.desc.width
    }

    pub fn height(&self) -> u32 {
        self.desc.height
    }
}
