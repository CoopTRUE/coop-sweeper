use std::time::Instant;

use iced::{
    Alignment, Color, Element, Length,
    widget::{Button, Image, button, container, row},
};

use crate::{assets::*, game::App, grid::GridConfig, message::Message, state::GameState};

pub fn header(app: &App) -> Element<'static, Message> {
    let score_display = match &app.state {
        GameState::Started(grid) | GameState::Won(grid) | GameState::Lost(grid) => {
            let mines = grid.count_mines() as isize;
            let flags = grid.count_flags() as isize;
            score_display(mines - flags)
        }
        GameState::CreationScreen(GridConfig { mines, .. })
        | GameState::Initialized(GridConfig { mines, .. }) => score_display(*mines as isize),
    };
    let time_display = time_display(app.started);
    let toggle_button = button_image(get_flag_image().opacity(app.click_mode.opacity()))
        .on_press(Message::ClickModeToggle);
    let hint_button = button("Hint?").on_press(match app.state {
        GameState::Started(..) => Message::RequestHint,
        _ => Message::NoOp,
    });
    let face_button = button_image(get_face_image(app.face)).on_press(Message::GameNew);
    container(
        row![
            // button(app.click_mode.to_string()).on_press(Message::ClickModeToggle),
            toggle_button,
            hint_button,
            score_display,
            face_button,
            time_display,
        ]
        .spacing(10)
        .align_y(Alignment::Center),
    )
    .align_x(Alignment::Center)
    .width(Length::Fill)
    .into()
}

fn score_display(score: isize) -> Element<'static, Message> {
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

fn time_display(started: Option<Instant>) -> Element<'static, Message> {
    let seconds = match started {
        Some(started) => started.elapsed().as_secs() as usize,
        None => 0,
    };

    let digits = [(seconds / 100) % 10, (seconds / 10) % 10, seconds % 10];
    row![
        get_clock_image(digits[0]).width(23),
        get_clock_image(digits[1]).width(23),
        get_clock_image(digits[2]).width(23)
    ]
    .into()
}

pub fn button_image(image: Image) -> Button<'static, Message> {
    button(image).padding(0).style(|_, _| button::Style {
        background: Some(iced::Background::Color(Color::TRANSPARENT)),
        ..Default::default()
    })
}
