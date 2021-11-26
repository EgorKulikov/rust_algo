use std::convert::TryInto;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

pub trait SignedInteger:
    Add<Output = Self>
    + AddAssign
    + Div<Output = Self>
    + DivAssign
    + Mul<Output = Self>
    + MulAssign
    + Rem<Output = Self>
    + RemAssign
    + Sub<Output = Self>
    + SubAssign
    + PartialEq
    + PartialOrd
    + From<i8>
    + Display
    + Clone
{
    type W: TryInto<Self> + From<Self> + Mul<Output = Self::W> + Rem<Output = Self::W>;

    fn wide_mul(lhs: Self, rhs: Self) -> Self::W;

    fn downcast(w: Self::W) -> Self;

    fn zero() -> Self;

    fn one() -> Self;
}

macro_rules! signed_integer_impl {
    ($t: ident, $w: ident) => {
        impl SignedInteger for $t {
            type W = $w;

            fn wide_mul(lhs: Self, rhs: Self) -> Self::W {
                (lhs as $w) * (rhs as $w)
            }

            fn downcast(w: Self::W) -> Self {
                w as $t
            }

            fn zero() -> Self {
                0
            }

            fn one() -> Self {
                1
            }
        }
    };
}

signed_integer_impl!(i8, i16);
signed_integer_impl!(i16, i32);
signed_integer_impl!(i32, i64);
signed_integer_impl!(i64, i128);
signed_integer_impl!(i128, i128);
