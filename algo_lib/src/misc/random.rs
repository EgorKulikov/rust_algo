use crate::collections::slice_ext::indices::Indices;
use crate::numbers::num_traits::algebra::IntegerSemiRingWithSub;
use crate::numbers::num_traits::ord::MinMax;
use crate::numbers::num_traits::primitive::Primitive;
// use crate::numbers::real::{IntoReal, Real};
use std::cell::RefCell;
use std::ops::{RangeBounds, Rem};
use std::time::SystemTime;

pub trait RandomTrait {
    fn gen_impl(&mut self) -> u64;

    fn gen_int<T>(&mut self) -> T
    where
        u64: Primitive<T>,
    {
        self.gen_impl().to()
    }

    fn gen_u128(&mut self) -> u128 {
        (self.gen_impl() as u128) << 64 | self.gen_impl() as u128
    }

    fn gen_i128(&mut self) -> i128 {
        self.gen_u128() as i128
    }

    fn gen_bool(&mut self) -> bool {
        (self.gen_impl() & 1) == 1
    }

    // fn gen_real(&mut self) -> Real {
    //     self.gen_impl().into_real() / u64::MAX
    // }

    /// Uniform-ish value in `0..n`. Types of up to 64 bits consume one
    /// `gen_impl` value, 128-bit types two.
    fn gen_bound<T: Rem<Output = T> + Primitive<u64> + Primitive<u128>>(&mut self, n: T) -> T
    where
        u64: Primitive<T>,
    {
        let n: u128 = Primitive::<u128>::to(n);
        self.gen_below(n, std::mem::size_of::<T>() > 8)
    }

    /// `gen_bound` on the `u128` image of the bound.
    #[doc(hidden)]
    fn gen_below<T: Primitive<u128>>(&mut self, n: u128, wide: bool) -> T {
        let value = if wide {
            self.gen_u128() % n
        } else {
            (self.gen_impl() % n as u64) as u128
        };
        Primitive::<u128>::from(value)
    }

    fn gen_range<T: IntegerSemiRingWithSub + Primitive<u64> + Primitive<u128> + MinMax>(
        &mut self,
        range: impl RangeBounds<T>,
    ) -> T
    where
        u64: Primitive<T>,
    {
        let f = match range.start_bound() {
            std::ops::Bound::Included(&s) => s,
            std::ops::Bound::Excluded(&s) => s + T::one(),
            std::ops::Bound::Unbounded => T::min_val(),
        };
        let t = match range.end_bound() {
            std::ops::Bound::Included(&e) => e,
            std::ops::Bound::Excluded(&e) => e - T::one(),
            std::ops::Bound::Unbounded => T::max_val(),
        };
        let wide = std::mem::size_of::<T>() > 8;
        if f == T::min_val() && t == T::max_val() {
            return if wide {
                Primitive::<u128>::from(self.gen_u128())
            } else {
                self.gen_int()
            };
        }
        // `t - f + 1` does not fit in a signed `T` once the range spans half of
        // the type, so work on the sign-extended `u128` images: the wrapping
        // difference is the true span, and the final cast wraps back into `T`.
        let from: u128 = Primitive::<u128>::to(f);
        let to: u128 = Primitive::<u128>::to(t);
        let span = to.wrapping_sub(from).wrapping_add(1);
        let offset: u128 = self.gen_below(span, wide);
        Primitive::<u128>::from(from.wrapping_add(offset))
    }
}

const NN: usize = 312;
const MM: usize = 156;
const MATRIX_A: u64 = 0xB5026F5AA96619E9;
const UM: u64 = 0xFFFFFFFF80000000;
const LM: u64 = 0x7FFFFFFF;
const F: u64 = 6364136223846793005;
const MAG01: [u64; 2] = [0, MATRIX_A];

pub struct Random {
    mt: [u64; NN],
    index: usize,
}

impl Random {
    pub fn new() -> Self {
        Self::new_with_seed(
            (SystemTime::UNIX_EPOCH.elapsed().unwrap().as_nanos() & 0xFFFFFFFFFFFFFFFF) as u64,
        )
    }

    pub fn new_with_seed(seed: u64) -> Self {
        let mut res = Self {
            mt: [0u64; NN],
            index: NN,
        };
        res.mt[0] = seed;
        for i in 1..NN {
            res.mt[i] = F
                .wrapping_mul(res.mt[i - 1] ^ (res.mt[i - 1] >> 62))
                .wrapping_add(i as u64);
        }
        res
    }
}

impl Default for Random {
    fn default() -> Self {
        Self::new()
    }
}

impl RandomTrait for Random {
    fn gen_impl(&mut self) -> u64 {
        if self.index == NN {
            for i in 0..(NN - MM) {
                let x = (self.mt[i] & UM) | (self.mt[i + 1] & LM);
                self.mt[i] = self.mt[i + MM] ^ (x >> 1) ^ MAG01[(x & 1) as usize];
            }
            for i in (NN - MM)..(NN - 1) {
                let x = (self.mt[i] & UM) | (self.mt[i + 1] & LM);
                self.mt[i] = self.mt[i + MM - NN] ^ (x >> 1) ^ MAG01[(x & 1) as usize];
            }
            let x = (self.mt[NN - 1] & UM) | (self.mt[0] & LM);
            self.mt[NN - 1] = self.mt[MM - 1] ^ (x >> 1) ^ MAG01[(x & 1) as usize];
            self.index = 0;
        }
        let mut x = self.mt[self.index];
        self.index += 1;
        x ^= (x >> 29) & 0x5555555555555555;
        x ^= (x << 17) & 0x71D67FFFEDA60000;
        x ^= (x << 37) & 0xFFF7EEE000000000;
        x ^= x >> 43;
        x
    }
}

thread_local! {
    // static RANDOM: RefCell<Random> = RefCell::new(Random::new_with_seed(33));
    static RANDOM: RefCell<Random> = RefCell::new(Random::new());
}

pub struct StaticRandom;

impl RandomTrait for StaticRandom {
    fn gen_impl(&mut self) -> u64 {
        // 1.73
        RANDOM.with(|r| r.borrow_mut().gen_impl())
        // RANDOM.with_borrow_mut(|r| r.gen_impl())
    }
}

pub trait Shuffle<T> {
    fn shuffle(&mut self) {
        self.shuffle_with(&mut StaticRandom);
    }
    fn shuffle_with(&mut self, rng: &mut impl RandomTrait);
    fn choice(&self) -> &T {
        self.choice_with(&mut StaticRandom)
    }
    fn choice_with(&self, rng: &mut impl RandomTrait) -> &T;
}

impl<T> Shuffle<T> for [T] {
    fn shuffle_with(&mut self, rng: &mut impl RandomTrait) {
        for i in self.indices() {
            let at = rng.gen_bound(i + 1);
            self.swap(i, at);
        }
    }
    fn choice_with(&self, rng: &mut impl RandomTrait) -> &T {
        let index = rng.gen_bound(self.len());
        &self[index]
    }
}

#[cfg(test)]
mod tests {
    use super::{Random, RandomTrait};

    #[test]
    fn gen_range_over_wide_signed_ranges() {
        let mut r = Random::new_with_seed(1);
        let (mut low, mut high) = (false, false);
        for _ in 0..1000 {
            let x: i32 = r.gen_range(-2_000_000_000..=2_000_000_000);
            assert!((-2_000_000_000..=2_000_000_000).contains(&x));
            low |= x < -1_000_000_000;
            high |= x > 1_000_000_000;
            let y: i32 = r.gen_range(0..);
            assert!(y >= 0);
            let z: i64 = r.gen_range(..0);
            assert!(z < 0);
            let w: i8 = r.gen_range(-128..=126);
            assert!(w <= 126);
        }
        assert!(low && high);
    }

    #[test]
    fn gen_range_keeps_the_sequence_of_narrow_ranges() {
        let mut a = Random::new_with_seed(7);
        let mut b = Random::new_with_seed(7);
        for _ in 0..100 {
            let x: i64 = a.gen_range(-5..=20);
            assert_eq!(x, -5 + (b.gen_impl() % 26) as i64);
        }
    }

    #[test]
    fn gen_bound_and_gen_range_with_128_bit_bounds() {
        let mut r = Random::new_with_seed(1);
        let n = (1u128 << 64) + 5;
        let mut above = false;
        for _ in 0..1000 {
            let x = r.gen_bound(n);
            assert!(x < n);
            above |= x >= 5;
            assert!(r.gen_bound(1u128 << 64) < 1u128 << 64);
            let y: i128 = r.gen_range(-(1i128 << 100)..1i128 << 100);
            assert!((-(1i128 << 100)..1i128 << 100).contains(&y));
        }
        assert!(above);
        let wide = (0..100).any(|_| r.gen_range::<i128>(..).unsigned_abs() > 1u128 << 70);
        assert!(wide);
    }
}
