use crate::cell::CellType;
use crate::grid::{
    GridLoc, GridSize, count_neighboring_mines, gen_grid, reveal_cell, reveal_surrounding_cells,
};
use crate::message::Message;
use crate::state::GameState;
use iced::Element;
use iced::widget::{button, column, grid as iced_grid, mouse_area, text};
use iced_aw::number_input;

use GameState::*;
use Message::*;

#[derive(Default)]
pub struct App {
    state: GameState,
}

impl App {
    pub fn update(&mut self, message: Message) {
        let state = std::mem::replace(&mut self.state, Uninitialized(GridSize::default(), 0));
        self.state = match (message, state) {
            (InputRows(rows), Uninitialized(mut size, mines)) => {
                size.rows = rows;
                Uninitialized(size, mines)
            }
            (InputCols(cols), Uninitialized(mut size, mines)) => {
                size.cols = cols;
                Uninitialized(size, mines)
            }
            (InputMines(mines), Uninitialized(size, _)) => Uninitialized(size, mines),
            (GameStart, Uninitialized(size, mines)) => Initialized(size, mines),
            (RevealClick(loc), Initialized(size, mines)) => Started(gen_grid(&size, &loc, mines)),
            (RevealClick(loc), Started(mut grid)) => {
                let cell = &mut grid[loc.row][loc.col];
                match cell.cell_type {
                    CellType::Hidden => {
                        reveal_cell(&mut grid, loc);
                    }
                    CellType::Revealed => {}
                    CellType::Flagged => {}
                }
                Started(grid)
            }
            (RevealSurroundingClick(loc), Started(mut grid)) => {
                reveal_surrounding_cells(&mut grid, loc);
                Started(grid)
            }
            (FlagClick(loc), Started(mut grid)) => {
                println!("FlagClick: {:?}", loc);
                let cell = &mut grid[loc.row][loc.col];
                match cell.cell_type {
                    CellType::Hidden => cell.cell_type = CellType::Flagged,
                    CellType::Flagged => cell.cell_type = CellType::Hidden,
                    CellType::Revealed => {}
                }
                Started(grid)
            }
            (a, b) => {
                println!("{:?}", a);
                println!("{:?}", b);
                b
            }
        }
    }
    pub fn view(&self) -> Element<'_, Message> {
        match &self.state {
            Uninitialized(size, mines) => column![
                number_input(&size.rows, 0..100, InputRows),
                number_input(&size.cols, 0..100, InputCols),
                number_input(mines, 0..100, InputMines),
                button("Start").on_press(GameStart)
            ]
            .into(),
            Initialized(size, _) => {
                let buttons = (0..size.rows).flat_map(|row| {
                    (0..size.cols).map(move |col| {
                        mouse_area(text("⬛"))
                            .on_press(RevealClick(GridLoc { row, col }))
                            .into()
                    })
                });
                iced_grid(buttons).columns(size.cols).spacing(10).into()
            }
            Started(grid) => {
                let buttons: Vec<_> = grid
                    .iter()
                    .enumerate()
                    .flat_map(|(row_idx, row)| {
                        row.iter().enumerate().map(move |(col_idx, cell)| {
                            cell.display(
                                count_neighboring_mines(&grid, row_idx, col_idx),
                                RevealClick(GridLoc {
                                    row: row_idx,
                                    col: col_idx,
                                }),
                                RevealSurroundingClick(GridLoc {
                                    row: row_idx,
                                    col: col_idx,
                                }),
                                FlagClick(GridLoc {
                                    row: row_idx,
                                    col: col_idx,
                                }),
                            )
                        })
                    })
                    .collect();
                iced_grid(buttons).columns(grid[0].len()).spacing(10).into()
            }
            Over(_) => text("Over").into(),
        }
    }
}
