#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub struct Point(f32, f32);

impl Default for Point {
    fn default() -> Self {
        Self(0f32, 0f32)
    }
}

impl From<(f32, f32)> for Point {
    fn from(value: (f32, f32)) -> Self {
        Self(value.0, value.1)
    }
}

impl Into<(f32, f32)> for Point {
    fn into(self) -> (f32, f32) {
        (self.0, self.1)
    }
}