use rl_gui::application::Application;
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;

use rl_gui::application::run;
use rl_gui::element::Element;
use rl_gui::widgets::button;
use rl_gui::widgets::button::Button;

fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    window.set_title("Hello World");
    window.set_inner_size(LogicalSize::new(1000, 600));

    // TODO: move the eventloop and window creation inside of the run function
    run::<Test>(event_loop, window);
}

#[derive(Clone)]
enum Color {
    Red,
    Blue,
    Green,
}

impl Color {
    pub fn to_rgb(&self) -> [f32; 3] {
        match self {
            Color::Red => [1.0, 0.0, 0.0],
            Color::Blue => [0.0, 1.0, 0.0],
            Color::Green => [0.0, 0.0, 1.0],
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Color::Red => Color::Blue,
            Color::Blue => Color::Green,
            Color::Green => Color::Red,
        }
    }
}

#[derive(Clone)]
enum TestMessage {
    RectangleClicked,
}

#[derive(Clone)]
struct Test {
    button: button::State,
    color: Color,
}

impl Application for Test {
    type Message = TestMessage;

    fn init() -> Self {
        let button = button::State::new();
        let color = Color::Red;
        Self { button, color }
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            TestMessage::RectangleClicked => self.color = self.color.next(),
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let rectangle = Button::new(
            &mut self.button,
            Some(String::from("Hello")),
            0.1,
            0.1,
            Some(TestMessage::RectangleClicked),
            self.color.to_rgb(),
        );
        rectangle.into()
    }
}
