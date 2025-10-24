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
        let delta = (self.radius * self.radius - dist * dist).sqrt();
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
