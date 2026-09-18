use crate::geometry::point::Point;
use crate::geometry::Base;
use std::cmp::Ordering;

/// Exact comparison of directions by polar angle in `(-pi, pi]`, as `atan2`
/// would order them, without floating point. The origin counts as angle 0.
/// Coordinates must be small enough for a cross product to fit in `T`.
pub fn arg_cmp<T: Base + Ord>(a: Point<T>, b: Point<T>) -> Ordering {
    let half = |p: Point<T>| -> u8 {
        let zero = T::zero();
        match (p.y.cmp(&zero), p.x.cmp(&zero)) {
            (Ordering::Less, _) => 0,
            (Ordering::Equal, Ordering::Less) => 3,
            (Ordering::Equal, _) => 1,
            (Ordering::Greater, _) => 2,
        }
    };
    half(a).cmp(&half(b)).then_with(|| {
        let cross = a.x * b.y - a.y * b.x;
        T::zero().cmp(&cross)
    })
}

pub fn sort_by_argument<T: Base + Ord>(points: &mut [Point<T>]) {
    points.sort_by(|&a, &b| arg_cmp(a, b));
}

#[cfg(test)]
mod tests {
    use super::{arg_cmp, sort_by_argument};
    use crate::geometry::point::Point;
    use crate::misc::random::{Random, RandomTrait};
    use std::cmp::Ordering;

    #[test]
    fn agrees_with_atan2() {
        let mut rng = Random::new_with_seed(321);
        let mut points: Vec<Point<i64>> = (0..400)
            .map(|_| Point::new(rng.gen_range(-6..=6i64), rng.gen_range(-6..=6i64)))
            .collect();
        points.push(Point::new(0, 0));
        points.push(Point::new(-1_000_000_000, 0));
        points.push(Point::new(-1_000_000_000, -1));
        points.push(Point::new(1_000_000_000, 999_999_999));
        for &a in &points {
            for &b in &points {
                let (fa, fb) = (
                    (a.y as f64).atan2(a.x as f64),
                    (b.y as f64).atan2(b.x as f64),
                );
                let cross = a.x as i128 * b.y as i128 - a.y as i128 * b.x as i128;
                let same_direction = cross == 0 && (fa - fb).abs() < 1e-9;
                let expected = if same_direction {
                    Ordering::Equal
                } else {
                    fa.partial_cmp(&fb).unwrap()
                };
                assert_eq!(arg_cmp(a, b), expected, "{:?} {:?}", (a.x, a.y), (b.x, b.y));
            }
        }
        sort_by_argument(&mut points);
        assert!(points
            .windows(2)
            .all(|w| arg_cmp(w[0], w[1]) != Ordering::Greater));
    }
}
