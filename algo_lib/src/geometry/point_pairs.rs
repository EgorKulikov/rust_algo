//! Closest and farthest pairs of integer (or any ordered-ring) points,
//! using squared distances only.

use crate::geometry::point::Point;
use crate::geometry::polygon::ConvexHull;
use crate::geometry::Base;

/// Indices of a pair of points at minimum distance (`points.len() >= 2`),
/// by divide and conquer in O(n log n).
pub fn closest_pair<T: Base + Ord>(points: &[Point<T>]) -> (usize, usize) {
    assert!(points.len() >= 2);
    let mut order: Vec<usize> = (0..points.len()).collect();
    order.sort_by_key(|&i| (points[i].x, points[i].y));
    let mut best = (order[0], order[1]);
    let mut best_dist = points[order[0]].square_dist_point(points[order[1]]);
    let mut buffer = order.clone();
    solve(points, &mut order, &mut buffer, &mut best, &mut best_dist);
    best
}

/// Sorts `ids` by y (merge sort) while updating the best pair.
fn solve<T: Base + Ord>(
    points: &[Point<T>],
    ids: &mut [usize],
    buffer: &mut [usize],
    best: &mut (usize, usize),
    best_dist: &mut T,
) {
    let n = ids.len();
    if n <= 3 {
        for i in 0..n {
            for j in i + 1..n {
                let d = points[ids[i]].square_dist_point(points[ids[j]]);
                if d < *best_dist {
                    *best_dist = d;
                    *best = (ids[i], ids[j]);
                }
            }
        }
        ids.sort_by_key(|&i| (points[i].y, points[i].x));
        return;
    }
    let mid = n / 2;
    let mid_x = points[ids[mid]].x;
    {
        let (left, right) = ids.split_at_mut(mid);
        let (buf_left, buf_right) = buffer.split_at_mut(mid);
        solve(points, left, buf_left, best, best_dist);
        solve(points, right, buf_right, best, best_dist);
    }
    // merge by y
    let (mut i, mut j) = (0, mid);
    for slot in buffer[..n].iter_mut() {
        let take_left = j == n || (i < mid && points[ids[i]].y <= points[ids[j]].y);
        if take_left {
            *slot = ids[i];
            i += 1;
        } else {
            *slot = ids[j];
            j += 1;
        }
    }
    ids.copy_from_slice(&buffer[..n]);
    // strip around the dividing line, compared by squared distances
    let mut strip: Vec<usize> = Vec::new();
    for &id in ids.iter() {
        let dx = points[id].x - mid_x;
        if dx * dx >= *best_dist {
            continue;
        }
        for &other in strip.iter().rev() {
            let dy = points[id].y - points[other].y;
            if dy * dy >= *best_dist {
                break;
            }
            let d = points[id].square_dist_point(points[other]);
            if d < *best_dist {
                *best_dist = d;
                *best = (other, id);
            }
        }
        strip.push(id);
    }
}

/// A pair of points at maximum distance (`points.len() >= 1`): convex hull
/// plus rotating calipers, O(n log n).
pub fn farthest_pair<T: Base + Ord>(points: &[Point<T>]) -> (Point<T>, Point<T>) {
    assert!(!points.is_empty());
    let mut copy = points.to_vec();
    let hull = copy.as_mut_slice().convex_hull().points;
    let h = hull.len();
    if h == 1 {
        return (hull[0], hull[0]);
    }
    if h == 2 {
        return (hull[0], hull[1]);
    }
    let cross = |o: Point<T>, a: Point<T>, b: Point<T>| {
        (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x)
    };
    let abs = |v: T| if v < T::zero() { T::zero() - v } else { v };
    let mut best = (hull[0], hull[1]);
    let mut best_dist = hull[0].square_dist_point(hull[1]);
    let mut j = 1;
    for i in 0..h {
        let next = (i + 1) % h;
        // advance the antipodal vertex while the triangle area keeps growing
        while abs(cross(hull[i], hull[next], hull[(j + 1) % h]))
            > abs(cross(hull[i], hull[next], hull[j]))
        {
            j = (j + 1) % h;
        }
        for &p in &[hull[i], hull[next]] {
            let d = p.square_dist_point(hull[j]);
            if d > best_dist {
                best_dist = d;
                best = (p, hull[j]);
            }
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::{closest_pair, farthest_pair};
    use crate::geometry::point::Point;
    use crate::misc::random::{Random, RandomTrait};

    #[test]
    fn match_brute_force() {
        let mut rng = Random::new_with_seed(331);
        for round in 0..400 {
            let n = rng.gen_range(2..=60usize);
            let range = if round % 3 == 0 { 3i64 } else { 1_000_000_000 };
            let points: Vec<Point<i64>> = (0..n)
                .map(|_| Point::new(rng.gen_range(-range..=range), rng.gen_range(-range..=range)))
                .collect();
            let mut min = i64::MAX;
            let mut max = 0;
            for i in 0..n {
                for j in i + 1..n {
                    let d = points[i].square_dist_point(points[j]);
                    min = min.min(d);
                    max = max.max(d);
                }
            }
            let (a, b) = closest_pair(&points);
            assert_ne!(a, b);
            assert_eq!(points[a].square_dist_point(points[b]), min);
            let (p, q) = farthest_pair(&points);
            assert_eq!(p.square_dist_point(q), max);
        }
        let line: Vec<Point<i64>> = (0..10).map(|i| Point::new(i, 2 * i)).collect();
        let (p, q) = farthest_pair(&line);
        assert_eq!(p.square_dist_point(q), 81 * 5);
        assert!(farthest_pair(&[Point::new(5i64, 5)]).0 == Point::new(5, 5));
    }
}
