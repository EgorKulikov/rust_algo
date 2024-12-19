use crate::numbers::num_traits::algebra::{
    IntegerMultiplicationMonoid, IntegerSemiRingWithSub, Zero,
};
use std::mem::swap;

pub fn extended_gcd<T: IntegerSemiRingWithSub + Copy>(a: T, b: T) -> (T, T, T) {
    if a == T::zero() {
        (b, T::zero(), T::one())
    } else {
        let (d, y, mut x) = extended_gcd(b % a, a);
        x -= b / a * y;
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

pub fn remainder<T: IntegerSemiRingWithSub + Copy>(a1: T, n1: T, a2: T, n2: T) -> Option<T> {
    let (d, m1, m2) = extended_gcd(n1, n2);
    if (a2 - a1) % d != T::zero() {
        return None;
    }
    let m = lcm(n1, n2);
    Some(((a1 * m2) % n1 * n2 + (a2 * m1) % n2 * n1) % m)
}
