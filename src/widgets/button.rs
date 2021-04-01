use crate::compositor::Compositor;
use crate::core::point::Point;
use crate::element::Element;
use crate::events::mouse;
use crate::events::Event;
use crate::layout::limits::Limits;
use crate::layout::node::Node;
use crate::primitives::Primitive;
use crate::viewport::Viewport;
use crate::Length;

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
    border_width: f32,
    min_width: u32,
    min_height: u32,
    hover_border_colour: [f32; 3],
    normal_border_colour: [f32; 3],
}

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
            normal_border_colour: [1.0, 1.0, 1.0], // TODO: add methods to change these
            hover_border_colour: [0.0, 0.0, 0.0],
            border_width: 1.0,
        }
    }
}

impl<'a, Message: Clone> super::Widget<Message> for Button<'a, Message> {
    fn draw(&self, node: Node) -> Primitive {
        let content = self.content.draw(node.clone());
        let button = Primitive::Quad {
            bounds: node.bounds,
            color: self.color,
            border_colour: self.state.current_border_color,
            border_width: self.border_width,
        };
        Primitive::Group {
            primitives: vec![button, content],
        }
    }

    fn on_event(
        &mut self,
        event: Event,
        cursor_position: Point,
        _viewport: Viewport,
        messages: &mut Vec<Message>,
        layout: Node,
        _compositor: &mut Compositor,
    ) {
        let bounds = layout.bounds;

        if bounds.contains(cursor_position) {
            self.state.current_border_color = self.hover_border_colour;
        } else {
            self.state.current_border_color = self.normal_border_colour;
        }

        if let Event::Mouse(event) = event {
            match event {
                mouse::Event::Pressed(button) => {
                    if let mouse::Button::Left = button {
                        if self.on_press.is_some() && bounds.contains(cursor_position) {
                            self.state.is_pressed = true;
                        }
                    }
                }
                mouse::Event::Released(button) => {
                    if let mouse::Button::Left = button {
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
                _ => {}
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
    current_border_color: [f32; 3],
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }
}
