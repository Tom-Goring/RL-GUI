use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::ControlFlow;

pub struct ApplicationState {
    cursor_position: winit::dpi::PhysicalPosition<f64>,
}

impl ApplicationState {
    pub fn new() -> ApplicationState {
        Self {
            cursor_position: PhysicalPosition::new(-1.0, -1.0),
        }
    }

    pub fn update(&mut self, event: &winit::event::WindowEvent<'_>) {
        match event {
            WindowEvent::CursorMoved { position, .. } => self.cursor_position = *position,
            WindowEvent::MouseInput {
                device_id,
                state,
                button,
                ..
            } => {
                println!("{:?}", self.cursor_position);
            }
            _ => {}
        }
    }
}
