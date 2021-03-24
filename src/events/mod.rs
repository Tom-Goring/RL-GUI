use crate::core::point::Point;
use winit::event::{ElementState, MouseButton, WindowEvent};

pub mod mouse;
pub mod window;

#[derive(Copy, Clone)]
pub enum Event {
    Mouse(mouse::Event),
    CloseRequested,
    KeyboardInput,
    Other,
    Window(window::Event),
}

impl From<&winit::event::WindowEvent<'_>> for Event {
    fn from(event: &winit::event::WindowEvent) -> Self {
        match event {
            WindowEvent::CursorMoved { position, .. } => Event::Mouse(mouse::Event::CursorMoved(
                Point::new(position.x as f32, position.y as f32),
            )),
            WindowEvent::CursorEntered { .. } => Event::Mouse(mouse::Event::CursorEnteredWindow),
            WindowEvent::CursorLeft { .. } => Event::Mouse(mouse::Event::CursorLeftWindow),
            WindowEvent::MouseInput { state, button, .. } => {
                let button = match button {
                    MouseButton::Left => mouse::Button::Left,
                    MouseButton::Right => mouse::Button::Right,
                    _ => mouse::Button::Other,
                };
                match state {
                    ElementState::Pressed => Event::Mouse(mouse::Event::Pressed(button)),
                    ElementState::Released => Event::Mouse(mouse::Event::Released(button)),
                }
            }
            WindowEvent::CloseRequested => Event::CloseRequested,
            WindowEvent::Resized(new_size) => Event::Window(window::Event::Resized {
                width: new_size.width,
                height: new_size.height,
            }),
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                Event::Window(window::Event::Resized {
                    width: new_inner_size.width,
                    height: new_inner_size.height,
                })
            }
            _ => Self::Other,
        }
    }
}
