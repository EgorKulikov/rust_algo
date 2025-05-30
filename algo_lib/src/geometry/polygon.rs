use crate::collections::iter_ext::cur_next::cur_next;
use crate::geometry::point::Point;
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
