use super::board::*;
use config::Config;
use super::cell::*;
use std::collections::HashSet;

pub fn new_board(config: Config) -> Board {
    let height = config.get_int("height").unwrap() as usize;
    let width = config.get_int("width").unwrap() as usize;

    Board::from_size(height, width)
}

pub fn evolve(board: &Board) -> Board {
    let mut candidates: Cells = Cells::new();

    for cell in &board.cells {
        candidates.insert(*cell);
        visit_neighbors(*cell, |c| {candidates.insert(c); });
    }

    let cells = candidates.into_iter().filter(|cell| {
        evaluate_cell(board.contains(*cell), neighbors(board, *cell))
    }).collect();

    Board::new(cells,board.height, board.width)
}

pub fn evaluate_cell(alive: bool, neighbors: usize) -> bool {
    if alive {
        neighbors == 2 || neighbors == 3
    } else {
        neighbors == 3
    }
}


pub fn visit_neighbors<F>(cell:Cell, mut visitor: F )
            where F: FnMut(Cell) {
    for row in cell.row - 1..=cell.row + 1 {
        for col in cell.col - 1..=cell.col + 1 {
            let visitee = Cell::new(row, col);
            if visitee != cell {
                visitor(visitee);
            }
        }
    }
}

pub fn neighbors(board: &Board, cell: Cell) -> usize {
    let mut n = 0;

    visit_neighbors(cell, |c| if board.contains(c) {n += 1});

    n
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn a_new_game_sets_up_a_board_from_configuration() {
        let mut config = config::Config::default();
        config.set("height", 5).unwrap();
        config.set("width", 4).unwrap();

        let board = new_board(config);

        assert_eq!(board.width(), 4);
        assert_eq!(board.height(), 5);
        assert!(board.is_empty());
    }

    #[test]
    fn all_neighbors_are_counted() {
        let cell = Cell::new(3, 2);
        let n = crate::engine::life::neighbors(&from_grid(r"
            . . . . .
            . . . . .
            . * * * .
            . * * * .
            . * * * .
            . . . . .
        "), cell);

        assert_eq!(n, 8);
    }

    #[test]
    fn a_live_cell_with_less_than_2_neighbors_dies() {
        for n in vec![0, 1] {
            assert!(!evaluate_cell(true, n));
        }
    }

    #[test]
    fn a_live_cell_with_2_or_3_neighbors_lives() {
        for n in vec![2, 3] {
            assert!(evaluate_cell(true, n));
        }
    }

    #[test]
    fn a_live_cell_with_more_than_three_neighbors_dies() {
        for n in vec![4, 5, 6, 7, 8] {
            assert!(!evaluate_cell(true, n));
        }
    }

    #[test]
    fn a_dead_cell_with_3_neighbors_is_born() {
        assert!(evaluate_cell(false, 3));
    }

    #[test]
    fn a_dead_cell_with_other_than_3_neighbors_dies() {
        for n in vec![0, 1, 2, 4, 5, 6, 7, 8] {
            assert!(!evaluate_cell(false, n))
        }
    }

    #[test]
    fn live_cells_that_should_stay_alive_stay_alive() {
        let candidates = vec![
            Cell::new(1, 1), // two neighbor
            Cell::new(1, 5), // three neighbors
        ];

        let before = from_grid(r"
            * . . . * * *
            . * . . . * .
            . . * . . . .
        ");

        let after = evolve(&before);

        for c in candidates {
            assert_alive(&before, c);
            assert_alive(&after, c);
        }
    }

    #[test]
    fn live_cells_that_should_die_die() {
        let candidates = vec![
            Cell::new(2, 3), // no neighbors
            Cell::new(1, 1), // one neighbor
            Cell::new(1, 5), // four neighbors
        ];

        let before = from_grid(r"
            * . . . * * *
            . * . . . * *
            . . . * . . .
        ");

        let after = evolve(&before);

        for c in candidates {
            assert_alive(&before, c);
            assert_dead(&after, c);
        }
    }

    #[test]
    fn dead_cells_that_should_be_born_are_born() {
        let candidates = vec![
            Cell::new(1, 5), // three neighbors
        ];

        let before = from_grid(r"
            . . . . * * *
            . . . . . . .
            . . . . . . .
        ");

        let after = evolve(&before);

        for c in candidates {
            assert_dead(&before, c);
            assert_alive(&after, c);
        }
    }

    fn assert_alive(board: &Board, cell: Cell) {
        assert!(board.contains(cell), "{:?} should contain {:?}", board, cell);
    }

    fn assert_dead(board: &Board, cell: Cell) {
        assert!(!board.contains(cell), "{:?} should not contain {:?}", board, cell);
    }

    pub fn from_grid(grid: &str) -> Board {

        if grid.is_empty() {
            panic!("Grid cannot be empty");
        } else {
            let mut cells:Cells = HashSet::new();
            let mut width = 0;
            let mut height = 0;

            for (y, line) in grid.trim().split("\n").enumerate() {
                height = y + 1;
                for (x, token) in line.trim().split(" ").enumerate() {
                    width = x + 1;
                    if token == "*" {
                        cells.insert(Cell::new(y as isize, x as isize));
                    }
                }
            }

            Board {cells, width, height}
        }
    }

}

