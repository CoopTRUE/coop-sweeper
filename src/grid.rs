use crate::cell::{Cell, CellType};
use rand::random_range;
use std::{cmp::min, collections::HashSet};

pub fn gen_grid(size: &GridSize, click: &GridLoc, mines: MinesAmt) -> Grid {
    let mut grid = vec![vec![Cell::default(); size.cols]; size.rows];
    grid[click.row][click.col].cell_type = CellType::Revealed;
    for _ in 0..mines {
        loop {
            let x = random_range(0..size.rows);
            let y = random_range(0..size.cols);
            if (x, y) != (click.row, click.col) && !grid[x][y].is_mine {
                grid[x][y].is_mine = true;
                break;
            }
        }
    }

    grid
}

pub fn count_neighboring_mines(grid: &Grid, row: usize, col: usize) -> u8 {
    let start_r = row.saturating_sub(1);
    let end_r = min(grid.len() - 1, row + 1);
    let start_c = col.saturating_sub(1);
    let end_c = min(grid[0].len() - 1, col + 1);

    let mut count = 0;
    for r in start_r..=end_r {
        for c in start_c..=end_c {
            if r == row && c == col {
                continue;
            }
            if grid[r][c].is_mine {
                count += 1;
            }
        }
    }
    count
}

/**
 * If a cell is hidden, reveals it and all adjacent cells that are not mines recursively, returning `Some(Ok(&Cell))`.
 *
 * If the cell is flagged, revealed, or doesn't exist, the function returns `None`.
 *
 * If the cell is a bomb, the function returns `Some(Err(&GridLoc))` containing the coordinates of the bomb (after revealing)
 */
pub fn reveal_cell<'a>(grid: &'a mut Grid, loc: GridLoc) -> Option<Result<&'a Cell, GridLoc>> {
    let cell_type = grid
        .get(loc.row)
        .and_then(|row| row.get(loc.col))?
        .cell_type;

    match cell_type {
        CellType::Hidden => {
            reveal_cell_recursive(grid, &mut HashSet::new(), &loc);
            grid.get(loc.row).and_then(|row| row.get(loc.col)).map(Ok)
        }
        CellType::Flagged => None,
        CellType::Revealed => grid.get(loc.row).and_then(|row| row.get(loc.col)).map(Ok),
    }
}

fn reveal_cell_recursive(grid: &mut Grid, visited: &mut HashSet<GridLoc>, loc: &GridLoc) {
    if visited.contains(loc) || (loc.row, loc.col) >= (grid.len(), grid[0].len()) {
        return;
    }

    let cell = &mut grid[loc.row][loc.col];
    match cell.cell_type {
        CellType::Flagged | CellType::Revealed => return,
        CellType::Hidden => {}
    }

    cell.cell_type = CellType::Revealed;
    if cell.is_mine {
        return;
    }

    visited.insert(loc.clone());

    let neighboring_mines = count_neighboring_mines(grid, loc.row, loc.col);
    if neighboring_mines == 0 {
        let start_r = loc.row.saturating_sub(1);
        let end_r = min(grid.len() - 1, loc.row + 1);
        let start_c = loc.col.saturating_sub(1);
        let end_c = min(grid[0].len() - 1, loc.col + 1);

        for r in start_r..=end_r {
            for c in start_c..=end_c {
                if r == loc.row && c == loc.col {
                    continue;
                }
                reveal_cell_recursive(grid, visited, &GridLoc { row: r, col: c });
            }
        }
    }
}

// pub fn reveal_surrounding_cells<'a>(grid: &'a mut Grid, loc: &GridLoc) -> Option<&'a Cell> {
//     let start_r = loc.row.saturating_sub(1);
//     let end_r = min(grid.len() - 1, loc.row + 1);
//     let start_c = loc.col.saturating_sub(1);
//     let end_c = min(grid[0].len() - 1, loc.col + 1);

//     let mut surrounding_flags: u8 = 0;
//     for r in start_r..=end_r {
//         for c in start_c..=end_c {
//             if r == loc.row && c == loc.col {
//                 continue;
//             }
//             let cell = &grid[r][c];
//             if cell.cell_type == CellType::Flagged {
//                 surrounding_flags += 1;
//             }
//         }
//     }

//     let neighboring_mines = count_neighboring_mines(grid, loc.row, loc.col);
//     if surrounding_flags == neighboring_mines {
//         for r in start_r..=end_r {
//             for c in start_c..=end_c {}
//         }
//     }
//     None
// }

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct GridLoc {
    pub row: usize,
    pub col: usize,
}

#[derive(Clone, Default, Debug)]
pub struct GridSize {
    pub rows: usize,
    pub cols: usize,
}

pub type Grid = Vec<Vec<Cell>>;
pub type MinesAmt = usize;
