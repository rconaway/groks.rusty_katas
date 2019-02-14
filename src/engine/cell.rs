#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct Cell {
    pub row: isize,
    pub col: isize
}

impl Cell {
    pub fn new(row:isize, col:isize) -> Cell {
        Cell{row, col}
    }
}

