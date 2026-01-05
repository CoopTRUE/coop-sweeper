use iced::{
    Alignment, Element, Length,
    widget::{button, container, row},
};

use crate::message::Message;

pub fn header() -> Element<'static, Message> {
    container(
        row![
            "hello",
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
