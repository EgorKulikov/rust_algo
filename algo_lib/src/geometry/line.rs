use crate::geometry::geo_utils::epsilon;
use crate::geometry::point::Point;
use crate::numbers::num_traits::field::Field;
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

    pub fn parallel(&self, p: Point<T>) -> Line<T> {
        Line::new(self.a, self.b, T::zero() - self.a * p.x - self.b * p.y)
    }

    pub fn perpendicular(&self, p: Point<T>) -> Line<T> {
        Line::new(
            T::zero() - self.b,
            self.a,
            T::zero() + self.b * p.x - self.a * p.y,
        )
    }
}

impl<T: Ring + Copy + PartialEq> Line<T> {
    pub fn is_parallel(&self, other: Line<T>) -> bool {
        self.a * other.b == self.b * other.a
    }

    pub fn is_perpendicular(&self, other: Line<T>) -> bool {
        self.a * other.a + self.b * other.b == T::zero()
    }
}

impl<T: Field + Copy> Line<T> {
    pub fn intersect(&self, other: Line<T>) -> Point<T> {
        let det = self.a * other.b - other.a * self.b;
        let x = (self.b * other.c - other.b * self.c) / det;
        let y = (other.a * self.c - self.a * other.c) / det;
        Point::new(x, y)
    }

    pub fn square_dist_point(&self, p: Point<T>) -> T {
        let val = self.value(p);
        val * val / (self.a * self.a + self.b * self.b)
    }
}

impl<T: Ring + Copy + PartialEq> Line<T> {
    pub fn contains(&self, p: Point<T>) -> bool {
        self.value(p) == T::zero()
    }
}

impl Line<f64> {
    pub fn new_f64(a: f64, b: f64, c: f64) -> Self {
        let h = f64::hypot(a, b);
        Self {
            a: a / h,
            b: b / h,
            c: c / h,
        }
    }

    pub fn dist_point(&self, p: Point<f64>) -> f64 {
        self.square_dist_point(p).sqrt()
    }

    pub fn contains_f64(&self, p: Point<f64>) -> bool {
        (self.a * p.x + self.b * p.y + self.c).abs() < 1e-9
    }

    pub fn is_parallel_f64(&self, other: Line<f64>) -> bool {
        (self.a * other.b - self.b * other.a).abs() < epsilon()
    }

    pub fn is_perpendicular_f64(&self, other: Line<f64>) -> bool {
        (self.a * other.a + self.b * other.b).abs() < epsilon()
    }
}
