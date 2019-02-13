#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct Cell {
    pub row: usize,
    pub col: usize
}

impl Cell {
    pub fn new(row:usize, col:usize) -> Cell {
        Cell{row, col}
    }
}

