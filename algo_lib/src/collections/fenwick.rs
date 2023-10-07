use crate::collections::min_max::MinimMaxim;
use crate::collections::slice_ext::legacy_fill::LegacyFill;
use crate::numbers::num_traits::add_sub::AddSub;
use crate::numbers::num_traits::zero_one::ZeroOne;
use std::ops::RangeBounds;

#[derive(Clone)]
pub struct FenwickTree<T> {
    value: Vec<T>,
}

impl<T: AddSub + ZeroOne> FenwickTree<T> {
    pub fn new(size: usize) -> Self {
        Self {
            value: vec![T::zero(); size],
        }
    }

    pub fn get_to(&self, mut to: usize) -> T {
        to.minim(self.value.len());
        let mut result = T::zero();
        while to > 0 {
            to -= 1;
            result += self.value[to];
            to &= to + 1;
        }
        result
    }

    pub fn get(&self, bounds: impl RangeBounds<usize>) -> T {
        let from = match bounds.start_bound() {
            std::ops::Bound::Included(&x) => x,
            std::ops::Bound::Excluded(&x) => x + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let to = match bounds.end_bound() {
            std::ops::Bound::Included(&x) => x + 1,
            std::ops::Bound::Excluded(&x) => x,
            std::ops::Bound::Unbounded => self.value.len(),
        };
        if from >= to {
            T::zero()
        } else {
            self.get_to(to) - self.get_to(from)
        }
    }

    pub fn add(&mut self, mut at: usize, v: T) {
        while at < self.value.len() {
            self.value[at] += v;
            at |= at + 1;
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        self.value
            .iter()
            .enumerate()
            // edition 2021
            .map(move |(i, _)| self.get(i..=i))
    }

    pub fn clear(&mut self) {
        self.value.legacy_fill(T::zero());
    }
}

impl<T: AddSub + ZeroOne> From<&[T]> for FenwickTree<T> {
    fn from(slice: &[T]) -> Self {
        let mut result = Self::new(slice.len());
        for (i, &v) in slice.iter().enumerate() {
            result.add(i, v);
        }
        result
    }
}
