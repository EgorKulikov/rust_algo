//! Non-recursive (bottom-up) segment trees over a power-of-two array.
//!
//! [`SegTree`] supports point set and range fold of a monoid;
//! [`LazySegTree`] adds range apply of a family of maps. Range operations
//! on the lazy tree never push tags: every node stores its own tag applied
//! to the fold of its children, and queries apply the tags of the ancestors
//! to the partial results on the way up.

use crate::collections::bounds::clamp;
use std::ops::RangeBounds;

pub trait Monoid {
    type V: Copy;
    fn e() -> Self::V;
    fn op(a: Self::V, b: Self::V) -> Self::V;
}

pub trait LazyMonoid: Monoid {
    type F: Copy;
    fn id() -> Self::F;
    /// Applies `f` to the fold of a whole segment (so `V` must carry whatever
    /// `f` needs, such as the segment length).
    fn mapping(f: Self::F, v: Self::V) -> Self::V;
    /// `f` applied after `g`.
    fn composition(f: Self::F, g: Self::F) -> Self::F;
}

#[derive(Clone)]
pub struct SegTree<M: Monoid> {
    n: usize,
    size: usize,
    d: Vec<M::V>,
}

impl<M: Monoid> SegTree<M> {
    pub fn new(n: usize) -> Self {
        Self::with_gen(n, |_| M::e())
    }

    pub fn from_slice(a: &[M::V]) -> Self {
        Self::with_gen(a.len(), |i| a[i])
    }

    pub fn with_gen(n: usize, mut f: impl FnMut(usize) -> M::V) -> Self {
        let size = n.next_power_of_two().max(1);
        let mut d = vec![M::e(); 2 * size];
        for i in 0..n {
            d[size + i] = f(i);
        }
        for i in (1..size).rev() {
            d[i] = M::op(d[2 * i], d[2 * i + 1]);
        }
        Self { n, size, d }
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn get(&self, at: usize) -> M::V {
        assert!(at < self.n);
        self.d[at + self.size]
    }

    pub fn set(&mut self, at: usize, v: M::V) {
        self.update(at, |x| *x = v);
    }

    pub fn update(&mut self, at: usize, f: impl FnOnce(&mut M::V)) {
        assert!(at < self.n);
        let mut i = at + self.size;
        f(&mut self.d[i]);
        while i > 1 {
            i >>= 1;
            self.d[i] = M::op(self.d[2 * i], self.d[2 * i + 1]);
        }
    }

    /// Fold over the range; `e()` for an empty range.
    pub fn query(&self, range: impl RangeBounds<usize>) -> M::V {
        let (from, to) = clamp(&range, self.n);
        let (mut l, mut r) = (from + self.size, to + self.size);
        let (mut sml, mut smr) = (M::e(), M::e());
        while l < r {
            if l & 1 == 1 {
                sml = M::op(sml, self.d[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                smr = M::op(self.d[r], smr);
            }
            l >>= 1;
            r >>= 1;
        }
        M::op(sml, smr)
    }

    pub fn all(&self) -> M::V {
        self.d[1]
    }

    /// Largest `r` such that `pred(query(l..r))` holds; `pred(e())` must be true.
    pub fn max_right(&self, l: usize, mut pred: impl FnMut(M::V) -> bool) -> usize {
        assert!(l <= self.n);
        if l == self.n {
            return self.n;
        }
        let mut l = l + self.size;
        let mut sm = M::e();
        loop {
            while l % 2 == 0 {
                l >>= 1;
            }
            if !pred(M::op(sm, self.d[l])) {
                while l < self.size {
                    l *= 2;
                    let cand = M::op(sm, self.d[l]);
                    if pred(cand) {
                        sm = cand;
                        l += 1;
                    }
                }
                return (l - self.size).min(self.n);
            }
            sm = M::op(sm, self.d[l]);
            l += 1;
            if l & l.wrapping_neg() == l {
                return self.n;
            }
        }
    }

    /// Smallest `l` such that `pred(query(l..r))` holds; `pred(e())` must be true.
    pub fn min_left(&self, r: usize, mut pred: impl FnMut(M::V) -> bool) -> usize {
        assert!(r <= self.n);
        if r == 0 {
            return 0;
        }
        let mut r = r + self.size;
        let mut sm = M::e();
        loop {
            r -= 1;
            while r > 1 && r % 2 == 1 {
                r >>= 1;
            }
            if !pred(M::op(self.d[r], sm)) {
                while r < self.size {
                    r = 2 * r + 1;
                    let cand = M::op(self.d[r], sm);
                    if pred(cand) {
                        sm = cand;
                        r -= 1;
                    }
                }
                return r + 1 - self.size;
            }
            sm = M::op(self.d[r], sm);
            if r & r.wrapping_neg() == r {
                return 0;
            }
        }
    }
}

#[derive(Clone)]
pub struct LazySegTree<M: LazyMonoid> {
    n: usize,
    size: usize,
    log: u32,
    d: Vec<M::V>,
    /// Tags of internal nodes only (`lz[k]` for `k < size`).
    lz: Vec<M::F>,
}

impl<M: LazyMonoid> LazySegTree<M> {
    pub fn new(n: usize) -> Self {
        Self::with_gen(n, |_| M::e())
    }

    pub fn from_slice(a: &[M::V]) -> Self {
        Self::with_gen(a.len(), |i| a[i])
    }

    pub fn with_gen(n: usize, mut f: impl FnMut(usize) -> M::V) -> Self {
        let size = n.next_power_of_two().max(1);
        let mut d = vec![M::e(); 2 * size];
        for i in 0..n {
            d[size + i] = f(i);
        }
        for i in (1..size).rev() {
            d[i] = M::op(d[2 * i], d[2 * i + 1]);
        }
        Self {
            n,
            size,
            log: size.trailing_zeros(),
            d,
            lz: vec![M::id(); size],
        }
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    #[inline]
    fn recompute(&mut self, k: usize) {
        self.d[k] = M::mapping(self.lz[k], M::op(self.d[2 * k], self.d[2 * k + 1]));
    }

    #[inline]
    fn all_apply(&mut self, k: usize, f: M::F) {
        self.d[k] = M::mapping(f, self.d[k]);
        if k < self.size {
            self.lz[k] = M::composition(f, self.lz[k]);
        }
    }

    fn push(&mut self, k: usize) {
        let f = self.lz[k];
        self.all_apply(2 * k, f);
        self.all_apply(2 * k + 1, f);
        self.lz[k] = M::id();
    }

    pub fn get(&self, at: usize) -> M::V {
        assert!(at < self.n);
        let mut k = at + self.size;
        let mut v = self.d[k];
        k >>= 1;
        while k >= 1 {
            v = M::mapping(self.lz[k], v);
            k >>= 1;
        }
        v
    }

    pub fn set(&mut self, at: usize, v: M::V) {
        self.update(at, |x| *x = v);
    }

    /// Mutates the element in place (its value with all tags applied).
    pub fn update(&mut self, at: usize, f: impl FnOnce(&mut M::V)) {
        assert!(at < self.n);
        let k = at + self.size;
        for i in (1..=self.log).rev() {
            self.push(k >> i);
        }
        f(&mut self.d[k]);
        for i in 1..=self.log {
            self.recompute(k >> i);
        }
    }

    /// Fold over the range; `e()` for an empty range.
    pub fn query(&self, range: impl RangeBounds<usize>) -> M::V {
        let (from, to) = clamp(&range, self.n);
        if from >= to {
            return M::e();
        }
        let (mut l, mut r) = (from + self.size, to + self.size);
        let (mut sml, mut smr) = (M::e(), M::e());
        let (mut has_l, mut has_r) = (false, false);
        while l < r {
            if l & 1 == 1 {
                sml = M::op(sml, self.d[l]);
                has_l = true;
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                smr = M::op(self.d[r], smr);
                has_r = true;
            }
            l >>= 1;
            r >>= 1;
            // Everything folded so far on the left lies under node `l - 1`,
            // everything on the right under node `r`.
            if has_l {
                sml = M::mapping(self.lz[l - 1], sml);
            }
            if has_r {
                smr = M::mapping(self.lz[r], smr);
            }
        }
        if has_l {
            let mut k = (l - 1) >> 1;
            while k >= 1 {
                sml = M::mapping(self.lz[k], sml);
                k >>= 1;
            }
        }
        if has_r {
            let mut k = r >> 1;
            while k >= 1 {
                smr = M::mapping(self.lz[k], smr);
                k >>= 1;
            }
        }
        M::op(sml, smr)
    }

    pub fn all(&self) -> M::V {
        self.d[1]
    }

    pub fn apply(&mut self, range: impl RangeBounds<usize>, f: M::F) {
        let (from, to) = clamp(&range, self.n);
        if from >= to {
            return;
        }
        let (l0, r0) = (from + self.size, to + self.size);
        // Maps need not commute, so pending tags above the boundary paths
        // are pushed down first; queries never need this.
        for i in (1..=self.log).rev() {
            if (l0 >> i) << i != l0 {
                self.push(l0 >> i);
            }
            if (r0 >> i) << i != r0 {
                self.push((r0 - 1) >> i);
            }
        }
        let (mut l, mut r) = (l0, r0);
        while l < r {
            if l & 1 == 1 {
                self.all_apply(l, f);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                self.all_apply(r, f);
            }
            l >>= 1;
            r >>= 1;
        }
        for i in 1..=self.log {
            if (l0 >> i) << i != l0 {
                self.recompute(l0 >> i);
            }
            if (r0 >> i) << i != r0 {
                self.recompute((r0 - 1) >> i);
            }
        }
    }

    pub fn apply_at(&mut self, at: usize, f: M::F) {
        self.apply(at..at + 1, f);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::misc::random::{Random, RandomTrait};

    const P: u64 = 998_244_353;

    /// Affine maps x -> a x + b composed left to right (non-commutative).
    struct Affine;
    impl Monoid for Affine {
        type V = (u64, u64);
        fn e() -> (u64, u64) {
            (1, 0)
        }
        fn op(a: (u64, u64), b: (u64, u64)) -> (u64, u64) {
            (a.0 * b.0 % P, (a.1 * b.0 + b.1) % P)
        }
    }

    /// (sum, len) with range affine updates.
    struct SumAffine;
    impl Monoid for SumAffine {
        type V = (u64, u64);
        fn e() -> (u64, u64) {
            (0, 0)
        }
        fn op(a: (u64, u64), b: (u64, u64)) -> (u64, u64) {
            ((a.0 + b.0) % P, a.1 + b.1)
        }
    }
    impl LazyMonoid for SumAffine {
        type F = (u64, u64);
        fn id() -> (u64, u64) {
            (1, 0)
        }
        fn mapping(f: (u64, u64), v: (u64, u64)) -> (u64, u64) {
            ((f.0 * v.0 + f.1 * v.1) % P, v.1)
        }
        fn composition(f: (u64, u64), g: (u64, u64)) -> (u64, u64) {
            (f.0 * g.0 % P, (f.0 * g.1 + f.1) % P)
        }
    }

    struct Max;
    impl Monoid for Max {
        type V = i64;
        fn e() -> i64 {
            i64::MIN
        }
        fn op(a: i64, b: i64) -> i64 {
            a.max(b)
        }
    }

    #[test]
    fn point_set_range_composite_vs_naive() {
        let mut rng = Random::new_with_seed(21);
        for n in [1usize, 2, 3, 5, 8, 17, 64, 100] {
            let mut arr: Vec<(u64, u64)> = (0..n)
                .map(|_| (rng.gen_range(1..P), rng.gen_range(0..P)))
                .collect();
            let mut st = SegTree::<Affine>::from_slice(&arr);
            for _ in 0..500 {
                if rng.gen_bool() {
                    let i = rng.gen_range(0..n);
                    let v = (rng.gen_range(1..P), rng.gen_range(0..P));
                    arr[i] = v;
                    st.set(i, v);
                    assert_eq!(st.get(i), v);
                } else {
                    let l = rng.gen_range(0..=n);
                    let r = rng.gen_range(l..=n);
                    let expected = arr[l..r]
                        .iter()
                        .fold(Affine::e(), |acc, &x| Affine::op(acc, x));
                    assert_eq!(st.query(l..r), expected);
                }
            }
            assert_eq!(st.all(), st.query(..));
        }
    }

    #[test]
    fn max_right_and_min_left() {
        let mut rng = Random::new_with_seed(22);
        for n in [1usize, 2, 7, 16, 33, 100] {
            let arr: Vec<i64> = (0..n).map(|_| rng.gen_range(0..20i64)).collect();
            let st = SegTree::<Max>::from_slice(&arr);
            for _ in 0..300 {
                let bound = rng.gen_range(0..25i64);
                let l = rng.gen_range(0..=n);
                let expected = (l..=n)
                    .rev()
                    .find(|&r| arr[l..r].iter().all(|&x| x < bound))
                    .unwrap();
                assert_eq!(
                    st.max_right(l, |v| v < bound),
                    expected,
                    "n={n} l={l} bound={bound}"
                );
                let r = rng.gen_range(0..=n);
                let expected = (0..=r)
                    .find(|&l| arr[l..r].iter().all(|&x| x < bound))
                    .unwrap();
                assert_eq!(
                    st.min_left(r, |v| v < bound),
                    expected,
                    "n={n} r={r} bound={bound}"
                );
            }
        }
    }

    #[test]
    fn lazy_short_sequences_on_tiny_trees() {
        let mut rng = Random::new_with_seed(99);
        for len in 1..=6 {
            for _ in 0..3000 {
                let n = rng.gen_range(1..=5usize);
                let mut arr: Vec<u64> = (0..n).map(|_| rng.gen_range(0..10u64)).collect();
                let mut st = LazySegTree::<SumAffine>::with_gen(n, |i| (arr[i], 1));
                for _ in 0..len {
                    match rng.gen_range(0..3u32) {
                        0 => {
                            let l = rng.gen_range(0..=n);
                            let r = rng.gen_range(l..=n);
                            let f = (rng.gen_range(1..3u64), rng.gen_range(0..3u64));
                            for x in &mut arr[l..r] {
                                *x = (f.0 * *x + f.1) % P;
                            }
                            st.apply(l..r, f);
                        }
                        1 => {
                            let i = rng.gen_range(0..n);
                            let v = rng.gen_range(0..10u64);
                            arr[i] = v;
                            st.set(i, (v, 1));
                        }
                        _ => {
                            let l = rng.gen_range(0..=n);
                            let r = rng.gen_range(l..=n);
                            let expected = arr[l..r].iter().fold(0, |acc, &x| (acc + x) % P);
                            assert_eq!(
                                st.query(l..r),
                                (expected, (r - l) as u64),
                                "n={n} {l}..{r}"
                            );
                        }
                    }
                    for (i, &x) in arr.iter().enumerate() {
                        assert_eq!(st.get(i), (x, 1), "n={n} get({i})");
                    }
                }
            }
        }
    }

    #[test]
    fn lazy_range_affine_range_sum_vs_naive() {
        let mut rng = Random::new_with_seed(23);
        for n in [1usize, 2, 3, 6, 8, 13, 64, 100, 129] {
            let mut arr: Vec<u64> = (0..n).map(|_| rng.gen_range(0..P)).collect();
            let mut st = LazySegTree::<SumAffine>::with_gen(n, |i| (arr[i], 1));
            for _ in 0..600 {
                match rng.gen_range(0..4u32) {
                    0 => {
                        let l = rng.gen_range(0..=n);
                        let r = rng.gen_range(l..=n);
                        let f = (rng.gen_range(0..P), rng.gen_range(0..P));
                        for x in &mut arr[l..r] {
                            *x = (f.0 * *x + f.1) % P;
                        }
                        st.apply(l..r, f);
                    }
                    1 => {
                        let l = rng.gen_range(0..=n);
                        let r = rng.gen_range(l..=n);
                        let expected = arr[l..r].iter().fold(0, |acc, &x| (acc + x) % P);
                        assert_eq!(st.query(l..r), (expected, (r - l) as u64), "n={n} {l}..{r}");
                    }
                    2 => {
                        let i = rng.gen_range(0..n);
                        let v = rng.gen_range(0..P);
                        arr[i] = v;
                        st.set(i, (v, 1));
                    }
                    _ => {
                        let i = rng.gen_range(0..n);
                        assert_eq!(st.get(i), (arr[i], 1));
                        st.update(i, |x| x.0 = (x.0 + 1) % P);
                        arr[i] = (arr[i] + 1) % P;
                    }
                }
            }
            let total = arr.iter().fold(0, |acc, &x| (acc + x) % P);
            assert_eq!(st.all(), (total, n as u64));
            assert_eq!(st.query(..), (total, n as u64));
        }
    }
}
