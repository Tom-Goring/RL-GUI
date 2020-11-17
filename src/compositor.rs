use crate::pipelines;
use crate::vertex::Vertex;

pub struct Compositor {
    _instance: wgpu::Instance,
    device: wgpu::Device,
    queue: wgpu::Queue,
    pub surface: super::surface::Surface,

    // Put pipelines in Renderer type?
    triangle_pipeline: pipelines::triangle::Pipeline,
    quad_pipeline: pipelines::quad::Pipeline,
}

impl Compositor {
    pub async fn new(window: &winit::window::Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: { wgpu::PowerPreference::default() },
                compatible_surface: None,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits {
                        max_bind_groups: 2,
                        ..wgpu::Limits::default()
                    },
                    shader_validation: false,
                },
                None,
            )
            .await
            .unwrap();

        let surface = super::surface::Surface::new(
            &device,
            unsafe { instance.create_surface(window) },
            window.inner_size().width,
            window.inner_size().height,
            wgpu::PresentMode::Mailbox,
        );

        let triangle_pipeline = crate::pipelines::triangle::Pipeline::new(
            &device,
            wgpu::TextureFormat::Bgra8UnormSrgb,
            &[
                Vertex {
                    position: [0.0, 0.5, 0.0],
                    color: [1.0, 0.0, 0.0],
                },
                Vertex {
                    position: [-0.5, -0.5, 0.0],
                    color: [0.0, 1.0, 0.0],
                },
                Vertex {
                    position: [0.5, -0.5, 0.0],
                    color: [0.0, 0.0, 1.0],
                },
            ],
        );

        let quad_pipeline = pipelines::quad::Pipeline::new(
            &device,
            wgpu::TextureFormat::Bgra8UnormSrgb,
            &[
                Vertex {
                    position: [0.0, 0.0, 0.0],
                    color: [0.5, 0.0, 0.5],
                },
                Vertex {
                    position: [0.5, 0.0, 0.0],
                    color: [0.5, 0.0, 0.5],
                },
                Vertex {
                    position: [0.5, 0.5, 0.0],
                    color: [0.0, 1.0, 0.5],
                },
                Vertex {
                    position: [0.0, 0.5, 0.0],
                    color: [0.5, 0.0, 0.5],
                },
            ],
            &[0, 1, 2, 0, 2, 3],
        );

        Self {
            _instance: instance,
            device,
            queue,
            triangle_pipeline,
            surface,
            quad_pipeline,
        }
    }

    pub fn draw(&mut self) {
        let swap_chain = self.device.create_swap_chain(
            &self.surface.surface,
            &wgpu::SwapChainDescriptor {
                usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                width: self.surface.width(),
                height: self.surface.height(),
                present_mode: wgpu::PresentMode::Fifo,
            },
        );

        let frame = swap_chain.get_current_frame().expect("Next frame");

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // self.triangle_pipeline.draw(&mut encoder, &frame);
        self.quad_pipeline.draw(&mut encoder, &frame);

        self.queue.submit(std::iter::once(encoder.finish()));
    }

    pub fn resize_window(&mut self, width: u32, height: u32) {
        self.surface.resize(&self.device, width, height);
    }
}
