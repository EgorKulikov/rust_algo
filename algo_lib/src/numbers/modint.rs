use crate::numbers::gcd::extended_gcd;
use crate::numbers::integer::{Integer, WeakInteger};
use crate::numbers::value::Value;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ModInt<T: Integer + Copy, V: Value<T>> {
    n: T,
    phantom: PhantomData<V>,
}

impl<T: Integer + Copy, V: Value<T>> ModInt<T, V> {
    pub fn new(n: T) -> Self {
        let mut res = Self {
            n: n % (V::VAL) + V::VAL,
            phantom: Default::default(),
        };
        res.safe();
        res
    }

    pub fn new_from_long(n: <T as Integer>::W) -> Self {
        let mut res = Self {
            n: <T as Integer>::downcast(n % (V::VAL).into()) + V::VAL,
            phantom: Default::default(),
        };
        res.safe();
        res
    }

    pub fn inverse(&self) -> Option<Self> {
        let (g, x, _) = extended_gcd(self.n, V::VAL);
        if g != T::one() {
            None
        } else {
            Some(Self::new_from_long(x))
        }
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

impl<T: Integer + Copy, V: Value<T>> From<T> for ModInt<T, V> {
    fn from(n: T) -> Self {
        Self::new(n)
    }
}

impl<T: Integer + Copy, V: Value<T>> AddAssign for ModInt<T, V> {
    fn add_assign(&mut self, rhs: Self) {
        self.n += rhs.n;
        self.safe();
    }
}

impl<T: Integer + Copy, V: Value<T>> Add for ModInt<T, V> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T: Integer + Copy, V: Value<T>> SubAssign for ModInt<T, V> {
    fn sub_assign(&mut self, rhs: Self) {
        self.n += V::VAL - rhs.n;
        self.safe();
    }
}

impl<T: Integer + Copy, V: Value<T>> Sub for ModInt<T, V> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<T: Integer + Copy, V: Value<T>> MulAssign for ModInt<T, V> {
    fn mul_assign(&mut self, rhs: Self) {
        self.n = <T as Integer>::downcast(
            <T as Integer>::W::from(self.n) * <T as Integer>::W::from(rhs.n)
                % <T as Integer>::W::from(V::VAL),
        );
    }
}

impl<T: Integer + Copy, V: Value<T>> Mul for ModInt<T, V> {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<T: Integer + Copy, V: Value<T>> DivAssign for ModInt<T, V> {
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs.inverse().unwrap();
    }
}

impl<T: Integer + Copy, V: Value<T>> Div for ModInt<T, V> {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        self /= rhs;
        self
    }
}

impl<T: Integer + Copy, V: Value<T>> Neg for ModInt<T, V> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.n = V::VAL - self.n;
        self.safe();
        self
    }
}

impl<T: Integer + Copy, V: Value<T>> Display for ModInt<T, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.n.fmt(f)
    }
}

impl<T: Integer, V: Value<T>> WeakInteger for ModInt<T, V> {
    type W = Self;
    fn zero() -> Self {
        Self::new(T::zero())
    }

    fn one() -> Self {
        Self::new(T::one())
    }

    fn from_u8(n: u8) -> Self {
        Self::new(T::from_u8(n))
    }

    fn downcast(w: Self::W) -> Self {
        w
    }
}
