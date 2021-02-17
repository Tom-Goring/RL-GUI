use futures::executor::block_on;

use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

use crate::application_state::ApplicationState;
use crate::core::bounds::Bounds;
use crate::core::size::Size;
use crate::element::Element;
use crate::layout::limits::Limits;
use crate::viewport::Viewport;

pub trait Application: 'static + Clone {
    type Message;

    fn init() -> Self;
    /// This should handle user defined events, allowing the render function to change its output based on app state
    fn update(&mut self, message: Self::Message);
    /// This should return the element tree to be processed by the compositor
    fn view(&mut self) -> Element<Self::Message>;
}

pub fn run<App: Application>(event_loop: EventLoop<()>, window: Window) {
    run_async::<App>(event_loop, window);
}

// Add compositor as type argument to allow for use of standardised rendering in app.render()
pub fn run_async<A: Application>(event_loop: EventLoop<()>, window: Window) {
    let mut compositor = block_on(super::compositor::Compositor::new(&window));

    let viewport = Viewport::new(window.inner_size().width, window.inner_size().height);
    let mut state = ApplicationState::new(viewport);

    let mut app = A::init();

    let mut messages = Vec::new();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::NewEvents(_) => {}
            Event::WindowEvent { event, .. } => {
                if should_exit(&event) {
                    *control_flow = ControlFlow::Exit;
                }
                match event {
                    WindowEvent::Resized(new_size) => {
                        compositor.resize_window(new_size.width, new_size.height);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        compositor.resize_window(new_inner_size.width, new_inner_size.height)
                    }
                    _ => {
                        state.update(&event);
                        {
                            // For every widget we need to pass it its bounds
                            // Therefore we need some sort of data structure to hold all widgets and their bounds
                            let mut ui = app.view();
                            // TODO: develop system to calculate locations and sizes based on user given data
                            ui.on_event(
                                event,
                                state.cursor_position,
                                state.viewport,
                                &mut messages,
                                Bounds {
                                    x: 0.0,
                                    y: 0.0,
                                    width: 1.0,
                                    height: 1.0,
                                },
                            );
                        }
                        for message in messages.drain(..) {
                            app.update(message);
                        }
                    }
                }
            }
            Event::DeviceEvent { .. } => {}
            Event::UserEvent(_) => {}
            Event::Suspended => {}
            Event::Resumed => {}
            Event::MainEventsCleared => window.request_redraw(),
            Event::RedrawRequested(_) => {
                let ui = app.view();
                let layout = ui.layout(
                    &mut compositor,
                    Limits::new(Size::ZERO, state.logical_size()),
                );
                let primitives = ui.draw(layout);
                compositor.draw(primitives);
            }
            Event::RedrawEventsCleared => {}
            Event::LoopDestroyed => {}
        }
    })
}

fn should_exit(event: &winit::event::WindowEvent<'_>) -> bool {
    match event {
        WindowEvent::CloseRequested => true,
        WindowEvent::KeyboardInput { input, .. } => matches!(input, KeyboardInput {
            state: ElementState::Pressed,
            virtual_keycode: Some(VirtualKeyCode::Escape),
            ..
        }),
        _ => false,
    }
}
