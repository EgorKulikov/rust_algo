use crate::numbers::num_traits::algebra::{IntegerSemiRing, MultiplicationMonoid};
use std::ops::Mul;

pub trait Power {
    #[must_use]
    fn power<T: IntegerSemiRing + Copy>(&self, exp: T) -> Self;
}

impl<S: MultiplicationMonoid + Copy> Power for S {
    fn power<T: IntegerSemiRing + Copy>(&self, exp: T) -> Self {
        if exp == T::zero() {
            S::one()
        } else {
            let mut res = self.power(exp / (T::one() + T::one()));
            res *= res;
            if exp % (T::one() + T::one()) == T::one() {
                res *= *self;
            }
            res
        }
    }
}

pub fn num_digs<S: IntegerSemiRing + Copy>(mut copy: S) -> usize {
    let ten = S::ten();
    let mut res = 0;
    while copy != S::zero() {
        copy /= ten;
        res += 1;
    }
    res
}

pub fn sum_digs<S: IntegerSemiRing + Copy>(mut copy: S) -> S {
    let ten = S::ten();
    let mut res = S::zero();
    while copy != S::zero() {
        res += copy % ten;
        copy /= ten;
    }
    res
}

pub fn digits<S: IntegerSemiRing + Copy>(mut copy: S) -> impl Iterator<Item = S> {
    let ten = S::ten();
    std::iter::from_fn(move || {
        if copy == S::zero() {
            None
        } else {
            let res = copy % ten;
            copy /= ten;
            Some(res)
        }
    })
}

pub trait Square {
    fn square(self) -> Self;
}

impl<T: Mul<Output = T> + Copy> Square for T {
    fn square(self) -> Self {
        self * self
    }
}
