#![allow(dead_code, unused_variables)]
use crate::compositor::Compositor;
use crate::core::bounds::Bounds;
use crate::core::point::Point;
use crate::core::size::Size;
use crate::element::Element;
use crate::events::{mouse, Event};
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
    text_size: u16,
    max_width: u32,
    padding: u16,
    on_change: Box<dyn Fn(String) -> Message>,
    background_colour: [f32; 3],
    border_colour: [f32; 3],
    border_width: f32,
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
            text_size: 30,
            max_width: u32::MAX,
            padding: 0,
            on_change: Box::new(on_change),
            background_colour: [1.0, 1.0, 1.0],
            border_colour: [0.5, 0.5, 0.5],
            border_width: 1.0,
        }
    }

    // padding
    // width
}

impl<'a, Message: Clone> Widget<Message> for TextInput<'a, Message> {
    fn draw(&self, node: Node) -> Primitive {
        let mut primitives = Vec::new();
        let text = if self.state.is_focused || !self.state.value.is_empty() {
            self.state.value.as_str()
        } else {
            self.placeholder.as_str()
        };

        let bounds = Bounds {
            x: node.bounds.x,
            y: node.bounds.y,
            width: node.bounds.width,
            height: node.bounds.height,
        };

        primitives.push(Primitive::Text {
            content: text.into(),
            bounds,
            size: self.text_size as f32,
        });

        let border_colour = if self.state.is_hovered || self.state.is_focused {
            [0.0, 0.0, 0.0]
        } else {
            self.border_colour
        };

        primitives.push(Primitive::Quad {
            bounds,
            color: self.background_colour,
            border_colour,
            border_width: self.border_width,
        });

        if self.state.is_focused {
            // cursor primitive
            primitives.push(Primitive::Quad {
                bounds: Bounds {
                    x: bounds.x + self.state.cursor_position.unwrap(),
                    y: bounds.y,
                    width: 1.0,
                    height: bounds.height,
                },
                color: [0.0, 0.0, 0.0],
                border_colour: [0.0, 0.0, 0.0],
                border_width: 0.0,
            });
        }

        Primitive::Group { primitives }
    }

    fn on_event(
        &mut self,
        event: Event,
        cursor_position: Point,
        viewport: Viewport,
        messages: &mut Vec<Message>,
        layout: Node,
        compositor: &mut Compositor,
    ) {
        let bounds = layout.bounds;

        if bounds.contains(cursor_position) {
            self.state.is_hovered = true;
        } else {
            self.state.is_hovered = false;
        }

        match event {
            Event::Mouse(mouse::Event::Pressed(mouse::Button::Left)) => {
                let is_clicked = bounds.contains(cursor_position);
                self.state.is_focused = is_clicked;

                if is_clicked {
                    let text_layout = &layout.children[0];
                    let target = cursor_position.x - text_layout.bounds.x;
                    let cursor_index =
                        compositor.find_cursor_position(&self.state.value, self.text_size, target);
                    self.state.cursor_index = Some(cursor_index);
                    self.state.cursor_position = Some(compositor.measure_cursor_position(
                        &self.state.value,
                        self.state.cursor_index.unwrap(),
                        self.text_size,
                    ));
                    println!("{:?}", self.state.cursor_position);
                } else {
                    self.state.cursor_index = None;
                }
            }
            Event::CloseRequested => {
                println!("Stop linting my shit");
            }
            _ => {}
        }
    }

    fn layout(&self, renderer: &mut Compositor, limits: Limits) -> Node {
        let text = if self.state.value.is_empty() && !self.state.is_focused {
            self.placeholder.as_str()
        } else {
            self.state.value.as_str()
        };

        let limits = limits.width(self.width).height(self.height);
        let bounds = limits.max;

        let (width, height) =
            renderer.measure_text(&self.placeholder, self.text_size as f32, bounds);
        let text_size = Size { width, height };

        let text = Node::new(text_size);

        let size = limits.resolve(Size::new(width, height));

        Node::with_children(size, vec![text])
    }
}

#[derive(Default, Clone)]
pub struct State {
    is_focused: bool,
    pub value: String,
    cursor_index: Option<usize>,
    cursor_position: Option<f32>,
    is_hovered: bool,
    // something to hold Cursor position and selection?
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
