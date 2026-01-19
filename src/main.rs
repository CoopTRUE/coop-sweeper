mod assets;
mod cell;
mod elements;
mod game;
mod grid;
mod message;
mod state;
mod theme;

use crate::game::App;

pub fn main() -> iced::Result {
    iced::application::timed(App::default, App::update, App::subscription, App::view)
        // .theme(theme::theme)
        .window_size((500, 700))
        .run()
}
