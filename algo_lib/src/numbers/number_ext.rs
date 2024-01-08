use crate::numbers::num_traits::algebra::{
    AdditionMonoidWithSub, IntegerSemiRing, MultiplicationMonoid,
};
use crate::numbers::num_traits::from_u8::FromU8;
use crate::when;
use std::ops::Mul;

pub trait Power {
    #[must_use]
    fn power<T: IntegerSemiRing + AdditionMonoidWithSub + Copy>(&self, exp: T) -> Self;
}

impl<S: MultiplicationMonoid + Copy> Power for S {
    fn power<T: IntegerSemiRing + AdditionMonoidWithSub + Copy>(&self, exp: T) -> Self {
        when! {
            exp == T::zero() => S::one(),
            exp % (T::one() + T::one()) == T::zero() => {
                let res = self.power(exp / (T::one() + T::one()));
                res * res
            },
            else => self.power(exp - T::one()) * (*self),
        }
    }
}

pub trait NumDigs {
    fn num_digs(&self) -> usize;
}

impl<S: IntegerSemiRing + FromU8 + Copy> NumDigs for S {
    fn num_digs(&self) -> usize {
        let mut copy = *self;
        let ten = S::from_u8(10);
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
