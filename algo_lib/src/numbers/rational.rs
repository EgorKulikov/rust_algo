use crate::io::output::{Output, Writable};
use crate::numbers::gcd::gcd;
use crate::numbers::num_traits::algebra::{IntegerRing, One, Zero};
use crate::numbers::num_traits::invertible::Invertible;
use crate::numbers::real::{IntoReal, Real};
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Always stored in lowest terms with a positive denominator, so the derived
/// `Hash` agrees with equality.
///
/// Operators and comparisons accept any right operand `A` with
/// `Rational<T>: From<A>`: another rational, an integer of type `T` or of a
/// narrower signed type, or a `(num, den)` pair.
#[derive(Copy, Clone, Hash)]
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

    pub fn new_int(num: T) -> Self {
        Self::new(num, T::one())
    }

    pub fn abs(mut self) -> Self {
        if self.num < T::zero() {
            self.num = T::zero() - self.num;
        }
        self
    }
}

impl<T: Copy + IntegerRing + Ord> Rational<T> {
    fn add_rational(self, rhs: Self) -> Self {
        Self::new(self.num * rhs.den + rhs.num * self.den, self.den * rhs.den)
    }

    fn sub_rational(self, rhs: Self) -> Self {
        Self::new(self.num * rhs.den - rhs.num * self.den, self.den * rhs.den)
    }

    fn mul_rational(self, rhs: Self) -> Self {
        Self::new(self.num * rhs.num, self.den * rhs.den)
    }

    fn div_rational(self, rhs: Self) -> Self {
        Self::new(self.num * rhs.den, self.den * rhs.num)
    }
}

macro_rules! rational_op_impl {
    ($($op: ident $method: ident $assign_op: ident $assign_method: ident $impl: ident),+) => {$(
        impl<T: Copy + IntegerRing + Ord, A> $op<A> for Rational<T>
        where
            Self: From<A>,
        {
            type Output = Self;

            fn $method(self, rhs: A) -> Self {
                self.$impl(Self::from(rhs))
            }
        }

        impl<T: Copy + IntegerRing + Ord, A> $assign_op<A> for Rational<T>
        where
            Self: From<A>,
        {
            fn $assign_method(&mut self, rhs: A) {
                *self = self.$impl(Self::from(rhs));
            }
        }
    )+};
}

rational_op_impl!(
    Add add AddAssign add_assign add_rational,
    Sub sub SubAssign sub_assign sub_rational,
    Mul mul MulAssign mul_assign mul_rational,
    Div div DivAssign div_assign div_rational
);

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

impl<T: Copy + IntegerRing + Ord, A: Copy> PartialEq<A> for Rational<T>
where
    Self: From<A>,
{
    fn eq(&self, other: &A) -> bool {
        let other = Self::from(*other);
        self.num == other.num && self.den == other.den
    }
}

impl<T: Copy + IntegerRing + Ord> Eq for Rational<T> {}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl<T: Copy + IntegerRing + Ord, A: Copy> PartialOrd<A> for Rational<T>
where
    Self: From<A>,
{
    fn partial_cmp(&self, other: &A) -> Option<Ordering> {
        Some(self.cmp(&Self::from(*other)))
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

/// `(num, den)`, reduced.
impl<T: Copy + IntegerRing + Ord> From<(T, T)> for Rational<T> {
    fn from((num, den): (T, T)) -> Self {
        Self::new(num, den)
    }
}

// Integers narrower than `T`, so that `r + 1` works for every `Rational<T>`.
macro_rules! from_narrower {
    ($($t: ty: $($a: ty)+;)+) => {$($(
        impl From<$a> for Rational<$t> {
            fn from(num: $a) -> Self {
                Self::new_internal(num as $t, 1)
            }
        }
    )+)+};
}

from_narrower!(
    i16: i8 u8;
    i32: i8 i16 u8 u16;
    i64: i8 i16 i32 u8 u16 u32;
    i128: i8 i16 i32 i64 u8 u16 u32 u64;
    isize: i8 i16 i32 u8 u16 u32;
);

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

impl<T: IntoReal> IntoReal for Rational<T> {
    fn into_real(self) -> Real {
        self.num.into_real() / self.den.into_real()
    }
}

impl<T: Copy + IntegerRing + Ord> Default for Rational<T> {
    fn default() -> Self {
        Self::zero()
    }
}

#[cfg(test)]
mod tests {
    use super::Rational;
    use crate::numbers::num_traits::algebra::{One, Zero};

    fn r(num: i64, den: i64) -> Rational<i64> {
        Rational::new(num, den)
    }

    #[test]
    fn operands_of_any_convertible_type() {
        let half = r(1, 2);
        assert_eq!(half + 1, r(3, 2));
        assert_eq!(half * 2, Rational::one());
        assert_eq!(half / 4i64, r(1, 8));
        assert_eq!(half - (1, 4), r(1, 4));
        assert_eq!(half + half, Rational::one());
        let mut x = half;
        x += 1i64;
        x *= (2, 3);
        x -= 1u8;
        x /= -1i16;
        assert_eq!(x, Rational::zero());
        let wide: Rational<i128> = Rational::new(1, 3);
        assert_eq!(wide + 5i64, Rational::new(16i128, 3));
        assert_eq!(wide * 3i32, Rational::one());
    }

    #[test]
    fn comparisons_with_any_convertible_type() {
        let third = r(1, 3);
        assert!(third == (2, 6));
        assert!(third < 1);
        assert!(third > 0i64);
        assert!(third != 1u32);
        assert!(r(5, 1) == 5);
        assert!(r(-7, 2) >= (-7, 2) && r(-7, 2) <= -3);
        assert_eq!(Rational::<i64>::from((6, -4)), r(-3, 2));
    }
}
