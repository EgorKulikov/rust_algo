use crate::geometry::point::Point;
use crate::geometry::Base;
use crate::numbers::num_traits::algebra::{Field, Zero};
use crate::numbers::real::{IntoReal, Real};

#[derive(Copy, Clone, Ord, PartialOrd, Hash)]
#[allow(clippy::derived_hash_with_manual_eq)]
pub struct Line<T: Base> {
    pub a: T,
    pub b: T,
    pub c: T,
}

impl<T: Base> Line<T> {
    pub fn new(a: T, b: T, c: T) -> Self {
        Self { a, b, c }
    }
}

impl Line<Real> {
    pub fn new_canonical(a: Real, b: Real, c: Real) -> Self {
        let h = Real::hypot(a, b);
        let a = a / h;
        let b = b / h;
        let c = c / h;
        if a < Real::zero() || (a == Real::zero() && b < Real::zero()) {
            Self::new(-a, -b, -c)
        } else {
            Self { a, b, c }
        }
    }
}

impl<T: Base> Line<T> {
    pub fn value(&self, p: Point<T>) -> T {
        self.a * p.x + self.b * p.y + self.c
    }

    pub fn parallel(&self, p: Point<T>) -> Line<T> {
        Line::new(self.a, self.b, T::zero() - self.a * p.x - self.b * p.y)
    }

    pub fn perpendicular(&self, p: Point<T>) -> Line<T> {
        Line::new(-self.b, self.a, self.b * p.x - self.a * p.y)
    }

    pub fn is_parallel(&self, other: Line<T>) -> bool {
        self.a * other.b == self.b * other.a
    }

    pub fn is_perpendicular(&self, other: Line<T>) -> bool {
        self.a * other.a + self.b * other.b == T::zero()
    }
}

impl<T: Base> Eq for Line<T> {}

impl<T: Base> PartialEq for Line<T> {
    fn eq(&self, other: &Self) -> bool {
        self.a * other.b == self.b * other.a && self.b * other.c == self.c * other.b
    }
}

impl<T: Field + Base> Line<T> {
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

impl<T: Field + Base + IntoReal> Line<T> {
    pub fn dist_point(&self, p: Point<T>) -> Real {
        self.square_dist_point(p).into_real().sqrt()
    }
}

impl<T: Base + PartialEq> Line<T> {
    pub fn contains(&self, p: Point<T>) -> bool {
        self.value(p) == T::zero()
    }
}
