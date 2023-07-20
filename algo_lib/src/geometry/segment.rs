use crate::geometry::line::Line;
use crate::geometry::point::Point;
use crate::numbers::num_traits::field::Field;
use crate::numbers::num_traits::real::RealTrait;
use crate::numbers::num_traits::ring::Ring;

#[derive(Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Segment<T> {
    pub p1: Point<T>,
    pub p2: Point<T>,
}

impl<T: Copy> Segment<T> {
    pub fn new(p1: Point<T>, p2: Point<T>) -> Self {
        Self { p1, p2 }
    }
}

impl<T: Ring + Copy> Segment<T> {
    pub fn line(&self) -> Line<T> {
        self.p1.line(self.p2)
    }
}

impl<T: Field + Copy + PartialEq + Ord> Segment<T> {
    pub fn contains(&self, p: Point<T>) -> bool {
        if self.p1 == self.p2 {
            return self.p1 == p;
        }
        let line = self.line();
        if !line.contains(p) {
            return false;
        }
        let x1 = self.p1.x.min(self.p2.x);
        let x2 = self.p1.x.max(self.p2.x);
        let y1 = self.p1.y.min(self.p2.y);
        let y2 = self.p1.y.max(self.p2.y);
        x1 <= p.x && p.x <= x2 && y1 <= p.y && p.y <= y2
    }

    pub fn square_dist_point(&self, p: Point<T>) -> T {
        if self.p1 == self.p2 {
            return self.p1.square_dist_point(p);
        }
        let line = self.line();
        let perp = line.perpendicular(p);
        let pp = line.intersect(perp);
        if self.contains(pp) {
            pp.square_dist_point(p)
        } else {
            self.p1
                .square_dist_point(p)
                .min(self.p2.square_dist_point(p))
        }
    }

    pub fn square_dist_segment(&self, s: Segment<T>) -> T {
        if self.p1 == self.p2 {
            return s.square_dist_point(self.p1);
        }
        if s.p1 == s.p2 {
            return self.square_dist_point(s.p1);
        }
        let l1 = self.line();
        let l2 = s.line();
        if !l1.is_parallel(l2) {
            let p = l1.intersect(l2);
            if self.contains(p) && s.contains(p) {
                return T::zero();
            }
        }
        self.square_dist_point(s.p1)
            .min(self.square_dist_point(s.p2))
            .min(s.square_dist_point(self.p1))
            .min(s.square_dist_point(self.p2))
    }
}

impl<T: RealTrait> Segment<T> {
    pub fn contains_real(&self, p: Point<T>) -> bool {
        if self.p1.same(self.p2) {
            return self.p1.same(p);
        }
        let line = self.line();
        if !line.contains_real(p) {
            return false;
        }
        let x1 = self.p1.x.min(self.p2.x);
        let x2 = self.p1.x.max(self.p2.x);
        let y1 = self.p1.y.min(self.p2.y);
        let y2 = self.p1.y.max(self.p2.y);
        x1 <= p.x + T::epsilon()
            && p.x <= x2 + T::epsilon()
            && y1 <= p.y + T::epsilon()
            && p.y <= y2 + T::epsilon()
    }

    pub fn dist_point(&self, p: Point<T>) -> T {
        if self.p1.same(self.p2) {
            return self.p1.dist_point(p);
        }
        let line = self.line();
        let perp = line.perpendicular(p);
        let pp = line.intersect(perp);
        if self.contains_real(pp) {
            pp.dist_point(p)
        } else {
            self.p1.dist_point(p).min(self.p2.dist_point(p))
        }
    }

    pub fn dist_segment(&self, s: Segment<T>) -> T {
        if self.p1.same(self.p2) {
            return s.dist_point(self.p1);
        }
        if s.p1.same(s.p2) {
            return self.dist_point(s.p1);
        }
        let l1 = self.line();
        let l2 = s.line();
        if !l1.is_parallel_real(l2) {
            let p = l1.intersect(l2);
            if self.contains_real(p) && s.contains_real(p) {
                return T::zero();
            }
        }
        self.dist_point(s.p1)
            .min(self.dist_point(s.p2))
            .min(s.dist_point(self.p1))
            .min(s.dist_point(self.p2))
    }
}
