use crate::collections::min_max::MinimMaxim;
use crate::numbers::num_traits::algebra::AdditionMonoidWithSub;
use std::ops::RangeBounds;

#[derive(Clone)]
pub struct FenwickTree<T> {
    value: Vec<T>,
}

/// Walk each cell into its Fenwick parent (`p = i | (i+1)`), applying `op`.
/// Used by both the bottom-up build (forward `+=`) and its inverse
/// (right-to-left `-=`) — they're the same walk in opposite directions.
fn parent_walk<T, F>(value: &mut [T], indices: impl Iterator<Item = usize>, mut op: F)
where
    T: Copy,
    F: FnMut(&mut T, T),
{
    let n = value.len();
    for i in indices {
        let p = i | (i + 1);
        if p < n {
            let v = value[i];
            op(&mut value[p], v);
        }
    }
}

impl<T: AdditionMonoidWithSub + Copy> FenwickTree<T> {
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
            result += unsafe { *self.value.get_unchecked(to) };
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
            unsafe { *self.value.get_unchecked_mut(at) += v };
            at |= at + 1;
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        // Inverse of `From<&[T]>`: same parent walk, reversed, subtracting.
        let mut work = self.value.clone();
        let n = work.len();
        parent_walk(&mut work, (0..n).rev(), |x, v| *x -= v);
        work.into_iter()
    }

    pub fn clear(&mut self) {
        self.value.fill(T::zero());
    }
}

impl<T: AdditionMonoidWithSub + Copy> From<&[T]> for FenwickTree<T> {
    fn from(slice: &[T]) -> Self {
        // O(n) bottom-up build: copy the input, then propagate each cell's
        // value into its Fenwick parent. n-1 additions total.
        let mut value: Vec<T> = slice.to_vec();
        let n = value.len();
        parent_walk(&mut value, 0..n, |x, v| *x += v);
        Self { value }
    }
}
