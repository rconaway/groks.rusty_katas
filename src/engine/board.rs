
#[derive(Debug)]
pub struct Board {}

impl Board {
    pub fn new(_grid: &str) -> Board {
        Board {}
    }

    pub fn empty() -> Board {
        Board {}
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
