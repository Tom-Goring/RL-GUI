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
    // The definition of what rendering primitives a widget should return given a set of bounds
    fn draw(&self, node: Node, compositor: &mut Compositor) -> Primitive;
    // The definition of what events a widget should handle and when
    fn on_event(
        &mut self,
        event: Event,
        cursor_position: Point,
        viewport: Viewport,
        messages: &mut Vec<Message>,
        layout: Node,
        compositor: &mut Compositor,
    );
    // The definition of how to calculate the layout of the widget given a certain set of limits - limits are passed
    // down while sizes are passed up
    fn layout(&self, renderer: &mut Compositor, limits: Limits) -> Node;
}
