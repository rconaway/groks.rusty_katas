use std::collections::HashSet;

pub type Cells = HashSet<Cell>;

#[derive(Debug, PartialEq)]
pub struct Board {
    pub cells: Cells,
    pub height: usize,
    pub width: usize,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct Cell {
    pub row: isize,
    pub col: isize,
}

pub fn evolve(board: &Board) -> Board {
    let mut candidates: Cells = Cells::new();

    for cell in &board.cells {
        candidates.insert(*cell);
        visit_neighbors(*cell, |c| { candidates.insert(c); });
    }

    let cells = candidates.into_iter().filter(|cell| {
        evaluate_cell(board.cells.contains(cell), neighbors(board, *cell))
    }).collect();

    Board { cells, height: board.height, width: board.width }
}

pub fn evaluate_cell(alive: bool, neighbors: usize) -> bool {
    if alive {
        neighbors == 2 || neighbors == 3
    } else {
        neighbors == 3
    }
}

pub fn visit_neighbors<F>(cell: Cell, mut visitor: F)
    where F: FnMut(Cell) {
    for row in cell.row - 1..=cell.row + 1 {
        for col in cell.col - 1..=cell.col + 1 {
            let visitee = Cell { row, col };
            if visitee != cell {
                visitor(visitee);
            }
        }
    }
}

pub fn neighbors(board: &Board, cell: Cell) -> usize {
    let mut n = 0;

    visit_neighbors(cell, |c| if board.cells.contains(&c) { n += 1 });

    n
}

#[cfg(test)]
pub mod tests {
    use super::*;

    mod neighbors {
        use super::super::*;
        use super::*;

        #[test]
        fn all_neighbors_are_counted() {
            let cell = Cell { row: 3, col: 2 };
            let n = neighbors(&from_grid(r"
            . . . . .
            . . . . .
            . * * * .
            . * * * .
            . * * * .
            . . . . .
        "), cell);

            assert_eq!(n, 8);
        }
    }

    mod evaluate_cell {
        use super::super::*;

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
    }

    pub mod evolve {
        use super::super::*;
        use super::*;

        #[test]
        fn live_cells_that_should_stay_alive_stay_alive() {
            let candidates = vec![
                Cell { row: 1, col: 1 }, // two neighbor
                Cell { row: 1, col: 5 }, // three neighbors
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
                Cell { row: 2, col: 3 }, // no neighbors
                Cell { row: 1, col: 1 }, // one neighbor
                Cell { row: 1, col: 5 }, // four neighbors
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
                Cell { row: 1, col: 5 }, // three neighbors
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
    }

    mod from_grid_tests {
        use super::*;

        #[test]
        #[should_panic]
        fn an_empty_grid_cannot_be_parsed() {
            let grid = "";
            from_grid(grid);
        }

        #[test]
        fn a_grid_with_all_empty_cells_is_empty() {
            let grid = ". . .\n. . .";
            let board = from_grid(grid);
            assert!(board.cells.is_empty());
        }

        #[test]
        fn the_board_dimensions_are_derived_from_the_grid() {
            let grid = ". . .\n. . .";
            let board = from_grid(grid);
            assert_eq!(board.height, 2);
            assert_eq!(board.width, 3);
        }

        #[test]
        fn a_grid_with_single_cell() {
            let grid = ". * .\n. . .";
            let board = from_grid(grid);
            assert_eq!(board.cells, vec![Cell { row: 0, col: 1 }].into_iter().collect());
        }

        #[test]
        fn leading_and_trailing_spaces_are_ignored() {
            let grid = "      . .    \n     . *     \n     . .   ";
            let board = from_grid(grid);
            assert_eq!(board.cells, vec![Cell { row: 1, col: 1 }].into_iter().collect());
            assert_eq!(board.width, 2);
            assert_eq!(board.height, 3);
        }

        #[test]
        fn leading_and_trailing_empty_lines_are_ignored() {
            let grid =
                r"
                        . . . .
                        ";
            let board = from_grid(grid);
            assert_eq!(board.width, 4);
            assert_eq!(board.height, 1);
        }

        #[test]
        fn a_grid_with_all_cells() {
            let grid = r"
            * * *
            * * *
        ";
            let board = from_grid(grid);
            assert_eq!(board.width, 3);
            assert_eq!(board.height, 2);
            let expected: Cells = (0..=1).into_iter().flat_map(|row| {
                (0..=2).into_iter().map(move |col| {
                    Cell { row, col }
                })
            }).collect();
            assert_eq!(board.cells, expected);
        }
    }

    mod integration {
        use super::super::*;
        use super::*;

        #[test]
        fn glider() {
            let glider1 = from_grid(r". . . . .
                                  . * . * .
                                  . . * * .
                                  . . * . .
                                  . . . . .");

            let glider2 = from_grid(r". . . . .
                                  . . . * .
                                  . * . * .
                                  . . * * .
                                  . . . . .");

            let glider3 = from_grid(r". . . . .
                                  . . * . .
                                  . . . * *
                                  . . * * .
                                  . . . . .");

            let glider4 = from_grid(r". . . . .
                                  . . . * .
                                  . . . . *
                                  . . * * *
                                  . . . . .");


            assert_eq!(evolve(&glider1), glider2);
            assert_eq!(evolve(&glider2), glider3);
            assert_eq!(evolve(&glider3), glider4);
        }


    }

    fn assert_alive(board: &Board, cell: Cell) {
        assert!(board.cells.contains(&cell), "{:?} should contain {:?}", board, cell);
    }

    fn assert_dead(board: &Board, cell: Cell) {
        assert!(!board.cells.contains(&cell), "{:?} should not contain {:?}", board, cell);
    }

    pub fn from_grid(grid: &str) -> Board {
        if grid.is_empty() {
            panic!("Grid cannot be empty");
        } else {
            let mut cells: Cells = HashSet::new();
            let mut width = 0;
            let mut height = 0;

            for (y, line) in grid.trim().split("\n").enumerate() {
                height = y + 1;
                for (x, token) in line.trim().split(" ").enumerate() {
                    width = x + 1;
                    if token == "*" {
                        cells.insert(Cell { row: y as isize, col: x as isize });
                    }
                }
            }

            Board { cells, width, height }
        }
    }
}

