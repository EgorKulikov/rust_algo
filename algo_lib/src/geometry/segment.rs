use crate::geometry::line::Line;
use crate::geometry::point::Point;
use crate::geometry::Base;
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
    fn contains_if_on_line(&self, p: Point<T>) -> bool {
        self.p1.min(self.p2) <= p && p <= self.p1.max(self.p2)
    }

    pub fn contains(&self, p: Point<T>) -> bool {
        if self.p1 == self.p2 {
            return self.p1 == p;
        }
        let line = self.line();
        line.contains(p) && self.contains_if_on_line(p)
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
            return if s.contains(self.p1) {
                SegmentIntersectionResult::Point(self.p1)
            } else {
                SegmentIntersectionResult::None
            };
        }
        if s.p1 == s.p2 {
            return if self.contains(s.p1) {
                SegmentIntersectionResult::Point(s.p1)
            } else {
                SegmentIntersectionResult::None
            };
        }
        let l1 = self.line();
        let l2 = s.line();
        if l1.is_parallel(l2) {
            return if l1 == l2 {
                let p11 = self.p1.min(self.p2);
                let p12 = self.p1.max(self.p2);
                let p21 = s.p1.min(s.p2);
                let p22 = s.p1.max(s.p2);
                let p1 = p11.max(p21);
                let p2 = p12.min(p22);
                match p1.cmp(&p2) {
                    std::cmp::Ordering::Less => {
                        SegmentIntersectionResult::Segment(Segment::new(p1, p2))
                    }
                    std::cmp::Ordering::Equal => SegmentIntersectionResult::Point(p1),
                    std::cmp::Ordering::Greater => SegmentIntersectionResult::None,
                }
            } else {
                SegmentIntersectionResult::None
            };
        }
        let p = l1.intersect(l2);
        if self.contains_if_on_line(p) && s.contains_if_on_line(p) {
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
