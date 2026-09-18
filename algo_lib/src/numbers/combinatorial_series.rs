//! Classic counting sequences computed through formal power series in
//! O(n log n) or O(n log^2 n). Everything is modulo a prime larger than `n`.

use crate::numbers::fps::factorial_tables;
use crate::numbers::mod_int::mod_utils::inverses;
use crate::numbers::mod_int::BaseModInt;
use crate::numbers::num_traits::algebra::IntegerSemiRing;
use crate::numbers::number_ext::Power;
use crate::numbers::polynomial::PolynomialOps;

impl<T: Into<u64> + IntegerSemiRing + Copy, M: BaseModInt<T>> PolynomialOps<T, M> {
    /// Coefficients of the rising factorial `x (x + 1) .. (x + n - 1)`:
    /// entry `k` is the unsigned Stirling number of the first kind `c(n, k)`.
    pub fn rising_factorial(&mut self, n: usize) -> Vec<M> {
        if n == 0 {
            return vec![M::one()];
        }
        let half = self.rising_factorial(n / 2);
        // P_{2m}(x) = P_m(x) * P_m(x + m)
        let shifted = self.taylor_shift(&half, M::from(n / 2));
        let mut res = self.multiply(&half, &shifted);
        if n % 2 == 1 {
            // times (x + n - 1)
            let last = M::from(n - 1);
            res.push(M::zero());
            for i in (1..res.len()).rev() {
                let lower = res[i - 1];
                res[i] = res[i] * last + lower;
            }
            res[0] *= last;
        }
        res
    }

    /// Signed Stirling numbers of the first kind `s(n, k)` for `k` in `0..=n`
    /// (coefficients of the falling factorial `x (x - 1) .. (x - n + 1)`).
    pub fn stirling_first(&mut self, n: usize) -> Vec<M> {
        let mut res = self.rising_factorial(n);
        for (k, x) in res.iter_mut().enumerate() {
            if (n - k) % 2 == 1 {
                *x = -*x;
            }
        }
        res
    }

    /// Stirling numbers of the second kind `S(n, k)` for `k` in `0..=n`.
    pub fn stirling_second(&mut self, n: usize) -> Vec<M> {
        let (_, inv_fact) = factorial_tables::<M>(n + 1);
        // S(n, k) = sum_i (-1)^(k-i) i^n / (i! (k-i)!)
        let a: Vec<M> = (0..=n).map(|i| M::from(i).power(n) * inv_fact[i]).collect();
        let b: Vec<M> = (0..=n)
            .map(|j| {
                if j % 2 == 1 {
                    -inv_fact[j]
                } else {
                    inv_fact[j]
                }
            })
            .collect();
        self.mul_trunc(&a, &b, n + 1)
    }

    /// Bell numbers `B_0 .. B_{n-1}`.
    pub fn bell_numbers(&mut self, n: usize) -> Vec<M> {
        if n == 0 {
            return Vec::new();
        }
        let (fact, inv_fact) = factorial_tables::<M>(n);
        let mut f = inv_fact.clone();
        f[0] = M::zero(); // e^x - 1
        let g = self.exp(&f, n);
        g.iter().zip(&fact).map(|(&x, &y)| x * y).collect()
    }

    /// Partition numbers `p(0) .. p(n-1)`.
    pub fn partition_numbers(&mut self, n: usize) -> Vec<M> {
        if n == 0 {
            return Vec::new();
        }
        // Euler: prod (1 - x^k) = sum_k (-1)^k x^(k (3k -+ 1) / 2)
        let mut f = vec![M::zero(); n];
        f[0] = M::one();
        let mut k = 1usize;
        loop {
            let low = k * (3 * k - 1) / 2;
            if low >= n {
                break;
            }
            let sign = if k % 2 == 1 { -M::one() } else { M::one() };
            f[low] += sign;
            let high = k * (3 * k + 1) / 2;
            if high < n {
                f[high] += sign;
            }
            k += 1;
        }
        self.inverse_series(&f, n)
    }

    /// Bernoulli numbers `B_0 .. B_{n-1}` (with `B_1 = -1/2`).
    pub fn bernoulli_numbers(&mut self, n: usize) -> Vec<M> {
        if n == 0 {
            return Vec::new();
        }
        let (fact, inv_fact) = factorial_tables::<M>(n + 1);
        // x / (e^x - 1) = 1 / sum x^i / (i + 1)!
        let f: Vec<M> = (0..n).map(|i| inv_fact[i + 1]).collect();
        let g = self.inverse_series(&f, n);
        g.iter().zip(&fact).map(|(&x, &y)| x * y).collect()
    }

    /// For every `t < n`, the number of subsets of `items` with sum `t`.
    pub fn count_subset_sums(&mut self, items: &[usize], n: usize) -> Vec<M> {
        if n == 0 {
            return Vec::new();
        }
        let mut count = vec![0usize; n];
        let mut zeros = 0u64;
        for &s in items {
            if s == 0 {
                zeros += 1;
            } else if s < n {
                count[s] += 1;
            }
        }
        // log prod (1 + x^s) = sum_s cnt_s sum_j (-1)^(j+1) x^(s j) / j
        let inv: Vec<M> = inverses(n);
        let mut lg = vec![M::zero(); n];
        for s in 1..n {
            if count[s] == 0 {
                continue;
            }
            let c = M::from(count[s]);
            for j in 1..=(n - 1) / s {
                let term = c * inv[j];
                if j % 2 == 1 {
                    lg[s * j] += term;
                } else {
                    lg[s * j] -= term;
                }
            }
        }
        let mut res = self.exp(&lg, n);
        // every zero-sized item doubles all counts
        let scale = (M::one() + M::one()).power(zeros);
        for x in res.iter_mut() {
            *x *= scale;
        }
        res
    }
}

#[cfg(test)]
#[allow(clippy::needless_range_loop)]
mod tests {
    use crate::misc::random::{Random, RandomTrait};
    use crate::numbers::mod_int::ModIntF as M;
    use crate::numbers::num_traits::algebra::{One, Zero};
    use crate::numbers::polynomial::PolynomialOps;

    #[test]
    fn stirling_numbers() {
        let mut ops: PolynomialOps<u32, M> = PolynomialOps::new();
        let limit = 70;
        // c(n, k) = c(n-1, k-1) + (n-1) c(n-1, k); S(n, k) = S(n-1, k-1) + k S(n-1, k)
        let mut c = vec![vec![M::zero(); limit + 1]; limit + 1];
        let mut s2 = vec![vec![M::zero(); limit + 1]; limit + 1];
        c[0][0] = M::one();
        s2[0][0] = M::one();
        for n in 1..=limit {
            for k in 1..=n {
                c[n][k] = c[n - 1][k - 1] + M::from(n - 1) * c[n - 1][k];
                s2[n][k] = s2[n - 1][k - 1] + M::from(k) * s2[n - 1][k];
            }
        }
        for n in [0usize, 1, 2, 3, 7, 8, 33, 64, 65, 70] {
            assert_eq!(ops.rising_factorial(n), c[n][..=n].to_vec(), "c({n}, k)");
            let signed: Vec<M> = (0..=n)
                .map(|k| if (n - k) % 2 == 1 { -c[n][k] } else { c[n][k] })
                .collect();
            assert_eq!(ops.stirling_first(n), signed);
            assert_eq!(ops.stirling_second(n), s2[n][..=n].to_vec(), "S({n}, k)");
        }
        // Bell numbers are row sums of S
        let bell = ops.bell_numbers(limit + 1);
        for n in 0..=limit {
            let expected = s2[n].iter().fold(M::zero(), |a, &b| a + b);
            assert_eq!(bell[n], expected, "B_{n}");
        }
        assert_eq!(ops.bell_numbers(0), Vec::<M>::new());
    }

    #[test]
    fn partitions_bernoulli_subset_sums() {
        let mut ops: PolynomialOps<u32, M> = PolynomialOps::new();
        let n = 200;
        // p via the coin DP
        let mut p = vec![M::zero(); n];
        p[0] = M::one();
        for part in 1..n {
            for t in part..n {
                let add = p[t - part];
                p[t] += add;
            }
        }
        assert_eq!(ops.partition_numbers(n), p);
        assert_eq!(ops.partition_numbers(1), vec![M::one()]);
        // sum_{j<=m} C(m+1, j) B_j = 0 for m >= 1
        let b = ops.bernoulli_numbers(60);
        assert_eq!(b[0], M::one());
        assert_eq!(b[1] * M::from(2usize), -M::one());
        let mut binom = vec![vec![M::zero(); 62]; 62];
        for i in 0..62 {
            binom[i][0] = M::one();
            for j in 1..=i {
                binom[i][j] = binom[i - 1][j - 1] + if j < i { binom[i - 1][j] } else { M::zero() };
            }
        }
        for m in 1..59 {
            let sum = (0..=m).fold(M::zero(), |acc, j| acc + binom[m + 1][j] * b[j]);
            assert_eq!(sum, M::zero(), "Bernoulli identity m={m}");
        }
        let mut rng = Random::new_with_seed(211);
        for _ in 0..20 {
            let limit = rng.gen_range(1..150usize);
            let items: Vec<usize> = (0..rng.gen_range(0..40usize))
                .map(|_| rng.gen_range(0..60usize))
                .collect();
            let mut dp = vec![M::zero(); limit];
            dp[0] = M::one();
            for &s in &items {
                for t in (s..limit).rev() {
                    let add = dp[t - s];
                    dp[t] += add;
                }
            }
            assert_eq!(ops.count_subset_sums(&items, limit), dp, "items={items:?}");
        }
    }
}
