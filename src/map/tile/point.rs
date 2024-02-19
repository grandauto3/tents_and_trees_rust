use std::ops::Add;

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub struct Point(pub f32, pub f32);

impl Point {
    pub fn new(input: (f32, f32)) -> Self {
        Self(input.0, input.1)
    }
}

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

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new((self.0 + rhs.0, self.1 + rhs.1))
    }
}

impl Add<(f32, f32)> for Point {
    type Output = Self;

    fn add(self, rhs: (f32, f32)) -> Self::Output {
        Point::new((self.0 + rhs.0, self.1 + rhs.1))
    }
}