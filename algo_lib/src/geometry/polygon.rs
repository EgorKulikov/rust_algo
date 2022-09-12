use crate::geometry::point::Point;
use crate::numbers::num_traits::ring::Ring;

pub trait ConvexHull<T> {
    fn convex_hull(self) -> Vec<Point<T>>;
}

impl<T: Ring + Copy + PartialOrd> ConvexHull<T> for &mut [Point<T>] {
    fn convex_hull(self) -> Vec<Point<T>> {
        self.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let p1 = self[0];
        let pn = *self.last().unwrap();
        let mut up = vec![p1];
        let mut down = vec![p1];
        for &p in self.iter().skip(1) {
            if p == pn || p1.line(pn).value(p) > T::zero() {
                while up.len() >= 2 && up[up.len() - 2].line(up[up.len() - 1]).value(p) <= T::zero()
                {
                    up.pop();
                }
                up.push(p);
            }
            if p == pn || p1.line(pn).value(p) < T::zero() {
                while down.len() >= 2
                    && down[down.len() - 2].line(down[down.len() - 1]).value(p) >= T::zero()
                {
                    down.pop();
                }
                down.push(p);
            }
        }
        let mut ans = up;
        ans.extend(down.into_iter().skip(1).rev().skip(1));
        ans
    }
}
