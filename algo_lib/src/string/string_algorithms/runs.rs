use crate::numbers::num_traits::algebra::Zero;
use crate::string::suffix_array::SuffixArray;
use std::fmt::Debug;

/// All runs (maximal repetitions) of `s` as `(period, from, to)`: `s[from..to]`
/// has minimal period `period`, length at least `2 * period`, and cannot be
/// extended. Sorted, without duplicates; there are fewer than `n` of them.
///
/// Lyndon-root method (Bannai et al.): for both orders of the alphabet, the
/// longest Lyndon word starting at each position is a candidate root, and it
/// is extended in both directions with longest-common-extension queries
/// (suffix arrays of the string and of its reverse). O(n log n).
///
/// Every element must compare greater than `T::zero()`, which the suffix
/// array uses as its end marker.
pub fn runs<T: Zero + Ord + Debug + Copy>(s: &[T]) -> Vec<(usize, usize, usize)> {
    let n = s.len();
    if n == 0 {
        return Vec::new();
    }
    let forward = SuffixArray::new(s);
    let reversed: Vec<T> = s.iter().rev().copied().collect();
    let backward = SuffixArray::new(&reversed);
    let extension = |sa: &SuffixArray<T>, i: usize, j: usize| -> usize {
        if i >= n || j >= n {
            0
        } else if i == j {
            n - i
        } else {
            sa.lcp(sa.get_pos_in_array(i), sa.get_pos_in_array(j))
        }
    };
    // common prefix of s[i..], s[j..]; common suffix of s[..i], s[..j]
    let lcp = |i: usize, j: usize| extension(&forward, i, j);
    let lcs = |i: usize, j: usize| {
        if i == 0 || j == 0 {
            0
        } else {
            extension(&backward, n - i, n - j)
        }
    };
    let mut res = Vec::new();
    let mut stack: Vec<usize> = Vec::new();
    let mut lyndon_end = vec![n; n];
    for inverted in [false, true] {
        stack.clear();
        for i in 0..=n {
            while let Some(&j) = stack.last() {
                if i < n {
                    let k = lcp(i, j);
                    if i + k < n && ((s[j + k] < s[i + k]) != inverted) {
                        break;
                    }
                }
                lyndon_end[j] = i;
                stack.pop();
            }
            stack.push(i);
        }
        for i in 0..n {
            let j = lyndon_end[i];
            let period = j - i;
            let from = i - lcs(i, j);
            let to = j + lcp(i, j);
            if to - from >= 2 * period {
                res.push((period, from, to));
            }
        }
    }
    res.sort_unstable();
    res.dedup();
    res
}

#[cfg(test)]
mod tests {
    use super::runs;
    use crate::misc::random::{Random, RandomTrait};

    fn minimal_period(w: &[u8]) -> usize {
        (1..=w.len())
            .find(|&p| (p..w.len()).all(|i| w[i] == w[i - p]))
            .unwrap()
    }

    fn naive(s: &[u8]) -> Vec<(usize, usize, usize)> {
        let n = s.len();
        let mut res = Vec::new();
        for p in 1..=n / 2 {
            let mut i = 0;
            while i + p < n {
                if s[i] != s[i + p] {
                    i += 1;
                    continue;
                }
                let from = i;
                while i + p < n && s[i] == s[i + p] {
                    i += 1;
                }
                let to = i + p;
                if to - from >= 2 * p && minimal_period(&s[from..to]) == p {
                    res.push((p, from, to));
                }
            }
        }
        res.sort_unstable();
        res
    }

    #[test]
    fn matches_naive() {
        let mut rng = Random::new_with_seed(311);
        assert!(runs::<u8>(&[]).is_empty());
        for _ in 0..600 {
            let n = rng.gen_range(1..=40usize);
            let alphabet = rng.gen_range(1..=3u8);
            let s: Vec<u8> = (0..n).map(|_| b'a' + rng.gen_range(0..alphabet)).collect();
            assert_eq!(runs(&s), naive(&s), "{:?}", String::from_utf8_lossy(&s));
        }
        let fib = {
            let (mut a, mut b) = (b"a".to_vec(), b"ab".to_vec());
            for _ in 0..12 {
                let next = [b.clone(), a.clone()].concat();
                a = b;
                b = next;
            }
            b
        };
        let r = runs(&fib);
        assert_eq!(r, naive(&fib));
        assert!(r.len() < fib.len());
    }
}
