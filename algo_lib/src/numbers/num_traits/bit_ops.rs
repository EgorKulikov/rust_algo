use crate::numbers::num_traits::algebra::{One, Zero};
use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, RangeInclusive, Shl, Sub,
};
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
    + Zero
    + One
    + PartialEq
{
    #[inline]
    fn bit(at: usize) -> Self {
        Self::one() << at
    }

    #[inline]
    fn is_set(&self, at: usize) -> bool {
        (*self >> at & Self::one()) == Self::one()
    }

    #[inline]
    fn set_bit(&mut self, at: usize) {
        *self |= Self::bit(at)
    }

    #[inline]
    fn unset_bit(&mut self, at: usize) {
        *self &= !Self::bit(at)
    }

    #[must_use]
    #[inline]
    fn with_bit(mut self, at: usize) -> Self {
        self.set_bit(at);
        self
    }

    #[must_use]
    #[inline]
    fn without_bit(mut self, at: usize) -> Self {
        self.unset_bit(at);
        self
    }

    #[inline]
    fn flip_bit(&mut self, at: usize) {
        *self ^= Self::bit(at)
    }

    #[must_use]
    #[inline]
    fn flipped_bit(mut self, at: usize) -> Self {
        self.flip_bit(at);
        self
    }

    fn all_bits(n: usize) -> Self {
        let mut res = Self::zero();
        for i in 0..n {
            res.set_bit(i);
        }
        res
    }

    fn iter_all(n: usize) -> RangeInclusive<Self> {
        Self::zero()..=Self::all_bits(n)
    }
}

pub struct BitIter<T> {
    cur: T,
    all: T,
    ended: bool,
}

impl<T: Copy> BitIter<T> {
    pub fn new(all: T) -> Self {
        Self {
            cur: all,
            all,
            ended: false,
        }
    }
}

impl<T: BitOps + Sub<Output = T>> Iterator for BitIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ended {
            return None;
        }
        let res = self.cur;
        if self.cur == T::zero() {
            self.ended = true;
        } else {
            self.cur = (self.cur - T::one()) & self.all;
        }
        Some(res)
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
            + One
            + Zero
            + PartialEq,
    > BitOps for T
{
}

pub trait Bits: BitOps {
    fn bits() -> u32;
}

macro_rules! bits_integer_impl {
    ($($t: ident $bits: expr),+) => {$(
        impl Bits for $t {
            fn bits() -> u32 {
                $bits
            }
        }
    )+};
}

bits_integer_impl!(i128 128, i64 64, i32 32, i16 16, i8 8, isize 64, u128 128, u64 64, u32 32, u16 16, u8 8, usize 64);
