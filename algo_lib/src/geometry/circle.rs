use crate::geometry::line::Line;
use crate::geometry::point::Point;
use crate::geometry::Base;
use crate::numbers::real::Real;

#[derive(Copy, Clone)]
pub struct Circle<T> {
    pub center: Point<T>,
    pub radius: T,
}

impl<T> Circle<T> {
    pub fn new(center: Point<T>, radius: T) -> Self {
        Circle { center, radius }
    }
}

impl<T: Base + Ord> Circle<T> {
    pub fn contains(&self, point: Point<T>) -> bool {
        self.center.square_dist_point(point) <= self.radius * self.radius
    }
}

impl Circle<Real> {
    pub fn intersect_line(&self, l: Line<Real>) -> Vec<Point<Real>> {
        let dist = l.dist_point(self.center);
        if dist > self.radius {
            return vec![];
        }
        let perp = l.perpendicular(self.center);
        let base = l.intersect(perp);
        if dist == self.radius {
            return vec![base];
        }
        // (perp.a, perp.b) runs along `l` but is a unit vector only for
        // canonical lines.
        let delta = (self.radius * self.radius - dist * dist).sqrt() / Real::hypot(perp.a, perp.b);
        vec![
            base + Point::new(perp.a, perp.b) * delta,
            base - Point::new(perp.a, perp.b) * delta,
        ]
    }

    pub fn intersect_circle(&self, d: Circle<Real>) -> Vec<Point<Real>> {
        let dist = self.center.dist_point(d.center);
        if dist == 0. {
            return vec![];
        }
        let a = (d.center.x - self.center.x) * 2;
        let b = (d.center.y - self.center.y) * 2;
        let f = d.radius * d.radius - self.radius * self.radius + self.center.value_square()
            - d.center.value_square();
        let l = Line::new_canonical(a, b, f);
        self.intersect_line(l)
    }

    pub fn tangent_points(&self, p: Point<Real>) -> Vec<Point<Real>> {
        let dist = self.center.dist_point(p);
        if dist < self.radius {
            return vec![];
        }
        if dist == self.radius {
            return vec![p];
        }
        let power = Circle::new(p, ((dist - self.radius) * (dist + self.radius)).sqrt());
        self.intersect_circle(power)
    }
}

#[cfg(test)]
mod tests {
    use super::Circle;
    use crate::geometry::point::Point;
    use crate::numbers::real::Real;

    fn pt(x: f64, y: f64) -> Point<Real> {
        Point::new(Real(x), Real(y))
    }

    #[test]
    fn intersect_line_that_is_not_normalized() {
        let c = Circle::new(pt(1.0, 2.0), Real(5.0));
        // y = 5 through two far apart points: the line coefficients are not unit length
        let pts = c.intersect_line(pt(-7.0, 5.0).line(pt(9.0, 5.0)));
        assert_eq!(pts.len(), 2);
        let mut xs: Vec<f64> = pts.iter().map(|p| p.x.0).collect();
        xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert!(
            (xs[0] + 3.0).abs() < 1e-9 && (xs[1] - 5.0).abs() < 1e-9,
            "{:?}",
            xs
        );
        for p in pts {
            assert!((p.y.0 - 5.0).abs() < 1e-9);
        }
    }

    #[test]
    fn intersect_line_tangent_and_missing() {
        let c = Circle::new(pt(0.0, 0.0), Real(1.0));
        assert_eq!(c.intersect_line(pt(-3.0, 1.0).line(pt(4.0, 1.0))).len(), 1);
        assert_eq!(c.intersect_line(pt(-3.0, 2.0).line(pt(4.0, 2.0))).len(), 0);
    }
}
