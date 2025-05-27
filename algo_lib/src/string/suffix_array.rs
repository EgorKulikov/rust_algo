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
        let mut sorted_suffixes = Vec::with_gen(str.len(), |x| x);
        sorted_suffixes.sort_by_key(|&id| str[id]);
        let mut class_eq = vec![0; n];
        for win in sorted_suffixes.windows(2) {
            if str[win[1]] != str[win[0]] {
                class_eq[win[1]] = class_eq[win[0]] + 1;
            } else {
                class_eq[win[1]] = class_eq[win[0]];
            }
        }
        let mut num_classes = class_eq.iter().max().unwrap() + 1;
        let mut suffixes_new = vec![0; n];
        let mut class_eq_new = vec![0; n];
        let mut cnt = vec![0; n];
        for lvl in 0.. {
            let half = 1 << lvl;
            if half >= n {
                break;
            }
            for (val_new, val) in suffixes_new.iter_mut().zip(sorted_suffixes.iter()) {
                let next = (*val as i32) - (half as i32);
                let next = if next < 0 { next + n as i32 } else { next };
                *val_new = next as usize;
            }
            cnt[..num_classes].fill(0);
            for &class_id in class_eq.iter() {
                cnt[class_id] += 1;
            }
            for i in 1..num_classes {
                cnt[i] += cnt[i - 1];
            }
            for i in (0..n).rev() {
                cnt[class_eq[suffixes_new[i]]] -= 1;
                sorted_suffixes[cnt[class_eq[suffixes_new[i]]]] = suffixes_new[i];
            }
            class_eq_new[sorted_suffixes[0]] = 0;
            num_classes = 1;
            for i in 1..n {
                let mid1 = (sorted_suffixes[i] + half) % n;
                let mid2 = (sorted_suffixes[i - 1] + half) % n;
                if class_eq[sorted_suffixes[i]] != class_eq[sorted_suffixes[i - 1]]
                    || class_eq[mid1] != class_eq[mid2]
                {
                    num_classes += 1;
                }
                class_eq_new[sorted_suffixes[i]] = num_classes - 1;
            }
            class_eq.copy_from_slice(&class_eq_new);
        }

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

impl<'a, T> IntoIterator for &'a SuffixArray<T> {
    type Item = usize;
    type IntoIter = Cloned<Iter<'a, usize>>;

    fn into_iter(self) -> Self::IntoIter {
        self.sorted_suffixes.iter().cloned()
    }
}
