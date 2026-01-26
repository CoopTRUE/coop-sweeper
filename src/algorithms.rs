use crate::cell::CellType;
use crate::grid::{Grid, GridLoc};

pub fn generate_brute_force_hints(grid: &mut Grid) -> Vec<GridLoc> {
    let mut hints = Vec::new();

    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            let loc = GridLoc { row, col };
            let is_revealed = matches!(grid.get(row, col).unwrap().cell_type, CellType::Revealed);

            if !is_revealed {
                continue;
            }

            let neighboring_hidden: Vec<GridLoc> = grid
                .neighbors(loc)
                .filter(|n| matches!(grid.get(n.row, n.col).unwrap().cell_type, CellType::Hidden))
                .collect();

            let neighboring_mines = grid.count_neighboring_mines(loc);
            let neighboring_flags = grid.count_neighboring_flags(loc);

            // If all mines around this cell are flagged, remaining hidden cells are safe to reveal
            if neighboring_flags == neighboring_mines && !neighboring_hidden.is_empty() {
                hints.extend(&neighboring_hidden);
            }

            // If the number of hidden cells equals the remaining mines, all hidden cells are mines
            let remaining_mines = neighboring_mines - neighboring_flags;
            if neighboring_hidden.len() as u8 == remaining_mines && remaining_mines > 0 {
                hints.extend(neighboring_hidden);
            }

            if !hints.is_empty() {
                hints.dedup();
                return hints;
            }
        }
    }

    hints
}
