#[derive(Copy, Clone, PartialOrd, PartialEq, Debug, Eq, Hash)]
pub struct Point {
    row: usize,
    column: usize,
}

impl Default for Point {
    fn default() -> Self {
        Self { row: 0, column: 0 }
    }
}

impl Point {
    pub fn new((row, column): (usize, usize)) -> Self {
        Self { row, column }
    }
}
