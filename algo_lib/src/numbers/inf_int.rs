use crate::io::output::{Output, Writable};
use crate::misc::value::Value;
use crate::numbers::num_traits::algebra::{
    AdditionMonoidWithSub, IntegerMultiplicationMonoid, Zero,
};
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct InfInt<T, V: Value<T>> {
    pub n: T,
    phantom: PhantomData<V>,
}

impl<T: Ord, V: Value<T>> InfInt<T, V> {
    pub fn new(n: T) -> Self {
        Self {
            n: n.min(V::val()),
            phantom: Default::default(),
        }
    }

    pub fn is_infinity(&self) -> bool {
        self.n == V::val()
    }
}

impl<T: AdditionMonoidWithSub + Ord + Copy, V: Value<T>> AddAssign for InfInt<T, V> {
    fn add_assign(&mut self, rhs: Self) {
        if rhs.is_infinity() || V::val() - rhs.n <= self.n {
            self.n = V::val();
        } else {
            self.n += rhs.n;
        }
    }
}

impl<T: AdditionMonoidWithSub + Ord + Copy, V: Value<T>> Add for InfInt<T, V> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T: IntegerMultiplicationMonoid + Zero + Ord + Copy, V: Value<T>> MulAssign for InfInt<T, V> {
    fn mul_assign(&mut self, rhs: Self) {
        if rhs.n != T::zero() && V::val() / rhs.n < self.n {
            self.n = V::val();
        } else {
            self.n *= rhs.n;
        }
    }
}

impl<T: IntegerMultiplicationMonoid + Zero + Ord + Copy, V: Value<T>> Mul for InfInt<T, V> {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<T: Writable, V: Value<T>> Writable for InfInt<T, V> {
    fn write(&self, output: &mut Output) {
        self.n.write(output)
    }
}
