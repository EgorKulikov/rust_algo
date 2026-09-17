use crate::misc::random::{RandomTrait, StaticRandom};
use crate::numbers::num_traits::primitive::Primitive;
use crate::when;

/// Montgomery arithmetic modulo an odd `n` with R = 2^64.
#[derive(Clone, Copy)]
pub struct Montgomery64 {
    n: u64,
    /// `-n^{-1} mod 2^64`
    n_inv: u64,
    /// `R^2 mod n`
    r2: u64,
    /// `R mod n`, the Montgomery form of one
    one: u64,
}

impl Montgomery64 {
    pub fn new(n: u64) -> Self {
        debug_assert!(n % 2 == 1);
        let mut inv = n;
        for _ in 0..6 {
            inv = inv.wrapping_mul(2u64.wrapping_sub(n.wrapping_mul(inv)));
        }
        let r = ((1u128 << 64) % n as u128) as u64;
        Self {
            n,
            n_inv: inv.wrapping_neg(),
            r2: (r as u128 * r as u128 % n as u128) as u64,
            one: r,
        }
    }

    #[inline(always)]
    fn reduce(self, t: u128) -> u64 {
        let lo = t as u64;
        let hi = (t >> 64) as u64;
        let m = lo.wrapping_mul(self.n_inv);
        let mn = m as u128 * self.n as u128;
        let r = hi as u128 + (mn >> 64) + (lo != 0) as u128;
        if r >= self.n as u128 {
            (r - self.n as u128) as u64
        } else {
            r as u64
        }
    }

    #[inline(always)]
    pub fn mul(self, a: u64, b: u64) -> u64 {
        self.reduce(a as u128 * b as u128)
    }

    #[inline(always)]
    pub fn add(self, a: u64, b: u64) -> u64 {
        let (s, overflow) = a.overflowing_add(b);
        if overflow || s >= self.n {
            s.wrapping_sub(self.n)
        } else {
            s
        }
    }

    #[inline(always)]
    pub fn sub(self, a: u64, b: u64) -> u64 {
        if a >= b {
            a - b
        } else {
            a.wrapping_sub(b).wrapping_add(self.n)
        }
    }

    pub fn to_mont(self, a: u64) -> u64 {
        self.mul(a % self.n, self.r2)
    }

    pub fn from_mont(self, a: u64) -> u64 {
        self.reduce(a as u128)
    }

    pub fn one(self) -> u64 {
        self.one
    }

    pub fn pow(self, mut base: u64, mut exp: u64) -> u64 {
        let mut res = self.one;
        while exp > 0 {
            if exp & 1 == 1 {
                res = self.mul(res, base);
            }
            base = self.mul(base, base);
            exp >>= 1;
        }
        res
    }
}

pub fn binary_gcd(mut a: u64, mut b: u64) -> u64 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    let shift = (a | b).trailing_zeros();
    a >>= a.trailing_zeros();
    loop {
        b >>= b.trailing_zeros();
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        b -= a;
        if b == 0 {
            return a << shift;
        }
    }
}

/// Deterministic Miller-Rabin for all `u64`.
pub fn is_prime(n: impl Primitive<u64>) -> bool {
    let n = n.to();
    if n < 2 {
        return false;
    }
    for p in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % p == 0 {
            return n == p;
        }
    }
    if n < 41 * 41 {
        return true;
    }
    let mont = Montgomery64::new(n);
    let s = (n - 1).trailing_zeros();
    let d = (n - 1) >> s;
    let one = mont.one();
    let minus_one = mont.sub(0, one);
    let bases: &[u64] = if n < 1 << 32 {
        &[2, 7, 61]
    } else {
        &[2, 325, 9375, 28178, 450775, 9780504, 1795265022]
    };
    'outer: for &a in bases {
        let a = a % n;
        if a == 0 {
            continue;
        }
        let mut x = mont.pow(mont.to_mont(a), d);
        if x == one || x == minus_one {
            continue;
        }
        for _ in 1..s {
            x = mont.mul(x, x);
            if x == minus_one {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

pub fn next_prime(mut n: u64) -> u64 {
    if n <= 2 {
        return 2;
    }
    n += 1 - (n & 1);
    while !is_prime(n) {
        n += 2;
    }
    n
}

/// Pollard's rho with Brent's cycle finding, batched gcds and Montgomery
/// arithmetic. Returns a nontrivial divisor of odd composite `n`, or `n`
/// when this attempt fails.
fn brent(n: u64, x0: u64, c: u64) -> u64 {
    let mont = Montgomery64::new(n);
    let c = mont.to_mont(c);
    let f = |x: u64| mont.add(mont.mul(x, x), c);
    let mut x = mont.to_mont(x0);
    let mut y = x;
    let mut ys = x;
    let mut q = mont.one();
    let mut g = 1;
    let mut l = 1usize;
    const M: usize = 512;
    while g == 1 {
        y = x;
        for _ in 0..l {
            x = f(x);
        }
        let mut k = 0;
        while k < l && g == 1 {
            ys = x;
            for _ in 0..M.min(l - k) {
                x = f(x);
                q = mont.mul(q, mont.sub(y, x));
            }
            g = binary_gcd(mont.from_mont(q), n);
            k += M;
        }
        l *= 2;
    }
    if g == n {
        loop {
            ys = f(ys);
            g = binary_gcd(mont.from_mont(mont.sub(y, ys)), n);
            if g != 1 {
                break;
            }
        }
    }
    g
}

pub fn find_divisor(n: u64) -> u64 {
    when! {
        n == 1 => 1,
        n % 2 == 0 => 2,
        is_prime(n) => n,
        else => {
            loop {
                let res = brent(
                    n,
                    StaticRandom.gen_range(2..n),
                    StaticRandom.gen_range(1..n),
                );
                if res != n {
                    return res;
                }
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::misc::random::Random;

    fn trial(n: u64) -> bool {
        if n < 2 {
            return false;
        }
        let mut i = 2;
        while i * i <= n {
            if n % i == 0 {
                return false;
            }
            i += 1;
        }
        true
    }

    #[test]
    fn small_numbers_match_trial_division() {
        for n in 0..200_000u64 {
            assert_eq!(is_prime(n), trial(n), "{n}");
        }
    }

    #[test]
    fn pseudoprimes_and_large_values() {
        for &n in &[
            2047u64,
            3277,
            4033,
            4681,
            8321,
            561,
            1105,
            1729,
            2465,
            2821,
            6601,
            8911,
            3_215_031_751,
            4_759_123_141,
            1_122_004_669_633,
            2_152_302_898_747,
            3_474_749_660_383,
            341_550_071_728_321,
            3_825_123_056_546_413_051,
            u64::MAX,
            4_294_967_291 * 4_294_967_279,
            999_999_999_989 * 2,
        ] {
            assert!(!is_prime(n), "{n} is composite");
        }
        for &p in &[
            2u64,
            3,
            61,
            1_000_000_007,
            998_244_353,
            4_294_967_291,
            4_294_967_311,
            (1u64 << 31) - 1,
            (1u64 << 61) - 1,
            u64::MAX - 58,
            9_223_372_036_854_775_783,
        ] {
            assert!(is_prime(p), "{p} is prime");
        }
        assert_eq!(next_prime(1_000_000_000_000), 1_000_000_000_039);
    }

    #[test]
    fn random_factorizations() {
        let mut rng = Random::new_with_seed(51);
        for _ in 0..300 {
            let n = rng.gen_range(2..1u64 << 62);
            let d = find_divisor(n);
            assert!(n % d == 0 && d >= 1, "{n} {d}");
            assert!(d == n || (1 < d && d < n));
            assert_eq!(d == n, is_prime(n));
        }
        // products of two large primes and prime squares
        for &(a, b) in &[
            (4_294_967_291u64, 4_294_967_279u64),
            (1_000_000_007, 998_244_353),
            (2_147_483_647, 2_147_483_647),
            (999_999_937, 999_999_937),
            (3_037_000_493, 3_037_000_453),
        ] {
            let n = a * b;
            let d = find_divisor(n);
            assert!(d == a || d == b, "{n} -> {d}");
        }
    }
}
