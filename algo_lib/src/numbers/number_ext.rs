use crate::numbers::num_traits::algebra::{IntegerSemiRing, MultiplicationMonoid};
use crate::numbers::num_traits::as_index::AsIndex;
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

pub trait NumDigs {
    fn num_digs(&self) -> usize;
}

impl<S: IntegerSemiRing + AsIndex + Copy> NumDigs for S {
    fn num_digs(&self) -> usize {
        let mut copy = *self;
        let ten = S::from_index(10);
        let mut res = 0;
        while copy != S::zero() {
            copy /= ten;
            res += 1;
        }
        res
    }
}

pub trait Square {
    fn square(self) -> Self;
}

impl<T: Mul<Output = T> + Copy> Square for T {
    fn square(self) -> Self {
        self * self
    }
}
