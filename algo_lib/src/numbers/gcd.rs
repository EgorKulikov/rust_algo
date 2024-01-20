use crate::numbers::num_traits::algebra::{
    IntegerMultiplicationMonoid, IntegerSemiRingWithSub, One, SemiRingWithSub, Zero,
};
use crate::numbers::num_traits::wideable::Wideable;
use std::mem::swap;

pub fn extended_gcd<T: IntegerSemiRingWithSub + Wideable + Copy>(a: T, b: T) -> (T, T::W, T::W)
where
    T::W: Copy + SemiRingWithSub,
{
    if a == T::zero() {
        (b, T::W::zero(), T::W::one())
    } else {
        let (d, y, mut x) = extended_gcd(b % a, a);
        x -= T::W::from(b / a) * y;
        (d, x, y)
    }
}

pub fn gcd<T: Copy + Zero + IntegerMultiplicationMonoid>(mut a: T, mut b: T) -> T {
    while b != T::zero() {
        a %= b;
        swap(&mut a, &mut b);
    }
    a
}

pub fn lcm<T: Copy + Zero + IntegerMultiplicationMonoid>(a: T, b: T) -> T {
    (a / gcd(a, b)) * b
}
