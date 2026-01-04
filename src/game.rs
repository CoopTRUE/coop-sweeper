use crate::grid::{Grid, GridLoc, GridSize};
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
            (RevealClick(loc), Initialized(size, mines)) => {
                let mut grid = Grid::new(size);
                grid.populate_mines(loc, mines);
                Started(grid)
            }
            (RevealClick(loc), Started(mut grid)) => {
                grid.cascade_reveal(loc);
                Started(grid)
            }
            (ChordClick(loc), Started(mut grid)) => {
                grid.chord_reveal(loc);
                Started(grid)
            }
            (FlagClick(loc), Started(mut grid)) => {
                grid.flag_cell(loc);
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
                let buttons = (0..grid.rows()).flat_map(|row| {
                    (0..grid.cols()).map(move |col| {
                        grid.get(row, col).unwrap().display(
                            grid.count_neighboring_mines(&GridLoc { row, col }),
                            RevealClick(GridLoc { row, col }),
                            ChordClick(GridLoc { row, col }),
                            FlagClick(GridLoc { row, col }),
                        )
                    })
                });
                iced_grid(buttons).columns(grid.cols()).spacing(10).into()
            }
            Over(_) => text("Over").into(),
        }
    }
}
