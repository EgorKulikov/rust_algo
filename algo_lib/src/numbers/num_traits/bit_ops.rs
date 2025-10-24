use std::ops::RangeInclusive;

pub trait BitOps: Sized {
    const BITS: usize;
    fn bit(at: usize) -> Self;
    fn is_set(&self, at: usize) -> bool;
    fn set_bit(&mut self, at: usize);
    fn unset_bit(&mut self, at: usize);
    fn with_bit(self, at: usize) -> Self;
    fn without_bit(self, at: usize) -> Self;
    fn flip_bit(&mut self, at: usize);
    fn flipped_bit(self, at: usize) -> Self;
    fn all_bits(n: usize) -> Self;
    fn iter_all(n: usize) -> RangeInclusive<Self>;
    fn lowest_bit(&self) -> usize;
    fn highest_bit(&self) -> usize;
    fn is_subset(&self, other: Self) -> bool;
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

macro_rules! bit_ops {
    ($($t: ty: $b: literal)+) => {
        $(impl BitOps for $t {
            const BITS: usize = $b;
            fn bit(at: usize) -> $t {
                1 << at
            }
            fn is_set(&self, at: usize) -> bool {
                (*self >> at & 1) == 1
            }
            fn set_bit(&mut self, at: usize) {
                *self |= 1 << at
            }
            fn unset_bit(&mut self, at: usize) {
                *self &= !(1 << at)
            }
            fn with_bit(mut self, at: usize) -> $t {
                self.set_bit(at);
                self
            }
            fn without_bit(mut self, at: usize) -> $t {
                self.unset_bit(at);
                self
            }
            fn flip_bit(&mut self, at: usize) {
                *self ^= 1 << at
            }
            fn flipped_bit(mut self, at: usize) -> $t {
                self.flip_bit(at);
                self
            }
            fn all_bits(n: usize) -> $t {
                (1 as $t << n).wrapping_sub(1)
            }
            fn iter_all(n: usize) -> RangeInclusive<$t> {
                0..=Self::all_bits(n)
            }
            fn lowest_bit(&self) -> usize {
                assert_ne!(*self, 0);
                self.trailing_zeros() as usize
            }
            fn highest_bit(&self) -> usize {
                assert_ne!(*self, 0);
                Self::BITS as usize - 1 - self.leading_zeros() as usize
            }
            fn is_subset(&self, other: $t) -> bool {
                (*self & other) == *self
            }
        }

        impl Iterator for BitIter<$t> {
            type Item = $t;

            fn next(&mut self) -> Option<Self::Item> {
                if self.ended {
                    return None;
                }
                let res = self.cur;
                if self.cur == 0 {
                    self.ended = true;
                } else {
                    self.cur = (self.cur - 1) & self.all;
                }
                Some(res)
            }
        }
        )+
    };
}

bit_ops!(u8: 8 u16: 16 u32: 32 u64: 64 u128: 128 usize: 64 i8: 8 i16: 16 i32: 32 i64: 64 i128: 128 isize: 64);
