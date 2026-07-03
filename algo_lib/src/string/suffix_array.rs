use crate::collections::bounds::clamp;
use crate::collections::slice_ext::permutation::Permutation;
use crate::collections::sparse_table::SparseTable;
use crate::collections::vec_ext::gen_vec::VecGen;
use crate::misc::owned_cell::OwnedCell;
use crate::numbers::num_traits::algebra::Zero;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::iter::Cloned;
use std::ops::{Index, RangeBounds};
use std::slice::Iter;

pub struct SuffixArray<T> {
    str: Vec<T>,
    sorted_suffixes: Vec<usize>,
    pos_in_sorted: Vec<usize>,
    lcp: Vec<u32>,
    lcp_sparse_table: OwnedCell<Option<SparseTable<u32>>>,
}

impl<T: Ord> SuffixArray<T> {
    pub fn get_pos_in_array(&self, pos_in_string: usize) -> usize {
        self.pos_in_sorted[pos_in_string]
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.sorted_suffixes.len()
    }

    fn lcp_sparse_table(&self) -> &SparseTable<u32> {
        unsafe {
            if self.lcp_sparse_table.as_ref().is_none() {
                *self.lcp_sparse_table.as_mut() = Some(SparseTable::new(self.lcp.clone(), ref_min));
            }
            self.lcp_sparse_table.as_ref().as_ref().unwrap()
        }
    }

    pub fn lcp(&self, p1: usize, p2: usize) -> usize {
        let min_pos = p1.min(p2);
        let max_pos = p1.max(p2);
        if min_pos == max_pos {
            return self.len() - self[max_pos] - 1;
        }
        if min_pos + 1 == max_pos {
            return self.lcp[min_pos] as usize;
        }
        let lcp_table = self.lcp_sparse_table();
        lcp_table.query(min_pos..max_pos) as usize
    }

    fn build_lcp(str: &[T], sorted_suffixes: &[usize], pos_in_sorted: &[usize]) -> Vec<u32> {
        let n = str.len();
        let mut lcp = vec![0; n - 1];
        let mut k = 0usize;
        for i in 0..n {
            k = k.saturating_sub(1);
            if pos_in_sorted[i] == n - 1 {
                k = 0;
                continue;
            }
            let j = sorted_suffixes[pos_in_sorted[i] + 1];
            while i + k < n && j + k < n && str[i + k] == str[j + k] {
                k += 1;
            }
            lcp[pos_in_sorted[i]] = k as u32
        }
        lcp
    }

    pub fn new(str: &[T]) -> Self
    where
        T: Zero + Ord + Debug + Copy,
    {
        let mut str = str.to_vec();
        str.push(T::zero());
        let n = str.len();
        // Rank-compress to a dense integer alphabet; the appended sentinel gets
        // rank 0, strictly smaller than everything else.
        let mut order = Vec::with_gen(n - 1, |x| x);
        order.sort_unstable_by_key(|&id| str[id]);
        let mut ranks = vec![0u32; n];
        let mut num_ranks = 1u32;
        for (pos, win) in order.windows(2).enumerate() {
            if pos == 0 {
                ranks[win[0]] = 1;
            }
            if str[win[1]] != str[win[0]] {
                num_ranks += 1;
            }
            ranks[win[1]] = num_ranks;
        }
        if n == 2 {
            ranks[order[0]] = 1;
        }
        let sorted_suffixes = sais(&ranks, num_ranks as usize + 1);
        let pos_in_sorted = sorted_suffixes.inv();
        let lcp = Self::build_lcp(&str, &sorted_suffixes, &pos_in_sorted);
        if cfg!(debug_assertions) {
            // too slow for debug mode?
            for (w, &lcp) in sorted_suffixes.windows(2).zip(lcp.iter()) {
                let first = &str[w[0]..];
                let second = &str[w[1]..];
                assert!(
                    first < second,
                    "[{} -> {:?}] not less than [{} -> {:?}]",
                    w[0],
                    &str[w[0]..],
                    w[1],
                    &str[w[1]..]
                );
                let lcp = lcp as usize;
                assert!(first[0..lcp] == second[0..lcp]);
                assert_ne!(first.get(lcp), second.get(lcp));
            }
        }
        Self {
            str,
            sorted_suffixes,
            pos_in_sorted,
            lcp,
            lcp_sparse_table: OwnedCell::new(None),
        }
    }

    pub fn find(&self, t: &[T]) -> (usize, usize) {
        let mut l = 0;
        let mut r = self.len() - 1;
        while l < r {
            let mid = (l + r + 1) / 2;
            if &self.str[self[mid]..] < t {
                l = mid;
            } else {
                r = mid - 1;
            }
        }
        let from = l + 1;
        let mut r = self.len() - 1;
        while l < r {
            let mid = (l + r + 1) / 2;
            if self.str[self[mid]..].starts_with(t) {
                l = mid;
            } else {
                r = mid - 1;
            }
        }
        (from, l + 1)
    }

    pub fn cmp(&self, s1: impl RangeBounds<usize>, s2: impl RangeBounds<usize>) -> Ordering {
        let (l1, r1) = clamp(&s1, self.len() - 1);
        let (l2, r2) = clamp(&s2, self.len() - 1);
        let len1 = r1 - l1;
        let len2 = r2 - l2;
        let lcp = self.lcp(self.get_pos_in_array(l1), self.get_pos_in_array(l2));
        if lcp >= len1.min(len2) {
            len1.cmp(&len2)
        } else {
            let s1 = &self.str[l1 + lcp];
            let s2 = &self.str[l2 + lcp];
            s1.cmp(s2)
        }
    }
}

impl<T> Index<usize> for SuffixArray<T> {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.sorted_suffixes[index]
    }
}

fn ref_min(a: &u32, b: &u32) -> u32 {
    *a.min(b)
}

// SA-IS, O(n). `s` must end with a unique smallest character (sentinel) and
// contain values in 0..alphabet.
fn sais(s: &[u32], alphabet: usize) -> Vec<usize> {
    const EMPTY: usize = usize::MAX;
    let n = s.len();
    if n == 1 {
        return vec![0];
    }
    let mut is_s = vec![false; n];
    is_s[n - 1] = true;
    for i in (0..n - 1).rev() {
        is_s[i] = s[i] < s[i + 1] || (s[i] == s[i + 1] && is_s[i + 1]);
    }
    let mut cnt = vec![0usize; alphabet];
    for &c in s {
        cnt[c as usize] += 1;
    }
    let bucket_starts = |bkt: &mut Vec<usize>| {
        let mut sum = 0;
        for (b, &c) in bkt.iter_mut().zip(cnt.iter()) {
            *b = sum;
            sum += c;
        }
    };
    let bucket_ends = |bkt: &mut Vec<usize>| {
        let mut sum = 0;
        for (b, &c) in bkt.iter_mut().zip(cnt.iter()) {
            sum += c;
            *b = sum;
        }
    };
    let mut sa = vec![EMPTY; n];
    let mut bkt = vec![0usize; alphabet];
    let induce = |sa: &mut Vec<usize>, bkt: &mut Vec<usize>, lms_in_order: &[usize]| {
        sa.fill(EMPTY);
        bucket_ends(bkt);
        for &p in lms_in_order.iter().rev() {
            let c = s[p] as usize;
            bkt[c] -= 1;
            sa[bkt[c]] = p;
        }
        bucket_starts(bkt);
        for i in 0..n {
            let j = sa[i];
            if j != EMPTY && j > 0 && !is_s[j - 1] {
                let c = s[j - 1] as usize;
                sa[bkt[c]] = j - 1;
                bkt[c] += 1;
            }
        }
        bucket_ends(bkt);
        for i in (0..n).rev() {
            let j = sa[i];
            if j != EMPTY && j > 0 && is_s[j - 1] {
                let c = s[j - 1] as usize;
                bkt[c] -= 1;
                sa[bkt[c]] = j - 1;
            }
        }
    };
    let lms: Vec<usize> = (1..n).filter(|&i| is_s[i] && !is_s[i - 1]).collect();
    induce(&mut sa, &mut bkt, &lms);
    let mut sorted_lms: Vec<usize> = sa
        .iter()
        .copied()
        .filter(|&p| p > 0 && is_s[p] && !is_s[p - 1])
        .collect();
    // Name LMS substrings by their rank; equal substrings get equal names.
    let mut next_lms = vec![0usize; n];
    for w in lms.windows(2) {
        next_lms[w[0]] = w[1];
    }
    next_lms[*lms.last().unwrap()] = n - 1;
    let mut name_of = vec![0u32; n];
    let mut num_names = 1u32;
    for w in sorted_lms.windows(2) {
        let (p1, p2) = (w[0], w[1]);
        let (e1, e2) = (next_lms[p1], next_lms[p2]);
        if e1 - p1 != e2 - p2 || s[p1..=e1] != s[p2..=e2] {
            num_names += 1;
        }
        name_of[p2] = num_names - 1;
    }
    if (num_names as usize) < sorted_lms.len() {
        let reduced: Vec<u32> = lms.iter().map(|&p| name_of[p]).collect();
        let reduced_sa = sais(&reduced, num_names as usize);
        for (target, &i) in sorted_lms.iter_mut().zip(reduced_sa.iter()) {
            *target = lms[i];
        }
    }
    induce(&mut sa, &mut bkt, &sorted_lms);
    sa
}

impl<'a, T> IntoIterator for &'a SuffixArray<T> {
    type Item = usize;
    type IntoIter = Cloned<Iter<'a, usize>>;

    fn into_iter(self) -> Self::IntoIter {
        self.sorted_suffixes.iter().cloned()
    }
}

#[cfg(test)]
mod test {
    use super::SuffixArray;

    #[test]
    fn sorted_order() {
        let sa = SuffixArray::new(b"banana");
        // Sorted: \0, a\0, ana\0, anana\0, banana\0, na\0, nana\0
        assert_eq!(sa[0], 6);
        assert_eq!(sa[1], 5);
        assert_eq!(sa[2], 3);
        assert_eq!(sa[3], 1);
        assert_eq!(sa[4], 0);
        assert_eq!(sa[5], 4);
        assert_eq!(sa[6], 2);
    }

    #[test]
    fn lcp_query() {
        let sa = SuffixArray::new(b"banana");
        // lcp("ana\0", "anana\0") = 3
        assert_eq!(sa.lcp(2, 3), 3);
        // lcp("a\0", "ana\0") = 1
        assert_eq!(sa.lcp(1, 2), 1);
    }

    #[test]
    fn find_pattern() {
        let sa = SuffixArray::new(b"banana");
        // "an" matches suffixes at sorted positions 2 and 3
        assert_eq!(sa.find(b"an"), (2, 4));
    }

    #[test]
    fn matches_naive() {
        use crate::misc::random::{Random, RandomTrait};

        let mut rng = Random::new_with_seed(42);
        for iter in 0..2000 {
            let len = rng.gen_range(1..=60usize);
            let alphabet = [1u8, 2, 3, 26, 200][iter % 5];
            let s: Vec<u8> = (0..len).map(|_| 1 + rng.gen_bound(alphabet)).collect();
            let mut with_sentinel = s.clone();
            with_sentinel.push(0);
            let mut expected: Vec<usize> = (0..with_sentinel.len()).collect();
            expected.sort_by(|&a, &b| with_sentinel[a..].cmp(&with_sentinel[b..]));
            let actual = SuffixArray::new(&s);
            assert!(
                expected.iter().copied().eq(actual.into_iter()),
                "mismatch on {:?}",
                s
            );
        }
    }
}
