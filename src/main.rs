mod cell;
mod game;
mod grid;

use crate::game::App;
use iced::widget::{Column, button, column, text};

pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view).run()
}

#[derive(Clone)]
enum Message {
    Increment,
    Decrement,
}

#[derive(Default)]
struct Counter {
    value: i64,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }
    fn view(&self) -> Column<'_, Message> {
        column![
            button("+").on_press(Message::Increment),
            text(self.value),
            button("-").on_press(Message::Decrement),
        ]
    }
}
