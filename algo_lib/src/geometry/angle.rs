use crate::geometry::geometry_utils::canonize_angle_base;
use crate::geometry::point::Point;
use crate::geometry::ray::Ray;
use crate::numbers::num_traits::algebra::Zero;
use crate::numbers::real::Real;

pub struct Angle<T> {
    pub origin: Point<T>,
    pub dir1: Point<T>,
    pub dir2: Point<T>,
}

impl<T> Angle<T> {
    pub fn new(origin: Point<T>, dir1: Point<T>, dir2: Point<T>) -> Self {
        Self { origin, dir1, dir2 }
    }
}

impl Angle<Real> {
    pub fn value(&self) -> Real {
        let a1 = (self.dir1 - self.origin).angle();
        let a2 = (self.dir2 - self.origin).angle();
        canonize_angle_base(a2 - a1, Real::zero())
    }

    pub fn bisector(&self) -> Ray<Real> {
        let angle = (self.dir1 - self.origin).angle();
        Ray::from_angle(self.origin, angle + self.value() / 2.)
    }
}
