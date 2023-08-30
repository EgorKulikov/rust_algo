use crate::numbers::num_traits::add_sub::AddSub;
use crate::numbers::num_traits::as_index::AsIndex;
use crate::numbers::num_traits::mul_div_rem::Multable;
use crate::numbers::num_traits::zero_one::ZeroOne;
use std::ops::{Add, Div};

pub fn factorials<T: ZeroOne + Multable + AsIndex>(len: usize) -> Vec<T> {
    let mut res = Vec::new();
    if len > 0 {
        res.push(T::one());
    }
    while res.len() < len {
        res.push((*res.last().unwrap()) * T::from_index(res.len()));
    }
    res
}

pub fn powers<T: ZeroOne + Multable + AsIndex>(base: T, len: usize) -> Vec<T> {
    let mut res = Vec::new();
    if len > 0 {
        res.push(T::one());
    }
    while res.len() < len {
        res.push((*res.last().unwrap()) * base);
    }
    res
}

pub fn factorial<T: ZeroOne + Multable + AsIndex>(n: usize) -> T {
    let mut res = T::one();
    for i in 1..=n {
        res *= T::from_index(i);
    }
    res
}

pub trait PartialSums<T> {
    fn partial_sums(&self) -> Vec<T>;
}

impl<T: ZeroOne + Add<Output = T> + Copy> PartialSums<T> for [T] {
    fn partial_sums(&self) -> Vec<T> {
        let mut res = Vec::with_capacity(self.len() + 1);
        res.push(T::zero());
        for i in self.iter() {
            res.push(*res.last().unwrap() + *i);
        }
        res
    }
}

pub trait UpperDiv {
    fn upper_div(self, other: Self) -> Self;
}

impl<T: Div<Output = T> + AddSub + ZeroOne + Copy> UpperDiv for T {
    fn upper_div(self, other: Self) -> Self {
        (self + other - Self::one()) / other
    }
}
