#![allow(dead_code, unused_variables)]
use crate::compositor::Compositor;
use crate::core::bounds::Bounds;
use crate::core::point::Point;
use crate::core::size::Size;
use crate::element::Element;
use crate::events::Event;
use crate::layout::limits::Limits;
use crate::layout::node::Node;
use crate::primitives::Primitive;
use crate::viewport::Viewport;
use crate::widgets::Widget;
use crate::Length;

pub struct TextInput<'a, Message> {
    state: &'a mut State,
    placeholder: String,
    width: Length,
    height: Length,
    text_size: f32,
    max_width: u32,
    padding: u16,
    on_change: Box<dyn Fn(String) -> Message>,
    background_color: [f32; 3],
}

impl<'a, Message> TextInput<'a, Message>
where
    Message: Clone,
{
    pub fn new<F>(state: &'a mut State, placeholder: &str, on_change: F) -> Self
    where
        F: 'static + Fn(String) -> Message,
    {
        TextInput {
            state,
            placeholder: String::from(placeholder),
            width: Length::Fill,
            height: Length::Fill,
            text_size: 30.0,
            max_width: u32::MAX,
            padding: 0,
            on_change: Box::new(on_change),
            background_color: [0.86, 0.86, 0.86],
        }
    }

    // padding
    // width
}

impl<'a, Message: Clone> Widget<Message> for TextInput<'a, Message> {
    fn draw(&self, node: Node) -> Primitive {
        let text = if self.state.value.is_empty() {
            self.placeholder.as_str()
        } else {
            self.state.value.as_str()
        };

        let bounds = Bounds {
            x: node.bounds.x,
            y: node.bounds.y,
            width: node.bounds.width,
            height: node.bounds.height,
        };

        let text_primitive = Primitive::Text {
            content: text.into(),
            bounds,
            size: self.text_size,
        };

        let background = Primitive::Quad {
            bounds,
            color: self.background_color,
        };

        Primitive::Group {
            primitives: vec![background, text_primitive],
        }
    }

    // oh my god its going to be a nightmare
    fn on_event(
        &mut self,
        event: Event,
        cursor_position: Point,
        viewport: Viewport,
        messages: &mut Vec<Message>,
        layout: Node,
    ) {
    }

    fn layout(&self, renderer: &mut Compositor, limits: Limits) -> Node {
        let text = if self.state.value.is_empty() {
            self.placeholder.as_str()
        } else {
            self.state.value.as_str()
        };

        let limits = limits.width(self.width).height(self.height);
        let bounds = limits.max;

        let (width, height) = renderer.measure_text(&text, self.text_size, bounds);

        let size = limits.resolve(Size::new(width, height));

        Node::new(size)
    }
}

#[derive(Default, Clone)]
pub struct State {
    is_focused: bool,
    value: String,
    // something to hold Cursor position?
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a, Message> From<TextInput<'a, Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(input: TextInput<'a, Message>) -> Element<'a, Message> {
        Element::new(input)
    }
}
