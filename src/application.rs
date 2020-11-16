use futures::executor::block_on;
use wgpu::PresentMode;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

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

    let mut app = App::init();
    // app should have state, which it then uses to render to the window

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::NewEvents(_) => {}
            Event::WindowEvent { event, window_id } => {
                if window.id() == window_id {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::KeyboardInput { input, .. } => match input {
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            } => *control_flow = ControlFlow::Exit,
                            _ => {}
                        },
                        WindowEvent::Resized(new_size) => {
                            compositor.resize_window(new_size.width, new_size.height)
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            compositor.resize_window(new_inner_size.width, new_inner_size.height);
                        }
                        _ => {}
                    }
                }
            }
            Event::DeviceEvent { .. } => {}
            Event::UserEvent(_) => {}
            Event::Suspended => {}
            Event::Resumed => {}
            Event::MainEventsCleared => window.request_redraw(),
            Event::RedrawRequested(_) => {
                app.render();
                compositor.draw();
            }
            Event::RedrawEventsCleared => {}
            Event::LoopDestroyed => {}
        }
    })
}
