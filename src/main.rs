#![allow(dead_code, unused_imports)]

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
use rl_gui::widgets::text_input;

use rl_gui::widgets::text_input::TextInput;
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
    TextInputChanged(String),
}

#[derive(Clone)]
struct Test {
    button: button::State,
    second_button: button::State,
    color: Color,
    second_color: Color,
    text_input: text_input::State,
}

impl Application for Test {
    type Message = TestMessage;

    fn init() -> Self {
        let button = button::State::new();
        let second_button = button::State::new();
        let color = Color::Red;
        let second_color = Color::Blue;
        let text_input = text_input::State::new();

        Self {
            button,
            second_button,
            color,
            second_color,
            text_input,
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
            TestMessage::TextInputChanged(new_string) => {
                println!("Text Input Changed to {}", new_string)
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        // ui!(
        // <Column padding=10.0>
        //     <Row padding=10.0>
        //         <Button state=self.button, color=self.color.to_rgb(), on_press=TestMessage::FirstButtonClicked>
        //             <Text value="First Button", size=30/>
        //         </Button>
        //         <Button state=self.second_button, color=self.second_color.to_rgb(),on_press=TestMessage::SecondButtonClicked>
        //             <Text value="Second Button", size=30/>
        //         </Button>
        //     </Row>
        //     <Row padding=10.0>
        //         <Text value="First Text", size=30/>
        //         <Text value="Second Text", size=30/>
        //         <TextInput placeholder="Enter text" size=30/>
        //     </Row>
        // </Column>
        // )

        Column::with_children(vec![
            Row::with_children(vec![
                Button::new(
                    &mut self.button,
                    Text::new("First Button", Some(30)).into(),
                    Some(TestMessage::FirstButtonClicked),
                    self.color.to_rgb(),
                )
                .into(),
                Button::new(
                    &mut self.second_button,
                    Text::new("Second Button", Some(30)).into(),
                    Some(TestMessage::SecondButtonClicked),
                    self.second_color.to_rgb(),
                )
                .into(),
            ])
            .padding(10.0)
            .into(),
            Row::with_children(vec![
                Text::new("First Text", Some(30)).into(),
                Text::new("Second Text", Some(30)).into(),
            ])
            .padding(10.0)
            .into(),
            Row::with_children(vec![
                TextInput::new(
                    &mut self.text_input,
                    "Placeholder",
                    TestMessage::TextInputChanged,
                )
                .into(),
                Text::new("Third Text", Some(30)).into(),
            ])
            .padding(10.0)
            .into(),
        ])
        .into()
    }
}
