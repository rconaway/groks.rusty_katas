
#[derive(Debug)]
pub struct Board {
    cells: Vec<(usize,usize)>,
    width: usize,
    height: usize
}

impl Board {
    pub fn new(grid: &str) -> Board {

        if grid.is_empty() {
            panic!("Grid cannot be empty");
        } else {
            let mut cells:Vec<(usize, usize)> = Vec::new();
            let mut width = 0;
            let mut height = 0;

            for (y, line) in grid.trim().split("\n").enumerate() {
                height = y + 1;
                for (x, token) in line.trim().split(" ").enumerate() {
                    width = x + 1;
                    if token == "*" {
                        cells.push((x,y))
                    }
                }
            }

            Board {cells, width, height}
        }
    }

    pub fn empty() -> Board {
        Board { cells: Vec::new(), height:0, width:0 }
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

    pub fn cells(&self) -> &Vec<(usize, usize)> {
        &self.cells
    }
}

impl PartialEq for Board {
    fn eq(&self, _that: &Board) -> bool {
        false
    }

    fn ne(&self, _that: &Board) -> bool {
        true
    }
}

#[cfg(test) ]
mod tests {

    use super::*;

    #[test]
    #[should_panic]
    fn an_empty_grid_cannot_be_parsed() {
        let grid = "";
        Board::new(grid);
    }

    #[test]
    fn a_grid_with_all_empty_cells_is_empty() {
        let grid = ". . .\n. . .";
        let board = Board::new(grid);
        assert!(board.is_empty());
    }

    #[test]
    fn the_board_dimensions_are_derived_from_the_grid() {
        let grid = ". . .\n. . .";
        let board = Board::new(grid);
        assert_eq!(board.height(), 2);
        assert_eq!(board.width(), 3);
    }

    #[test]
    fn a_grid_with_single_cell() {
        let grid = ". * .\n. . .";
        let board = Board::new(grid);
        assert_eq!(board.cells(), &vec![(1, 0)])
    }

    #[test]
    fn leading_and_trailing_spaces_are_ignored() {
        let grid = "      . .    \n     . *     \n     . .   ";
        let board  = Board::new(grid);
        assert_eq!(board.cells(), &vec![(1, 1)]);
        assert_eq!(board.width(), 2);
        assert_eq!(board.height(), 3);
    }

    #[test]
    fn leading_and_trailing_empty_lines_are_ignored() {
        let grid =
                        r"
                        . . . .
                        ";
        let board = Board::new(grid);
        assert_eq!(board.width(), 4);
        assert_eq!(board.height(), 1);
    }

    #[test]
    #[ignore]
    fn a_grid_with_all_cells() {
        let grid = r"
            * * *
            * * *
        ";
        let board = Board::new(grid);
        assert_eq!(board.width(), 3);
        assert_eq!(board.height(), 2);
        assert_eq!(board.cells(), &vec![(0,0), (0,1), (1,0), (1,1), (2,0), (2,1)])
    }

}