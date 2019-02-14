use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct Cell {
    pub row: isize,
    pub col: isize,
}

pub type Board = HashSet<Cell>;

pub fn evolve(board: &Board) -> Board {
    board
        .into_iter()
        .flat_map(|cell| neighbors(*cell))
        .filter(|cell| {
            evaluate_cell(
                board.contains(cell),
                living_neighbors(*cell, board).count(),
            )
        })
        .collect()
}

pub fn evaluate_cell(alive: bool, neighbors: usize) -> bool {
    match alive {
        true => neighbors == 2 || neighbors == 3,
        false => neighbors == 3
    }
}

fn neighbors<'a>(cell: Cell) -> impl Iterator<Item=Cell> + 'a {
    (cell.row - 1..=cell.row + 1)
        .flat_map(move |row| {
            (cell.col - 1..=cell.col + 1)
                .map(move |col| {
                    Cell { row, col }
                })
        })
        .filter(move |c| *c != cell)
}

pub fn living_neighbors<'a>(cell: Cell, board: &'a Board) -> impl Iterator<Item=Cell> + 'a {
    neighbors(cell).filter(move |c| board.contains(c))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    mod neighbors_tests {
        use super::super::*;
        use super::*;

        #[test]
        fn all_neighbors_are_enumerated() {
            let cell = Cell { row: 2, col: 3 };
            let all = neighbors(cell).collect::<HashSet<Cell>>();

            let expected = [
                Cell { row: 1, col: 2 }, Cell { row: 1, col: 3 }, Cell { row: 1, col: 4 },
                Cell { row: 2, col: 2 }, Cell { row: 2, col: 4 },
                Cell { row: 3, col: 2 }, Cell { row: 3, col: 3 }, Cell { row: 3, col: 4 },
            ].iter().cloned().collect();

            assert_eq!(all, expected);
        }


        #[test]
        fn correctly_reports_all_living_neighbors() {
            let cell = Cell { row: 3, col: 2 };
            let grid = from_grid(r"
                . . . . .
                . . . . .
                . * * * .
                . * * * .
                . * * * .
                . . . . .
            ");

            let all: HashSet<Cell> = living_neighbors(cell, &grid).collect();

            let expected = [
                Cell { row: 2, col: 1 }, Cell { row: 2, col: 2 }, Cell { row: 2, col: 3 },
                Cell { row: 3, col: 1 }, Cell { row: 3, col: 3 },
                Cell { row: 4, col: 1 }, Cell { row: 4, col: 2 }, Cell { row: 4, col: 3 },
            ].iter().cloned().collect();

            assert_eq!(all, expected);
        }

        #[test]
        fn correctly_reports_no_living_neighbors() {
            let cell = Cell { row: 3, col: 2 };
            let grid = from_grid(r"
                . . . . .
                . . . . .
                . . . . .
                . . * . .
                . . . . .
                . . . . .
            ");

            let none: HashSet<Cell> = living_neighbors(cell, &grid).collect();

            assert!(none.is_empty());
        }
    }

    mod evaluate_cell_tests {
        use super::super::*;

        #[test]
        fn a_live_cell_with_less_than_2_neighbors_dies() {
            for n in 0..=1 {
                assert!(!evaluate_cell(true, n));
            }
        }

        #[test]
        fn a_live_cell_with_2_or_3_neighbors_lives() {
            for n in 2..=3 {
                assert!(evaluate_cell(true, n));
            }
        }

        #[test]
        fn a_live_cell_with_more_than_three_neighbors_dies() {
            for n in 4..=8 {
                assert!(!evaluate_cell(true, n));
            }
        }

        #[test]
        fn a_dead_cell_with_3_neighbors_is_born() {
            assert!(evaluate_cell(false, 3));
        }

        #[test]
        fn a_dead_cell_with_other_than_3_neighbors_dies() {
            for n in (0..=2).chain(4..=8) {
                assert!(!evaluate_cell(false, n))
            }
        }
    }

    mod evolve_tests {
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
        fn an_empty_grid_is_empty() {
            let grid = "";
            let board = from_grid(grid);
            assert!(board.is_empty());
        }

        #[test]
        fn a_grid_with_all_empty_cells_is_empty() {
            let grid = ". . .\n. . .";
            let board = from_grid(grid);
            assert!(board.is_empty());
        }

        #[test]
        fn a_grid_with_single_cell() {
            let grid = ". * .\n. . .";
            let board = from_grid(grid);
            assert_eq!(board, vec![Cell { row: 0, col: 1 }].into_iter().collect());
        }

        #[test]
        fn leading_and_trailing_spaces_are_ignored() {
            let grid = "      . .    \n     . *     \n     . .   ";
            let board = from_grid(grid);
            assert_eq!(board, vec![Cell { row: 1, col: 1 }].into_iter().collect());
        }

        #[test]
        fn a_grid_with_all_cells() {
            let grid = r"
                * * *
                * * *
            ";
            let board = from_grid(grid);
            let expected: Board = (0..=1).into_iter().flat_map(|row| {
                (0..=2).into_iter().map(move |col| {
                    Cell { row, col }
                })
            }).collect();
            assert_eq!(board, expected);
        }
    }

    mod integration_tests {
        use super::super::*;
        use super::*;

        #[test]
        fn glider() {
            let glider1 = from_grid(r"
                . . . . .
                . * . * .
                . . * * .
                . . * . .
                . . . . .
            ");

            let glider2 = from_grid(r"
                . . . . .
                . . . * .
                . * . * .
                . . * * .
                . . . . .
            ");

            let glider3 = from_grid(r"
                . . . . .
                . . * . .
                . . . * *
                . . * * .
                . . . . .
            ");

            let glider4 = from_grid(r"
                . . . . .
                . . . * .
                . . . . *
                . . * * *
                . . . . .
            ");

            assert_eq!(evolve(&glider1), glider2);
            assert_eq!(evolve(&glider2), glider3);
            assert_eq!(evolve(&glider3), glider4);
        }
    }

    fn assert_alive(board: &Board, cell: Cell) {
        assert!(board.contains(&cell), "{:?} should contain {:?}", board, cell);
    }

    fn assert_dead(board: &Board, cell: Cell) {
        assert!(!board.contains(&cell), "{:?} should not contain {:?}", board, cell);
    }

    pub fn from_grid(grid: &str) -> Board {
        grid
            .trim()
            .split("\n")
            .enumerate()
            .flat_map(|(y, line)| {
                line
                    .trim()
                    .split(" ")
                    .enumerate()
                    .filter(|(_, token)| token == &"*")
                    .map(move |(x, _)| {
                        Cell { row: y as isize, col: x as isize }
                    })
            }).collect()
    }
}

