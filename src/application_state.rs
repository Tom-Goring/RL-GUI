use crate::core::point::Point;
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
            // WindowEvent::MouseInput { state, button, .. } => match state {
            //     ElementState::Pressed => {
            //         if let MouseButton::Left = button {
            //             println!(
            //                 "{:?}",
            //                 self.viewport.normalise_window_coords(self.cursor_position)
            //             )
            //         }
            //     }
            //     ElementState::Released => {}
            // },
            _ => {}
        }
    }
}
