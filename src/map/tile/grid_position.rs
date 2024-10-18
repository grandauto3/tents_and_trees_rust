#[derive(Default, Copy, Clone, PartialOrd, PartialEq, Debug, Eq, Hash)]
pub struct GridPosition {
    row: usize,
    column: usize,
}

impl GridPosition {
    pub fn new((row, column): (usize, usize)) -> Self {
        Self { row, column }
    }
}
