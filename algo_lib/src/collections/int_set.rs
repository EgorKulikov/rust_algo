//! Set of integers in `0..n` as a hierarchy of 64-ary bitmaps: every
//! operation, including predecessor and successor queries, walks at most
//! `log_64 n` words.

#[derive(Clone)]
pub struct IntSet {
    n: usize,
    len: usize,
    /// `levels[0]` has one bit per element; `levels[k + 1]` has one bit per
    /// nonzero word of `levels[k]`. The top level is a single word.
    levels: Vec<Vec<u64>>,
}

impl IntSet {
    pub fn new(n: usize) -> Self {
        let mut levels = Vec::new();
        let mut size = n.max(1);
        loop {
            let words = size.div_ceil(64);
            levels.push(vec![0u64; words]);
            if words == 1 {
                break;
            }
            size = words;
        }
        Self { n, len: 0, levels }
    }

    pub fn capacity(&self) -> usize {
        self.n
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn contains(&self, x: usize) -> bool {
        x < self.n && self.levels[0][x >> 6] >> (x & 63) & 1 == 1
    }

    /// Returns whether `x` was newly inserted.
    pub fn insert(&mut self, mut x: usize) -> bool {
        assert!(x < self.n);
        if self.contains(x) {
            return false;
        }
        self.len += 1;
        for level in self.levels.iter_mut() {
            let word = &mut level[x >> 6];
            let was_empty = *word == 0;
            *word |= 1 << (x & 63);
            if !was_empty {
                break;
            }
            x >>= 6;
        }
        true
    }

    /// Returns whether `x` was present.
    pub fn remove(&mut self, mut x: usize) -> bool {
        if !self.contains(x) {
            return false;
        }
        self.len -= 1;
        for level in self.levels.iter_mut() {
            let word = &mut level[x >> 6];
            *word &= !(1 << (x & 63));
            if *word != 0 {
                break;
            }
            x >>= 6;
        }
        true
    }

    /// Smallest element `>= x`.
    pub fn next(&self, x: usize) -> Option<usize> {
        if x >= self.n {
            return None;
        }
        let mut x = x;
        let mut level = 0;
        loop {
            if level == self.levels.len() {
                return None;
            }
            let words = &self.levels[level];
            let (w, b) = (x >> 6, x & 63);
            if w >= words.len() {
                return None;
            }
            let masked = words[w] & (u64::MAX << b);
            if masked != 0 {
                x = (w << 6) | masked.trailing_zeros() as usize;
                break;
            }
            // Nothing in this word: continue with the next word one level up.
            x = w + 1;
            level += 1;
        }
        while level > 0 {
            level -= 1;
            x = (x << 6) | self.levels[level][x].trailing_zeros() as usize;
        }
        Some(x)
    }

    /// Largest element `<= x`.
    pub fn prev(&self, x: usize) -> Option<usize> {
        let mut x = x.min(self.n.saturating_sub(1));
        if self.n == 0 {
            return None;
        }
        let mut level = 0;
        loop {
            if level == self.levels.len() {
                return None;
            }
            let words = &self.levels[level];
            let (w, b) = (x >> 6, x & 63);
            let masked = words[w] & (u64::MAX >> (63 - b));
            if masked != 0 {
                x = (w << 6) | (63 - masked.leading_zeros() as usize);
                break;
            }
            if w == 0 {
                return None;
            }
            x = w - 1;
            level += 1;
        }
        while level > 0 {
            level -= 1;
            x = (x << 6) | (63 - self.levels[level][x].leading_zeros() as usize);
        }
        Some(x)
    }

    pub fn min(&self) -> Option<usize> {
        self.next(0)
    }

    pub fn max(&self) -> Option<usize> {
        self.prev(usize::MAX)
    }

    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        let mut cur = self.next(0);
        std::iter::from_fn(move || {
            let x = cur?;
            cur = self.next(x + 1);
            Some(x)
        })
    }

    pub fn clear(&mut self) {
        for level in self.levels.iter_mut() {
            level.fill(0);
        }
        self.len = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::IntSet;
    use crate::misc::random::{Random, RandomTrait};
    use std::collections::BTreeSet;

    #[test]
    fn matches_btreeset() {
        let mut rng = Random::new_with_seed(71);
        for n in [1usize, 2, 63, 64, 65, 4096, 4097, 300_000] {
            let mut set = IntSet::new(n);
            let mut reference = BTreeSet::new();
            assert_eq!(set.next(0), None);
            assert_eq!(set.prev(n), None);
            for _ in 0..3000 {
                let x = rng.gen_range(0..n);
                match rng.gen_range(0..5u32) {
                    0 | 1 => assert_eq!(set.insert(x), reference.insert(x)),
                    2 => assert_eq!(set.remove(x), reference.remove(&x)),
                    3 => {
                        assert_eq!(
                            set.next(x),
                            reference.range(x..).next().copied(),
                            "next({x}) n={n}"
                        );
                        assert_eq!(
                            set.prev(x),
                            reference.range(..=x).next_back().copied(),
                            "prev({x}) n={n}"
                        );
                    }
                    _ => {
                        assert_eq!(set.contains(x), reference.contains(&x));
                        assert_eq!(set.len(), reference.len());
                        assert_eq!(set.min(), reference.iter().next().copied());
                        assert_eq!(set.max(), reference.iter().next_back().copied());
                    }
                }
            }
            assert_eq!(
                set.iter().collect::<Vec<_>>(),
                reference.iter().copied().collect::<Vec<_>>()
            );
            assert_eq!(set.next(n), None);
            assert_eq!(set.next(n + 5), None);
            assert_eq!(set.prev(usize::MAX), reference.iter().next_back().copied());
            set.clear();
            assert!(set.is_empty() && set.min().is_none() && set.max().is_none());
        }
    }
}
