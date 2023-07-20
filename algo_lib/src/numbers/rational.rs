use crate::io::output::{Output, Writable};
use crate::numbers::gcd::gcd;
use crate::numbers::num_traits::add_sub::AddSub;
use crate::numbers::num_traits::mul_div_rem::MulDivRem;
use crate::numbers::num_traits::real::{IntoReal, Real};
use crate::numbers::num_traits::zero_one::ZeroOne;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

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

impl<T: Copy + ZeroOne + MulDivRem + AddSub + Ord> Rational<T> {
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

impl<T: Copy + ZeroOne + MulDivRem + AddSub + Ord> Add for Rational<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.num * rhs.den + rhs.num * self.den, self.den * rhs.den)
    }
}

impl<T: Copy + ZeroOne + MulDivRem + AddSub + Ord> Sub for Rational<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.num * rhs.den - rhs.num * self.den, self.den * rhs.den)
    }
}

impl<T: Copy + ZeroOne + MulDivRem + AddSub + Ord> Mul for Rational<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.num * rhs.num, self.den * rhs.den)
    }
}

impl<T: Copy + ZeroOne + MulDivRem + AddSub + Ord> Div for Rational<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.num * rhs.den, self.den * rhs.num)
    }
}

impl<T: Copy + ZeroOne + MulDivRem + AddSub + Ord> AddAssign for Rational<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl<T: Copy + ZeroOne + MulDivRem + AddSub + Ord> SubAssign for Rational<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl<T: Copy + ZeroOne + MulDivRem + AddSub + Ord> MulAssign for Rational<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl<T: Copy + ZeroOne + MulDivRem + AddSub + Ord> DivAssign for Rational<T> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs
    }
}

impl<T: Copy + ZeroOne + MulDivRem + AddSub + Ord + Debug> Debug for Rational<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.num.fmt(f)?;
        <char as Debug>::fmt(&'/', f)?;
        self.den.fmt(f)
    }
}

impl<T: Copy + ZeroOne + MulDivRem + AddSub + Ord + Display> Display for Rational<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.num.fmt(f)?;
        <char as Display>::fmt(&'/', f)?;
        self.den.fmt(f)
    }
}

impl<T: Copy + ZeroOne + MulDivRem + AddSub + Ord + Writable> Writable for Rational<T> {
    fn write(&self, output: &mut Output) {
        self.num.write(output);
        output.put(b'/');
        self.den.write(output);
    }
}

impl<T: Copy + ZeroOne + MulDivRem + AddSub + Ord> PartialOrd for Rational<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.num * other.den).cmp(&(other.num * self.den)))
    }
}

impl<T: Copy + ZeroOne + MulDivRem + AddSub + Ord> Ord for Rational<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.num * other.den).cmp(&(other.num * self.den))
    }
}

impl<T: Copy + ZeroOne + MulDivRem + AddSub + Ord> ZeroOne for Rational<T> {
    fn zero() -> Self {
        Self::new(T::zero(), T::one())
    }

    fn one() -> Self {
        Self::new(T::one(), T::one())
    }
}

impl<T: ZeroOne> From<T> for Rational<T> {
    fn from(num: T) -> Self {
        Self::new_internal(num, T::one())
    }
}

pub trait ToRational
where
    Self: Sized,
{
    fn rat(self) -> Rational<Self>;
}

impl<T: ZeroOne> ToRational for T {
    fn rat(self) -> Rational<Self> {
        self.into()
    }
}

impl<T: IntoReal> Rational<T> {
    pub fn real(self) -> Real {
        self.num.into_real() / self.den.into_real()
    }
}
