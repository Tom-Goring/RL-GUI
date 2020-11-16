use rl_gui::application::Application;
use wgpu::{CommandEncoder, Device, Queue};
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ControlFlow, EventLoop};

use rl_gui;
use rl_gui::application::run;
use rl_gui::surface::Surface;
use rl_gui::{surface, triangle};

fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    window.set_title("Hello World");
    window.set_inner_size(LogicalSize::new(1000, 600));

    run::<Test>(event_loop, window);
}

struct Test {
    pipeline: triangle::Pipeline,
}

impl Application for Test {
    fn init(device: &Device, queue: &Queue, surface: &mut Surface) -> Self {
        Self {
            pipeline: triangle::Pipeline::new(&device, wgpu::TextureFormat::Bgra8UnormSrgb),
        }
    }

    fn update(&mut self, event: WindowEvent, control_flow: &mut ControlFlow) {}

    fn render(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        surface: &mut surface::Surface,
    ) {
        let swap_chain = device.create_swap_chain(
            &surface.surface,
            &wgpu::SwapChainDescriptor {
                usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                width: surface.width(),
                height: surface.height(),
                present_mode: wgpu::PresentMode::Fifo,
            },
        );

        let frame = swap_chain.get_current_frame().expect("Next frame");

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        self.pipeline.draw(&mut encoder, &frame);

        queue.submit(std::iter::once(encoder.finish()));
    }
}
