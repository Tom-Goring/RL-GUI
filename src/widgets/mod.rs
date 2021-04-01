use crate::compositor::Compositor;
use crate::core::point::Point;
use crate::events::Event;
use crate::layout::limits::Limits;
use crate::layout::node::Node;
use crate::primitives::Primitive;
use crate::viewport::Viewport;

pub mod button;
pub mod column;
pub mod row;
pub mod text;
pub mod text_input;

pub trait Widget<Message> {
    fn draw(&self, node: Node) -> Primitive;
    fn on_event(
        &mut self,
        event: Event,
        cursor_position: Point,
        viewport: Viewport,
        messages: &mut Vec<Message>,
        layout: Node,
        compositor: &mut Compositor,
    );
    fn layout(&self, renderer: &mut Compositor, limits: Limits) -> Node;
}
