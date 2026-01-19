use crate::grid::{GridConfig, GridLoc};

#[derive(Clone, Debug)]
pub enum Message {
    // FaceHold,
    ClickRelease,

    NoOp,

    InputRows(usize),
    InputCols(usize),
    InputMines(usize),
    InputGridConfig(GridConfig),

    GameNew,
    GameStart,

    ClickModeToggle,

    RevealClick(GridLoc),
    ChordClick(GridLoc),
    FlagClick(GridLoc),

    HighlightCells(Vec<GridLoc>),

    Quit,
}
