use crate::numbers::integer::Integer;
use crate::types::value::Value;
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub struct InfInt<T: Integer, V: Value<T>> {
    n: T,
    phantom: PhantomData<V>,
}

impl<T: Integer, V: Value<T>> InfInt<T, V> {
    pub fn new(n: T) -> Self {
        Self {
            n: n.min(V::val()),
            phantom: Default::default(),
        }
    }

    pub fn is_infty(&self) -> bool {
        self.n == V::val()
    }
}

impl<T: Integer, V: Value<T>> AddAssign for InfInt<T, V> {
    fn add_assign(&mut self, rhs: Self) {
        if rhs.is_infty() || V::val() - rhs.n <= self.n {
            self.n = V::val();
        } else {
            self.n += rhs.n;
        }
    }
}

impl<T: Integer, V: Value<T>> Add for InfInt<T, V> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T: Integer, V: Value<T>> MulAssign for InfInt<T, V> {
    fn mul_assign(&mut self, rhs: Self) {
        if rhs.n != T::zero() && V::val() / rhs.n < self.n {
            self.n = V::val();
        } else {
            self.n *= rhs.n;
        }
    }
}

impl<T: Integer, V: Value<T>> Mul for InfInt<T, V> {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}
