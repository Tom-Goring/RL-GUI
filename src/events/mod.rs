use crate::core::point::Point;
use crate::events::keyboard::KeyCode;
use winit::event::{ElementState, MouseButton, VirtualKeyCode, WindowEvent};

pub mod keyboard;
pub mod mouse;
pub mod window;

#[derive(Copy, Clone, Debug)]
pub enum Event {
    Mouse(mouse::Event),
    CloseRequested,
    Keyboard(keyboard::Event),
    Other,
    Window(window::Event),
}

pub fn convert_event(event: &winit::event::WindowEvent<'_>, scale_factor: f64) -> Event {
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
        WindowEvent::Resized(new_size) => {
            let logical_size = new_size.to_logical(scale_factor);
            Event::Window(window::Event::Resized {
                width: logical_size.width,
                height: logical_size.height,
            })
        }
        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
            let logical_size = new_inner_size.to_logical(scale_factor);
            Event::Window(window::Event::Resized {
                width: logical_size.width,
                height: logical_size.height,
            })
        }
        WindowEvent::ReceivedCharacter(c) => {
            Event::Keyboard(keyboard::Event::ReceivedCharacter(*c))
        }
        WindowEvent::KeyboardInput {
            input:
                winit::event::KeyboardInput {
                    virtual_keycode: Some(virtual_keycode),
                    state,
                    ..
                },
            ..
        } => {
            let code = match virtual_keycode {
                VirtualKeyCode::Left => KeyCode::Left,
                VirtualKeyCode::Right => KeyCode::Right,
                VirtualKeyCode::Escape => KeyCode::Escape,
                VirtualKeyCode::Back => KeyCode::Backspace,
                VirtualKeyCode::Return => KeyCode::Enter,
                VirtualKeyCode::Space => KeyCode::Space,
                _ => KeyCode::Other,
            };

            match state {
                ElementState::Pressed => Event::Keyboard(keyboard::Event::KeyPressed {
                    key_code: code,
                    shift: false,
                    control: false,
                    alt: false,
                }),
                ElementState::Released => Event::Keyboard(keyboard::Event::KeyReleased {
                    key_code: code,
                    shift: false,
                    control: false,
                    alt: false,
                }),
            }
        }
        _ => Event::Other,
    }
}
