use crate::grid::GridLoc;

#[derive(Clone, Debug)]
pub enum Message {
    NoOp,
    InputRows(usize),
    InputCols(usize),
    InputMines(usize),
    GameNew,
    GameStart,
    ClickModeToggle,
    RevealClick(GridLoc),
    ChordClick(GridLoc),
    FlagClick(GridLoc),
    Quit,
}
