use crate::numbers::signed_integers::SignedInteger;
use crate::numbers::value::Value;
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

pub struct ModInt<T: SignedInteger + Copy, V: Value<T>> {
    n: T,
    phantom: PhantomData<V>,
}

impl<T: SignedInteger + Copy, V: Value<T>> ModInt<T, V> {
    pub fn new(n: T) -> Self {
        let mut res = Self {
            n: n % (V::VAL),
            phantom: Default::default(),
        };
        res.safe();
        res
    }

    fn safe(&mut self) -> &mut Self {
        debug_assert!(self.n >= T::zero());
        debug_assert!(self.n < V::VAL + V::VAL);
        if self.n >= V::VAL {
            self.n -= V::VAL;
        }
        self
    }
}

impl<T: SignedInteger + Copy, V: Value<T>> From<T> for ModInt<T, V> {
    fn from(n: T) -> Self {
        Self::new(n)
    }
}

impl<T: SignedInteger + Copy, V: Value<T>> AddAssign for ModInt<T, V> {
    fn add_assign(&mut self, rhs: Self) {
        self.n += rhs.n;
        self.safe();
    }
}

impl<T: SignedInteger + Copy, V: Value<T>> Add for ModInt<T, V> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T: SignedInteger + Copy, V: Value<T>> SubAssign for ModInt<T, V> {
    fn sub_assign(&mut self, rhs: Self) {
        self.n += V::VAL - rhs.n;
        self.safe();
    }
}

impl<T: SignedInteger + Copy, V: Value<T>> Sub for ModInt<T, V> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<T: SignedInteger + Copy, V: Value<T>> MulAssign for ModInt<T, V> {
    fn mul_assign(&mut self, rhs: Self) {
        self.n = T::downcast(T::wide_mul(self.n, rhs.n) % T::W::from(V::VAL));
        self.safe();
    }
}

impl<T: SignedInteger + Copy, V: Value<T>> Mul for ModInt<T, V> {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}
