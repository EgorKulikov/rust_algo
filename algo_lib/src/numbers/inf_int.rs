use crate::io::output::{Output, Writable};
use crate::misc::value::Value;
use crate::numbers::num_traits::add_sub::AddSub;
use crate::numbers::num_traits::mul_div_rem::MulDiv;
use crate::numbers::num_traits::zero_one::ZeroOne;
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct InfInt<T: AddSub + MulDiv + ZeroOne + Ord + Copy, V: Value<T>> {
    n: T,
    phantom: PhantomData<V>,
}

impl<T: AddSub + MulDiv + ZeroOne + Ord + Copy, V: Value<T>> InfInt<T, V> {
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

impl<T: AddSub + MulDiv + ZeroOne + Ord + Copy, V: Value<T>> AddAssign for InfInt<T, V> {
    fn add_assign(&mut self, rhs: Self) {
        if rhs.is_infinity() || V::val() - rhs.n <= self.n {
            self.n = V::val();
        } else {
            self.n += rhs.n;
        }
    }
}

impl<T: AddSub + MulDiv + ZeroOne + Ord + Copy, V: Value<T>> Add for InfInt<T, V> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T: AddSub + MulDiv + ZeroOne + Ord + Copy, V: Value<T>> MulAssign for InfInt<T, V> {
    fn mul_assign(&mut self, rhs: Self) {
        if rhs.n != T::zero() && V::val() / rhs.n < self.n {
            self.n = V::val();
        } else {
            self.n *= rhs.n;
        }
    }
}

impl<T: AddSub + MulDiv + ZeroOne + Ord + Copy, V: Value<T>> Mul for InfInt<T, V> {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<T: AddSub + MulDiv + ZeroOne + Ord + Copy + Writable, V: Value<T>> Writable for InfInt<T, V> {
    fn write(&self, output: &mut Output) {
        self.n.write(output)
    }
}
