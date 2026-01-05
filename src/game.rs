use crate::cell::Cell;
use crate::grid::{CellChordResult, CellRevealResult, Grid, GridLoc, GridSize};
use crate::message::Message;
use crate::state::GameState;
use iced::Element;
use iced::widget::{button, column, container, grid as iced_grid, row, stack, text};
use iced::{Alignment, Border, Color, Length};
use iced_aw::number_input;

use GameState::*;
use Message::*;

#[derive(Default)]
pub struct App {
    state: GameState,
}

impl App {
    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
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
                // use rand::SeedableRng;
                // use rand_chacha::ChaCha20Rng;
                // let mut rng = ChaCha20Rng::seed_from_u64(6767);
                // grid.populate_mines_with_rng(loc, mines, &mut rng);
                grid.populate_mines(loc, mines);
                Started(grid)
            }
            (RevealClick(loc), Started(mut grid)) => match grid.cascade_reveal(loc) {
                CellRevealResult::Mine => Over(grid),
                _ => Started(grid),
            },
            (ChordClick(loc), Started(mut grid)) => match grid.chord_reveal(loc) {
                CellChordResult::Mines(_mines) => Over(grid),
                _ => Started(grid),
            },
            (FlagClick(loc), Started(mut grid)) => {
                grid.flag_cell(loc);
                Started(grid)
            }
            (Quit, _) => {
                std::process::exit(0);
            }
            (message, state) => {
                unreachable!("Unhandled message: {:?}, {:?}", message, state);
            }
        };
        iced::Task::none()
    }
    pub fn view(&self) -> Element<'_, Message> {
        let content: Element<'_, Message> = match &self.state {
            Uninitialized(size, mines) => column![
                text("🎮 Minesweeper").size(32),
                row![
                    text("Rows:").width(60),
                    number_input(&size.rows, 5..50, InputRows).width(100),
                ]
                .spacing(10)
                .align_y(Alignment::Center),
                row![
                    text("Cols:").width(60),
                    number_input(&size.cols, 5..50, InputCols).width(100),
                ]
                .spacing(10)
                .align_y(Alignment::Center),
                row![
                    text("Mines:").width(60),
                    number_input(mines, 1..999, InputMines).width(100),
                ]
                .spacing(10)
                .align_y(Alignment::Center),
                row![
                    button("Start Game")
                        .on_press(GameStart)
                        .padding(10)
                        .style(button::success),
                    button("Quit")
                        .on_press(Quit)
                        .padding(10)
                        .style(button::danger),
                ]
                .spacing(10),
            ]
            .spacing(15)
            .padding(30)
            .into(),
            Initialized(size, _) => {
                let buttons = (0..size.rows).flat_map(|row| {
                    (0..size.cols).map(move |col| {
                        (Cell::default()).display(
                            0,
                            RevealClick(GridLoc { row, col }),
                            RevealClick(GridLoc { row, col }),
                            RevealClick(GridLoc { row, col }),
                        )
                    })
                });
                column![
                    iced_grid(buttons).columns(size.cols).spacing(5),
                    button("Quit")
                        .on_press(Quit)
                        .padding(10)
                        .style(button::danger),
                ]
                .spacing(10)
                .align_x(Alignment::Center)
                .into()
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
                column![
                    iced_grid(buttons).columns(grid.cols()).spacing(5),
                    button("Quit")
                        .on_press(Quit)
                        .padding(10)
                        .style(button::danger),
                ]
                .spacing(10)
                .align_x(Alignment::Center)
                .into()
            }
            Over(grid) => {
                let buttons = (0..grid.rows()).flat_map(|row| {
                    (0..grid.cols()).map(move |col| {
                        grid.get(row, col).unwrap().display(
                            grid.count_neighboring_mines(&GridLoc { row, col }),
                            Quit,
                            Quit,
                            Quit,
                        )
                    })
                });
                let grid_view = iced_grid(buttons).columns(grid.cols()).spacing(5);

                let overlay = container(
                    column![
                        text("💥 Game Over! 💥").size(48),
                        text("You hit a mine!").size(24),
                        button("Quit")
                            .on_press(Quit)
                            .padding(10)
                            .style(button::danger),
                    ]
                    .spacing(10)
                    .align_x(Alignment::Center),
                )
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .style(|_theme| container::Style {
                    background: Some(iced::Background::Color(Color {
                        a: 0.7,
                        ..Default::default()
                    })),
                    text_color: Some(Color::WHITE),
                    border: Border::default(),
                    ..Default::default()
                });

                stack![grid_view, overlay].into()
            }
        };

        content
    }
}
