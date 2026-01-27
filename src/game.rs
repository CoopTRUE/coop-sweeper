use std::time::Instant;

use crate::algorithms;
use crate::{
    assets::Face,
    cell::Cell,
    elements::header,
    grid::{CellChordResult, CellRevealResult, Grid, GridConfig, GridLoc},
    message::Message,
    state::{Difficulty, GameState},
    theme::*,
};
use iced::time::{self, seconds};
use iced::{Alignment, Background, Border, Color, Element, Length, Task, window};
use iced::{
    Subscription,
    widget::{button, column, container, grid as iced_grid, row, stack, text},
};
use iced_aw::number_input;

use GameState::*;
use Message::*;

#[derive(Clone, Copy, Debug, Default)]
pub enum ClickMode {
    #[default]
    Reveal,
    Flag,
}

impl ClickMode {
    pub fn opacity(self) -> f32 {
        match self {
            ClickMode::Reveal => 1.0,
            ClickMode::Flag => 0.6,
        }
    }
}

impl ClickMode {
    pub fn toggle(&mut self) {
        *self = match self {
            ClickMode::Reveal => ClickMode::Flag,
            ClickMode::Flag => ClickMode::Reveal,
        };
    }
    pub fn to_string(self) -> &'static str {
        match self {
            ClickMode::Reveal => "Normal Mode",
            ClickMode::Flag => "Flag Mode",
        }
    }
}

pub struct App {
    pub state: GameState,
    pub click_mode: ClickMode,
    pub face: Face,
    pub now: Instant,
    pub started: Option<Instant>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: GameState::default(),
            click_mode: ClickMode::default(),
            face: Face::default(),
            now: Instant::now(),
            started: None,
        }
    }
}

impl App {
    pub fn subscription(&self) -> Subscription<Message> {
        let is_animating = matches!(&self.state, Started(grid) if grid.is_animating(self.now));
        if is_animating {
            window::frames().map(|_| Message::NoOp)
        } else {
            match self.state {
                Started(..) => time::every(seconds(1)).map(|_| Message::NoOp),
                _ => Subscription::none(),
            }
        }
    }
    pub fn update(&mut self, message: Message, now: Instant) -> Task<Message> {
        self.now = now;
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
                self.started = Some(self.now);
                Started(grid)
            }
            (RevealClick(loc), Started(mut grid)) => {
                self.face = Face::Surprised;
                grid.clear_highlights(self.now);
                let hit_mine = matches!(grid.cascade_reveal(loc), CellRevealResult::Mine);
                Self::resolve_game_state(grid, hit_mine)
            }
            (ChordClick(loc), Started(mut grid)) => {
                self.face = Face::Surprised;
                grid.clear_highlights(self.now);
                let hit_mine = matches!(grid.chord_reveal(loc), CellChordResult::Mines(..));
                Self::resolve_game_state(grid, hit_mine)
            }
            (FlagClick(loc), Started(mut grid)) => {
                grid.clear_highlights(self.now);
                grid.flag_cell(loc);
                Self::resolve_game_state(grid, false)
            }
            (Quit, ..) => {
                std::process::exit(0);
            }
            (NoOp, state) => state,
            (RequestHint, Started(mut grid)) => {
                let hints = algorithms::generate_brute_force_hints(&mut grid);
                grid.highlight_cells(hints, self.now);
                Started(grid)
            }
            (message, state) => {
                unreachable!("Unhandled message: {:?}, {:?}", message, state);
            }
        };

        Task::none()
    }

    fn resolve_game_state(mut grid: Grid, hit_mine: bool) -> GameState {
        if hit_mine {
            grid.reveal_all();
            Lost(grid)
        } else if grid.is_won() {
            grid.reveal_all();
            Won(grid)
        } else {
            Started(grid)
        }
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

    /// Renders a grid with all interactions disabled (for game over states).
    fn render_disabled_grid(&self, grid: &Grid) -> iced_grid::Grid<'_, Message> {
        let buttons = (0..grid.rows()).flat_map(|row| {
            (0..grid.cols()).map(move |col| {
                grid.get(row, col).unwrap().display(
                    grid.count_neighboring_mines(GridLoc { row, col }),
                    NoOp,
                    NoOp,
                    NoOp,
                    self.now,
                )
            })
        });
        iced_grid(buttons).columns(grid.cols())
    }

    /// Creates a centered overlay container with semi-transparent background.
    fn overlay<'a>(
        content: impl Into<Element<'a, Message>>,
        alpha: f32,
    ) -> container::Container<'a, Message> {
        container(content)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(move |_theme| container::Style {
                background: Some(Background::Color(Color {
                    a: alpha,
                    ..Default::default()
                })),
                text_color: Some(Color::WHITE),
                border: Border::default(),
                ..Default::default()
            })
    }

    pub fn view(&self) -> Element<'_, Message> {
        let grid_inner: Element<'_, Message> = match &self.state {
            CreationScreen(GridConfig { mines, size }) => {
                let cells = (0..size.rows).flat_map(|_| {
                    (0..size.cols).map(|_| (Cell::default()).display(0, NoOp, NoOp, NoOp, self.now))
                });
                let grid_view = iced_grid(cells).columns(size.cols);
                let difficulties = row(Difficulty::DIFF_ALL.iter().map(Difficulty::display));

                let overlay = Self::overlay(
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
                    0.8,
                );

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
                            self.now,
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
                            self.now,
                        )
                    })
                });
                iced_grid(buttons).columns(grid.cols()).into()
            }
            Won(grid) => {
                let grid_view = self.render_disabled_grid(grid);

                let overlay = Self::overlay(
                    column![
                        text("🎉 Game Won! 🎉").size(48),
                        text("You found all the mines!").size(24),
                        button("Quit")
                            .on_press(Quit)
                            .padding(10)
                            .style(button::success),
                    ]
                    .spacing(10)
                    .align_x(Alignment::Center),
                    0.7,
                );

                stack![grid_view, overlay].into()
            }
            Lost(grid) => {
                let grid_view = self.render_disabled_grid(grid);

                let overlay = Self::overlay(
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
                    0.7,
                );

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
        container(column![header(self), grid])
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
