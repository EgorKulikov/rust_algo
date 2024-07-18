use crate::numbers::num_traits::algebra::{AdditionMonoid, IntegerRing, MultiplicationMonoid};
use crate::numbers::num_traits::as_index::AsIndex;

pub fn factorials<T: MultiplicationMonoid + Copy + AsIndex>(len: usize) -> Vec<T> {
    let mut res = Vec::new();
    if len > 0 {
        res.push(T::one());
    }
    while res.len() < len {
        res.push((*res.last().unwrap()) * T::from_index(res.len()));
    }
    res
}

pub fn powers<T: MultiplicationMonoid + Copy>(base: T, len: usize) -> Vec<T> {
    let mut res = Vec::new();
    if len > 0 {
        res.push(T::one());
    }
    while res.len() < len {
        res.push((*res.last().unwrap()) * base);
    }
    res
}

pub struct Powers<T: MultiplicationMonoid + Copy> {
    small: Vec<T>,
    big: Vec<T>,
}

impl<T: MultiplicationMonoid + Copy> Powers<T> {
    pub fn new(base: T, len: usize) -> Self {
        let small = powers(base, len);
        let big = powers(small[len - 1] * base, len);
        Self { small, big }
    }

    pub fn power(&self, exp: usize) -> T {
        debug_assert!(exp < self.small.len() * self.small.len());
        self.big[exp / self.small.len()] * self.small[exp % self.small.len()]
    }
}

pub fn factorial<T: MultiplicationMonoid + AsIndex>(n: usize) -> T {
    let mut res = T::one();
    for i in 1..=n {
        res *= T::from_index(i);
    }
    res
}

pub trait PartialSums<T> {
    fn partial_sums(&self) -> Vec<T>;
}

impl<T: AdditionMonoid + Copy> PartialSums<T> for [T] {
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

impl<T: IntegerRing + Copy> UpperDiv for T {
    fn upper_div(self, other: Self) -> Self {
        (self + other - Self::one()) / other
    }
}
