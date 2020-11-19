use futures::executor::block_on;

use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

use crate::application_state::ApplicationState;

pub trait Application: 'static {
    fn init() -> Self;
    fn update(&mut self, event: WindowEvent, control_flow: &mut ControlFlow);
    fn render(&mut self);
}

pub fn run<App: Application>(event_loop: EventLoop<()>, window: Window) {
    block_on(run_async::<App>(event_loop, window));
}

// Add compositor as type argument to allow for use of standardised rendering in app.render()
pub async fn run_async<App: Application>(event_loop: EventLoop<()>, window: Window) {
    let mut compositor = super::compositor::Compositor::new(&window).await;

    let mut state = ApplicationState::new();

    let mut app = App::init();

    // TODO: Add event system to handle events triggered by user actions on user defined widgets

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
                app.render();
                compositor.draw();
            }
            Event::RedrawEventsCleared => {}
            Event::LoopDestroyed => {}
        }
    })
}

fn should_exit(event: &winit::event::WindowEvent<'_>) -> bool {
    match event {
        WindowEvent::CloseRequested => true,
        WindowEvent::KeyboardInput { input, .. } => match input {
            KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::Escape),
                ..
            } => true,
            _ => false,
        },
        _ => false,
    }
}
