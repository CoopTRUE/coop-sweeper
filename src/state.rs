use iced::{Element, widget::button};

use crate::{
    grid::{Grid, GridConfig, GridSize},
    message::Message,
};

#[derive(Debug)]
pub enum GameState {
    CreationScreen(GridConfig),
    Initialized(GridConfig),
    Started(Grid),
    Won(Grid),
    Lost(Grid),
}

impl Default for GameState {
    fn default() -> Self {
        Self::Initialized(Difficulty::default().state.clone())
    }
}

#[derive(Debug)]
pub struct Difficulty {
    pub name: &'static str,
    pub state: &'static GridConfig,
}

impl Difficulty {
    pub const DIFF_EASY: Self = Self {
        name: "Easy",
        state: &GridConfig {
            size: GridSize { rows: 9, cols: 9 },
            mines: 10,
        },
    };
    pub const DIFF_MEDIUM: Self = Self {
        name: "Medium",
        state: &GridConfig {
            size: GridSize { rows: 16, cols: 16 },
            mines: 40,
        },
    };
    pub const DIFF_HARD: Self = Self {
        name: "Hard",
        state: &GridConfig {
            size: GridSize { rows: 16, cols: 30 },
            mines: 99,
        },
    };
    pub const DIFF_EXTREME: Self = Self {
        name: "Extreme",
        state: &GridConfig {
            size: GridSize { rows: 30, cols: 24 },
            mines: 160,
        },
    };
    pub const DIFF_ALL: &[Self] = &[
        Self::DIFF_EASY,
        Self::DIFF_MEDIUM,
        Self::DIFF_HARD,
        Self::DIFF_EXTREME,
    ];

    pub fn display(&self) -> Element<'static, Message> {
        button(self.name)
            .on_press(Message::InputGridConfig(self.state.clone()))
            .into()
    }
}

impl Default for Difficulty {
    fn default() -> Self {
        Self::DIFF_EASY
    }
}
