use rl_gui::application::Application;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ControlFlow, EventLoop};

use rl_gui;
use rl_gui::application::run;

fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    window.set_title("Hello World");
    window.set_inner_size(LogicalSize::new(1000, 600));

    run::<Test>(event_loop, window);
}

struct Test {}

impl Application for Test {
    fn init() -> Self {
        Self {}
    }

    fn update(&mut self, _event: WindowEvent, _control_flow: &mut ControlFlow) {}

    fn render(&mut self) {}
}
