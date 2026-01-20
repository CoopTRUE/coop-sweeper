use iced::{
    Alignment, Element, Length,
    widget::{button, container, mouse_area, row},
};

use crate::{assets::*, game::App, message::Message};

pub fn header(app: &App) -> Element<'static, Message> {
    container(
        row![
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
