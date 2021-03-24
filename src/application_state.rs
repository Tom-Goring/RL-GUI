use crate::core::point::Point;
use crate::core::size::Size;
use crate::events::Event;
use crate::viewport::Viewport;

use crate::events::mouse;
use crate::events::window;

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

    pub fn update(&mut self, event: crate::events::Event) {
        match event {
            Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::CursorMoved(position) => {
                    self.cursor_position = Point::new(position.x, position.y)
                }
                _ => {}
            },
            Event::Window(window_event) => match window_event {
                window::Event::Resized { width, height } => {
                    self.viewport = Viewport::new(width, height)
                }
            },
            _ => {}
        }
    }

    pub fn logical_size(&self) -> Size<f32> {
        self.viewport.logical_size()
    }
}
