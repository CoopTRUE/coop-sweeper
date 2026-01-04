use crate::cell::Cell;
use rand::random_range;

pub fn gen_grid(size: &GridSize, click: &GridLoc, mines: MinesAmt) -> Grid {
    let mut grid = vec![vec![Cell::default(); size.rows]; size.cols];
    for _ in 0..mines {
        loop {
            let x = random_range(0..size.rows);
            let y = random_range(0..size.cols);
            if (x, y) != (click.row, click.col) && !grid[y][x].is_mine {
                grid[y][x].is_mine = true;
                break;
            }
        }
    }

    grid
}

#[derive(Clone, Default, Debug)]
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
