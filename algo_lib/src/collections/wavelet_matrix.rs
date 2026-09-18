//! Wavelet matrix over a static sequence of `u64` values: k-th smallest,
//! counting and frequency queries on ranges in O(bits) each.

use crate::collections::bounds::clamp;
use std::ops::RangeBounds;

/// Bit vector with O(1) rank via interleaved (bits, prefix count) blocks.
#[derive(Clone)]
struct BitVec {
    /// `blocks[i] = (bits, ones before this block)`
    blocks: Vec<(u64, u32)>,
    zeros: usize,
}

impl BitVec {
    fn new(bits: impl Iterator<Item = bool>, len: usize) -> Self {
        let mut blocks = vec![(0u64, 0u32); len / 64 + 1];
        for (i, b) in bits.enumerate() {
            if b {
                blocks[i / 64].0 |= 1 << (i % 64);
            }
        }
        let mut ones = 0u32;
        for block in blocks.iter_mut() {
            block.1 = ones;
            ones += block.0.count_ones();
        }
        Self {
            blocks,
            zeros: len - ones as usize,
        }
    }

    /// Number of ones in `0..at`.
    #[inline]
    fn rank1(&self, at: usize) -> usize {
        let (bits, before) = self.blocks[at / 64];
        before as usize + (bits & ((1u64 << (at % 64)) - 1)).count_ones() as usize
    }

    #[inline]
    fn rank0(&self, at: usize) -> usize {
        at - self.rank1(at)
    }
}

#[derive(Clone)]
pub struct WaveletMatrix {
    n: usize,
    /// Levels from the most significant bit down.
    levels: Vec<BitVec>,
    bits: u32,
}

impl WaveletMatrix {
    pub fn new(values: &[u64]) -> Self {
        let n = values.len();
        let max = values.iter().copied().max().unwrap_or(0);
        let bits = (64 - max.leading_zeros()).max(1);
        let mut cur = values.to_vec();
        let mut next = vec![0u64; n];
        let mut levels = Vec::with_capacity(bits as usize);
        for level in (0..bits).rev() {
            let bv = BitVec::new(cur.iter().map(|&v| v >> level & 1 == 1), n);
            // stable partition: zeros first, then ones
            let mut zero_at = 0;
            let mut one_at = bv.zeros;
            for &v in &cur {
                if v >> level & 1 == 1 {
                    next[one_at] = v;
                    one_at += 1;
                } else {
                    next[zero_at] = v;
                    zero_at += 1;
                }
            }
            std::mem::swap(&mut cur, &mut next);
            levels.push(bv);
        }
        Self { n, levels, bits }
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// `k`-th smallest (0-based) value in the range, or `None` if `k` is out
    /// of bounds.
    pub fn kth_smallest(&self, range: impl RangeBounds<usize>, mut k: usize) -> Option<u64> {
        let (mut l, mut r) = clamp(&range, self.n);
        if k >= r.saturating_sub(l) {
            return None;
        }
        let mut res = 0u64;
        for (i, bv) in self.levels.iter().enumerate() {
            let level = self.bits - 1 - i as u32;
            let (l0, r0) = (bv.rank0(l), bv.rank0(r));
            let zeros = r0 - l0;
            if k < zeros {
                l = l0;
                r = r0;
            } else {
                k -= zeros;
                res |= 1 << level;
                l = bv.zeros + (l - l0);
                r = bv.zeros + (r - r0);
            }
        }
        Some(res)
    }

    pub fn kth_largest(&self, range: impl RangeBounds<usize>, k: usize) -> Option<u64> {
        let (l, r) = clamp(&range, self.n);
        let len = r.saturating_sub(l);
        if k >= len {
            return None;
        }
        self.kth_smallest(l..r, len - 1 - k)
    }

    /// Number of values in the range that are `< upper`.
    pub fn count_less(&self, range: impl RangeBounds<usize>, upper: u64) -> usize {
        let (mut l, mut r) = clamp(&range, self.n);
        if l >= r {
            return 0;
        }
        if self.bits < 64 && upper >> self.bits != 0 {
            return r - l;
        }
        let mut res = 0;
        for (i, bv) in self.levels.iter().enumerate() {
            let level = self.bits - 1 - i as u32;
            let (l0, r0) = (bv.rank0(l), bv.rank0(r));
            if upper >> level & 1 == 1 {
                res += r0 - l0;
                l = bv.zeros + (l - l0);
                r = bv.zeros + (r - r0);
            } else {
                l = l0;
                r = r0;
            }
        }
        res
    }

    /// Number of values in the range that lie in `lower..upper`.
    pub fn count_between(&self, range: impl RangeBounds<usize>, lower: u64, upper: u64) -> usize {
        if lower >= upper {
            return 0;
        }
        let (l, r) = clamp(&range, self.n);
        self.count_less(l..r, upper) - self.count_less(l..r, lower)
    }

    /// Number of occurrences of `value` in the range.
    pub fn frequency(&self, range: impl RangeBounds<usize>, value: u64) -> usize {
        match value.checked_add(1) {
            Some(next) => self.count_between(range, value, next),
            None => {
                let (l, r) = clamp(&range, self.n);
                r.saturating_sub(l) - self.count_less(l..r, value)
            }
        }
    }

    /// Largest value `< upper` in the range.
    pub fn prev_value(&self, range: impl RangeBounds<usize>, upper: u64) -> Option<u64> {
        let (l, r) = clamp(&range, self.n);
        let c = self.count_less(l..r, upper);
        if c == 0 {
            None
        } else {
            self.kth_smallest(l..r, c - 1)
        }
    }

    /// Smallest value `>= lower` in the range.
    pub fn next_value(&self, range: impl RangeBounds<usize>, lower: u64) -> Option<u64> {
        let (l, r) = clamp(&range, self.n);
        let c = self.count_less(l..r, lower);
        self.kth_smallest(l..r, c)
    }
}

#[cfg(test)]
mod tests {
    use super::WaveletMatrix;
    use crate::misc::random::{Random, RandomTrait};

    #[test]
    fn matches_naive() {
        let mut rng = Random::new_with_seed(121);
        for (n, max) in [
            (1usize, 1u64),
            (2, 1),
            (10, 5),
            (64, 100),
            (65, 3),
            (500, 1_000_000_000),
            (300, u64::MAX),
        ] {
            let values: Vec<u64> = (0..n)
                .map(|_| {
                    if max == u64::MAX {
                        rng.gen_u128() as u64
                    } else {
                        rng.gen_range(0..=max)
                    }
                })
                .collect();
            let wm = WaveletMatrix::new(&values);
            for _ in 0..1500 {
                let l = rng.gen_range(0..=n);
                let r = rng.gen_range(l..=n);
                let mut sorted = values[l..r].to_vec();
                sorted.sort();
                let k = rng.gen_range(0..=(r - l));
                assert_eq!(
                    wm.kth_smallest(l..r, k),
                    sorted.get(k).copied(),
                    "kth n={n} {l}..{r} k={k}"
                );
                assert_eq!(wm.kth_largest(l..r, k), sorted.iter().rev().nth(k).copied());
                let x = if rng.gen_bool() && !values.is_empty() {
                    values[rng.gen_range(0..n)]
                } else if max == u64::MAX {
                    rng.gen_u128() as u64
                } else {
                    rng.gen_range(0..=max + 2)
                };
                let y = x.saturating_add(rng.gen_range(0..10u64));
                assert_eq!(
                    wm.count_less(l..r, x),
                    sorted.iter().filter(|&&v| v < x).count(),
                    "count_less {x}"
                );
                assert_eq!(
                    wm.count_between(l..r, x, y),
                    sorted.iter().filter(|&&v| x <= v && v < y).count()
                );
                assert_eq!(
                    wm.frequency(l..r, x),
                    sorted.iter().filter(|&&v| v == x).count()
                );
                assert_eq!(
                    wm.prev_value(l..r, x),
                    sorted.iter().rev().find(|&&v| v < x).copied()
                );
                assert_eq!(
                    wm.next_value(l..r, x),
                    sorted.iter().find(|&&v| v >= x).copied()
                );
            }
        }
        let empty = WaveletMatrix::new(&[]);
        assert_eq!(empty.kth_smallest(.., 0), None);
        assert_eq!(empty.count_less(.., 5), 0);
    }
}
