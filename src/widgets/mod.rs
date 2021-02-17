use crate::compositor::Compositor;
use crate::core::bounds::Bounds;
use crate::core::point::Point;
use crate::layout::limits::Limits;
use crate::layout::node::Node;
use crate::primitives::Primitive;
use crate::viewport::Viewport;
use winit::event::WindowEvent;

pub mod button;
pub mod row;
pub mod text;

// TODO: Come up with layout trait method and a type for it to return
pub trait Widget<Message> {
    fn draw(&self, node: Node) -> Primitive;
    fn on_event(
        &mut self,
        event: WindowEvent,
        cursor_position: Point,
        viewport: Viewport,
        messages: &mut Vec<Message>,
        bounds: Bounds,
    );
    fn layout(&self, renderer: &mut Compositor, limits: Limits) -> Node;
}
