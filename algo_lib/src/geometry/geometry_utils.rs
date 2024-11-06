use crate::numbers::num_traits::algebra::Zero;
use crate::numbers::real::Real;

pub fn canonize_angle(angle: Real) -> Real {
    canonize_angle_base(angle, Real::zero() - Real::PI)
}

pub fn canonize_angle_base(angle: Real, base: Real) -> Real {
    let two = 2.;
    let mut angle = angle;
    while angle < base - Real::epsilon() {
        angle += Real::PI * two;
    }
    while angle > base + Real::PI * two - Real::epsilon() {
        angle -= Real::PI * two;
    }
    angle
}
