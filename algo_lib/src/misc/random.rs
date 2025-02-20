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

    fn gen_bound<T: Rem<Output = T> + Primitive<u64>>(&mut self, n: T) -> T
    where
        u64: Primitive<T>,
    {
        (self.gen_impl() % n.to()).to()
    }

    fn gen_range<T: IntegerSemiRingWithSub + Primitive<u64> + MinMax>(
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
        if f == T::min_val() && t == T::max_val() {
            self.gen_int()
        } else {
            f + self.gen_bound(t - f + T::one())
        }
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

pub trait Shuffle {
    fn shuffle(&mut self);
}

impl<T> Shuffle for [T] {
    fn shuffle(&mut self) {
        for i in self.indices() {
            let at = StaticRandom.gen_bound(i + 1);
            self.swap(i, at);
        }
    }
}
