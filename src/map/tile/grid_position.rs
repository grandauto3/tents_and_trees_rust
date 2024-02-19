#[derive(Copy, Clone, PartialOrd, PartialEq, Debug, Eq, Hash)]
pub struct GridPosition {
    row: usize,
    column: usize,
}

impl Default for GridPosition {
    fn default() -> Self {
        Self { row: 0, column: 0 }
    }
}

impl GridPosition {
    pub fn new((row, column): (usize, usize)) -> Self {
        Self { row, column }
    }
}
