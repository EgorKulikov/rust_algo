use crate::geometry::circle::Circle;
use crate::geometry::point::Point;
use crate::misc::random::{RandomTrait, StaticRandom};
use crate::numbers::real::Real;

type P = (f64, f64);

fn circle_two(a: P, b: P) -> (P, f64) {
    let c = ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0);
    (c, dist2(c, a))
}

fn circle_three(a: P, b: P, c: P) -> Option<(P, f64)> {
    let (bx, by) = (b.0 - a.0, b.1 - a.1);
    let (cx, cy) = (c.0 - a.0, c.1 - a.1);
    let d = 2.0 * (bx * cy - by * cx);
    if d == 0.0 {
        return None;
    }
    let (b2, c2) = (bx * bx + by * by, cx * cx + cy * cy);
    let center = (a.0 + (cy * b2 - by * c2) / d, a.1 + (bx * c2 - cx * b2) / d);
    Some((center, dist2(center, a)))
}

fn dist2(a: P, b: P) -> f64 {
    (a.0 - b.0) * (a.0 - b.0) + (a.1 - b.1) * (a.1 - b.1)
}

/// Smallest circle containing all points (`points` nonempty), by Welzl's
/// randomized incremental algorithm in expected O(n).
pub fn min_enclosing_circle(points: &[Point<Real>]) -> Circle<Real> {
    assert!(!points.is_empty());
    let mut pts: Vec<P> = points.iter().map(|p| (p.x.0, p.y.0)).collect();
    for i in (1..pts.len()).rev() {
        pts.swap(i, StaticRandom.gen_range(0..=i));
    }
    // A tiny relative slack keeps boundary points from triggering rebuilds.
    let inside = |c: (P, f64), p: P| dist2(c.0, p) <= c.1 * (1.0 + 1e-12) + 1e-18;
    let mut cur = (pts[0], 0.0);
    for i in 1..pts.len() {
        if inside(cur, pts[i]) {
            continue;
        }
        cur = (pts[i], 0.0);
        for j in 0..i {
            if inside(cur, pts[j]) {
                continue;
            }
            cur = circle_two(pts[i], pts[j]);
            for k in 0..j {
                if inside(cur, pts[k]) {
                    continue;
                }
                if let Some(c) = circle_three(pts[i], pts[j], pts[k]) {
                    cur = c;
                }
            }
        }
    }
    Circle::new(
        Point::new(Real(cur.0 .0), Real(cur.0 .1)),
        Real(cur.1.sqrt()),
    )
}

#[cfg(test)]
mod tests {
    use super::min_enclosing_circle;
    use crate::geometry::point::Point;
    use crate::misc::random::{Random, RandomTrait};
    use crate::numbers::real::Real;

    #[test]
    fn encloses_and_is_minimal() {
        let mut rng = Random::new_with_seed(341);
        for _ in 0..300 {
            let n = rng.gen_range(1..=30usize);
            let range = if rng.gen_bool() { 3i64 } else { 1_000_000 };
            let raw: Vec<(f64, f64)> = (0..n)
                .map(|_| {
                    (
                        rng.gen_range(-range..=range) as f64,
                        rng.gen_range(-range..=range) as f64,
                    )
                })
                .collect();
            let points: Vec<Point<Real>> = raw
                .iter()
                .map(|&(x, y)| Point::new(Real(x), Real(y)))
                .collect();
            let circle = min_enclosing_circle(&points);
            let (cx, cy, r) = (circle.center.x.0, circle.center.y.0, circle.radius.0);
            let eps = 1e-6 * (1.0 + r);
            for &(x, y) in &raw {
                assert!(((x - cx).powi(2) + (y - cy).powi(2)).sqrt() <= r + eps);
            }
            // brute force: best circle through two or three points that encloses everything
            let mut best = f64::MAX;
            let encloses = |c: (f64, f64), rr: f64| {
                raw.iter()
                    .all(|&(x, y)| ((x - c.0).powi(2) + (y - c.1).powi(2)).sqrt() <= rr + eps)
            };
            if n == 1 {
                best = 0.0;
            }
            for i in 0..n {
                for j in i + 1..n {
                    let (c, r2) = super::circle_two(raw[i], raw[j]);
                    if encloses(c, r2.sqrt()) {
                        best = best.min(r2.sqrt());
                    }
                    for k in j + 1..n {
                        if let Some((c, r2)) = super::circle_three(raw[i], raw[j], raw[k]) {
                            if encloses(c, r2.sqrt()) {
                                best = best.min(r2.sqrt());
                            }
                        }
                    }
                }
            }
            assert!(
                (r - best).abs() <= 1e-6 * (1.0 + best),
                "r={r} best={best} n={n}"
            );
        }
    }
}
