/// Lyndon factorization by Duval's algorithm in O(n): the start of every
/// factor, followed by `len`. Factors are non-increasing Lyndon words.
pub fn lyndon_factorization<T: Ord>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let mut res = Vec::new();
    let mut i = 0;
    while i < n {
        let (mut j, mut k) = (i + 1, i);
        while j < n && s[k] <= s[j] {
            k = if s[k] < s[j] { i } else { k + 1 };
            j += 1;
        }
        while i <= k {
            res.push(i);
            i += j - k;
        }
    }
    res.push(n);
    res
}

#[cfg(test)]
mod tests {
    use super::lyndon_factorization;
    use crate::misc::random::{Random, RandomTrait};

    fn is_lyndon(w: &[u8]) -> bool {
        (1..w.len()).all(|i| w < &w[i..])
    }

    #[test]
    fn factors_are_lyndon_and_non_increasing() {
        let mut rng = Random::new_with_seed(281);
        assert_eq!(lyndon_factorization::<u8>(&[]), vec![0]);
        for _ in 0..500 {
            let n = rng.gen_range(1..=30usize);
            let alphabet = rng.gen_range(1..=3u8);
            let s: Vec<u8> = (0..n).map(|_| b'a' + rng.gen_range(0..alphabet)).collect();
            let cuts = lyndon_factorization(&s);
            assert_eq!((cuts[0], *cuts.last().unwrap()), (0, n));
            let factors: Vec<&[u8]> = cuts.windows(2).map(|w| &s[w[0]..w[1]]).collect();
            assert!(
                factors.iter().all(|f| !f.is_empty() && is_lyndon(f)),
                "{s:?}"
            );
            assert!(factors.windows(2).all(|p| p[0] >= p[1]), "{s:?}");
        }
    }
}
