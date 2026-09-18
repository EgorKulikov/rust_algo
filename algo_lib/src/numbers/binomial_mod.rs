//! Binomial coefficients modulo an arbitrary (composite) modulus for huge
//! arguments: Granville's generalization of Lucas' theorem for each prime
//! power, combined by the Chinese remainder theorem. Setup costs O(sum of the
//! prime powers) time and memory; each query O(log n) per prime power.

use crate::numbers::primes::factorize::Factorize;

struct PrimePower {
    p: u64,
    e: u32,
    q: u64,
    /// `fact[i]` = product of all `j <= i` not divisible by `p`, modulo `q`.
    fact: Vec<u32>,
}

impl PrimePower {
    fn new(p: u64, e: u32) -> Self {
        let q = p.pow(e);
        assert!(q <= 1 << 26, "prime power {q} is too large for a table");
        let mut fact = vec![1u32; q as usize];
        for i in 1..q as usize {
            fact[i] = if i as u64 % p == 0 {
                fact[i - 1]
            } else {
                (fact[i - 1] as u64 * i as u64 % q) as u32
            };
        }
        Self { p, e, q, fact }
    }

    fn pow(&self, mut base: u64, mut exp: u64) -> u64 {
        let mut res = 1 % self.q;
        base %= self.q;
        while exp > 0 {
            if exp & 1 == 1 {
                res = res * base % self.q;
            }
            base = base * base % self.q;
            exp >>= 1;
        }
        res
    }

    /// `n!` with all factors `p` removed, modulo `q`.
    fn unit_factorial(&self, mut n: u64) -> u64 {
        let full = self.fact[self.q as usize - 1] as u64;
        let mut res = 1 % self.q;
        while n > 0 {
            res = res * self.pow(full, n / self.q) % self.q;
            res = res * self.fact[(n % self.q) as usize] as u64 % self.q;
            n /= self.p;
        }
        res
    }

    fn valuation(&self, mut n: u64) -> u64 {
        let mut v = 0;
        while n > 0 {
            n /= self.p;
            v += n;
        }
        v
    }

    fn inverse(&self, a: u64) -> u64 {
        // a is a unit modulo q; extended Euclid on signed 128-bit values
        let (mut r0, mut r1) = (self.q as i128, a as i128);
        let (mut t0, mut t1) = (0i128, 1i128);
        while r1 != 0 {
            let quot = r0 / r1;
            (r0, r1) = (r1, r0 - quot * r1);
            (t0, t1) = (t1, t0 - quot * t1);
        }
        debug_assert_eq!(r0, 1);
        t0.rem_euclid(self.q as i128) as u64
    }

    fn c(&self, n: u64, k: u64) -> u64 {
        let v = self.valuation(n) - self.valuation(k) - self.valuation(n - k);
        if v >= self.e as u64 {
            return 0;
        }
        let unit = self.unit_factorial(n)
            * self.inverse(self.unit_factorial(k) * self.unit_factorial(n - k) % self.q)
            % self.q;
        unit * self.p.pow(v as u32) % self.q
    }
}

pub struct BinomialMod {
    modulus: u64,
    parts: Vec<PrimePower>,
}

impl BinomialMod {
    /// Every prime power dividing `modulus` must be at most 2^26.
    pub fn new(modulus: u64) -> Self {
        assert!(modulus >= 1);
        let parts = modulus
            .prime_divisors()
            .into_iter()
            .map(|(p, e)| PrimePower::new(p, e as u32))
            .collect();
        Self { modulus, parts }
    }

    pub fn modulus(&self) -> u64 {
        self.modulus
    }

    /// `C(n, k) mod modulus`; zero when `k > n`.
    pub fn c(&self, n: u64, k: u64) -> u64 {
        if k > n {
            return 0;
        }
        // Garner-style CRT over the pairwise coprime prime powers.
        let mut res = 0u128;
        let mut prod = 1u128;
        for part in &self.parts {
            let r = part.c(n, k) as u128;
            let q = part.q as u128;
            // find t with res + prod * t == r (mod q)
            let diff = (r + q - res % q) % q;
            let t = diff * part.inverse((prod % q) as u64) as u128 % q;
            res += prod * t;
            prod *= q;
        }
        (res % self.modulus as u128) as u64
    }
}

#[cfg(test)]
mod tests {
    use super::BinomialMod;
    use crate::misc::random::{Random, RandomTrait};

    #[test]
    fn matches_pascal_triangle() {
        let limit = 70usize;
        for m in (1..=130u64).chain([
            256,
            243,
            1000,
            1024,
            3125,
            999_983,
            1_000_000,
            720_720,
            1 << 20,
        ]) {
            let binom = BinomialMod::new(m);
            let mut row = vec![1 % m];
            for n in 0..limit {
                for (k, &expected) in row.iter().enumerate() {
                    assert_eq!(binom.c(n as u64, k as u64), expected, "C({n},{k}) mod {m}");
                }
                assert_eq!(binom.c(n as u64, n as u64 + 1), 0);
                let mut next = vec![1 % m; n + 2];
                for k in 1..=n {
                    next[k] = (row[k - 1] + row[k]) % m;
                }
                row = next;
            }
        }
    }

    #[test]
    fn identities_for_huge_arguments() {
        let mut rng = Random::new_with_seed(221);
        for &m in &[
            2u64,
            12,
            1000,
            1 << 20,
            999_983,
            1_000_000,
            614_889_782_588_491_410 % 60_000_000 + 2,
            46_189 * 27 * 64,
        ] {
            let binom = BinomialMod::new(m);
            for _ in 0..200 {
                let n = rng.gen_range(2..1_000_000_000_000_000_000u64);
                assert_eq!(binom.c(n, 0), 1 % m);
                assert_eq!(binom.c(n, 1), n % m);
                let pairs = (n as u128 * (n as u128 - 1) / 2 % m as u128) as u64;
                assert_eq!(binom.c(n, 2), pairs);
                let k = rng.gen_range(1..n.min(1_000_000));
                assert_eq!(binom.c(n, k), binom.c(n, n - k));
                assert_eq!(
                    binom.c(n, k),
                    (binom.c(n - 1, k - 1) + binom.c(n - 1, k)) % m,
                    "n={n} k={k} m={m}"
                );
            }
        }
    }
}
