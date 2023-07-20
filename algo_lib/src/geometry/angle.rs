use crate::geometry::geometry_utils::canonize_angle_base;
use crate::geometry::point::Point;
use crate::geometry::ray::Ray;
use crate::numbers::num_traits::real::RealTrait;

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

impl<T: RealTrait> Angle<T> {
    pub fn value(&self) -> T {
        let a1 = (self.dir1 - self.origin).angle();
        let a2 = (self.dir2 - self.origin).angle();
        canonize_angle_base(a2 - a1, T::zero())
    }

    pub fn bissector(&self) -> Ray<T> {
        let angle = (self.dir1 - self.origin).angle();
        Ray::from_angle(self.origin, angle + self.value() / (T::one() + T::one()))
    }
}
