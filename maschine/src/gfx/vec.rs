use std::ops::Add;

pub type UVec2 = Vec2<u32>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }
}

impl UVec2 {
    pub fn oob(&self, bounds: UVec2) -> bool {
        self.x > bounds.x || self.y > bounds.y
    }
}

impl<T: Add<Output = T>> Add<Vec2<T>> for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2::<T>::new(self.x + rhs.x, self.y + rhs.y)
    }
}
