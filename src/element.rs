use crate::compositor::Compositor;
use crate::core::point::Point;
use crate::events::Event;
use crate::layout::limits::Limits;
use crate::layout::node::Node;
use crate::primitives::Primitive;
use crate::viewport::Viewport;
use crate::widgets::Widget;

/// A generic widget - it holds any type that can be broken down into primitives via the Widget trait
pub struct Element<'a, Message> {
    content: Box<dyn Widget<Message> + 'a>,
}

impl<'a, Message> Element<'a, Message> {
    pub fn new(content: impl Widget<Message> + 'a) -> Self {
        Element {
            content: Box::new(content),
        }
    }

    pub fn draw(&self, node: Node) -> Primitive {
        self.content.draw(node)
    }

    pub fn layout(&self, compositor: &mut Compositor, limits: Limits) -> Node {
        self.content.layout(compositor, limits)
    }

    pub fn on_event(
        &mut self,
        event: Event,
        cursor_position: Point,
        viewport: Viewport,
        messages: &mut Vec<Message>,
        layout: Node,
    ) {
        self.content
            .on_event(event, cursor_position, viewport, messages, layout);
    }
}
