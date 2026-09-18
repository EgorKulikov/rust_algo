//! Modular arithmetic helpers on plain integers: primitive roots, discrete
//! logarithms for any modulus, best rational approximations, nim product.

use crate::collections::fx_hash_map::FxHashMap;
use crate::numbers::primes::factorize::Factorize;
use crate::numbers::primes::prime::{binary_gcd, Montgomery64};
use std::sync::OnceLock;

/// Smallest primitive root modulo an odd prime `p` (or 1 for `p == 2`).
pub fn primitive_root(p: u64) -> u64 {
    if p == 2 {
        return 1;
    }
    let mont = Montgomery64::new(p);
    let one = mont.one();
    let exponents: Vec<u64> = (p - 1)
        .prime_divisors()
        .into_iter()
        .map(|(q, _)| (p - 1) / q)
        .collect();
    (2..p)
        .find(|&g| {
            let gm = mont.to_mont(g);
            exponents.iter().all(|&e| mont.pow(gm, e) != one)
        })
        .unwrap()
}

fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
    (a as u128 * b as u128 % m as u128) as u64
}

/// Smallest `x >= 0` with `a^x == b (mod m)`, for any `m >= 1` (`0^0 = 1`).
/// Baby-step giant-step after stripping common factors, O(sqrt(m)).
pub fn discrete_log(a: u64, b: u64, m: u64) -> Option<u64> {
    assert!(m >= 1);
    let (a, b) = (a % m, b % m);
    if m == 1 || b == 1 % m {
        return Some(0);
    }
    // Strip gcd(a, m) until a becomes invertible modulo the remaining part.
    let mut m_cur = m;
    let mut b_cur = b;
    let mut scale = 1 % m; // a^steps / (product of stripped gcds), modulo m_cur
    let mut steps = 0u64;
    loop {
        let g = binary_gcd(a, m_cur);
        if g == 1 {
            break;
        }
        if b_cur % g != 0 {
            return None;
        }
        m_cur /= g;
        b_cur /= g;
        scale = mul_mod(scale % m_cur, (a / g) % m_cur, m_cur);
        steps += 1;
        // a^steps == b directly?
        if mul_pow(a, steps, m) == b {
            return Some(steps);
        }
        if m_cur == 1 {
            // everything is congruent from here on
            return (mul_pow(a, steps, m) == b).then_some(steps);
        }
    }
    let am = a % m_cur;
    // solve scale * am^y == b_cur (mod m_cur), gcd(am, m_cur) == 1
    let block = (m_cur as f64).sqrt() as u64 + 1;
    let mut table: FxHashMap<u64, u64> = FxHashMap::default();
    let mut cur = b_cur % m_cur;
    for j in 0..block {
        // b_cur * am^j; later entries overwrite, keeping the largest j
        table.insert(cur, j);
        cur = mul_mod(cur, am, m_cur);
    }
    let giant = mul_pow(am, block, m_cur);
    let mut lhs = scale % m_cur;
    for i in 1..=block + 1 {
        lhs = mul_mod(lhs, giant, m_cur);
        if let Some(&j) = table.get(&lhs) {
            let y = i * block - j;
            let x = y + steps;
            if mul_pow(a, x, m) == b {
                return Some(x);
            }
        }
    }
    None
}

fn mul_pow(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut res = 1 % m;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            res = mul_mod(res, base, m);
        }
        base = mul_mod(base, base, m);
        exp >>= 1;
    }
    res
}

/// Best rational approximations with numerator and denominator at most
/// `limit`: returns `(lower, upper)` fractions as `(num, den)` such that
/// `lower` is the largest fraction with `is_at_most(num, den)` true and
/// `upper` the smallest with it false. `is_at_most(p, q)` must be monotone
/// (true for small `p / q`, false for large) and true for `0 / 1`. The upper
/// bound is `(1, 0)` (infinity) if the predicate never fails.
pub fn stern_brocot_search(
    limit: u64,
    mut is_at_most: impl FnMut(u64, u64) -> bool,
) -> ((u64, u64), (u64, u64)) {
    let (mut lo, mut hi) = ((0u64, 1u64), (1u64, 0u64));
    loop {
        // Walk right as far as possible: lo + k * hi stays on the true side.
        let fits = |a: (u64, u64), b: (u64, u64), k: u64| -> Option<(u64, u64)> {
            let num = a.0.checked_add(b.0.checked_mul(k)?)?;
            let den = a.1.checked_add(b.1.checked_mul(k)?)?;
            (num <= limit && den <= limit).then_some((num, den))
        };
        let mut moved = false;
        for right in [true, false] {
            let (from, step) = if right { (lo, hi) } else { (hi, lo) };
            // largest k with from + k * step within the limit and on `right`'s side
            let mut k = 0u64;
            let mut jump = 1u64;
            while let Some(f) = fits(from, step, k + jump) {
                if is_at_most(f.0, f.1) != right {
                    break;
                }
                k += jump;
                jump = jump.saturating_mul(2);
            }
            while jump > 0 {
                if let Some(f) = fits(from, step, k + jump) {
                    if is_at_most(f.0, f.1) == right {
                        k += jump;
                    }
                }
                jump /= 2;
            }
            if k > 0 {
                moved = true;
                let f = fits(from, step, k).unwrap();
                if right {
                    lo = f;
                } else {
                    hi = f;
                }
            }
        }
        if !moved {
            return (lo, hi);
        }
    }
}

fn nim_table() -> &'static [[u8; 256]; 256] {
    static TABLE: OnceLock<Box<[[u8; 256]; 256]>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut t = Box::new([[0u8; 256]; 256]);
        // small products by the recursive definition on 4-bit then 8-bit halves
        fn slow(a: u64, b: u64, bits: u32) -> u64 {
            if a < 2 || b < 2 {
                return a * b;
            }
            let half = bits / 2;
            let mask = (1u64 << half) - 1;
            let (a1, a0, b1, b0) = (a >> half, a & mask, b >> half, b & mask);
            let high = slow(a1, b1, half);
            let low = slow(a0, b0, half);
            let cross = slow(a1 ^ a0, b1 ^ b0, half) ^ low;
            (cross << half) ^ low ^ slow(high, 1 << (half - 1), half)
        }
        for a in 0..256usize {
            for b in 0..256usize {
                t[a][b] = slow(a as u64, b as u64, 8) as u8;
            }
        }
        t
    })
}

fn nim_rec(a: u64, b: u64, bits: u32) -> u64 {
    if bits == 8 {
        return nim_table()[a as usize][b as usize] as u64;
    }
    let half = bits / 2;
    let mask = (1u64 << half) - 1;
    let (a1, a0, b1, b0) = (a >> half, a & mask, b >> half, b & mask);
    let high = nim_rec(a1, b1, half);
    let low = nim_rec(a0, b0, half);
    let cross = nim_rec(a1 ^ a0, b1 ^ b0, half) ^ low;
    (cross << half) ^ low ^ nim_rec(high, 1 << (half - 1), half)
}

/// Product of nimbers below 2^64 (multiplication in the field of order
/// 2^64 that extends XOR addition).
pub fn nim_product(a: u64, b: u64) -> u64 {
    nim_rec(a, b, 64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::misc::random::{Random, RandomTrait};
    use crate::numbers::primes::sieve::primes;

    #[test]
    fn primitive_roots() {
        for p in primes(400).into_iter().map(|p| p as u64).chain([
            998_244_353,
            1_000_000_007,
            4_611_686_018_427_387_847,
        ]) {
            let g = primitive_root(p);
            if p < 400 {
                let mut seen = std::collections::BTreeSet::new();
                let mut x = 1 % p;
                for _ in 0..p - 1 {
                    seen.insert(x);
                    x = x * g % p;
                }
                assert_eq!(seen.len() as u64, p - 1, "p={p} g={g}");
                // minimality
                for smaller in 1..g {
                    let mut y = 1 % p;
                    let mut order = 0;
                    loop {
                        y = y * smaller % p;
                        order += 1;
                        if y == 1 % p {
                            break;
                        }
                    }
                    assert!(order < p - 1 || p == 2);
                }
            }
        }
        assert_eq!(primitive_root(998_244_353), 3);
        assert_eq!(primitive_root(1_000_000_007), 5);
    }

    #[test]
    fn discrete_log_matches_brute_force() {
        for m in 1..=60u64 {
            for a in 0..m {
                for b in 0..m {
                    let mut expected = None;
                    let mut cur = 1 % m;
                    for x in 0..=2 * m {
                        if cur == b {
                            expected = Some(x);
                            break;
                        }
                        cur = cur * a % m;
                    }
                    assert_eq!(discrete_log(a, b, m), expected, "{a}^x = {b} mod {m}");
                }
            }
        }
        let mut rng = Random::new_with_seed(361);
        for _ in 0..50 {
            let m = rng.gen_range(2..1_000_000_000_000u64);
            let a = rng.gen_range(0..m);
            let x = rng.gen_range(0..1_000_000u64);
            let b = mul_pow(a, x, m);
            let got = discrete_log(a, b, m).unwrap();
            assert!(got <= x && mul_pow(a, got, m) == b);
        }
    }

    #[test]
    fn stern_brocot_brackets_a_fraction() {
        let mut rng = Random::new_with_seed(362);
        for _ in 0..300 {
            let limit = rng.gen_range(1..=40u64);
            let (tn, td) = (rng.gen_range(0..200u64), rng.gen_range(1..60u64));
            // predicate: p / q <= tn / td
            let (lo, hi) = stern_brocot_search(limit, |p, q| {
                (p as u128) * (td as u128) <= (tn as u128) * (q as u128)
            });
            let mut best_lo = (0u64, 1u64);
            let mut best_hi = (1u64, 0u64);
            for q in 1..=limit {
                for p in 0..=limit {
                    if p * td <= tn * q {
                        if p * best_lo.1 > best_lo.0 * q {
                            best_lo = (p, q);
                        }
                    } else if p * best_hi.1 < best_hi.0 * q {
                        best_hi = (p, q);
                    }
                }
            }
            assert_eq!(
                lo.0 * best_lo.1,
                best_lo.0 * lo.1,
                "lower limit={limit} target={tn}/{td}"
            );
            assert_eq!(
                hi.0 * best_hi.1,
                best_hi.0 * hi.1,
                "upper limit={limit} target={tn}/{td}"
            );
            assert_eq!(binary_gcd(lo.0, lo.1), 1);
        }
    }

    #[test]
    fn nim_product_is_a_field_multiplication() {
        assert_eq!(nim_product(2, 2), 3);
        assert_eq!(nim_product(4, 4), 6);
        assert_eq!(nim_product(2, 3), 1);
        assert_eq!(nim_product(16, 16), 24);
        let mut rng = Random::new_with_seed(363);
        for _ in 0..2000 {
            let (a, b, c) = (
                rng.gen_u128() as u64,
                rng.gen_u128() as u64,
                rng.gen_u128() as u64,
            );
            assert_eq!(nim_product(a, 1), a);
            assert_eq!(nim_product(a, 0), 0);
            assert_eq!(nim_product(a, b), nim_product(b, a));
            assert_eq!(
                nim_product(nim_product(a, b), c),
                nim_product(a, nim_product(b, c))
            );
            assert_eq!(nim_product(a, b ^ c), nim_product(a, b) ^ nim_product(a, c));
        }
        // small products agree with the mex definition
        let mut table = vec![vec![0u64; 16]; 16];
        for a in 0..16usize {
            for b in 0..16usize {
                let mut seen = std::collections::BTreeSet::new();
                for x in 0..a {
                    for y in 0..b {
                        seen.insert(table[x][b] ^ table[a][y] ^ table[x][y]);
                    }
                }
                table[a][b] = (0..).find(|v| !seen.contains(v)).unwrap();
                assert_eq!(nim_product(a as u64, b as u64), table[a][b], "{a} x {b}");
            }
        }
    }
}
