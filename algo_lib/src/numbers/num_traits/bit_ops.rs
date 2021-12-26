use crate::numbers::num_traits::zero_one::ZeroOne;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl};
use std::ops::{ShlAssign, Shr, ShrAssign};

pub trait BitOps:
    Copy
    + BitAnd<Output = Self>
    + BitAndAssign
    + BitOr<Output = Self>
    + BitOrAssign
    + BitXor<Output = Self>
    + BitXorAssign
    + Not<Output = Self>
    + Shl<usize, Output = Self>
    + ShlAssign<usize>
    + Shr<usize, Output = Self>
    + ShrAssign<usize>
    + ZeroOne
    + PartialEq
{
    fn bit(at: usize) -> Self {
        Self::one() << at
    }

    fn is_set(&self, at: usize) -> bool {
        (*self >> at & Self::one()) == Self::one()
    }

    fn set_bit(&mut self, at: usize) {
        *self |= Self::bit(at)
    }

    fn unset_bit(&mut self, at: usize) {
        *self &= !Self::bit(at)
    }

    #[must_use]
    fn with_bit(mut self, at: usize) -> Self {
        self.set_bit(at);
        self
    }

    #[must_use]
    fn without_bit(mut self, at: usize) -> Self {
        self.unset_bit(at);
        self
    }

    fn flip_bit(&mut self, at: usize) {
        *self ^= Self::bit(at)
    }

    fn all_bits(n: usize) -> Self {
        let mut res = Self::zero();
        for i in 0..n {
            res.set_bit(i);
        }
        res
    }
}

impl<
        T: Copy
            + BitAnd<Output = Self>
            + BitAndAssign
            + BitOr<Output = Self>
            + BitOrAssign
            + BitXor<Output = Self>
            + BitXorAssign
            + Not<Output = Self>
            + Shl<usize, Output = Self>
            + ShlAssign<usize>
            + Shr<usize, Output = Self>
            + ShrAssign<usize>
            + ZeroOne
            + PartialEq,
    > BitOps for T
{
}

pub trait Bits: BitOps {
    fn bits() -> u32;
}

macro_rules! bits_integer_impl {
    ($t: ident, $bits: expr) => {
        impl Bits for $t {
            fn bits() -> u32 {
                $bits
            }
        }
    };
}

bits_integer_impl!(i128, 128);
bits_integer_impl!(i64, 64);
bits_integer_impl!(i32, 32);
bits_integer_impl!(i16, 16);
bits_integer_impl!(i8, 8);
bits_integer_impl!(isize, 64);
bits_integer_impl!(u128, 128);
bits_integer_impl!(u64, 64);
bits_integer_impl!(u32, 32);
bits_integer_impl!(u16, 16);
bits_integer_impl!(u8, 8);
bits_integer_impl!(usize, 64);
