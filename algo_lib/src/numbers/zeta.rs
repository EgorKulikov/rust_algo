//! Zeta and Moebius transforms over the subset lattice and the divisibility
//! lattice, with the convolutions they diagonalize (AND, OR, XOR, gcd, lcm),
//! ranked subset convolution, and the exponential of a set power series.
//!
//! Subset-indexed arrays have a power-of-two length. Divisor-indexed arrays
//! use indices `1..len`; index 0 is ignored and left untouched.

use crate::numbers::fwht::FWHT;
use crate::numbers::num_traits::algebra::{AdditionMonoid, AdditionMonoidWithSub, SemiRingWithSub};
use crate::numbers::primes::sieve::primes;
use std::ops::DivAssign;

fn check_pow2(n: usize) {
    assert!(n.is_power_of_two(), "length must be a power of two");
}

/// `a[mask] = sum of a[sub] over sub ⊆ mask`
pub fn subset_zeta<T: AdditionMonoid + Copy>(a: &mut [T]) {
    check_pow2(a.len());
    let mut bit = 1;
    while bit < a.len() {
        for mask in 0..a.len() {
            if mask & bit != 0 {
                let low = a[mask ^ bit];
                a[mask] += low;
            }
        }
        bit <<= 1;
    }
}

/// Inverse of [`subset_zeta`].
pub fn subset_mobius<T: AdditionMonoidWithSub + Copy>(a: &mut [T]) {
    check_pow2(a.len());
    let mut bit = 1;
    while bit < a.len() {
        for mask in 0..a.len() {
            if mask & bit != 0 {
                let low = a[mask ^ bit];
                a[mask] -= low;
            }
        }
        bit <<= 1;
    }
}

/// `a[mask] = sum of a[sup] over sup ⊇ mask`
pub fn superset_zeta<T: AdditionMonoid + Copy>(a: &mut [T]) {
    check_pow2(a.len());
    let mut bit = 1;
    while bit < a.len() {
        for mask in 0..a.len() {
            if mask & bit == 0 {
                let high = a[mask | bit];
                a[mask] += high;
            }
        }
        bit <<= 1;
    }
}

/// Inverse of [`superset_zeta`].
pub fn superset_mobius<T: AdditionMonoidWithSub + Copy>(a: &mut [T]) {
    check_pow2(a.len());
    let mut bit = 1;
    while bit < a.len() {
        for mask in 0..a.len() {
            if mask & bit == 0 {
                let high = a[mask | bit];
                a[mask] -= high;
            }
        }
        bit <<= 1;
    }
}

fn pointwise<T: SemiRingWithSub + Copy>(mut a: Vec<T>, b: &[T]) -> Vec<T> {
    for (x, &y) in a.iter_mut().zip(b) {
        *x *= y;
    }
    a
}

/// `c[k] = sum of a[i] * b[j] over i | j == k`
pub fn or_convolution<T: SemiRingWithSub + Copy>(a: &[T], b: &[T]) -> Vec<T> {
    assert_eq!(a.len(), b.len());
    let (mut fa, mut fb) = (a.to_vec(), b.to_vec());
    subset_zeta(&mut fa);
    subset_zeta(&mut fb);
    let mut c = pointwise(fa, &fb);
    subset_mobius(&mut c);
    c
}

/// `c[k] = sum of a[i] * b[j] over i & j == k`
pub fn and_convolution<T: SemiRingWithSub + Copy>(a: &[T], b: &[T]) -> Vec<T> {
    assert_eq!(a.len(), b.len());
    let (mut fa, mut fb) = (a.to_vec(), b.to_vec());
    superset_zeta(&mut fa);
    superset_zeta(&mut fb);
    let mut c = pointwise(fa, &fb);
    superset_mobius(&mut c);
    c
}

/// `c[k] = sum of a[i] * b[j] over i ^ j == k`
pub fn xor_convolution<T: SemiRingWithSub + DivAssign + From<usize> + Copy>(
    a: &[T],
    b: &[T],
) -> Vec<T> {
    assert_eq!(a.len(), b.len());
    let (mut fa, mut fb) = (a.to_vec(), b.to_vec());
    fa.fwht(false);
    fb.fwht(false);
    let mut c = pointwise(fa, &fb);
    c.fwht(true);
    c
}

/// `a[n] = sum of a[d] over d | n`, for indices `1..a.len()`.
pub fn divisor_zeta<T: AdditionMonoid + Copy>(a: &mut [T]) {
    let n = a.len();
    for p in primes(n) {
        for k in 1..=(n - 1) / p {
            let low = a[k];
            a[k * p] += low;
        }
    }
}

/// Inverse of [`divisor_zeta`].
pub fn divisor_mobius<T: AdditionMonoidWithSub + Copy>(a: &mut [T]) {
    let n = a.len();
    for p in primes(n) {
        for k in (1..=(n - 1) / p).rev() {
            let low = a[k];
            a[k * p] -= low;
        }
    }
}

/// `a[n] = sum of a[m] over multiples m of n`, for indices `1..a.len()`.
pub fn multiple_zeta<T: AdditionMonoid + Copy>(a: &mut [T]) {
    let n = a.len();
    for p in primes(n) {
        for k in (1..=(n - 1) / p).rev() {
            let high = a[k * p];
            a[k] += high;
        }
    }
}

/// Inverse of [`multiple_zeta`].
pub fn multiple_mobius<T: AdditionMonoidWithSub + Copy>(a: &mut [T]) {
    let n = a.len();
    for p in primes(n) {
        for k in 1..=(n - 1) / p {
            let high = a[k * p];
            a[k] -= high;
        }
    }
}

/// `c[k] = sum of a[i] * b[j] over gcd(i, j) == k`, indices `1..len`.
pub fn gcd_convolution<T: SemiRingWithSub + Copy>(a: &[T], b: &[T]) -> Vec<T> {
    assert_eq!(a.len(), b.len());
    let (mut fa, mut fb) = (a.to_vec(), b.to_vec());
    multiple_zeta(&mut fa);
    multiple_zeta(&mut fb);
    let mut c = pointwise(fa, &fb);
    multiple_mobius(&mut c);
    c
}

/// `c[k] = sum of a[i] * b[j] over lcm(i, j) == k`, indices `1..len`; pairs
/// whose lcm is `>= len` are dropped.
pub fn lcm_convolution<T: SemiRingWithSub + Copy>(a: &[T], b: &[T]) -> Vec<T> {
    assert_eq!(a.len(), b.len());
    let (mut fa, mut fb) = (a.to_vec(), b.to_vec());
    divisor_zeta(&mut fa);
    divisor_zeta(&mut fb);
    let mut c = pointwise(fa, &fb);
    divisor_mobius(&mut c);
    c
}

/// `c[k] = sum of a[i] * b[j] over i | j == k, i & j == 0`, in O(2^n n^2).
pub fn subset_convolution<T: SemiRingWithSub + Copy>(a: &[T], b: &[T]) -> Vec<T> {
    assert_eq!(a.len(), b.len());
    check_pow2(a.len());
    let size = a.len();
    let n = size.trailing_zeros() as usize;
    let width = n + 1;
    // Ranked transforms: row `mask` holds one value per popcount.
    let rank = |src: &[T]| {
        let mut f = vec![T::zero(); size * width];
        for (mask, &x) in src.iter().enumerate() {
            f[mask * width + mask.count_ones() as usize] = x;
        }
        let mut bit = 1;
        while bit < size {
            for mask in 0..size {
                if mask & bit != 0 {
                    let top = mask.count_ones() as usize;
                    for r in 0..top {
                        let low = f[(mask ^ bit) * width + r];
                        f[mask * width + r] += low;
                    }
                }
            }
            bit <<= 1;
        }
        f
    };
    let fa = rank(a);
    let fb = rank(b);
    let mut fc = vec![T::zero(); size * width];
    for mask in 0..size {
        let top = mask.count_ones() as usize;
        let (ra, rb) = (&fa[mask * width..][..width], &fb[mask * width..][..width]);
        let rc = &mut fc[mask * width..][..width];
        // A row only feeds supersets, so after the Moebius step just the
        // ranks in `top..=n` are ever read.
        for i in 0..=top {
            for j in top - i..=(n - i).min(top) {
                rc[i + j] += ra[i] * rb[j];
            }
        }
    }
    let mut bit = 1;
    while bit < size {
        for mask in 0..size {
            if mask & bit != 0 {
                for r in mask.count_ones() as usize..width {
                    let low = fc[(mask ^ bit) * width + r];
                    fc[mask * width + r] -= low;
                }
            }
        }
        bit <<= 1;
    }
    (0..size)
        .map(|mask| fc[mask * width + mask.count_ones() as usize])
        .collect()
}

/// Exponential of a set power series with `a[0] == 0`:
/// `res[S] = sum over partitions of S into nonempty blocks of prod a[block]`.
pub fn sps_exp<T: SemiRingWithSub + Copy>(a: &[T]) -> Vec<T> {
    check_pow2(a.len());
    assert!(a[0] == T::zero(), "sps_exp needs a[0] == 0");
    let mut res = Vec::with_capacity(a.len());
    res.push(T::one());
    let mut half = 1;
    while half < a.len() {
        // Sets containing the new top element: pick its block, exp the rest.
        let upper = subset_convolution(&res, &a[half..2 * half]);
        res.extend(upper);
        half *= 2;
    }
    res
}

#[cfg(test)]
#[allow(clippy::needless_range_loop)]
mod tests {
    use super::*;
    use crate::misc::random::{Random, RandomTrait};
    use crate::numbers::gcd::gcd;
    use crate::numbers::mod_int::ModIntF as M;
    use crate::numbers::num_traits::algebra::{One, Zero};

    fn random_vec(rng: &mut Random, len: usize) -> Vec<M> {
        (0..len).map(|_| M::new(rng.gen_u128() as u32)).collect()
    }

    #[test]
    fn subset_lattice() {
        let mut rng = Random::new_with_seed(191);
        for n in 0..=7usize {
            let size = 1 << n;
            let a = random_vec(&mut rng, size);
            let b = random_vec(&mut rng, size);
            let mut z = a.clone();
            subset_zeta(&mut z);
            for mask in 0..size {
                let expected = (0..size)
                    .filter(|s| s & mask == *s)
                    .fold(M::zero(), |acc, s| acc + a[s]);
                assert_eq!(z[mask], expected);
            }
            subset_mobius(&mut z);
            assert_eq!(z, a);
            let mut z = a.clone();
            superset_zeta(&mut z);
            for mask in 0..size {
                let expected = (0..size)
                    .filter(|s| s & mask == mask)
                    .fold(M::zero(), |acc, s| acc + a[s]);
                assert_eq!(z[mask], expected);
            }
            superset_mobius(&mut z);
            assert_eq!(z, a);
            let naive = |op: &dyn Fn(usize, usize) -> Option<usize>| {
                let mut c = vec![M::zero(); size];
                for i in 0..size {
                    for j in 0..size {
                        if let Some(k) = op(i, j) {
                            c[k] += a[i] * b[j];
                        }
                    }
                }
                c
            };
            assert_eq!(or_convolution(&a, &b), naive(&|i, j| Some(i | j)));
            assert_eq!(and_convolution(&a, &b), naive(&|i, j| Some(i & j)));
            assert_eq!(xor_convolution(&a, &b), naive(&|i, j| Some(i ^ j)));
            assert_eq!(
                subset_convolution(&a, &b),
                naive(&|i, j| (i & j == 0).then_some(i | j)),
                "n={n}"
            );
        }
    }

    #[test]
    fn divisibility_lattice() {
        let mut rng = Random::new_with_seed(192);
        for len in [1usize, 2, 3, 10, 31, 64, 100] {
            let mut a = random_vec(&mut rng, len);
            let mut b = random_vec(&mut rng, len);
            a[0] = M::zero();
            b[0] = M::zero();
            let mut z = a.clone();
            divisor_zeta(&mut z);
            for n in 1..len {
                let expected = (1..=n)
                    .filter(|d| n % d == 0)
                    .fold(M::zero(), |acc, d| acc + a[d]);
                assert_eq!(z[n], expected);
            }
            divisor_mobius(&mut z);
            assert_eq!(z, a);
            let mut z = a.clone();
            multiple_zeta(&mut z);
            for n in 1..len {
                let expected = (1..len)
                    .filter(|m| m % n == 0)
                    .fold(M::zero(), |acc, m| acc + a[m]);
                assert_eq!(z[n], expected);
            }
            multiple_mobius(&mut z);
            assert_eq!(z, a);
            let mut by_gcd = vec![M::zero(); len];
            let mut by_lcm = vec![M::zero(); len];
            for i in 1..len {
                for j in 1..len {
                    let g = gcd(i, j);
                    by_gcd[g] += a[i] * b[j];
                    let l = i / g * j;
                    if l < len {
                        by_lcm[l] += a[i] * b[j];
                    }
                }
            }
            assert_eq!(gcd_convolution(&a, &b), by_gcd, "len={len}");
            assert_eq!(lcm_convolution(&a, &b), by_lcm, "len={len}");
        }
    }

    #[test]
    fn set_power_series_exp() {
        let mut rng = Random::new_with_seed(193);
        for n in 0..=6usize {
            let size = 1 << n;
            let mut a = random_vec(&mut rng, size);
            a[0] = M::zero();
            // naive: res[S] = sum over blocks B containing the lowest element of S of a[B] * res[S \\ B]
            let mut expected = vec![M::zero(); size];
            expected[0] = M::one();
            for s in 1..size {
                let low = s & s.wrapping_neg();
                let mut sub = s;
                while sub > 0 {
                    if sub & low != 0 {
                        let rest = expected[s ^ sub];
                        expected[s] += a[sub] * rest;
                    }
                    sub = (sub - 1) & s;
                }
            }
            assert_eq!(sps_exp(&a), expected, "n={n}");
        }
    }
}
