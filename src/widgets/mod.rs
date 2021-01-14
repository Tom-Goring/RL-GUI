use crate::point::Point;
use crate::primitives::Primitive;
use crate::viewport::Viewport;
use winit::event::WindowEvent;

pub mod button;

pub trait Widget<Message> {
    fn draw(&self) -> Primitive;
    fn contains(&self, point: Point) -> bool;
    fn on_event(
        &mut self,
        event: WindowEvent,
        cursor_position: Point,
        viewport: Viewport,
        messages: &mut Vec<Message>,
    );
}
