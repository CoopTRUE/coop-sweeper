use crate::grid::{Grid, GridSize, MinesAmt};

#[derive(Debug, Default)]
pub enum GameState {
    #[default]
    HomeScreen,
    CreationScreen(GridSize, MinesAmt),
    Initialized(GridSize, MinesAmt),
    Started(Grid),
    Over(Grid),
}

impl GameState {
    pub const ALL_DIFFS: &'static [GameState; 4] = &[
        GameState::DIFF_EASY,
        GameState::DIFF_MEDIUM,
        GameState::DIFF_HARD,
        GameState::DIFF_EXTREME,
    ];

    pub const DEFAULT_DIFF: GameState = GameState::Initialized(GridSize { rows: 9, cols: 9 }, 10);

    pub const DIFF_EASY: GameState = GameState::CreationScreen(GridSize { rows: 9, cols: 9 }, 10);
    pub const DIFF_MEDIUM: GameState =
        GameState::CreationScreen(GridSize { rows: 16, cols: 16 }, 40);
    pub const DIFF_HARD: GameState = GameState::CreationScreen(GridSize { rows: 16, cols: 30 }, 99);
    pub const DIFF_EXTREME: GameState =
        GameState::CreationScreen(GridSize { rows: 30, cols: 24 }, 160);
}
