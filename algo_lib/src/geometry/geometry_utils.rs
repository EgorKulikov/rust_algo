use crate::numbers::num_traits::real::RealTrait;

pub fn canonize_angle<T: RealTrait>(angle: T) -> T {
    canonize_angle_base(angle, T::zero() - T::PI)
}

pub fn canonize_angle_base<T: RealTrait>(angle: T, base: T) -> T {
    let two = T::one() + T::one();
    let mut angle = angle;
    while angle < base - T::epsilon() {
        angle += T::PI * two;
    }
    while angle > base + T::PI * two - T::epsilon() {
        angle -= T::PI * two;
    }
    angle
}
