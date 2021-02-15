use crate::core::bounds::Bounds;
use crate::core::point::Point;
use crate::primitives::Primitive;
use crate::viewport::Viewport;
use winit::event::WindowEvent;

pub mod button;
pub mod row;

pub trait Widget<Message> {
    fn draw(&self) -> Primitive;
    fn on_event(
        &mut self,
        event: WindowEvent,
        cursor_position: Point,
        viewport: Viewport,
        messages: &mut Vec<Message>,
        bounds: Bounds,
    );
}
