use winit::dpi::PhysicalPosition;
use winit::event::WindowEvent;

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
            WindowEvent::MouseInput { .. } => {
                println!("{:?}", self.cursor_position);
            }
            _ => {}
        }
    }
}
