use crate::io::output::{Output, Writable};
use crate::numbers::gcd::gcd;
use crate::numbers::num_traits::algebra::{IntegerRing, One, Zero};
use crate::numbers::num_traits::invertible::Invertible;
use crate::numbers::real::{IntoReal, Real};
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub struct Rational<T> {
    num: T,
    den: T,
}

impl<T> Rational<T> {
    fn new_internal(num: T, den: T) -> Self {
        Self { num, den }
    }
}

impl<T: Copy> Rational<T> {
    pub fn num(&self) -> T {
        self.num
    }

    pub fn den(&self) -> T {
        self.den
    }
}

impl<T: Copy + IntegerRing + Ord> Rational<T> {
    pub fn new(num: T, den: T) -> Self {
        assert!(den != T::zero());
        let mut g = gcd(num, den);

        if g < T::zero() {
            g = T::zero() - g;
        }
        if den < T::zero() {
            g = T::zero() - g;
        }
        Self::new_internal(num / g, den / g)
    }

    pub fn abs(mut self) -> Self {
        if self.num < T::zero() {
            self.num = T::zero() - self.num;
        }
        self
    }
}

impl<T: Copy + IntegerRing + Ord> Add for Rational<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.num * rhs.den + rhs.num * self.den, self.den * rhs.den)
    }
}

impl<T: Copy + IntegerRing + Ord> Sub for Rational<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.num * rhs.den - rhs.num * self.den, self.den * rhs.den)
    }
}

impl<T: Copy + IntegerRing + Ord> Mul for Rational<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.num * rhs.num, self.den * rhs.den)
    }
}

impl<T: Copy + IntegerRing + Ord> Div for Rational<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.num * rhs.den, self.den * rhs.num)
    }
}

impl<T: Copy + IntegerRing + Ord> AddAssign for Rational<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl<T: Copy + IntegerRing + Ord> SubAssign for Rational<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl<T: Copy + IntegerRing + Ord> MulAssign for Rational<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl<T: Copy + IntegerRing + Ord> DivAssign for Rational<T> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs
    }
}

impl<T: Copy + Debug> Debug for Rational<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.num.fmt(f)?;
        <char as Debug>::fmt(&'/', f)?;
        self.den.fmt(f)
    }
}

impl<T: Copy + Display> Display for Rational<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.num.fmt(f)?;
        <char as Display>::fmt(&'/', f)?;
        self.den.fmt(f)
    }
}

impl<T: Copy + Writable> Writable for Rational<T> {
    fn write(&self, output: &mut Output) {
        self.num.write(output);
        output.put(b'/');
        self.den.write(output);
    }
}

impl<T: Copy + IntegerRing + Ord> PartialOrd for Rational<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.num * other.den).cmp(&(other.num * self.den)))
    }
}

impl<T: Copy + IntegerRing + Ord> Ord for Rational<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.num * other.den).cmp(&(other.num * self.den))
    }
}

impl<T: Copy + IntegerRing + Ord> Zero for Rational<T> {
    fn zero() -> Self {
        Self::new(T::zero(), T::one())
    }
}

impl<T: Copy + IntegerRing + Ord> One for Rational<T> {
    fn one() -> Self {
        Self::new(T::one(), T::one())
    }
}

impl<T: One> From<T> for Rational<T> {
    fn from(num: T) -> Self {
        Self::new_internal(num, T::one())
    }
}

impl<T: IntegerRing + Ord + Copy> Invertible for Rational<T> {
    type Output = Self;

    fn inv(&self) -> Option<Self::Output> {
        if self.num == T::zero() {
            None
        } else {
            Some(Self::new(self.den, self.num))
        }
    }
}

impl<T: IntegerRing + Ord + Copy> Neg for Rational<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new_internal(-self.num, self.den)
    }
}

pub trait ToRational
where
    Self: Sized,
{
    fn rat(self) -> Rational<Self>;
}

impl<T: One> ToRational for T {
    fn rat(self) -> Rational<Self> {
        self.into()
    }
}

impl<T: IntoReal> Rational<T> {
    pub fn real(self) -> Real {
        self.num.into_real() / self.den.into_real()
    }
}
