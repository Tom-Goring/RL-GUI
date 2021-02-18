use crate::core::point::Point;
use crate::core::size::Size;
use crate::viewport::Viewport;
use winit::event::WindowEvent;

pub struct ApplicationState {
    pub cursor_position: Point,
    pub viewport: Viewport,
}

impl ApplicationState {
    pub fn new(viewport: Viewport) -> ApplicationState {
        Self {
            cursor_position: Point::new(-1.0, -1.0),
            viewport,
        }
    }

    pub fn update(&mut self, event: &winit::event::WindowEvent<'_>) {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_position = Point::new(position.x as f32, position.y as f32)
            }
            WindowEvent::Resized(new_size) => {
                self.viewport = Viewport::new(new_size.width, new_size.height);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                self.viewport = Viewport::new(new_inner_size.width, new_inner_size.height);
            }
            _ => {}
        }
    }

    pub fn logical_size(&self) -> Size<f32> {
        self.viewport.logical_size()
    }
}
