use std::collections::HashSet;
use super::cell::Cell;

pub type Cells = HashSet<Cell>;

#[derive(Debug, PartialEq)]
pub struct Board {
    pub cells: Cells,
    pub height: usize,
    pub width: usize,
}

impl Board {
    pub fn new(cells: Cells, height:usize, width:usize) -> Board {
        Board{cells, height, width}
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

    pub fn from_size(height:usize, width:usize) -> Board {
        Board {height, width, cells:HashSet::new()}
    }

    pub fn empty() -> Board {
        Board { cells: HashSet::new(), height:0, width:0 }
    }

    pub fn is_empty(&self) -> bool {
        true
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn cells(&self) -> &Cells {
        &self.cells
    }

    pub fn evolve(&self) -> Board {
        Board::empty()
    }

    pub fn contains(&self, cell: Cell) -> bool {
        self.cells.contains(&cell)
    }
}

#[cfg(test) ]
mod tests {

    mod from_grid_tests {
        use super::super::*;

        #[test]
        #[should_panic]
        fn an_empty_grid_cannot_be_parsed() {
            let grid = "";
            Board::from_grid(grid);
        }

        #[test]
        fn a_grid_with_all_empty_cells_is_empty() {
            let grid = ". . .\n. . .";
            let board = Board::from_grid(grid);
            assert!(board.is_empty());
        }

        #[test]
        fn the_board_dimensions_are_derived_from_the_grid() {
            let grid = ". . .\n. . .";
            let board = Board::from_grid(grid);
            assert_eq!(board.height(), 2);
            assert_eq!(board.width(), 3);
        }

        #[test]
        fn a_grid_with_single_cell() {
            let grid = ". * .\n. . .";
            let board = Board::from_grid(grid);
            assert_eq!(board.cells(), &vec![Cell::new(0, 1)].into_iter().collect());
        }

        #[test]
        fn leading_and_trailing_spaces_are_ignored() {
            let grid = "      . .    \n     . *     \n     . .   ";
            let board = Board::from_grid(grid);
            assert_eq!(board.cells(), &vec![Cell::new(1, 1)].into_iter().collect());
            assert_eq!(board.width(), 2);
            assert_eq!(board.height(), 3);
        }

        #[test]
        fn leading_and_trailing_empty_lines_are_ignored() {
            let grid =
                r"
                        . . . .
                        ";
            let board = Board::from_grid(grid);
            assert_eq!(board.width(), 4);
            assert_eq!(board.height(), 1);
        }

        #[test]
        fn a_grid_with_all_cells() {
            let grid = r"
            * * *
            * * *
        ";
            let board = Board::from_grid(grid);
            assert_eq!(board.width(), 3);
            assert_eq!(board.height(), 2);
            let expected: Cells = (0..=1).into_iter().flat_map(|r| {
                (0..=2).into_iter().map(move |c| {
                    Cell::new(r, c)
                })
            }).collect();
            assert_eq!(board.cells(), &expected);
        }
    }

    mod evolve_tests {

        use super::super::*;

        #[test]
        fn an_empty_board_evolves_to_empty() {
            let board = Board::from_grid(r"
                                    . . . .
                                    . . . .
                                    . . . .
                                    . . . .
                                    ");

            let board = board.evolve();

            assert!(board.is_empty());
        }

        #[ignore]
        #[test]
        fn a_glider_evolves() {
            let glider1 = Board::from_grid(r". . . . .
                                  . * . * .
                                  . . * * .
                                  . . * . .
                                  . . . . .");

            let glider2 = Board::from_grid(r". . . . .
                                  . . . * .
                                  . * . * .
                                  . . * * .
                                  . . . . .");

            assert_eq!(glider1.evolve(), glider2);
        }



    }
}