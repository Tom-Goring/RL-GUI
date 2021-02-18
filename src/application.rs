use futures::executor::block_on;

use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

use crate::application_state::ApplicationState;
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
    let mut compositor = block_on(super::compositor::Compositor::new());
    let viewport = Viewport::new(window.inner_size().width, window.inner_size().height);
    let surface = compositor.create_surface(&window);

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
                state.update(&event);
                {
                    let mut ui = app.view();
                    let layout = ui.layout(
                        &mut compositor,
                        Limits::new(Size::ZERO, state.logical_size()),
                    );
                    ui.on_event(
                        event,
                        state.cursor_position,
                        state.viewport,
                        &mut messages,
                        layout,
                    );
                }
                for message in messages.drain(..) {
                    app.update(message);
                }
            }
            Event::DeviceEvent { .. } => {}
            Event::UserEvent(_) => {}
            Event::Suspended => {}
            Event::Resumed => {}
            Event::MainEventsCleared => window.request_redraw(),
            Event::RedrawRequested(_) => {
                let physical_size = state.viewport.physical_size();

                let mut swap_chain = compositor.create_swap_chain(
                    &surface,
                    physical_size.width,
                    physical_size.height,
                );

                let ui = app.view();
                let layout = ui.layout(
                    &mut compositor,
                    Limits::new(Size::ZERO, state.logical_size()),
                );
                let primitives = ui.draw(layout);
                compositor.draw(&mut swap_chain, primitives, &viewport);
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
