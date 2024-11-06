use crate::geometry::base::Base;
use crate::geometry::line::Line;
use crate::geometry::point::Point;
use crate::numbers::num_traits::algebra::Field;
use crate::numbers::real::{IntoReal, Real};

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

impl<T: Base> Segment<T> {
    pub fn line(&self) -> Line<T> {
        self.p1.line(self.p2)
    }

    pub fn square_len(&self) -> T {
        self.p1.square_dist_point(self.p2)
    }
}

impl<T: Base + Ord> Segment<T> {
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
}

pub enum SegmentIntersectionResult<T> {
    None,
    Point(Point<T>),
    Segment(Segment<T>),
}

impl<T: Field + Base + PartialEq + Ord> Segment<T> {
    pub fn intersect_segment(&self, s: Self) -> SegmentIntersectionResult<T> {
        if self.p1 == self.p2 {
            if s.contains(self.p1) {
                return SegmentIntersectionResult::Point(self.p1);
            }
            return SegmentIntersectionResult::None;
        }
        if s.p1 == s.p2 {
            if self.contains(s.p1) {
                return SegmentIntersectionResult::Point(s.p1);
            }
            return SegmentIntersectionResult::None;
        }
        let l1 = self.line();
        let l2 = s.line();
        if l1.is_parallel(l2) {
            if l1 == l2 {
                let p11 = self.p1.min(self.p2);
                let p12 = self.p1.max(self.p2);
                let p21 = s.p1.min(s.p2);
                let p22 = s.p1.max(s.p2);
                if p12 < p21 || p22 < p11 {
                    return SegmentIntersectionResult::None;
                }
                if p12 == p21 {
                    return SegmentIntersectionResult::Point(p12);
                }
                if p22 == p11 {
                    return SegmentIntersectionResult::Point(p22);
                }
                return SegmentIntersectionResult::Segment(Segment::new(
                    p11.max(p21),
                    p12.min(p22),
                ));
            }
            return SegmentIntersectionResult::None;
        }
        let p = l1.intersect(l2);
        if self.contains(p) && s.contains(p) {
            SegmentIntersectionResult::Point(p)
        } else {
            SegmentIntersectionResult::None
        }
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

impl<T: Base + IntoReal> Segment<T> {
    pub fn len(&self) -> Real {
        self.p1.square_dist_point(self.p2).into_real().sqrt()
    }
}

impl<T: Field + Base + PartialEq + Ord + IntoReal> Segment<T> {
    pub fn dist_point(&self, p: Point<T>) -> Real {
        self.square_dist_point(p).into_real().sqrt()
    }

    pub fn dist_segment(&self, s: Segment<T>) -> Real {
        self.square_dist_segment(s).into_real().sqrt()
    }
}
