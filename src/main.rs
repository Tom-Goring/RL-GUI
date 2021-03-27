use rl_gui::application::Application;
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;

use rl_gui::application::run;
use rl_gui::element::Element;
use rl_gui::widgets::button;
use rl_gui::widgets::button::Button;
use rl_gui::widgets::column::Column;
use rl_gui::widgets::row::Row;
use rl_gui::widgets::text::Text;

use rl_macro::ui;

fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    window.set_title("Test GUI");
    window.set_inner_size(LogicalSize::new(1000, 600));

    // TODO: move the event loop and window creation inside of the run function
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
            Color::Green => [0.0, 1.0, 0.0],
            Color::Blue => [0.0, 0.0, 1.0],
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Color::Red => Color::Green,
            Color::Green => Color::Blue,
            Color::Blue => Color::Red,
        }
    }
}

#[derive(Clone)]
enum TestMessage {
    FirstButtonClicked,
    SecondButtonClicked,
}

#[derive(Clone)]
struct Test {
    button: button::State,
    second_button: button::State,
    color: Color,
    second_color: Color,
}

impl Application for Test {
    type Message = TestMessage;

    fn init() -> Self {
        let button = button::State::new();
        let second_button = button::State::new();
        let color = Color::Red;
        let second_color = Color::Blue;

        Self {
            button,
            second_button,
            color,
            second_color,
        }
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            TestMessage::FirstButtonClicked => {
                self.color = self.color.next();
            }
            TestMessage::SecondButtonClicked => {
                self.second_color = self.second_color.next();
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        ui!(
        <Column padding=10.0>
            <Row padding=10.0>
                <Button state=self.button, color=self.color.to_rgb(), on_press=TestMessage::FirstButtonClicked>
                    <Text value="First Button", size=30/>
                </Button>
                <Button state=self.second_button, color=self.second_color.to_rgb(),on_press=TestMessage::SecondButtonClicked>
                    <Text value="Second Button", size=30/>
                </Button>
            </Row>
            <Row padding=10.0>
                <Text value="First Text", size=30/>
                <Text value="Second Text", size=30/>
            </Row>
        </Column>
        )
    }
}
