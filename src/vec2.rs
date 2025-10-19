use std::ops::{Add, AddAssign, DivAssign, Sub};

use num_traits::Pow;

#[derive(Clone, Copy, PartialEq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T: AddAssign> AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: AddAssign> Add for Vec2<T> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn dist2(self, other: Self) -> T
    where
        T: Sub<Output = T> + Pow<u8, Output = T> + Add<Output = T>,
    {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2)
    }

    pub fn unwrap(self) -> (T, T) {
        (self.x, self.y)
    }
}

impl<A: Clone, T: DivAssign<A>> DivAssign<A> for Vec2<T> {
    fn div_assign(&mut self, rhs: A) {
        self.x /= rhs.clone();
        self.y /= rhs;
    }
}

pub type Point = Vec2<f32>;
