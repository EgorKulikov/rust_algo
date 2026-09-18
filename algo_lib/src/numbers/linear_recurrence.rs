//! Linear recurrences over a field: finding the shortest one that generates
//! a sequence (Berlekamp-Massey) and jumping to its `k`-th term
//! (Bostan-Mori, O(d log d log k) with FFT multiplication).

use crate::numbers::mod_int::BaseModInt;
use crate::numbers::num_traits::algebra::Field;
use crate::numbers::polynomial::PolynomialOps;

/// Shortest `c` with `s[i] = sum_j c[j] * s[i - 1 - j]` for all `i >= c.len()`.
/// The answer is determined by the sequence once `s.len() >= 2 * c.len()`.
pub fn berlekamp_massey<T: Field + Copy>(s: &[T]) -> Vec<T> {
    // cur: current connection polynomial C(x) = 1 - sum c_j x^{j+1}, stored
    // as the coefficients c_j. prev: the one before the last length change.
    let mut cur: Vec<T> = Vec::new();
    let mut prev: Vec<T> = Vec::new();
    let mut prev_at = 0usize;
    let mut prev_delta = T::one();
    let mut has_prev = false;
    for i in 0..s.len() {
        let mut delta = s[i];
        for (j, &c) in cur.iter().enumerate() {
            delta -= c * s[i - 1 - j];
        }
        if delta == T::zero() {
            continue;
        }
        if !has_prev {
            has_prev = true;
            prev = cur.clone();
            prev_at = i;
            prev_delta = delta;
            cur = vec![T::zero(); i + 1];
            continue;
        }
        // cur += (delta / prev_delta) * x^{i - prev_at} * (1 - prev)
        let coef = delta / prev_delta;
        let shift = i - prev_at - 1;
        let mut next = cur.clone();
        if next.len() < shift + 1 + prev.len() {
            next.resize(shift + 1 + prev.len(), T::zero());
        }
        next[shift] += coef;
        for (j, &p) in prev.iter().enumerate() {
            next[shift + 1 + j] -= coef * p;
        }
        if i + 1 - cur.len() > prev_at + 1 - prev.len() {
            prev = cur;
            prev_at = i;
            prev_delta = delta;
        }
        cur = next;
    }
    cur
}

/// `k`-th term (0-based) of the sequence starting with `init` and continuing
/// by `s[i] = sum_j c[j] * s[i - 1 - j]`; `init.len()` must be at least
/// `c.len()`.
pub fn kth_term<T: Into<u64>, M: BaseModInt<T>>(
    init: &[M],
    c: &[M],
    mut k: u64,
    ops: &mut PolynomialOps<T, M>,
) -> M {
    let d = c.len();
    assert!(init.len() >= d);
    if (k as u128) < init.len() as u128 {
        return init[k as usize];
    }
    if d == 0 {
        return M::zero();
    }
    // Generating function P(x) / Q(x), Q = 1 - sum c_j x^{j+1}, deg P < d.
    let mut q = Vec::with_capacity(d + 1);
    q.push(M::one());
    q.extend(c.iter().map(|&x| -x));
    let mut p = ops.multiply(&init[..d], &q);
    p.truncate(d);
    while k > 0 {
        // Multiply numerator and denominator by Q(-x): the new denominator is
        // even, so only every other coefficient of the numerator matters.
        let mut q_neg = q.clone();
        for x in q_neg.iter_mut().skip(1).step_by(2) {
            *x = -*x;
        }
        let u = ops.multiply(&p, &q_neg);
        let v = ops.multiply(&q, &q_neg);
        p = u.into_iter().skip((k & 1) as usize).step_by(2).collect();
        q = v.into_iter().step_by(2).collect();
        k >>= 1;
    }
    p.first().copied().unwrap_or_else(M::zero) / q[0]
}

/// `k`-th term of the shortest linear recurrence consistent with `s`.
pub fn guess_kth_term<T: Into<u64>, M: BaseModInt<T>>(
    s: &[M],
    k: u64,
    ops: &mut PolynomialOps<T, M>,
) -> M {
    let c = berlekamp_massey(s);
    kth_term(s, &c, k, ops)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::misc::random::{Random, RandomTrait};
    use crate::numbers::mod_int::{ModInt7, ModIntF};
    use crate::numbers::num_traits::algebra::{One, Zero};

    fn extend<M: BaseModInt<u32>>(init: &[M], c: &[M], len: usize) -> Vec<M> {
        let mut s = init.to_vec();
        while s.len() < len {
            let i = s.len();
            let next = c
                .iter()
                .enumerate()
                .fold(M::zero(), |acc, (j, &x)| acc + x * s[i - 1 - j]);
            s.push(next);
        }
        s
    }

    fn check<M: BaseModInt<u32> + std::fmt::Debug>(seed: u64) {
        let mut rng = Random::new_with_seed(seed);
        let mut ops = PolynomialOps::new();
        for d in [0usize, 1, 2, 3, 7, 20, 70, 150] {
            let c: Vec<M> = (0..d)
                .map(|_| M::from(rng.gen_u128() as u32 as usize))
                .collect();
            let init: Vec<M> = (0..d)
                .map(|_| M::from(rng.gen_u128() as u32 as usize))
                .collect();
            let s = extend(&init, &c, 2 * d + 40);
            let found = berlekamp_massey(&s);
            assert!(found.len() <= d, "d={d} found {}", found.len());
            assert_eq!(extend(&s[..found.len()], &found, s.len()), s, "d={d}");
            for k in [0u64, 1, d as u64, 2 * d as u64 + 39, 1000, 5000] {
                let expected = extend(&init, &c, k as usize + 1)[k as usize];
                assert_eq!(kth_term(&init, &c, k, &mut ops), expected, "d={d} k={k}");
                assert_eq!(
                    guess_kth_term(&s, k, &mut ops),
                    expected,
                    "guess d={d} k={k}"
                );
            }
            // huge k: consistency between the true and the recovered recurrence
            let k = 1_000_000_000_000_000_000u64 + d as u64;
            assert_eq!(
                kth_term(&init, &c, k, &mut ops),
                guess_kth_term(&s, k, &mut ops)
            );
        }
    }

    #[test]
    fn random_recurrences() {
        check::<ModIntF>(181);
        check::<ModInt7>(182);
    }

    #[test]
    fn fibonacci() {
        type M = ModInt7;
        let s: Vec<M> = [0usize, 1, 1, 2, 3, 5, 8, 13]
            .iter()
            .map(|&x| M::from(x))
            .collect();
        assert_eq!(berlekamp_massey(&s), vec![M::one(), M::one()]);
        let mut ops = PolynomialOps::new();
        assert_eq!(
            guess_kth_term(&s, 90, &mut ops),
            M::from(2_880_067_194_370_816_120usize % 1_000_000_007)
        );
        // constant and zero sequences
        assert_eq!(berlekamp_massey(&[M::zero(); 5]), Vec::<M>::new());
        assert_eq!(berlekamp_massey(&[M::from(7usize); 5]), vec![M::one()]);
        assert_eq!(
            guess_kth_term(&[M::from(7usize); 5], u64::MAX, &mut ops),
            M::from(7usize)
        );
    }
}
