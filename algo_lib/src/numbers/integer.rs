use crate::io::input::Readable;
use crate::io::output::Writable;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

pub trait WeakInteger:
    Add<Output = Self>
    + AddAssign
    + Div<Output = Self>
    + DivAssign
    + Mul<Output = Self>
    + MulAssign
    + Sub<Output = Self>
    + SubAssign
    + PartialEq
    + Display
    + Copy
    + Readable
    + Writable
{
    type W: From<Self> + WeakInteger;

    fn zero() -> Self;
    fn one() -> Self;
    fn from_u8(n: u8) -> Self;
    fn downcast(w: Self::W) -> Self;
}

pub trait Integer: WeakInteger + Ord + Rem<Output = Self> + RemAssign {
    type W: From<Self> + Integer;

    const SIGNED: bool;

    fn downcast(w: <Self as Integer>::W) -> Self;
}

macro_rules! integer_impl {
    ($t: ident, $w: ident, $s: expr) => {
        impl WeakInteger for $t {
            type W = $w;

            fn zero() -> Self {
                0
            }

            fn one() -> Self {
                0
            }

            fn from_u8(n: u8) -> Self {
                n as $t
            }

            fn downcast(w: Self::W) -> Self {
                w as $t
            }
        }

        impl Integer for $t {
            type W = $w;

            const SIGNED: bool = $s;

            fn downcast(w: <Self as Integer>::W) -> Self {
                w as $t
            }
        }
    };
}

integer_impl!(i128, i128, true);
integer_impl!(i64, i128, true);
integer_impl!(i32, i64, true);
integer_impl!(i16, i32, true);
integer_impl!(i8, i16, true);
integer_impl!(isize, isize, true);
integer_impl!(u128, u128, false);
integer_impl!(u64, u128, false);
integer_impl!(u32, u64, false);
integer_impl!(u16, u32, false);
integer_impl!(u8, u16, false);
integer_impl!(usize, usize, false);
