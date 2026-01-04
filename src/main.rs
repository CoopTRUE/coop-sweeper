mod cell;
mod game;
mod grid;
mod message;
mod state;

use crate::game::App;

pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view).run()
}
