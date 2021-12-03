use crate::numbers::num_traits::add_sub::AddSub;
use crate::numbers::num_traits::mul_div_rem::{MulDivRem, Multable};
use crate::numbers::num_traits::zero_one::ZeroOne;

pub trait Power {
    fn power<T: ZeroOne + PartialEq + MulDivRem + AddSub + Copy>(&self, exp: T) -> Self;
}

impl<S: ZeroOne + Copy + Multable> Power for S {
    fn power<T: ZeroOne + PartialEq + MulDivRem + AddSub + Copy>(&self, exp: T) -> Self {
        if exp == T::zero() {
            S::one()
        } else if exp % (T::one() + T::one()) == T::zero() {
            let res = self.power(exp / (T::one() + T::one()));
            res * res
        } else {
            self.power(exp - T::one()) * (*self)
        }
    }
}
