use crate::{
    assets::Face,
    cell::Cell,
    elements::header,
    grid::{CellChordResult, CellRevealResult, Grid, GridConfig, GridLoc},
    message::Message,
    state::{Difficulty, GameState},
    theme::*,
};
use iced::widget::{button, column, container, grid as iced_grid, mouse_area, row, stack, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};
use iced_aw::number_input;

use GameState::*;
use Message::*;

#[derive(Debug, Default, Clone, Copy)]
pub enum ClickMode {
    #[default]
    Reveal,
    Flag,
}

impl ClickMode {
    pub fn toggle(&mut self) {
        *self = match self {
            ClickMode::Reveal => ClickMode::Flag,
            ClickMode::Flag => ClickMode::Reveal,
        };
    }
    pub fn to_string(&self) -> &'static str {
        match self {
            ClickMode::Reveal => "Normal Mode",
            ClickMode::Flag => "Flag Mode",
        }
    }
}

#[derive(Default)]
pub struct App {
    pub state: GameState,
    pub click_mode: ClickMode,
    pub face: Face,
}

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        let state = std::mem::take(&mut self.state);
        self.state = match (message, state) {
            // (FaceHold, state) => {
            //     self.face = Face::Surprised;
            //     println!("FaceHold");
            //     state
            // }
            (ClickRelease, state) => {
                self.face = Face::Happy;
                println!("FaceRelease");
                state
            }
            (GameNew, _) => CreationScreen(Difficulty::default().state.clone()),
            (ClickModeToggle, state) => {
                self.click_mode.toggle();
                state
            }
            (InputRows(rows), CreationScreen(GridConfig { mines, mut size })) => {
                size.rows = rows;
                CreationScreen(GridConfig { mines, size })
            }
            (InputCols(cols), CreationScreen(GridConfig { mines, mut size })) => {
                size.cols = cols;
                CreationScreen(GridConfig { mines, size })
            }
            (InputMines(mines), CreationScreen(GridConfig { size, .. })) => {
                CreationScreen(GridConfig { mines, size })
            }
            (InputGridConfig(grid_config), CreationScreen(..)) => CreationScreen(grid_config),
            (GameStart, CreationScreen(GridConfig { mines, size })) => {
                Initialized(GridConfig { mines, size })
            }
            (RevealClick(loc), Initialized(GridConfig { mines, size })) => {
                self.face = Face::Surprised;
                let mut grid = Grid::new(size);
                // use rand::SeedableRng;
                // use rand_chacha::ChaCha20Rng;s
                // let mut rng = ChaCha20Rng::seed_from_u64(6767);
                // grid.populate_mines_with_rng(loc, mines, &mut rng);
                grid.populate_mines(loc, mines);
                print!("Initialized");
                Started(grid)
            }
            (RevealClick(loc), Started(mut grid)) => {
                self.face = Face::Surprised;
                match grid.cascade_reveal(loc) {
                    CellRevealResult::Mine => Over(grid),
                    _ => Started(grid),
                }
            }
            (ChordClick(loc), Started(mut grid)) => {
                self.face = Face::Surprised;
                match grid.chord_reveal(loc) {
                    CellChordResult::Mines(_mines) => Over(grid),
                    _ => Started(grid),
                }
            }
            (FlagClick(loc), Started(mut grid)) => {
                grid.flag_cell(loc);
                Started(grid)
            }
            (Quit, _) => {
                std::process::exit(0);
            }
            (NoOp, state) => state,
            (message, state) => {
                unreachable!("Unhandled message: {:?}, {:?}", message, state);
            }
        };
        Task::none()
    }
    fn create_message_handler(&self, message: Message) -> Message {
        match self.click_mode {
            ClickMode::Reveal => message,
            ClickMode::Flag => match message {
                RevealClick(loc) => FlagClick(loc),
                FlagClick(loc) => RevealClick(loc),
                ChordClick(loc) => ChordClick(loc),
                _ => unreachable!("Unhandled message: {:?}", message),
            },
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let grid_inner: Element<'_, Message> = match &self.state {
            HomeScreen => {
                let cells = (0..9)
                    .flat_map(|_| (0..9).map(|_| (Cell::default()).display(0, NoOp, NoOp, NoOp)));
                iced_grid(cells).columns(9).spacing(5).into()
            }
            CreationScreen(GridConfig { mines, size }) => {
                let cells = (0..size.rows).flat_map(|_| {
                    (0..size.cols).map(|_| (Cell::default()).display(0, NoOp, NoOp, NoOp))
                });
                let grid_view = iced_grid(cells).columns(size.cols);
                let difficulties = row(Difficulty::DIFF_ALLL.iter().map(Difficulty::display));

                let overlay = container(
                    column![
                        text("🎮 Minesweeper").size(32),
                        difficulties,
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
                    .align_x(Alignment::Center),
                )
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .style(|_theme| container::Style {
                    background: Some(Background::Color(Color {
                        a: 0.8,
                        ..Default::default()
                    })),
                    text_color: Some(Color::WHITE),
                    border: Border::default(),
                    ..Default::default()
                });

                stack![grid_view, overlay].into()
            }
            Initialized(GridConfig { size, .. }) => {
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
                iced_grid(buttons).columns(size.cols).into()
            }
            Started(grid) => {
                let buttons = (0..grid.rows()).flat_map(|row| {
                    (0..grid.cols()).map(move |col| {
                        grid.get(row, col).unwrap().display(
                            grid.count_neighboring_mines(GridLoc { row, col }),
                            self.create_message_handler(RevealClick(GridLoc { row, col })),
                            self.create_message_handler(ChordClick(GridLoc { row, col })),
                            self.create_message_handler(FlagClick(GridLoc { row, col })),
                        )
                    })
                });
                iced_grid(buttons).columns(grid.cols()).into()
            }
            Over(grid) => {
                let buttons = (0..grid.rows()).flat_map(|row| {
                    (0..grid.cols()).map(move |col| {
                        grid.get(row, col).unwrap().display(
                            grid.count_neighboring_mines(GridLoc { row, col }),
                            NoOp,
                            NoOp,
                            NoOp,
                        )
                    })
                });
                let grid_view = iced_grid(buttons).columns(grid.cols());

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
                    background: Some(Background::Color(Color {
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
        let grid = container(grid_inner)
            .padding(20)
            .center_y(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(GRID_CONTAINER_BACKGROUND_COLOR),
                ..Default::default()
            });
        container(column![header(&self), grid])
            .style(|_theme| container::Style {
                background: Some(BACKGROUND_COLOR),
                text_color: Some(TEXT_COLOR),
                ..Default::default()
            })
            .center(Length::Fill)
            .padding(20)
            .into()
    }
}
