use iced::{
    Alignment, Element, Length, Shadow, Vector, color,
    widget::{button, button::Style, container, row},
};

use crate::{game::ClickMode, message::Message};

pub fn header(click_mode: ClickMode) -> Element<'static, Message> {
    container(
        row![
            button(click_mode.to_string()).on_press(Message::ClickModeToggle),
            button("new game")
                .on_press(Message::GameNew)
                .style(|_theme, _status| {
                    Style {
                        shadow: Shadow {
                            color: color!(0, 0, 0, 0.35),
                            offset: Vector::new(2.0, 2.0),
                            blur_radius: 0.0,
                        },
                        ..Style::default()
                    }
                }),
            "world"
        ]
        .spacing(10)
        .align_y(Alignment::Center),
    )
    .align_x(Alignment::Center)
    .width(Length::Fill)
    .into()
}
