use futures::executor::block_on;

use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

use crate::application_state::ApplicationState;
use crate::element::Element;
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
    block_on(run_async::<App>(event_loop, window));
}

// Add compositor as type argument to allow for use of standardised rendering in app.render()
pub async fn run_async<A: Application>(event_loop: EventLoop<()>, window: Window) {
    let mut compositor = super::compositor::Compositor::new(&window).await;

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
                    // TODO: Move size stuff into a viewport struct & add scale factors
                    WindowEvent::Resized(new_size) => {
                        compositor.resize_window(new_size.width, new_size.height);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        compositor.resize_window(new_inner_size.width, new_inner_size.height)
                    }
                    _ => {
                        state.update(&event);
                        {
                            let mut ui = app.view();
                            ui.on_event(
                                event,
                                state.cursor_position,
                                state.viewport,
                                &mut messages,
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
                // TODO: Add generic element widget output for user defined render function, to can be passed into the Renderer
                let ui = app.view();
                compositor.draw(ui.as_primitive());
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
