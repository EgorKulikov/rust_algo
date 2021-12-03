use crate::numbers::num_traits::add_sub::AddSub;
use crate::numbers::num_traits::mul_div_rem::{MulDivRem, Multable};
use crate::numbers::num_traits::wideable::Wideable;
use crate::numbers::num_traits::zero_one::ZeroOne;
use std::mem::swap;

pub fn extended_gcd<T: Copy + ZeroOne + AddSub + MulDivRem + Wideable + PartialEq>(
    a: T,
    b: T,
) -> (T, T::W, T::W)
where
    T::W: Copy + ZeroOne + AddSub + Multable,
{
    if a == T::zero() {
        (b, T::W::zero(), T::W::one())
    } else {
        let (d, y, mut x) = extended_gcd(b % a, a);
        x -= T::W::from(b / a) * y;
        (d, x, y)
    }
}

pub fn gcd<T: Copy + ZeroOne + MulDivRem + PartialEq>(mut a: T, mut b: T) -> T {
    while b != T::zero() {
        a %= b;
        swap(&mut a, &mut b);
    }
    a
}

pub fn lcm<T: Copy + ZeroOne + MulDivRem + PartialEq>(a: T, b: T) -> T {
    (a / gcd(a, b)) * b
}
