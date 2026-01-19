use crate::assets::*;
use crate::message::Message;
use iced::{Element, widget::mouse_area};

const DIGIT_LOOKUP: [&str; 9] = ["0", "1", "2", "3", "4", "5", "6", "7", "8"];

#[derive(Clone, Copy, Debug, Default)]
pub enum CellType {
    #[default]
    Hidden,
    Revealed,
    Flagged,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Cell {
    pub cell_type: CellType,
    pub is_mine: bool,
}

impl Cell {
    pub fn to_string(self, neighboring_mines: u8) -> &'static str {
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
    pub fn display(
        &self,
        neighboring_mines: u8,
        on_reveal: Message,
        on_chord: Message,
        on_flag: Message,
    ) -> Element<'static, Message> {
        let sprite = match self.cell_type {
            CellType::Hidden => get_unrevealed_cell_image(),
            CellType::Revealed => {
                if self.is_mine {
                    get_mine_image()
                } else {
                    get_cell_image(neighboring_mines)
                }
            }
            CellType::Flagged => get_flag_image(),
        };
        let cell_type = self.cell_type;

        mouse_area(sprite)
            .on_press(match cell_type {
                // Flagged calls won't do anything, so we don't need to handle them here
                CellType::Hidden | CellType::Flagged => on_reveal,
                CellType::Revealed => on_chord,
            })
            .on_right_press(on_flag)
            .on_release(Message::ClickRelease)
            .into()
    }
}
