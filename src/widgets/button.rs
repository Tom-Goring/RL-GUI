use crate::element::Element;
use crate::point::Point;
use crate::primitives::Primitive;
use crate::viewport::Viewport;
use winit::event::{ElementState, MouseButton, WindowEvent};

/// All widgets should have a draw function with takes the location to draw said widget to
/// Unsure what state a widget should hold right now.
/// Define a `drawable` type that returns primitives?
pub struct Button<'a, Message: Clone> {
    state: &'a mut State,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    on_press: Option<Message>,
    color: [f32; 3],
}

// TODO: Add layout functions
// Something like a layout enum with flex/grid maybe? Relative/absolute?

impl<'a, Message: Clone> Button<'a, Message> {
    pub fn new(
        state: &'a mut State,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        on_press: Option<Message>,
        color: [f32; 3],
    ) -> Self {
        Button {
            state,
            x,
            y,
            width,
            height,
            on_press,
            color,
        }
    }
}

impl<'a, Message: Clone> super::Widget<Message> for Button<'a, Message> {
    fn draw(&self) -> Primitive {
        Primitive::Quad {
            position: [self.x, self.y],
            color: self.color,
            size: [self.width, self.height],
        }
    }

    fn contains(&self, point: Point) -> bool {
        point.x > self.x
            && point.x < self.x + self.width
            && point.y < self.y
            && point.y > self.y - self.height
    }

    fn on_event(
        &mut self,
        event: WindowEvent,
        cursor_position: Point,
        viewport: Viewport,
        messages: &mut Vec<Message>,
    ) {
        if let WindowEvent::MouseInput { button, state, .. } = event {
            if let MouseButton::Left = button {
                match state {
                    ElementState::Pressed => {
                        if self.on_press.is_some()
                            && self.contains(viewport.normalise_window_coords(cursor_position))
                        {
                            self.state.is_pressed = true;
                        }
                    }
                    ElementState::Released => {
                        if let Some(on_press) = self.on_press.clone() {
                            if self.state.is_pressed {
                                self.state.is_pressed = false;
                                if self.contains(viewport.normalise_window_coords(cursor_position))
                                {
                                    messages.push(on_press);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

impl<'a, Message> From<Button<'a, Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(rect: Button<'a, Message>) -> Element<'a, Message> {
        Element::new(rect)
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
