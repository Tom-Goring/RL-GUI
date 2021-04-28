#![allow(dead_code, unused_imports)]

use rl_gui::application::run;
use rl_gui::application::Application;
use rl_gui::element::Element;
use rl_gui::widgets::button;
use rl_gui::widgets::row::Row;
use rl_macro::ui;

use rl_gui::widgets::button::Button;
use rl_gui::widgets::column::Column;
use rl_gui::widgets::text::Text;

fn main() {
    run::<Calculator>("Calculator Example", (350, 500));
}

const BUTTON_BACKGROUND: [f32; 3] = [0.8, 0.8, 0.8];

#[derive(Clone)]
enum Message {
    NumberClicked(u8),
    AddClicked,
    SubtractClicked,
    MultiplyClicked,
    DivideClicked,
    EqualsClicked,
    ClearClicked,
}

#[derive(Clone)]
struct Calculator {
    current_display: String,

    button_1: button::State,
    button_2: button::State,
    button_3: button::State,
    button_4: button::State,
    button_5: button::State,
    button_6: button::State,
    button_7: button::State,
    button_8: button::State,
    button_9: button::State,
    button_0: button::State,

    clear_button: button::State,
    add_button: button::State,
    subtract_button: button::State,
    divide_button: button::State,
    multiply_button: button::State,
    equals_button: button::State,

    number_just_entered: bool,
    symbol_just_entered: bool,
}

impl Application for Calculator {
    type Message = Message;

    fn init() -> Self {
        Self {
            current_display: String::new(),

            button_1: button::State::new(),
            button_2: button::State::new(),
            button_3: button::State::new(),
            button_4: button::State::new(),
            button_5: button::State::new(),
            button_6: button::State::new(),
            button_7: button::State::new(),
            button_8: button::State::new(),
            button_9: button::State::new(),
            button_0: button::State::new(),

            clear_button: button::State::new(),
            add_button: button::State::new(),
            subtract_button: button::State::new(),
            divide_button: button::State::new(),
            multiply_button: button::State::new(),
            equals_button: button::State::new(),

            number_just_entered: false,
            symbol_just_entered: false,
        }
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::NumberClicked(n) => {
                self.symbol_just_entered = false;
                self.current_display.push_str(&n.to_string());
            }
            Message::AddClicked => {
                if !self.symbol_just_entered {
                    self.current_display.push('+');
                }
            }
            Message::SubtractClicked => {
                if !self.symbol_just_entered {
                    self.current_display.push('-');
                }
            }
            Message::MultiplyClicked => {
                if !self.symbol_just_entered {
                    self.current_display.push('*');
                }
            }
            Message::DivideClicked => {
                if !self.symbol_just_entered {
                    self.current_display.push('/');
                }
            }
            Message::EqualsClicked => {
                let result = meval::eval_str(&self.current_display).unwrap();
                self.current_display = result.to_string();
            }
            Message::ClearClicked => {
                self.current_display = String::new();
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        Column::with_children(vec![
            Text::new(&self.current_display, Some(50)).into(),
            Row::with_children(vec![
                Button::new(
                    &mut self.button_1,
                    Text::new("1", Some(50)).into(),
                    Some(Message::NumberClicked(1)),
                    BUTTON_BACKGROUND,
                )
                .min_width(60)
                .into(),
                Button::new(
                    &mut self.button_2,
                    Text::new("2", Some(50)).into(),
                    Some(Message::NumberClicked(2)),
                    BUTTON_BACKGROUND,
                )
                .min_width(60)
                .into(),
                Button::new(
                    &mut self.button_3,
                    Text::new("3", Some(50)).into(),
                    Some(Message::NumberClicked(3)),
                    BUTTON_BACKGROUND,
                )
                .min_width(60)
                .into(),
                Button::new(
                    &mut self.add_button,
                    Text::new("+", Some(50)).into(),
                    Some(Message::AddClicked),
                    BUTTON_BACKGROUND,
                )
                .min_width(60)
                .into(),
            ])
            .padding(1.0)
            .into(),
            Row::with_children(vec![
                Button::new(
                    &mut self.button_4,
                    Text::new("4", Some(50)).into(),
                    Some(Message::NumberClicked(4)),
                    BUTTON_BACKGROUND,
                )
                .min_width(60)
                .into(),
                Button::new(
                    &mut self.button_5,
                    Text::new("5", Some(50)).into(),
                    Some(Message::NumberClicked(5)),
                    BUTTON_BACKGROUND,
                )
                .min_width(60)
                .into(),
                Button::new(
                    &mut self.button_6,
                    Text::new("6", Some(50)).into(),
                    Some(Message::NumberClicked(6)),
                    BUTTON_BACKGROUND,
                )
                .min_width(60)
                .into(),
                Button::new(
                    &mut self.subtract_button,
                    Text::new("-", Some(50)).into(),
                    Some(Message::SubtractClicked),
                    BUTTON_BACKGROUND,
                )
                .min_width(60)
                .into(),
            ])
            .padding(1.0)
            .into(),
            Row::with_children(vec![
                Button::new(
                    &mut self.button_7,
                    Text::new("7", Some(50)).into(),
                    Some(Message::NumberClicked(7)),
                    BUTTON_BACKGROUND,
                )
                .min_width(60)
                .into(),
                Button::new(
                    &mut self.button_8,
                    Text::new("8", Some(50)).into(),
                    Some(Message::NumberClicked(8)),
                    BUTTON_BACKGROUND,
                )
                .min_width(60)
                .into(),
                Button::new(
                    &mut self.button_9,
                    Text::new("9", Some(50)).into(),
                    Some(Message::NumberClicked(9)),
                    BUTTON_BACKGROUND,
                )
                .min_width(60)
                .into(),
                Button::new(
                    &mut self.multiply_button,
                    Text::new("*", Some(50)).into(),
                    Some(Message::MultiplyClicked),
                    BUTTON_BACKGROUND,
                )
                .min_width(60)
                .into(),
            ])
            .padding(1.0)
            .into(),
            Row::with_children(vec![
                Button::new(
                    &mut self.button_0,
                    Text::new("0", Some(50)).into(),
                    Some(Message::NumberClicked(0)),
                    BUTTON_BACKGROUND,
                )
                .min_width(60)
                .into(),
                Button::new(
                    &mut self.equals_button,
                    Text::new("=", Some(50)).into(),
                    Some(Message::EqualsClicked),
                    BUTTON_BACKGROUND,
                )
                .min_width(60)
                .into(),
                Button::new(
                    &mut self.clear_button,
                    Text::new("C", Some(50)).into(),
                    Some(Message::ClearClicked),
                    BUTTON_BACKGROUND,
                )
                .min_width(60)
                .into(),
                Button::new(
                    &mut self.divide_button,
                    Text::new("/", Some(50)).into(),
                    Some(Message::DivideClicked),
                    BUTTON_BACKGROUND,
                )
                .min_width(60)
                .into(),
            ])
            .padding(1.0)
            .into(),
        ])
        .padding(1.0)
        .into()

        // ui!(
        // <Column padding=10.0>
        //     <Row padding=10.0>
        //         <Text value=&self.current_display, size=30/>
        //     </Row>
        //     <Row padding=10.0>
        //         <Button state=self.button_1, color=BUTTON_BACKGROUND, on_press=Message::NumberClicked(1)> <Text value="1", size=30/> </Button>
        //         <Button state=self.button_2, color=BUTTON_BACKGROUND>, on_press=Message::NumberClicked(2), <Text value="2", size=30/> </Button>
        //         <Button state=self.button_3, color=BUTTON_BACKGROUND>, on_press=Message::NumberClicked(3), <Text value="3", size=30/> </Button>
        //         <Button state=self.add_button, color=BUTTON_BACKGROUND, on_press=Message::AddClicked> <Text value="+", size=30/> </Button>
        //     </Row>
        //     <Row padding=10.0>
        //         <Button state=self.button_4, color=BUTTON_BACKGROUND, on_press=Message::NumberClicked(4)> <Text value="4", size=30/> </Button>
        //         <Button state=self.button_5, color=BUTTON_BACKGROUND, on_press=Message::NumberClicked(5)> <Text value="5", size=30/> </Button>
        //         <Button state=self.button_6, color=BUTTON_BACKGROUND, on_press=Message::NumberClicked(6)> <Text value="6", size=30/> </Button>
        //         <Button state=self.subtract_button, color=BUTTON_BACKGROUND, on_press=Message::SubtractClicked> <Text value="-", size=30/> </Button>
        //     </Row>
        //     <Row padding=10.0>
        //         <Button state=self.button_7, color=BUTTON_BACKGROUND, on_press=Message::NumberClicked(7)> <Text value="7", size=30/> </Button>
        //         <Button state=self.button_8, color=BUTTON_BACKGROUND, on_press=Message::NumberClicked(8)> <Text value="8", size=30/> </Button>
        //         <Button state=self.button_9, color=BUTTON_BACKGROUND, on_press=Message::NumberClicked(9)> <Text value="9", size=30/> </Button>
        //         <Button state=self.multiply_button, color=BUTTON_BACKGROUND, on_press=Message::MultiplyClicked> <Text value="*", size=30/> </Button>
        //     </Row>
        //     <Row padding=10.0>
        //         <Button state=self.button_0, color=BUTTON_BACKGROUND, on_press=Message::NumberClicked(0)> <Text value="0", size=30/> </Button>
        //         <Button state=self.equals_button, color=BUTTON_BACKGROUND, on_press=Message::EqualsClicked> <Text value="=", size=30/> </Button>
        //         <Button state=self.clear_button, color=BUTTON_BACKGROUND, on_press=Message::ClearClicked> <Text value="C", size=30/> </Button>
        //         <Button state=self.divide_button, color=BUTTON_BACKGROUND, on_press=Message::DivideClicked> <Text value="/", size=30/> </Button>
        //     </Row>
        // </Column>
        // )
    }
}
