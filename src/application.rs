use futures::executor::block_on;

use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

use crate::application_state::ApplicationState;
use crate::core::size::Size;
use crate::element::Element;
use crate::events::convert_event;
use crate::layout::limits::Limits;
use crate::viewport::Viewport;

pub trait Application: 'static + Clone {
    type Message;

    // Initialises the application state. This should contain the initial state of the application and its widgets.
    fn init() -> Self;
    // Handles user-defined Messages that are spawned on widget events. Typically follows some form of pattern matching
    // on the message enum.
    fn update(&mut self, message: Self::Message);
    // Returns the widget tree. This is a set of (possibly only one) widget(s) each defining it's own layout and view
    // functions that can be called recursively to generate the necessary positions and primitives to be drawn by the
    // renderer
    fn view(&mut self) -> Element<Self::Message>;
}

pub fn run<App: Application>(event_loop: EventLoop<()>, window: Window) {
    run_async::<App>(event_loop, window);
}

// Add compositor as type argument to allow for use of standardised rendering in app.render()
pub fn run_async<A: Application>(event_loop: EventLoop<()>, window: Window) {
    let mut compositor = block_on(super::compositor::Compositor::new());
    let viewport = Viewport::new(
        window.inner_size().width,
        window.inner_size().height,
        window.scale_factor(),
    );
    let surface = compositor.create_surface(&window);
    // window.set_resizable(false);

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
                let event = convert_event(&event, window.scale_factor());
                state.update(event, &window);
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
                        &mut compositor,
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
                compositor.draw(&mut swap_chain, primitives, &state.viewport);
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
