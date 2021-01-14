use crate::point::Point;
use crate::primitives::Primitive;
use crate::viewport::Viewport;
use crate::widgets::Widget;
use winit::event::WindowEvent;

/// A generic widget - it holds any type that can be broken down into primitives via the Drawable trait
pub struct Element<'a, Message> {
    content: Box<dyn Widget<Message> + 'a>,
}

impl<'a, Message> Element<'a, Message> {
    pub fn new(content: impl Widget<Message> + 'a) -> Self {
        Element {
            content: Box::new(content),
        }
    }

    pub fn as_primitive(&self) -> Primitive {
        self.content.draw()
    }

    pub fn contains(&self, point: Point) -> bool {
        self.content.contains(point)
    }

    pub fn on_event(
        &mut self,
        event: WindowEvent,
        cursor_position: Point,
        viewport: Viewport,
        messages: &mut Vec<Message>,
    ) {
        self.content
            .on_event(event, cursor_position, viewport, messages);
    }
}
