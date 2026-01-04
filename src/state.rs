use crate::grid::{Grid, GridSize, MinesAmt};

#[derive(Debug)]
pub enum GameState {
    Uninitialized(GridSize, MinesAmt),
    Initialized(GridSize, MinesAmt),
    Started(Grid),
    Over(Grid),
}

impl Default for GameState {
    fn default() -> Self {
        // GameState::Uninitialized(GridSize::default(), 0)
        GameState::MEDIUM
    }
}

impl GameState {
    const EASY: GameState = GameState::Uninitialized(GridSize { rows: 9, cols: 9 }, 10);
    const MEDIUM: GameState = GameState::Uninitialized(GridSize { rows: 16, cols: 16 }, 40);
    const HARD: GameState = GameState::Uninitialized(GridSize { rows: 16, cols: 30 }, 99);
    const EXTREME: GameState = GameState::Uninitialized(GridSize { rows: 30, cols: 24 }, 160);
}
