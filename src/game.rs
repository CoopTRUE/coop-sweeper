use crate::cell::CellType;
use crate::grid::{Grid, GridLoc, GridSize, MinesAmt, gen_grid};
use iced::widget::{button, column, grid as iced_grid, row, text};
use iced::{Element, keyboard};
use iced_aw::number_input;

#[derive(Clone, Debug)]
pub enum Message {
    InputRows(usize),
    InputCols(usize),
    InputMines(usize),
    GameStart,
    RevealClick(GridLoc),
    FlagClick(GridLoc),
}

#[derive(Clone, Debug)]
pub enum GameState {
    Uninitialized(GridSize, MinesAmt),
    Initialized(GridSize, MinesAmt),
    Started(Grid),
    Over(Grid),
}

impl Default for GameState {
    fn default() -> Self {
        Self::Uninitialized(GridSize::default(), 0)
    }
}

#[derive(Default)]
pub struct App {
    state: GameState,
}

impl App {
    pub fn update(&mut self, message: Message) {
        match (message, self.state.clone()) {
            (Message::InputRows(rows), GameState::Uninitialized(mut size, mines)) => {
                size.rows = rows;
                self.state = GameState::Uninitialized(size, mines);
            }
            (Message::InputCols(cols), GameState::Uninitialized(mut size, mines)) => {
                size.cols = cols;
                self.state = GameState::Uninitialized(size, mines);
            }
            (Message::InputMines(mines), GameState::Uninitialized(size, _)) => {
                self.state = GameState::Uninitialized(size, mines);
            }
            (Message::GameStart, GameState::Uninitialized(size, mines)) => {
                self.state = GameState::Initialized(size, mines);
            }
            (Message::RevealClick(loc), GameState::Initialized(size, mines)) => {
                self.state = GameState::Started(gen_grid(&size, &loc, mines));
            }
            (Message::RevealClick(loc), GameState::Started(mut grid)) => {
                match grid[loc.row][loc.col].cell_type {
                    CellType::Hidden => grid[loc.row][loc.col].cell_type = CellType::Revealed,
                    _ => {}
                }
                self.state = GameState::Started(grid);
            }
            (Message::FlagClick(loc), GameState::Started(mut grid)) => {
                match grid[loc.row][loc.col].cell_type {
                    CellType::Hidden => grid[loc.row][loc.col].cell_type = CellType::Flagged,
                    CellType::Flagged => grid[loc.row][loc.col].cell_type = CellType::Hidden,
                    CellType::Revealed => {}
                }
                self.state = GameState::Started(grid);
            }
            (a @ _, b @ _) => {
                println!("{:?}", a);
                println!("{:?}", b)
            }
        }
    }
    pub fn view(&self) -> Element<'_, Message> {
        println!("{:?}", self.state);
        match &self.state {
            GameState::Uninitialized(size, mines) => column![
                number_input(&size.rows, 0..100, Message::InputRows),
                number_input(&size.cols, 0..100, Message::InputCols),
                number_input(mines, 0..100, Message::InputMines),
                button("Start").on_press(Message::GameStart)
            ]
            .into(),
            GameState::Initialized(size, _) => {
                let buttons = (0..size.rows).flat_map(|row| {
                    (0..size.cols).map(move |col| {
                        button("⬛")
                            .on_press(Message::RevealClick(GridLoc { row, col }))
                            .into()
                    })
                });
                iced_grid(buttons)
                    .columns(size.cols)
                    .spacing(10)
                    .into()
            }
            GameState::Started(grid) => {
                let buttons: Vec<_> = grid.iter().enumerate().flat_map(|(row_idx, row)| {
                    row.iter().enumerate().map(move |(col_idx, cell)| {
                        let cell_text = cell.to_string();
                        button(text(cell_text))
                            .on_press(Message::RevealClick(GridLoc {
                                row: row_idx,
                                col: col_idx,
                            }))
                            .into()
                    })
                }).collect();
                iced_grid(buttons)
                    .columns(grid[0].len())
                    .spacing(10)
                    .into()
            }
            GameState::Over(_) => text("Over").into(),
        }
    }
}
