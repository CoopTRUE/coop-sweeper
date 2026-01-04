use iced::{
    Element,
    widget::{mouse_area, text},
};

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
    fn to_string(&self, neighboring_mines: u8) -> String {
        match self.cell_type {
            CellType::Hidden => "⬛".to_string(),
            CellType::Revealed => {
                if self.is_mine {
                    "💣".to_string()
                } else {
                    neighboring_mines.to_string()
                }
            }
            CellType::Flagged => "🚩".to_string(),
        }
    }
    pub fn display<'a, Message: 'a + Clone>(
        &self,
        neighboring_mines: u8,
        on_reveal: Message,
        on_flag: Message,
    ) -> Element<'a, Message> {
        let cell_text = self.to_string(neighboring_mines);
        mouse_area(text(cell_text))
            .on_press(on_reveal)
            .on_right_press(on_flag)
            .into()
    }
}
