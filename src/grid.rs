use rand::Rng;

use crate::cell::{Cell, CellType};
use std::{cmp::min, collections::HashSet, fmt, time::Instant};

#[derive(Clone, Debug)]
pub struct GridConfig {
    pub size: GridSize,
    pub mines: MinesAmt,
}

pub enum CellRevealResult {
    Success,
    Mine,
    AlreadyRevealed,
    Flagged,
    OutOfBounds,
}

pub enum CellFlagResult {
    Success,
    // Already flagged cells will be toggled back to hidden
    // AlreadyFlagged,
    AlreadyRevealed,
    OutOfBounds,
}

pub enum CellChordResult {
    Success,
    Mines(Vec<GridLoc>),
    InvalidFlagCount,
    Hidden,
    Flagged,
    OutOfBounds,
}

#[derive(Debug)]
pub struct Grid {
    cells: Vec<Vec<Cell>>,
    populated: bool,
}

impl Grid {
    // ==================== Constructor ====================

    /// Creates a new grid with the specified dimensions.
    ///
    /// All cells are initialized to their default state (hidden, no mine).
    /// Call `populate_mines` to place mines after the first click.
    pub fn new(size: GridSize) -> Self {
        Self {
            cells: vec![vec![Cell::default(); size.cols]; size.rows],
            populated: false,
        }
    }

    // ==================== Initialization ====================

    /// Populates the grid with mines, avoiding the specified location.
    ///
    /// This ensures the first click is never a mine. After placing mines,
    /// automatically performs a cascade reveal from the clicked location.
    pub fn populate_mines(&mut self, loc: GridLoc, mines: MinesAmt) {
        self.populate_mines_with_rng(loc, mines, &mut rand::rng());
    }

    /// Populates the grid with mines using a custom RNG for deterministic testing.
    ///
    /// # Panics
    /// Panics if the grid has already been populated.
    pub fn populate_mines_with_rng<R: Rng>(&mut self, loc: GridLoc, mines: MinesAmt, rng: &mut R) {
        if self.populated {
            unreachable!("Grid already populated");
        }
        for _ in 0..mines {
            loop {
                let x = rng.random_range(0..self.rows());
                let y = rng.random_range(0..self.cols());

                if (x, y) != (loc.row, loc.col) && !self.cells[x][y].is_mine {
                    self.cells[x][y].is_mine = true;
                    break;
                }
            }
        }
        self.cascade_reveal(loc);
    }

    // ==================== Dimension Accessors ====================

    /// Returns the number of rows in the grid.
    pub fn rows(&self) -> usize {
        self.cells.len()
    }

    /// Returns the number of columns in the grid.
    pub fn cols(&self) -> usize {
        self.cells[0].len()
    }

    // ==================== Cell Accessors ====================

    /// Returns a reference to the cell at the specified location, if valid.
    pub fn get(&self, row: usize, col: usize) -> Option<&Cell> {
        self.cells.get(row)?.get(col)
    }

    /// Returns a mutable reference to the cell at the specified location, if valid.
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut Cell> {
        self.cells.get_mut(row)?.get_mut(col)
    }

    /// Returns an iterator over all neighboring cell locations (excluding the center).
    ///
    /// Neighbors include all 8 adjacent cells (orthogonal and diagonal),
    /// automatically clamped to grid boundaries.
    pub fn neighbors(&self, loc: GridLoc) -> impl Iterator<Item = GridLoc> {
        let start_r = loc.row.saturating_sub(1);
        let end_r = min(self.rows() - 1, loc.row + 1);
        let start_c = loc.col.saturating_sub(1);
        let end_c = min(self.cols() - 1, loc.col + 1);

        (start_r..=end_r).flat_map(move |r| {
            (start_c..=end_c).filter_map(move |c| {
                if r == loc.row && c == loc.col {
                    None
                } else {
                    Some(GridLoc { row: r, col: c })
                }
            })
        })
    }

    // ==================== Counting & Queries ====================

    /// Returns the total number of mines in the grid.
    pub fn count_mines(&self) -> usize {
        self.cells
            .iter()
            .flatten()
            .filter(|cell| cell.is_mine)
            .count()
    }

    /// Returns the total number of flagged cells in the grid.
    pub fn count_flags(&self) -> usize {
        self.cells
            .iter()
            .flatten()
            .filter(|cell| matches!(cell.cell_type, CellType::Flagged))
            .count()
    }

    /// Returns the count of flagged cells adjacent to the specified location.
    pub fn count_neighboring_flags(&self, loc: GridLoc) -> u8 {
        self.neighbors(loc)
            .filter(|n| matches!(self.cells[n.row][n.col].cell_type, CellType::Flagged))
            .count() as u8
    }

    /// Returns the count of mines adjacent to the specified location.
    ///
    /// This is the number displayed on revealed cells.
    pub fn count_neighboring_mines(&self, loc: GridLoc) -> u8 {
        self.neighbors(loc)
            .filter(|n| self.cells[n.row][n.col].is_mine)
            .count() as u8
    }

    /// Returns `true` if all mines are flagged.
    pub fn all_mines_flagged(&self) -> bool {
        self.cells
            .iter()
            .flatten()
            .filter(|cell| cell.is_mine)
            .all(|cell| matches!(cell.cell_type, CellType::Flagged))
    }

    /// Returns `true` if the game is won.
    pub fn is_won(&self) -> bool {
        self.count_mines() == self.count_flags() && self.all_mines_flagged()
    }

    /// Returns `true` if any cell highlight animation is currently in progress.
    pub fn is_animating(&self, now: Instant) -> bool {
        self.cells
            .iter()
            .any(|row| row.iter().any(|cell| cell.highlight.is_animating(now)))
    }

    // ==================== Reveal Operations ====================

    /// Reveals a single cell without cascading.
    ///
    /// Returns the result indicating success, mine hit, or why the reveal failed.
    fn reveal_cell(&mut self, loc: GridLoc) -> CellRevealResult {
        let Some(cell) = self.get_mut(loc.row, loc.col) else {
            return CellRevealResult::OutOfBounds;
        };
        match cell.cell_type {
            CellType::Hidden => {
                cell.cell_type = CellType::Revealed;
                if cell.is_mine {
                    CellRevealResult::Mine
                } else {
                    CellRevealResult::Success
                }
            }
            CellType::Revealed => CellRevealResult::AlreadyRevealed,
            CellType::Flagged => CellRevealResult::Flagged,
        }
    }

    /// Reveals a cell and recursively reveals all adjacent cells if there are no neighboring mines.
    ///
    /// This implements the classic minesweeper "flood fill" behavior where clicking
    /// on an empty cell (0 neighboring mines) automatically reveals all connected
    /// empty cells and their borders.
    ///
    /// Returns `Mine` if a mine was revealed, otherwise `Success`.
    pub fn cascade_reveal(&mut self, loc: GridLoc) -> CellRevealResult {
        self.cascade_reveal_recursive(&loc, &mut HashSet::new())
    }

    /// Internal recursive helper for cascade reveal.
    ///
    /// Uses a visited set to prevent infinite loops and redundant processing.
    fn cascade_reveal_recursive(
        &mut self,
        loc: &GridLoc,
        visited: &mut HashSet<GridLoc>,
    ) -> CellRevealResult {
        if loc.row >= self.rows() || loc.col >= self.cols() {
            return CellRevealResult::OutOfBounds;
        }

        if visited.contains(loc) {
            return CellRevealResult::Success;
        }

        let Some(cell) = self.get_mut(loc.row, loc.col) else {
            return CellRevealResult::OutOfBounds;
        };

        match cell.cell_type {
            CellType::Revealed => return CellRevealResult::AlreadyRevealed,
            CellType::Flagged => return CellRevealResult::Flagged,
            CellType::Hidden => {}
        }

        cell.cell_type = CellType::Revealed;
        if cell.is_mine {
            return CellRevealResult::Mine;
        }

        visited.insert(*loc);

        let neighboring_mines = self.count_neighboring_mines(*loc);
        if neighboring_mines == 0 {
            let neighbor_locs: Vec<_> = self.neighbors(*loc).collect();
            for neighbor in neighbor_locs {
                self.cascade_reveal_recursive(&neighbor, visited);
            }
        }

        CellRevealResult::Success
    }

    /// Performs a chord reveal (middle-click behavior) on a revealed cell.
    ///
    /// If the number of adjacent flags matches the cell's mine count,
    /// reveals all non-flagged neighbors. This is a common speedrun technique.
    ///
    /// Returns `Mines(locs)` if any mines were hit (game over), indicating
    /// which mines were incorrectly unflagged.
    pub fn chord_reveal(&mut self, loc: GridLoc) -> CellChordResult {
        if loc.row >= self.rows() || loc.col >= self.cols() {
            return CellChordResult::OutOfBounds;
        }

        let Some(cell) = self.get(loc.row, loc.col) else {
            return CellChordResult::OutOfBounds;
        };

        match cell.cell_type {
            CellType::Hidden => return CellChordResult::Hidden,
            CellType::Flagged => return CellChordResult::Flagged,
            CellType::Revealed => {}
        }

        let neighboring_mines = self.count_neighboring_mines(loc);
        let neighboring_flags = self.count_neighboring_flags(loc);

        if neighboring_flags != neighboring_mines {
            return CellChordResult::InvalidFlagCount;
        }

        let neighbor_locs: Vec<_> = self.neighbors(loc).collect();
        let mut mines_hit = Vec::new();
        for neighbor in neighbor_locs {
            if matches!(self.cascade_reveal(neighbor), CellRevealResult::Mine) {
                mines_hit.push(neighbor);
            }
        }

        if mines_hit.is_empty() {
            CellChordResult::Success
        } else {
            CellChordResult::Mines(mines_hit)
        }
    }

    /// Reveals all cells in the grid.
    ///
    /// Used at game end to show the complete board state.
    pub fn reveal_all(&mut self) {
        for row in 0..self.rows() {
            for col in 0..self.cols() {
                self.reveal_cell(GridLoc { row, col });
            }
        }
    }

    // ==================== Flag Operations ====================

    /// Toggles the flag state of a cell.
    ///
    /// - Hidden cells become flagged
    /// - Flagged cells become hidden (unflagged)
    /// - Revealed cells cannot be flagged
    pub fn flag_cell(&mut self, loc: GridLoc) -> CellFlagResult {
        let Some(cell) = self.get_mut(loc.row, loc.col) else {
            return CellFlagResult::OutOfBounds;
        };
        match cell.cell_type {
            CellType::Hidden => {
                cell.cell_type = CellType::Flagged;
                CellFlagResult::Success
            }
            CellType::Revealed => CellFlagResult::AlreadyRevealed,
            CellType::Flagged => {
                cell.cell_type = CellType::Hidden;
                CellFlagResult::Success
            }
        }
    }

    // ==================== Highlight Operations ====================

    /// Activates the highlight animation on the specified cells.
    ///
    /// Used for visual feedback, such as indicating incorrectly flagged mines.
    pub fn highlight_cells(&mut self, locs: Vec<GridLoc>, now: Instant) {
        for loc in locs {
            self.get_mut(loc.row, loc.col)
                .unwrap()
                .highlight
                .go_mut(true, now);
        }
    }

    /// Deactivates highlight animations on all cells.
    pub fn clear_highlights(&mut self, now: Instant) {
        for row in 0..self.rows() {
            for col in 0..self.cols() {
                self.get_mut(row, col).unwrap().highlight.go_mut(false, now);
            }
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (row_index, row) in self.cells.iter().enumerate() {
            if row_index > 0 {
                writeln!(f)?;
            }
            for (col_index, cell) in row.iter().enumerate() {
                if col_index > 0 {
                    write!(f, " ")?;
                }
                let neighboring_mines = self.count_neighboring_mines(GridLoc {
                    row: row_index,
                    col: col_index,
                });
                write!(
                    f,
                    "{}{}",
                    cell.to_string(neighboring_mines),
                    if neighboring_mines == 0 { "" } else { " " }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct GridLoc {
    pub row: usize,
    pub col: usize,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GridSize {
    pub rows: usize,
    pub cols: usize,
}

pub type MinesAmt = usize;

#[cfg(test)]
mod tests {

    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;

    #[test]
    fn test_grid_populate_mines() {
        let mut rng = ChaCha20Rng::seed_from_u64(6767);
        let mut grid = Grid::new(GridSize { rows: 9, cols: 9 });
        grid.populate_mines_with_rng(GridLoc { row: 4, col: 4 }, 10, &mut rng);
        // grid.populate_mines(GridLoc { row: 4, col: 4 }, 10);
        grid.reveal_all();
        println!("{}", grid);
    }
}
