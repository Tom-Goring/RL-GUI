use crate::compositor::Compositor;
use crate::core::point::Point;
use crate::element::Element;
use crate::layout::limits::Limits;
use crate::layout::node::Node;
use crate::primitives::Primitive;
use crate::viewport::Viewport;
use crate::Length;
use winit::event::{ElementState, MouseButton, WindowEvent};

/// All widgets should have a draw function with takes the location to draw said widget to
/// Unsure what state a widget should hold right now.
/// Define a `drawable` type that returns primitives?
pub struct Button<'a, Message: Clone> {
    state: &'a mut State,
    content: Element<'a, Message>,
    width: Length,
    height: Length,
    on_press: Option<Message>,
    color: [f32; 3],
    min_width: u32,
    min_height: u32,
}

// TODO: do something with the label (like render it lol)
impl<'a, Message: Clone> Button<'a, Message> {
    pub fn new(
        state: &'a mut State,
        content: Element<'a, Message>,
        on_press: Option<Message>,
        color: [f32; 3],
    ) -> Self {
        Button {
            state,
            content,
            height: Length::Shrink,
            width: Length::Shrink,
            on_press,
            color,
            min_width: 0,
            min_height: 0,
        }
    }
}

impl<'a, Message: Clone> super::Widget<Message> for Button<'a, Message> {
    fn draw(&self, node: Node) -> Primitive {
        let content = self.content.draw(node.clone());
        let button = Primitive::Quad {
            bounds: node.bounds,
            color: self.color,
        };
        Primitive::Group {
            primitives: vec![button, content],
        }
    }

    fn on_event(
        &mut self,
        event: WindowEvent,
        cursor_position: Point,
        _viewport: Viewport,
        messages: &mut Vec<Message>,
        layout: Node,
    ) {
        let bounds = layout.bounds;
        if let WindowEvent::MouseInput { button, state, .. } = event {
            if let MouseButton::Left = button {
                match state {
                    ElementState::Pressed => {
                        if self.on_press.is_some() && bounds.contains(cursor_position) {
                            self.state.is_pressed = true;
                        }
                    }
                    ElementState::Released => {
                        if let Some(on_press) = self.on_press.clone() {
                            if self.state.is_pressed {
                                self.state.is_pressed = false;
                                if bounds.contains(cursor_position) {
                                    messages.push(on_press);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn layout(&self, compositor: &mut Compositor, limits: Limits) -> Node {
        let limits = limits
            .min_width(self.min_width)
            .min_height(self.min_height)
            .width(self.width)
            .height(self.height);

        let content = self.content.layout(compositor, limits);
        let size = limits.resolve(content.size());
        Node::with_children(size, vec![content])
    }
}

impl<'a, Message> From<Button<'a, Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(button: Button<'a, Message>) -> Element<'a, Message> {
        Element::new(button)
    }
}

#[derive(Default, Clone)]
pub struct State {
    is_pressed: bool,
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }
}
