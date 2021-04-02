#![allow(dead_code, unused_variables)]

mod cursor;
mod value;

use crate::compositor::Compositor;
use crate::core::bounds::Bounds;
use crate::core::point::Point;
use crate::core::size::Size;
use crate::element::Element;
use crate::events::keyboard;
use crate::events::keyboard::KeyCode;
use crate::events::{mouse, Event};
use crate::layout::limits::Limits;
use crate::layout::node::Node;
use crate::primitives::Primitive;
use crate::viewport::Viewport;
use crate::widgets::text_input::cursor::Cursor;
use crate::widgets::text_input::value::TextValue;
use crate::widgets::Widget;
use crate::Length;

pub struct TextInput<'a, Message> {
    state: &'a mut State,
    value: TextValue,
    placeholder: String,

    width: Length,
    height: Length,

    max_width: u32,
    text_size: u16,
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
    pub fn new<F>(state: &'a mut State, placeholder: &str, value: &str, on_change: F) -> Self
    where
        F: 'static + Fn(String) -> Message,
    {
        TextInput {
            state,
            value: TextValue::new(value),
            placeholder: String::from(placeholder),

            width: Length::Fill,
            height: Length::Fill,
            max_width: u32::MAX,

            text_size: 30,
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
    fn draw(&self, node: Node, compositor: &mut Compositor) -> Primitive {
        let mut primitives = Vec::new();
        let text = if self.state.is_focused || !self.value.is_empty() {
            self.value.to_string()
        } else {
            self.placeholder.clone()
        };

        let bounds = Bounds {
            x: node.bounds.x,
            y: node.bounds.y,
            width: node.bounds.width,
            height: node.bounds.height,
        };

        primitives.push(Primitive::Text {
            content: text.clone(),
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

        // cursor primitive
        if self.state.is_focused {
            let offset =
                compositor.measure_cursor_position(&text, self.state.cursor.index, self.text_size);
            primitives.push(Primitive::Quad {
                bounds: Bounds {
                    x: bounds.x + offset,
                    y: bounds.y,
                    width: 0.4,
                    height: bounds.height,
                },
                color: [0.3, 0.3, 0.3],
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
                    let cursor_index = compositor.find_cursor_position(
                        &self.value.to_string(),
                        self.text_size,
                        target,
                    );
                    self.state.cursor.index = cursor_index;
                }
            }
            Event::Keyboard(keyboard::Event::ReceivedCharacter(c)) => {
                if !c.is_control() {
                    self.value.insert(self.state.cursor.index, c);
                    self.state.cursor.move_right(&self.value);
                    messages.push((self.on_change)(self.value.to_string()));
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) => {
                if self.state.is_focused {
                    match key_code {
                        KeyCode::Left => {
                            self.state.cursor.move_left();
                        }
                        KeyCode::Right => {
                            self.state.cursor.move_right(&self.value);
                        }
                        KeyCode::Escape => {
                            self.state.is_focused = false;
                        }
                        KeyCode::Backspace => {
                            if self.state.cursor.index != 0 {
                                self.value.remove(self.state.cursor.index - 1);
                                self.state.cursor.move_left();
                                messages.push((self.on_change)(self.value.to_string()))
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    fn layout(&self, renderer: &mut Compositor, limits: Limits) -> Node {
        let text = if self.value.is_empty() && !self.state.is_focused {
            self.placeholder.clone()
        } else {
            self.value.to_string()
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
    is_hovered: bool,
    cursor: Cursor,
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
