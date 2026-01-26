use iced::{
    Alignment, Element, Length,
    widget::{button, container, mouse_area, row},
};

use crate::{assets::*, game::App, grid::GridConfig, message::Message, state::GameState};

pub fn header(app: &App) -> Element<'static, Message> {
    let score_display = match &app.state {
        GameState::Started(grid) | GameState::Over(grid) => {
            let mines = grid.count_mines() as isize;
            let flags = grid.count_flags() as isize;
            score_display(mines - flags)
        }
        GameState::CreationScreen(GridConfig { mines, .. })
        | GameState::Initialized(GridConfig { mines, .. }) => score_display(*mines as isize),
    };
    container(
        row![
            score_display,
            button("Hint?").on_press(Message::RequestHint),
            button(app.click_mode.to_string()).on_press(Message::ClickModeToggle),
            mouse_area(get_face_image(app.face).width(40).height(40)).on_press(Message::GameNew),
        ]
        .spacing(10)
        .align_y(Alignment::Center),
    )
    .align_x(Alignment::Center)
    .width(Length::Fill)
    .into()
}

pub fn score_display(score: isize) -> Element<'static, Message> {
    let abs_score = score.unsigned_abs();
    let digits = [
        (abs_score / 100) % 10,
        (abs_score / 10) % 10,
        abs_score % 10,
    ];

    let row = if score < 0 {
        row![
            get_minus_image(),
            get_clock_image(digits[1]),
            get_clock_image(digits[2])
        ]
    } else {
        row![
            get_clock_image(digits[0]),
            get_clock_image(digits[1]),
            get_clock_image(digits[2])
        ]
    };

    row.into()
}
