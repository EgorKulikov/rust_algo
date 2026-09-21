use crate::collections::iter_ext::cur_next::cur_next;
use crate::geometry::point::Point;
use crate::geometry::segment::Segment;
use crate::geometry::Base;
use crate::numbers::num_traits::algebra::Field;

pub struct Polygon<T> {
    pub points: Vec<Point<T>>,
}

impl<T> Polygon<T> {
    pub fn new(points: Vec<Point<T>>) -> Self {
        Self { points }
    }
}

impl<T: Field + Base> Polygon<T> {
    pub fn area(&self) -> T {
        self.double_area() / (T::one() + T::one())
    }
}

impl<T: Base> Polygon<T> {
    pub fn double_area(&self) -> T {
        let mut ans = T::zero();
        for (i, j) in cur_next(self.points.len()) {
            ans += self.points[i].x * self.points[j].y;
            ans -= self.points[i].y * self.points[j].x;
        }
        ans
    }
}

impl<T: Base + Ord> Polygon<T> {
    pub fn contains(&self, point: Point<T>) -> bool {
        let mut pos = false;
        let mut neg = false;
        for (i, j) in cur_next(self.points.len()) {
            let seg = Segment::new(self.points[i], self.points[j]);
            if seg.contains(point) {
                return true;
            }
            if self.points[i] == self.points[j] {
                // A repeated vertex: the zero-length edge has no side.
                continue;
            }
            let val = seg.line().value(point);
            if val >= T::zero() {
                pos = true;
            }
            if val <= T::zero() {
                neg = true;
            }
        }
        // No side seen at all means every vertex is the same point, and that
        // point was handled by the segment check above.
        (pos || neg) && !(pos && neg)
    }
}

pub trait ConvexHull<T> {
    fn convex_hull(self) -> Polygon<T>;
}

impl<T: Base + PartialOrd> ConvexHull<T> for &mut [Point<T>] {
    fn convex_hull(self) -> Polygon<T> {
        self.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let p1 = self[0];
        let pn = *self.last().unwrap();
        let mut up = vec![p1];
        let mut down = vec![p1];
        for &p in self.iter().skip(1) {
            if p == pn || p1.line(pn).value(p) > T::zero() {
                while up.len() >= 2 && up[up.len() - 2].line(p).value(up[up.len() - 1]) <= T::zero()
                {
                    up.pop();
                }
                up.push(p);
            }
            if p == pn || p1.line(pn).value(p) < T::zero() {
                while down.len() >= 2
                    && down[down.len() - 2].line(p).value(down[down.len() - 1]) >= T::zero()
                {
                    down.pop();
                }
                down.push(p);
            }
        }
        let mut ans = up;
        ans.extend(down.into_iter().skip(1).rev().skip(1));
        if ans.len() == 2 && ans[0] == ans[1] {
            ans.pop();
        }
        Polygon::new(ans)
    }
}

#[cfg(test)]
mod tests {
    use super::Polygon;
    use crate::geometry::point::Point;

    #[test]
    fn contains_with_a_repeated_vertex() {
        let square = |points: Vec<(i64, i64)>| {
            Polygon::new(points.into_iter().map(|(x, y)| Point::new(x, y)).collect())
        };
        let plain = square(vec![(0, 0), (2, 0), (2, 2), (0, 2)]);
        let repeated = square(vec![(0, 0), (2, 0), (2, 0), (2, 2), (0, 2)]);
        for x in -1..=3 {
            for y in -1..=3 {
                let p = Point::new(x, y);
                assert_eq!(repeated.contains(p), plain.contains(p), "({}, {})", x, y);
            }
        }
        assert!(repeated.contains(Point::new(1, 1)));
    }

    #[test]
    fn contains_for_a_polygon_that_is_a_single_point() {
        for copies in 1..=3 {
            let point = Polygon::new(vec![Point::new(0i64, 0); copies]);
            assert!(point.contains(Point::new(0, 0)));
            assert!(!point.contains(Point::new(-1, -1)));
            assert!(!point.contains(Point::new(1, 0)));
        }
    }
}
