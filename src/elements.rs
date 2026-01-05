use iced::{
    Alignment, Element, Length,
    widget::{button, container, row},
};

use crate::{game::ClickMode, message::Message};

pub fn header(click_mode: ClickMode) -> Element<'static, Message> {
    container(
        row![
            button(click_mode.to_string()).on_press(Message::ClickModeToggle),
            button("new game").on_press(Message::GameNew),
            "world"
        ]
        .spacing(10)
        .align_y(Alignment::Center),
    )
    .align_x(Alignment::Center)
    .width(Length::Fill)
    .into()
}
