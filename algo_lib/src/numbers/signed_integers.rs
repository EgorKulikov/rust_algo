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
    type W: TryInto<Self> + TryFrom<Self> + Mul<Output = Self::W> + Rem<Output = Self::W>;

    fn wide_mul(lhs: Self, rhs: Self) -> Self::W;
}

macro_rules! signed_integer_impl {
    ($t: ident, $w: ident) => {
        impl SignedInteger for $t {
            type W = $w;

            fn wide_mul(lhs: Self, rhs: Self) -> Self::W {
                $w::try_from(lhs).unwrap() * $w::try_from(rhs).unwrap()
            }
        }
    };
}

signed_integer_impl!(i8, i16);
signed_integer_impl!(i16, i32);
signed_integer_impl!(i32, i64);
signed_integer_impl!(i64, i128);
signed_integer_impl!(i128, i128);
signed_integer_impl!(isize, i128);

// impl SignedInteger for i8 {
//     type W = i16;
//
//     fn wide_mul(lhs: Self, rhs: Self) -> W {
//         lhs.into() * rhs.into()
//     }
// }

// impl SignedInteger for i16 {}
// impl SignedInteger for i32 {}
// impl SignedInteger for i64 {}
// impl SignedInteger for i128 {}
// impl SignedInteger for isize {}
