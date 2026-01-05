use iced::{
    Border, Color, Element, Length,
    widget::{container, mouse_area, text},
};

const DIGIT_LOOKUP: [&str; 9] = ["0", "1", "2", "3", "4", "5", "6", "7", "8"];

#[derive(Debug, Clone, Default, Copy)]
pub enum CellType {
    #[default]
    Hidden,
    Revealed,
    Flagged,
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub cell_type: CellType,
    pub is_mine: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            cell_type: CellType::default(),
            is_mine: false,
        }
    }
}

impl Cell {
    pub fn to_string(&self, neighboring_mines: u8) -> &'static str {
        match self.cell_type {
            CellType::Hidden => "⬛",
            CellType::Revealed => {
                if self.is_mine {
                    "💣"
                } else if neighboring_mines == 0 {
                    "  "
                } else {
                    DIGIT_LOOKUP[neighboring_mines as usize]
                }
            }
            CellType::Flagged => "🚩",
        }
    }
    pub fn display<'a, Message: 'a + Clone>(
        &self,
        neighboring_mines: u8,
        on_reveal: Message,
        on_chord: Message,
        on_flag: Message,
    ) -> Element<'a, Message> {
        let (cell_text, text_color): (&'static str, Color) = match self.cell_type {
            CellType::Hidden => ("", Color::from_rgb(0.7, 0.7, 0.7)),
            CellType::Revealed => {
                if self.is_mine {
                    ("💣", Color::from_rgb(1.0, 0.0, 0.0))
                } else {
                    (
                        DIGIT_LOOKUP[neighboring_mines as usize],
                        match neighboring_mines {
                            0 => Color::from_rgb(0.8, 0.8, 0.8),
                            1 => Color::from_rgb(0.0, 0.0, 1.0), // Blue
                            2 => Color::from_rgb(0.0, 0.5, 0.0), // Green
                            3 => Color::from_rgb(1.0, 0.0, 0.0), // Red
                            4 => Color::from_rgb(0.0, 0.0, 0.5), // Dark blue
                            5 => Color::from_rgb(0.5, 0.0, 0.0), // Dark red
                            6 => Color::from_rgb(0.0, 0.5, 0.5), // Cyan
                            7 => Color::from_rgb(0.0, 0.0, 0.0), // Black
                            _ => Color::from_rgb(0.5, 0.5, 0.5), // Gray
                        },
                    )
                }
            }
            CellType::Flagged => ("🚩", Color::from_rgb(1.0, 0.5, 0.0)),
        };
        let cell_type = self.cell_type;

        let (bg_color, border_color) = match cell_type {
            CellType::Hidden => (
                Color::from_rgb(0.4, 0.4, 0.4),
                Color::from_rgb(0.6, 0.6, 0.6),
            ),
            CellType::Revealed => (
                Color::from_rgb(0.9, 0.9, 0.9),
                Color::from_rgb(0.7, 0.7, 0.7),
            ),
            CellType::Flagged => (
                Color::from_rgb(0.5, 0.3, 0.1),
                Color::from_rgb(0.6, 0.6, 0.6),
            ),
        };

        let cell_container = container(text(cell_text).color(text_color).size(18))
            // .width(Length::Fixed(32.0))
            // .height(Length::Fixed(32.0))
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(move |_theme| container::Style {
                background: Some(iced::Background::Color(bg_color)),
                text_color: Some(text_color),
                border: Border {
                    color: border_color,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            });

        mouse_area(cell_container)
            .on_press(match cell_type {
                // Flagged calls won't do anything, so we don't need to handle them here
                CellType::Hidden | CellType::Flagged => on_reveal,
                CellType::Revealed => on_chord,
            })
            .on_right_press(on_flag)
            .into()
    }
}
