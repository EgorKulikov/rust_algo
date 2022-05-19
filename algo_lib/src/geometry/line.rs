use crate::geometry::point::Point;
use crate::numbers::num_traits::ring::Ring;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Line<T> {
    pub a: T,
    pub b: T,
    pub c: T,
}

impl<T> Line<T> {
    pub fn new(a: T, b: T, c: T) -> Self {
        Self { a, b, c }
    }
}

impl<T: Ring + Copy> Line<T> {
    pub fn value(&self, p: Point<T>) -> T {
        self.a * p.x + self.b * p.y + self.c
    }
}

impl Line<f64> {
    pub fn dist(&self, p: Point<f64>) -> f64 {
        self.value(p)
    }
}
