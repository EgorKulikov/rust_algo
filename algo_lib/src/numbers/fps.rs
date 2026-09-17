//! Formal power series operations truncated to a given number of terms,
//! built on [`PolynomialOps`]. All functions return exactly `n` coefficients
//! (ascending order) unless documented otherwise, and read only the first
//! `n` coefficients of their inputs. The dense operations are quasi-linear
//! Newton iterations; the `*_sparse` ones run in O(n * nonzeros) and are
//! preferable when the input has few nonzero coefficients.

use crate::numbers::mod_int::mod_utils::{inverses, mod_sqrt};
use crate::numbers::mod_int::BaseModInt;
use crate::numbers::num_traits::algebra::IntegerSemiRing;
use crate::numbers::number_ext::Power;
use crate::numbers::polynomial::PolynomialOps;

impl<T: Into<u64> + IntegerSemiRing + Copy, M: BaseModInt<T>> PolynomialOps<T, M> {
    /// First `n` coefficients of `a * b`.
    pub fn mul_trunc(&mut self, a: &[M], b: &[M], n: usize) -> Vec<M> {
        let mut res = vec![M::zero(); n];
        if n == 0 || a.is_empty() || b.is_empty() {
            return res;
        }
        let a = &a[..a.len().min(n)];
        let b = &b[..b.len().min(n)];
        if a.len().min(b.len()) <= 60 {
            for (i, &x) in a.iter().enumerate() {
                for (j, &y) in b.iter().take(n - i).enumerate() {
                    res[i + j] += x * y;
                }
            }
        } else {
            self.fft_mut().multiply_fix_len(a, b, &mut res);
        }
        res
    }

    /// `1 / f` modulo `x^n`; `f[0]` must be nonzero.
    pub fn inverse_series(&mut self, f: &[M], n: usize) -> Vec<M> {
        assert!(
            !f.is_empty() && f[0] != M::zero(),
            "inverse needs f[0] != 0"
        );
        let mut g = vec![f[0].inv().unwrap()];
        while g.len() < n {
            let size = (2 * g.len()).min(n);
            let mut c = self.mul_trunc(&f[..f.len().min(size)], &g, size);
            for x in c.iter_mut() {
                *x = -*x;
            }
            c[0] += M::one() + M::one();
            g = self.mul_trunc(&g, &c, size);
        }
        g.truncate(n);
        g
    }

    pub fn derivative(&mut self, f: &[M]) -> Vec<M> {
        f.iter()
            .enumerate()
            .skip(1)
            .map(|(i, &c)| c * M::from(i))
            .collect()
    }

    /// Antiderivative with zero constant term (one coefficient longer).
    pub fn integral(&mut self, f: &[M]) -> Vec<M> {
        let inv: Vec<M> = inverses(f.len() + 1);
        let mut res = Vec::with_capacity(f.len() + 1);
        res.push(M::zero());
        res.extend(f.iter().enumerate().map(|(i, &c)| c * inv[i + 1]));
        res
    }

    /// `log f` modulo `x^n`; `f[0]` must be one.
    pub fn log(&mut self, f: &[M], n: usize) -> Vec<M> {
        assert!(!f.is_empty() && f[0] == M::one(), "log needs f[0] == 1");
        if n == 0 {
            return Vec::new();
        }
        let inv = self.inverse_series(f, n);
        let d = self.derivative(&f[..f.len().min(n)]);
        let q = self.mul_trunc(&d, &inv, n - 1);
        let mut res = self.integral(&q);
        res.resize(n, M::zero());
        res
    }

    /// `exp f` modulo `x^n`; `f[0]` must be zero (an empty `f` is fine).
    pub fn exp(&mut self, f: &[M], n: usize) -> Vec<M> {
        assert!(f.is_empty() || f[0] == M::zero(), "exp needs f[0] == 0");
        if n == 0 {
            return Vec::new();
        }
        let mut g = vec![M::one()];
        while g.len() < n {
            let size = (2 * g.len()).min(n);
            let mut h = self.log(&g, size);
            for (i, x) in h.iter_mut().enumerate() {
                *x = if i < f.len() { f[i] } else { M::zero() } - *x;
            }
            h[0] += M::one();
            g = self.mul_trunc(&g, &h, size);
        }
        g
    }

    /// `f^k` modulo `x^n` for a nonnegative integer `k`.
    pub fn pow(&mut self, f: &[M], k: u64, n: usize) -> Vec<M> {
        let mut res = vec![M::zero(); n];
        if n == 0 {
            return res;
        }
        if k == 0 {
            res[0] = M::one();
            return res;
        }
        let Some(t) = f.iter().position(|&c| c != M::zero()) else {
            return res;
        };
        if t as u128 * k as u128 >= n as u128 {
            return res;
        }
        let shift = t * k as usize;
        let m = n - shift;
        let lead = f[t];
        let lead_inv = lead.inv().unwrap();
        let unit: Vec<M> = f[t..f.len().min(t + m)]
            .iter()
            .map(|&c| c * lead_inv)
            .collect();
        let p: u64 = M::module().into();
        let k_mod = M::from((k % p) as usize);
        let mut lg = self.log(&unit, m);
        for x in lg.iter_mut() {
            *x *= k_mod;
        }
        let scale = lead.power(k % (p - 1));
        let g = self.exp(&lg, m);
        for (i, &c) in g.iter().enumerate() {
            res[shift + i] = c * scale;
        }
        res
    }

    /// A square root of `f` modulo `x^n`, if one exists.
    pub fn sqrt(&mut self, f: &[M], n: usize) -> Option<Vec<M>> {
        let mut res = vec![M::zero(); n];
        if n == 0 {
            return Some(res);
        }
        let Some(t) = f.iter().take(n).position(|&c| c != M::zero()) else {
            return Some(res);
        };
        if t % 2 == 1 {
            return None;
        }
        let m = n - t / 2;
        let g = &f[t..f.len().min(t + m)];
        let mut r = vec![mod_sqrt(g[0])?];
        let half = (M::one() + M::one()).inv().unwrap();
        while r.len() < m {
            let size = (2 * r.len()).min(m);
            let inv = self.inverse_series(&r, size);
            let q = self.mul_trunc(g, &inv, size);
            let mut next = q;
            for (i, x) in next.iter_mut().enumerate() {
                if i < r.len() {
                    *x += r[i];
                }
                *x *= half;
            }
            r = next;
        }
        res[t / 2..].copy_from_slice(&r[..m]);
        Some(res)
    }

    fn nonzeros(f: &[M], n: usize) -> Vec<(usize, M)> {
        f.iter()
            .take(n)
            .enumerate()
            .filter(|&(i, &c)| i > 0 && c != M::zero())
            .map(|(i, &c)| (i, c))
            .collect()
    }

    /// `f^k` modulo `x^n` for `f[0] == 1` and any field exponent `k`.
    fn pow_sparse_unit(&mut self, f: &[M], k: M, n: usize) -> Vec<M> {
        let nz = Self::nonzeros(f, n);
        let inv: Vec<M> = inverses(n);
        let mut g = vec![M::zero(); n];
        if n > 0 {
            g[0] = M::one();
        }
        for i in 1..n {
            let mut acc = M::zero();
            for &(m, c) in nz.iter().take_while(|&&(m, _)| m <= i) {
                acc += (k * M::from(m) - M::from(i - m)) * c * g[i - m];
            }
            g[i] = acc * inv[i];
        }
        g
    }

    /// `1 / f` modulo `x^n` in O(n * nonzeros); `f[0]` must be nonzero.
    pub fn inverse_sparse(&mut self, f: &[M], n: usize) -> Vec<M> {
        assert!(
            !f.is_empty() && f[0] != M::zero(),
            "inverse needs f[0] != 0"
        );
        let nz = Self::nonzeros(f, n);
        let g0 = f[0].inv().unwrap();
        let mut g = vec![M::zero(); n];
        if n > 0 {
            g[0] = g0;
        }
        for i in 1..n {
            let mut acc = M::zero();
            for &(m, c) in nz.iter().take_while(|&&(m, _)| m <= i) {
                acc += c * g[i - m];
            }
            g[i] = -acc * g0;
        }
        g
    }

    /// `exp f` modulo `x^n` in O(n * nonzeros); `f[0]` must be zero.
    pub fn exp_sparse(&mut self, f: &[M], n: usize) -> Vec<M> {
        assert!(f.is_empty() || f[0] == M::zero(), "exp needs f[0] == 0");
        let nz = Self::nonzeros(f, n);
        let inv: Vec<M> = inverses(n);
        let mut g = vec![M::zero(); n];
        if n > 0 {
            g[0] = M::one();
        }
        for i in 1..n {
            let mut acc = M::zero();
            for &(m, c) in nz.iter().take_while(|&&(m, _)| m <= i) {
                acc += M::from(m) * c * g[i - m];
            }
            g[i] = acc * inv[i];
        }
        g
    }

    /// `log f` modulo `x^n` in O(n * nonzeros); `f[0]` must be one.
    pub fn log_sparse(&mut self, f: &[M], n: usize) -> Vec<M> {
        assert!(!f.is_empty() && f[0] == M::one(), "log needs f[0] == 1");
        let nz = Self::nonzeros(f, n);
        let inv: Vec<M> = inverses(n);
        let mut g = vec![M::zero(); n];
        for i in 1..n {
            let mut acc = M::zero();
            for &(m, c) in nz.iter().take_while(|&&(m, _)| m < i) {
                acc += M::from(i - m) * c * g[i - m];
            }
            let fi = if i < f.len() { f[i] } else { M::zero() };
            g[i] = fi - acc * inv[i];
        }
        g
    }

    /// `f^k` modulo `x^n` in O(n * nonzeros) for a nonnegative integer `k`.
    pub fn pow_sparse(&mut self, f: &[M], k: u64, n: usize) -> Vec<M> {
        let mut res = vec![M::zero(); n];
        if n == 0 {
            return res;
        }
        if k == 0 {
            res[0] = M::one();
            return res;
        }
        let Some(t) = f.iter().position(|&c| c != M::zero()) else {
            return res;
        };
        if t as u128 * k as u128 >= n as u128 {
            return res;
        }
        let shift = t * k as usize;
        let m = n - shift;
        let lead = f[t];
        let lead_inv = lead.inv().unwrap();
        let unit: Vec<M> = f[t..f.len().min(t + m)]
            .iter()
            .map(|&c| c * lead_inv)
            .collect();
        let p: u64 = M::module().into();
        let g = self.pow_sparse_unit(&unit, M::from((k % p) as usize), m);
        let scale = lead.power(k % (p - 1));
        for (i, &c) in g.iter().enumerate() {
            res[shift + i] = c * scale;
        }
        res
    }

    /// A square root of `f` modulo `x^n` in O(n * nonzeros), if one exists.
    pub fn sqrt_sparse(&mut self, f: &[M], n: usize) -> Option<Vec<M>> {
        let mut res = vec![M::zero(); n];
        if n == 0 {
            return Some(res);
        }
        let Some(t) = f.iter().take(n).position(|&c| c != M::zero()) else {
            return Some(res);
        };
        if t % 2 == 1 {
            return None;
        }
        let m = n - t / 2;
        let lead = f[t];
        let s = mod_sqrt(lead)?;
        let lead_inv = lead.inv().unwrap();
        let unit: Vec<M> = f[t..f.len().min(t + m)]
            .iter()
            .map(|&c| c * lead_inv)
            .collect();
        let half = (M::one() + M::one()).inv().unwrap();
        let g = self.pow_sparse_unit(&unit, half, m);
        for (i, &c) in g.iter().enumerate() {
            res[t / 2 + i] = c * s;
        }
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::misc::random::{Random, RandomTrait};
    use crate::numbers::mod_int::mod_utils::mod_sqrt;
    use crate::numbers::mod_int::ModIntF as M;
    use crate::numbers::num_traits::algebra::{One, Zero};
    use crate::numbers::polynomial::PolynomialOps;

    fn rnd(rng: &mut Random) -> M {
        M::new(rng.gen_u128() as u32)
    }

    fn naive_mul(a: &[M], b: &[M], n: usize) -> Vec<M> {
        let mut res = vec![M::zero(); n];
        for (i, &x) in a.iter().enumerate().take(n) {
            for (j, &y) in b.iter().enumerate().take(n - i) {
                res[i + j] += x * y;
            }
        }
        res
    }

    fn random_poly(rng: &mut Random, len: usize) -> Vec<M> {
        (0..len).map(|_| rnd(rng)).collect()
    }

    #[test]
    fn tonelli_shanks() {
        let mut rng = Random::new_with_seed(41);
        let mut residues = 0;
        for _ in 0..500 {
            let x = rnd(&mut rng);
            let sq = x * x;
            let r = mod_sqrt(sq).unwrap();
            assert!(r == x || r == -x);
            if let Some(r) = mod_sqrt(x) {
                assert_eq!(r * r, x);
                residues += 1;
            }
        }
        assert!(residues > 150 && residues < 350);
        assert_eq!(mod_sqrt(M::zero()), Some(M::zero()));
    }

    #[test]
    fn inverse_and_log_exp_round_trips() {
        let mut rng = Random::new_with_seed(42);
        let mut ops = PolynomialOps::new();
        for n in [1usize, 2, 3, 7, 61, 64, 100, 257, 1000] {
            let mut f = random_poly(&mut rng, n);
            f[0] = M::one();
            let inv = ops.inverse_series(&f, n);
            let mut one = vec![M::zero(); n];
            one[0] = M::one();
            assert_eq!(naive_mul(&f, &inv, n), one, "inverse n={n}");
            assert_eq!(ops.inverse_sparse(&f, n), inv, "inverse_sparse n={n}");
            let lg = ops.log(&f, n);
            assert_eq!(ops.log_sparse(&f, n), lg, "log_sparse n={n}");
            let back = ops.exp(&lg, n);
            assert_eq!(back, f, "exp(log f) n={n}");
            assert_eq!(ops.exp_sparse(&lg, n), f, "exp_sparse n={n}");
            // exp against the quadratic recurrence directly
            let mut g = random_poly(&mut rng, n);
            g[0] = M::zero();
            assert_eq!(ops.exp(&g, n), ops.exp_sparse(&g, n), "exp n={n}");
            // inverse with a non-unit constant term
            let mut h = random_poly(&mut rng, n);
            if h[0] == M::zero() {
                h[0] = M::one() + M::one();
            }
            let inv = ops.inverse_series(&h, n);
            assert_eq!(naive_mul(&h, &inv, n), one);
        }
    }

    #[test]
    fn pow_and_sqrt() {
        let mut rng = Random::new_with_seed(43);
        let mut ops = PolynomialOps::new();
        for n in [1usize, 2, 5, 64, 100, 300] {
            let f = random_poly(&mut rng, n);
            let mut expected = vec![M::zero(); n];
            expected[0] = M::one();
            for k in 0..5u64 {
                assert_eq!(ops.pow(&f, k, n), expected, "pow k={k} n={n}");
                assert_eq!(ops.pow_sparse(&f, k, n), expected, "pow_sparse k={k} n={n}");
                expected = naive_mul(&expected, &f, n);
            }
            let big = 1_234_567_890_123u64;
            assert_eq!(
                ops.pow(&f, big, n),
                ops.pow_sparse(&f, big, n),
                "pow big n={n}"
            );
            // leading zeros
            let mut g = vec![M::zero(); 3];
            g.extend(random_poly(&mut rng, n));
            let g3 = naive_mul(&naive_mul(&g, &g, n), &g, n);
            assert_eq!(ops.pow(&g, 3, n), g3);
            assert_eq!(ops.pow_sparse(&g, 3, n), g3);
            assert_eq!(ops.pow(&g, 1_000_000, n), vec![M::zero(); n]);
            assert_eq!(ops.pow(&[], 5, n), vec![M::zero(); n]);
            // sqrt of a square
            let sq = naive_mul(&f, &f, n);
            let r = ops.sqrt(&sq, n).unwrap();
            assert_eq!(naive_mul(&r, &r, n), sq, "sqrt n={n}");
            let r2 = ops.sqrt_sparse(&sq, n).unwrap();
            assert_eq!(naive_mul(&r2, &r2, n), sq, "sqrt_sparse n={n}");
            let mut shifted = vec![M::zero(); 4];
            shifted.extend_from_slice(&sq);
            let r = ops.sqrt(&shifted, n).unwrap();
            assert_eq!(naive_mul(&r, &r, n), shifted[..n].to_vec());
            let mut odd = vec![M::zero(); 1];
            odd.extend_from_slice(&sq);
            if n > 1 {
                assert_eq!(ops.sqrt(&odd, n), None);
                assert_eq!(ops.sqrt_sparse(&odd, n), None);
            }
            let mut non_residue = f.clone();
            non_residue[0] = M::one() + M::one() + M::one(); // 3 is a non-residue mod 998244353
            assert_eq!(ops.sqrt(&non_residue, n), None);
            assert_eq!(ops.sqrt(&[], n), Some(vec![M::zero(); n]));
        }
    }

    #[test]
    fn sparse_inputs_match_dense() {
        let mut rng = Random::new_with_seed(44);
        let mut ops = PolynomialOps::new();
        let n = 2000;
        let mut f = vec![M::zero(); n];
        f[0] = M::one();
        for _ in 0..5 {
            let i = rng.gen_range(1..n);
            f[i] = rnd(&mut rng);
        }
        assert_eq!(ops.inverse_sparse(&f, n), ops.inverse_series(&f, n));
        assert_eq!(ops.log_sparse(&f, n), ops.log(&f, n));
        assert_eq!(
            ops.pow_sparse(&f, 98_765_432_109, n),
            ops.pow(&f, 98_765_432_109, n)
        );
        let mut g = f.clone();
        g[0] = M::zero();
        assert_eq!(ops.exp_sparse(&g, n), ops.exp(&g, n));
        let mut sq = vec![M::zero(); n];
        sq[0] = M::from(4usize);
        sq[7] = rnd(&mut rng);
        assert_eq!(ops.sqrt_sparse(&sq, n), ops.sqrt(&sq, n));
    }

    #[test]
    fn large_round_trips() {
        let mut rng = Random::new_with_seed(45);
        let mut ops = PolynomialOps::new();
        let n = 1 << 15;
        let mut f = random_poly(&mut rng, n);
        f[0] = M::one();
        let inv = ops.inverse_series(&f, n);
        let mut prod = ops.mul_trunc(&f, &inv, n);
        prod[0] -= M::one();
        assert!(prod.iter().all(|&c| c == M::zero()));
        let lg = ops.log(&f, n);
        assert_eq!(ops.exp(&lg, n), f);
        let sq = ops.mul_trunc(&f, &f, n);
        let r = ops.sqrt(&sq, n).unwrap();
        assert!(r == f || r.iter().zip(&f).all(|(&a, &b)| a == -b));
        let p3 = ops.pow(&f, 3, n);
        assert_eq!(p3, ops.mul_trunc(&sq, &f, n));
    }
}
