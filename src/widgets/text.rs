use crate::compositor::Compositor;
use crate::core::length::Length;
use crate::core::point::Point;
use crate::core::size::Size;
use crate::element::Element;
use crate::events::Event;
use crate::layout::limits::Limits;
use crate::layout::node::Node;
use crate::primitives::Primitive;
use crate::viewport::Viewport;
use crate::widgets::Widget;

pub struct Text {
    content: String,
    size: Option<u16>,
    width: Length,
    height: Length,
}

impl Text {
    pub fn new<T: Into<String>>(label: T, size: Option<u16>) -> Self {
        Text {
            content: label.into(),
            size,
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }
}

impl<Message> Widget<Message> for Text {
    fn draw(&self, node: Node, _compositor: &mut Compositor) -> Primitive {
        Primitive::Text {
            content: self.content.clone(),
            bounds: node.bounds,
            size: self.size.unwrap() as f32, // TODO: clean this unwrap up
        }
    }

    fn on_event(
        &mut self,
        _event: Event,
        _cursor_position: Point,
        _viewport: Viewport,
        _messages: &mut Vec<Message>,
        _layout: Node,
        _compositor: &mut Compositor,
    ) {
    }

    fn layout(&self, renderer: &mut Compositor, limits: Limits) -> Node {
        let limits = limits.width(self.width).height(self.height);
        let size = self.size.unwrap(); // TODO: clean this unwrap up
        let bounds = limits.max;

        let min_height = renderer.measure_text("1", size as f32, bounds).1;
        let (width, height) = renderer.measure_text(&self.content, size as f32, bounds);

        let size = limits.resolve(Size::new(width, height.max(min_height)));

        Node::new(size)
    }
}

impl<'a, Message> From<Text> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(text: Text) -> Element<'a, Message> {
        Element::new(text)
    }
}
