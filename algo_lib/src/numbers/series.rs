use crate::numbers::num_traits::algebra::Ring;
use crate::numbers::number_ext::Power;
use std::ops::Div;

pub fn sum_arithmetic_series<T: Ring + Div<Output = T> + Copy>(first: T, step: T, len: T) -> T {
    (first + first + step * (len - T::one())) * len / (T::one() + T::one())
}

pub fn sum_geometric_series<T: Ring + Div<Output = T> + Copy>(first: T, ratio: T, len: usize) -> T {
    first * (T::one() - ratio.power(len)) / (T::one() - ratio)
}
