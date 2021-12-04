use crate::io::output::{Output, Writable};
use crate::numbers::gcd::gcd;
use crate::numbers::num_traits::add_sub::AddSub;
use crate::numbers::num_traits::mul_div_rem::MulDivRem;
use crate::numbers::num_traits::zero_one::ZeroOne;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Rational<T: Copy + ZeroOne + MulDivRem + AddSub + Ord> {
    num: T,
    den: T,
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
        Self {
            num: num / g,
            den: den / g,
        }
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

impl<T: Copy + ZeroOne + MulDivRem + AddSub + Ord + Hash> Hash for Rational<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.num, self.den).hash(state)
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
