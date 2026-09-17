//! O(1) range minimum over a static array with O(n) words of extra memory:
//! blocks of 64 keep a per-position mask of in-block candidates (monotonic
//! stack), and a sparse table covers whole blocks.

use crate::collections::bounds::clamp;
use std::ops::RangeBounds;

#[derive(Clone)]
pub struct Rmq<T> {
    values: Vec<T>,
    /// `mask[i]`: bit `j` set iff position `block_start + j <= i` is a
    /// candidate minimum for queries ending at `i` within the block.
    mask: Vec<u64>,
    /// `table[k][b]`: argmin over blocks `b .. b + 2^k`.
    table: Vec<Vec<u32>>,
}

impl<T: Copy + Ord> Rmq<T> {
    pub fn new(values: Vec<T>) -> Self {
        let n = values.len();
        let mut mask = vec![0u64; n];
        let mut stack: Vec<usize> = Vec::with_capacity(64);
        for i in 0..n {
            if i % 64 == 0 {
                stack.clear();
                mask[i] = 1;
            } else {
                let mut m = mask[i - 1];
                while let Some(&top) = stack.last() {
                    if values[top] > values[i] {
                        m &= !(1u64 << (top % 64));
                        stack.pop();
                    } else {
                        break;
                    }
                }
                mask[i] = m | (1u64 << (i % 64));
            }
            stack.push(i);
        }
        let blocks = (n + 63) / 64;
        let mut table = Vec::new();
        if blocks > 0 {
            let first: Vec<u32> = (0..blocks)
                .map(|b| {
                    let end = ((b + 1) * 64).min(n);
                    (b * 64 + mask[end - 1].trailing_zeros() as usize) as u32
                })
                .collect();
            table.push(first);
            let mut k = 1;
            while (1 << k) <= blocks {
                let prev = &table[k - 1];
                let half = 1 << (k - 1);
                let cur: Vec<u32> = (0..=blocks - (1 << k))
                    .map(|b| {
                        let (x, y) = (prev[b], prev[b + half]);
                        if values[y as usize] < values[x as usize] {
                            y
                        } else {
                            x
                        }
                    })
                    .collect();
                table.push(cur);
                k += 1;
            }
        }
        Self {
            values,
            mask,
            table,
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn values(&self) -> &[T] {
        &self.values
    }

    #[inline]
    fn in_block(&self, l: usize, r: usize) -> usize {
        // positions l..=r inside one block
        let m = self.mask[r] & (u64::MAX << (l % 64));
        (r & !63) + m.trailing_zeros() as usize
    }

    #[inline]
    fn better(&self, a: usize, b: usize) -> usize {
        if self.values[b] < self.values[a] {
            b
        } else {
            a
        }
    }

    /// Position of the leftmost minimum in the (nonempty) range.
    pub fn argmin(&self, range: impl RangeBounds<usize>) -> usize {
        let (l, to) = clamp(&range, self.values.len());
        assert!(l < to, "empty range");
        let r = to - 1;
        let (bl, br) = (l / 64, r / 64);
        if bl == br {
            return self.in_block(l, r);
        }
        let mut best = self.in_block(l, bl * 64 + 63);
        if bl + 1 < br {
            let count = br - bl - 1;
            let k = usize::BITS as usize - 1 - count.leading_zeros() as usize;
            let a = self.table[k][bl + 1] as usize;
            let b = self.table[k][br - (1 << k)] as usize;
            best = self.better(best, self.better(a, b));
        }
        self.better(best, self.in_block(br * 64, r))
    }

    pub fn min(&self, range: impl RangeBounds<usize>) -> T {
        self.values[self.argmin(range)]
    }
}

#[cfg(test)]
mod tests {
    use super::Rmq;
    use crate::misc::random::{Random, RandomTrait};

    #[test]
    fn matches_naive() {
        let mut rng = Random::new_with_seed(81);
        for n in [1usize, 2, 63, 64, 65, 127, 128, 129, 1000, 5000] {
            let values: Vec<i32> = (0..n).map(|_| rng.gen_range(0..50i32)).collect();
            let rmq = Rmq::new(values.clone());
            for _ in 0..2000 {
                let l = rng.gen_range(0..n);
                let r = rng.gen_range(l..n);
                let expected = (l..=r).min_by_key(|&i| (values[i], i)).unwrap();
                assert_eq!(rmq.argmin(l..=r), expected, "n={n} {l}..={r}");
                assert_eq!(rmq.min(l..r + 1), values[expected]);
            }
            assert_eq!(
                rmq.argmin(..),
                (0..n).min_by_key(|&i| (values[i], i)).unwrap()
            );
        }
    }
}
