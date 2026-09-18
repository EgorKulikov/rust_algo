use crate::misc::random::{Random, RandomTrait};
use crate::numbers::mod_int::prime_fft::PrimeFFT;
use crate::numbers::mod_int::ModIntF;
use crate::numbers::num_traits::algebra::Zero;

/// For every alignment `i` in `0..=text.len() - pattern.len()`, whether the
/// pattern matches `text[i..]` when `wildcard` (in either string) matches
/// any character. Three NTT products of `sum p t (p - t)^2` with random
/// character weights, so a false positive has probability about `n / 1e9`.
pub fn wildcard_matches(text: &[u8], pattern: &[u8], wildcard: u8) -> Vec<bool> {
    let (n, m) = (text.len(), pattern.len());
    if m > n {
        return Vec::new();
    }
    if m == 0 {
        return vec![true; n + 1];
    }
    type M = ModIntF;
    let mut rng = Random::new();
    let weights: Vec<M> = (0..256)
        .map(|c| {
            if c == wildcard as usize {
                M::zero()
            } else {
                M::new(rng.gen_range(1..998_244_353u32))
            }
        })
        .collect();
    let t1: Vec<M> = text.iter().map(|&c| weights[c as usize]).collect();
    let p1: Vec<M> = pattern.iter().rev().map(|&c| weights[c as usize]).collect();
    let square = |v: &[M]| -> Vec<M> { v.iter().map(|&x| x * x).collect() };
    let cube = |v: &[M]| -> Vec<M> { v.iter().map(|&x| x * x * x).collect() };
    let mut fft = PrimeFFT::<M>::new();
    // sum p^3 t - 2 p^2 t^2 + p t^3 over the window
    let a = fft.multiply(&cube(&p1), &t1);
    let b = fft.multiply(&square(&p1), &square(&t1));
    let c = fft.multiply(&p1, &cube(&t1));
    (0..=n - m)
        .map(|i| {
            let k = i + m - 1;
            a[k] - b[k] - b[k] + c[k] == M::zero()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::wildcard_matches;
    use crate::misc::random::{Random, RandomTrait};

    #[test]
    fn matches_naive() {
        let mut rng = Random::new_with_seed(301);
        for _ in 0..300 {
            let n = rng.gen_range(0..=120usize);
            let m = rng.gen_range(0..=n + 2);
            let gen = |rng: &mut Random, len: usize| -> Vec<u8> {
                (0..len)
                    .map(|_| {
                        if rng.gen_range(0..4u32) == 0 {
                            b'*'
                        } else {
                            b'a' + rng.gen_range(0..2u8)
                        }
                    })
                    .collect()
            };
            let text = gen(&mut rng, n);
            let pattern = gen(&mut rng, m);
            let expected: Vec<bool> = if m > n {
                Vec::new()
            } else {
                (0..=n - m)
                    .map(|i| {
                        (0..m).all(|j| {
                            text[i + j] == b'*' || pattern[j] == b'*' || text[i + j] == pattern[j]
                        })
                    })
                    .collect()
            };
            assert_eq!(wildcard_matches(&text, &pattern, b'*'), expected);
        }
    }
}
