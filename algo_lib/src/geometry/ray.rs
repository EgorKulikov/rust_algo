use crate::geometry::line::Line;
use crate::geometry::point::Point;
use crate::numbers::num_traits::field::Field;
use crate::numbers::num_traits::real::RealTrait;
use crate::numbers::num_traits::ring::Ring;

pub struct Ray<T> {
    pub origin: Point<T>,
    pub direction: Point<T>,
}

impl<T> Ray<T> {
    pub fn new(origin: Point<T>, direction: Point<T>) -> Self {
        Self { origin, direction }
    }
}

impl<T: Ring + Copy> Ray<T> {
    pub fn line(&self) -> Line<T> {
        self.origin.line(self.direction)
    }
}

impl<T: Field + Copy + Ord> Ray<T> {
    pub fn contains(&self, p: Point<T>) -> bool {
        if p == self.origin {
            return true;
        }
        let line = self.line();
        if !line.contains(p) {
            return false;
        }
        if self.direction.x != self.origin.x
            && (p.x > self.origin.x) != (self.direction.x > self.origin.x)
        {
            return false;
        }
        if self.direction.y != self.origin.y
            && (p.y > self.origin.y) != (self.direction.y > self.origin.y)
        {
            return false;
        }
        true
    }

    pub fn square_dist_point(&self, p: Point<T>) -> T {
        let line = self.line();
        let perp = line.perpendicular(p);
        let pp = line.intersect(perp);
        if self.contains(pp) {
            pp.square_dist_point(p)
        } else {
            self.origin.square_dist_point(p)
        }
    }
}

impl<T: RealTrait> Ray<T> {
    pub fn angle(&self) -> T {
        (self.direction - self.origin).angle()
    }

    pub fn from_angle(origin: Point<T>, angle: T) -> Self {
        Self::new(origin, origin + Point::from_polar(T::one(), angle))
    }

    pub fn dist_point(&self, p: Point<T>) -> T {
        self.square_dist_point(p).sqrt()
    }
}
