use std::fmt;

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

impl fmt::Display for Cell {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self.cell_type {
            CellType::Hidden => "⬛",
            CellType::Revealed => {
                if self.is_mine {
                    "💣"
                } else {
                    "⬜"
                }
            }
            CellType::Flagged => "🚩",
        })?;
        Ok(())
    }
}
