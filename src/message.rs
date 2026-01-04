use crate::grid::GridLoc;

#[derive(Clone, Debug)]
pub enum Message {
    InputRows(usize),
    InputCols(usize),
    InputMines(usize),
    GameStart,
    RevealClick(GridLoc),
    ChordClick(GridLoc),
    FlagClick(GridLoc),
}
