#![allow(clippy::all)]
#![allow(dead_code)]
use algo_lib::misc::random::{Random, RandomTrait};
use algo_lib::misc::value::DynamicValue;
use algo_lib::numbers::mod_int::convolution::Convolution;
use algo_lib::numbers::mod_int::fft::FFT;
use algo_lib::numbers::mod_int::prime_fft::PrimeFFT;
use algo_lib::numbers::mod_int::{BaseModInt, ModInt, ModInt64, ModInt7, ModIntF};
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_traits::invertible::Invertible;
use algo_lib::numbers::number_ext::Power;
use algo_lib::{dynamic_value, value};

#[path = "audit_modpoly_support/prime_fft_scalar.rs"]
mod prime_fft_scalar;
use prime_fft_scalar::PrimeFFT as ScalarPrimeFFT;

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[test]
fn modint_dynamic_basic() {
    dynamic_value!(DM: u32);
    type M = ModInt<DM>;
    let mut rng = Random::new_with_seed(1);
    for &m in &[1u32, 2, 3, 4, 7, 12, 97, 100, 1_000_000_007, 1 << 30, 998244353] {
        DM::set(m);
        assert_eq!(M::module(), m);
        let mm = m as i128;
        let specials: Vec<i64> = vec![0, 1, -1, m as i64, m as i64 - 1, -(m as i64), m as i64 + 1, i64::MAX, i64::MIN, i64::MIN + 1, i32::MAX as i64, i32::MIN as i64, u32::MAX as i64];
        for &s in &specials {
            assert_eq!(M::from(s).val() as i128, (s as i128).rem_euclid(mm), "from i64 {s} mod {m}");
            if s >= i32::MIN as i64 && s <= i32::MAX as i64 {
                assert_eq!(M::from(s as i32).val() as i128, (s as i128).rem_euclid(mm), "from i32 {s} mod {m}");
            }
            if s >= 0 && s <= u32::MAX as i64 {
                assert_eq!(M::from(s as u32).val() as i128, (s as i128).rem_euclid(mm));
                assert_eq!(M::from(s as usize).val() as i128, (s as i128).rem_euclid(mm));
            }
        }
        for &u in &[(1usize << 63) - 1, 1usize << 62, u32::MAX as usize + 1] {
            assert_eq!(M::from(u).val() as u128, u as u128 % m as u128, "from usize {u} mod {m}");
        }
        for _ in 0..2000 {
            let a = rng.gen_range(0..m.min(50).max(1)) as i64 + if rng.gen_bool() { 0 } else { (m as i64 - 50).max(0) };
            let b = rng.gen_range(0..m.min(50).max(1)) as i64 + if rng.gen_bool() { 0 } else { (m as i64 - 50).max(0) };
            let (a, b) = (a.rem_euclid(m as i64), b.rem_euclid(m as i64));
            let (x, y) = (M::from(a), M::from(b));
            assert_eq!((x + y).val() as i128, (a + b) as i128 % mm);
            assert_eq!((x - y).val() as i128, (a - b).rem_euclid(m as i64) as i128);
            assert_eq!((x * y).val() as i128, (a as i128 * b as i128) % mm);
            assert_eq!((-x).val() as i128, (-a).rem_euclid(m as i64) as i128);
            let inv = y.inv();
            if gcd(b as u128, m as u128) == 1 {
                let inv = inv.expect("inverse must exist");
                assert_eq!((inv * y).val(), 1 % m);
                assert_eq!(((x / y) * y).val(), x.val());
            } else {
                assert!(inv.is_none(), "inv of {b} mod {m}");
            }
            let e = rng.gen_range(0..20u64);
            let mut expect = 1i128 % mm;
            for _ in 0..e {
                expect = expect * a as i128 % mm;
            }
            assert_eq!(x.power(e).val() as i128, expect);
        }
    }
}

#[test]
fn modint64_basic() {
    dynamic_value!(DM64: u64);
    type M = ModInt64<DM64>;
    let mut rng = Random::new_with_seed(2);
    for &m in &[1u64, 2, 3, 12, 1_000_000_007, (1 << 61) - 1, 1_000_000_000_000_000_003, (1 << 62) - 57, 4611686018427387904 - 1] {
        DM64::set(m);
        let mm = m as i128;
        for &s in &[0i64, 1, -1, i64::MAX, i64::MIN, i64::MIN + 1, m as i64, -(m as i64), m as i64 - 1] {
            assert_eq!(M::from(s).val() as i128, (s as i128).rem_euclid(mm), "from i64 {s} mod {m}");
        }
        for &s in &[0i128, -1, i128::MAX, i128::MIN, i128::MIN + 1, mm, -mm, mm * mm] {
            assert_eq!(M::from(s).val() as i128, s.rem_euclid(mm), "from i128 {s} mod {m}");
        }
        for &u in &[usize::MAX, 1usize << 63, (1usize << 63) - 1] {
            assert_eq!(M::from(u).val() as u128, u as u128 % m as u128, "from usize {u} mod {m}");
        }
        for &u in &[u64::MAX, 1u64 << 63, m, m + 1, m.wrapping_sub(1) % m.max(1)] {
            assert_eq!(M::from(u).val(), u % m);
        }
        for _ in 0..2000 {
            let a = (rng.gen_u128() % m as u128) as u64;
            let b = if rng.gen_bool() { m - 1 - (rng.gen_range(0..10u64) % m) } else { (rng.gen_u128() % m as u128) as u64 };
            let (x, y) = (M::new(a), M::new(b));
            assert_eq!((x + y).val() as u128, (a as u128 + b as u128) % m as u128);
            assert_eq!((x - y).val() as i128, (a as i128 - b as i128).rem_euclid(mm));
            assert_eq!((x * y).val() as u128, (a as u128 * b as u128) % m as u128);
            if gcd(b as u128, m as u128) == 1 {
                assert_eq!((y.inv().unwrap() * y).val(), 1 % m);
            } else {
                assert!(y.inv().is_none());
            }
        }
    }
}

#[test]
fn bug_fft_new_large_ntt_prime() {
    // 2013265921 = 15 * 2^27 + 1 is a prime below i32::MAX
    value!(P: u32 = 2013265921);
    type M = ModInt<P>;
    let mut fft = FFT::<M>::new();
    let a: Vec<M> = (0..100usize).map(|i| M::from(i + 1)).collect();
    let c = fft.multiply(&a, &a);
    let mut expected = vec![M::zero(); 199];
    for i in 0..100 {
        for j in 0..100 {
            expected[i + j] += a[i] * a[j];
        }
    }
    assert!(c == expected);
}

#[test]
fn bug_modint_from_usize_high_bit() {
    type M = ModInt7;
    let u = usize::MAX; // 2^64 - 1
    assert_eq!(M::from(u).val() as u128, u as u128 % 1_000_000_007);
}

#[test]
fn bug_modint64_large_modulus_signed() {
    value!(Big: u64 = 9223372036854775783); // largest prime below 2^63
    type M = ModInt64<Big>;
    let m = 9223372036854775783i128;
    for &s in &[0i64, 1, -1, i64::MAX, i64::MIN, 5_000_000_000_000_000_000] {
        assert_eq!(M::from(s).val() as i128, (s as i128).rem_euclid(m), "from {s}");
    }
}

fn naive_mul<T, M: BaseModInt<T>>(a: &[M], b: &[M]) -> Vec<M> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut r = vec![M::zero(); a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            r[i + j] += x * y;
        }
    }
    r
}

fn rnd_vec<M: BaseModInt<u32>>(rng: &mut Random, n: usize, mode: u32) -> Vec<M> {
    let p = M::module();
    (0..n)
        .map(|_| match mode {
            0 => M::from(rng.gen_range(0..p)),
            1 => M::from(p - 1),
            2 => M::from(rng.gen_range(0..2u32) * (p - 1)),
            _ => M::from(rng.gen_range(0..3u32) % p),
        })
        .collect()
}

fn ntt_lengths() -> Vec<usize> {
    let mut v = vec![0usize, 1, 2, 3, 7, 8, 9, 15, 16, 17];
    v.extend(31..=65);
    v.extend([100, 127, 128, 129, 200, 255, 256, 257, 500]);
    v
}

macro_rules! gen_check_prime_fft {
    ($name:ident, $fft:ident) => {
fn $name<M: BaseModInt<u32> + std::fmt::Debug>(seed: u64) {
    let mut rng = Random::new_with_seed(seed);
    let mut fft = $fft::<M>::new();
    let max_len = 1usize << (M::module() - 1).trailing_zeros();
    let lens = ntt_lengths();
    // big call first so the cached table is larger than later calls need
    for round in 0..3 {
        for &n in &lens {
            for &m in &lens {
                if n.min(m) > 60 && (n + m - 1).next_power_of_two() > max_len {
                    continue;
                }
                if round > 0 && rng.gen_range(0..4u32) != 0 {
                    continue;
                }
                let mode = rng.gen_range(0..4u32);
                let a: Vec<M> = rnd_vec(&mut rng, n, mode);
                let b: Vec<M> = rnd_vec(&mut rng, m, mode);
                let expected = naive_mul(&a, &b);
                assert_eq!(fft.multiply(&a, &b), expected, "n={n} m={m} mode={mode}");
                if n == m {
                    assert_eq!(fft.multiply(&a, &a), naive_mul(&a, &a), "square n={n}");
                }
                // fix len
                let len = rng.gen_range(0..n + m + 5);
                let mut res = vec![M::one(); len];
                fft.multiply_fix_len(&a, &b, &mut res);
                let mut want = expected.clone();
                want.resize(len, M::zero());
                assert_eq!(res, want, "fix n={n} m={m} len={len}");
            }
        }
    }
    // unbalanced
    for &(n, m) in &[(1usize, 3000usize), (61, 1500), (2000, 61), (62, 963), (1000, 1000)] {
        if (n + m - 1).next_power_of_two() > max_len {
            continue;
        }
        let a: Vec<M> = rnd_vec(&mut rng, n, 1);
        let b: Vec<M> = rnd_vec(&mut rng, m, 1);
        assert_eq!(fft.multiply(&a, &b), naive_mul(&a, &b));
        let a: Vec<M> = rnd_vec(&mut rng, 70, 0);
        assert_eq!(fft.multiply(&a, &a), naive_mul(&a, &a));
    }
    // inverse series
    for n in [0usize, 1, 2, 3, 60, 119, 120, 121, 122, 127, 128, 129, 200, 256, 257, 300, 511, 512, 513, 1000] {
        if n.next_power_of_two() > max_len && n > 120 {
            continue;
        }
        for flen in [1usize, 2, 3, n / 2 + 1, n.max(1), n + 5, 2 * n + 7] {
            let mode = rng.gen_range(0..4u32);
            let mut f: Vec<M> = rnd_vec(&mut rng, flen, mode);
            if f[0] == M::zero() {
                f[0] = M::one();
            }
            let g = fft.inverse_series(&f, n);
            assert_eq!(g.len(), n);
            let mut prod = naive_mul(&f, &g);
            prod.resize(n, M::zero());
            for i in 0..n {
                assert_eq!(prod[i], if i == 0 { M::one() } else { M::zero() }, "inv n={n} flen={flen} i={i}");
            }
        }
    }
    // power
    let a: Vec<M> = rnd_vec(&mut rng, 5, 0);
    let mut e = vec![M::one()];
    for k in 0..12 {
        assert_eq!(fft.power(&a, k), e, "power {k}");
        e = naive_mul(&e, &a);
    }
}

    };
}
gen_check_prime_fft!(check_prime_fft, PrimeFFT);
gen_check_prime_fft!(check_prime_fft_scalar, ScalarPrimeFFT);

#[test]
fn prime_fft_primes() {
    check_prime_fft::<ModIntF>(11);
    value!(P1: u32 = 167772161);
    check_prime_fft::<ModInt<P1>>(12);
    value!(P2: u32 = 469762049);
    check_prime_fft::<ModInt<P2>>(13);
    value!(P3: u32 = 754974721);
    check_prime_fft::<ModInt<P3>>(14);
    value!(P4: u32 = 1004535809);
    check_prime_fft::<ModInt<P4>>(15);
    value!(P5: u32 = 1073479681); // just below 2^30? 
    let _ = P5 {};
    value!(P6: u32 = 12289);
    check_prime_fft::<ModInt<P6>>(16);
    value!(P7: u32 = 7681);
    check_prime_fft::<ModInt<P7>>(17);
    value!(P8: u32 = 65537);
    check_prime_fft::<ModInt<P8>>(18);
    value!(P9: u32 = 257);
    check_prime_fft::<ModInt<P9>>(19);
    value!(P10: u32 = 985661441);
    check_prime_fft::<ModInt<P10>>(20);
    value!(P11: u32 = 1045430273);
    check_prime_fft::<ModInt<P11>>(21);
}

#[test]
fn prime_fft_primes_scalar() {
    check_prime_fft_scalar::<ModIntF>(11);
    value!(P1: u32 = 167772161);
    check_prime_fft_scalar::<ModInt<P1>>(12);
    value!(P2: u32 = 469762049);
    check_prime_fft_scalar::<ModInt<P2>>(13);
    value!(P3: u32 = 754974721);
    check_prime_fft_scalar::<ModInt<P3>>(14);
    value!(P4: u32 = 1004535809);
    check_prime_fft_scalar::<ModInt<P4>>(15);
    value!(P5: u32 = 1073479681); // just below 2^30? 
    let _ = P5 {};
    value!(P6: u32 = 12289);
    check_prime_fft_scalar::<ModInt<P6>>(16);
    value!(P7: u32 = 7681);
    check_prime_fft_scalar::<ModInt<P7>>(17);
    value!(P8: u32 = 65537);
    check_prime_fft_scalar::<ModInt<P8>>(18);
    value!(P9: u32 = 257);
    check_prime_fft_scalar::<ModInt<P9>>(19);
    value!(P10: u32 = 985661441);
    check_prime_fft_scalar::<ModInt<P10>>(20);
    value!(P11: u32 = 1045430273);
    check_prime_fft_scalar::<ModInt<P11>>(21);
}


#[test]
fn prime_fft_scalar_vs_simd_large() {
    let mut rng = Random::new_with_seed(99);
    let mut fast = PrimeFFT::<ModIntF>::new();
    let mut slow = ScalarPrimeFFT::<ModIntF>::new();
    for &(n, m, mode) in &[(1usize << 17, 1usize << 17, 1u32), (100_000, 33_333, 0), (70, 200_000, 2), (1 << 18, 1 << 18, 0)] {
        let a: Vec<ModIntF> = rnd_vec(&mut rng, n, mode);
        let b: Vec<ModIntF> = rnd_vec(&mut rng, m, mode);
        let c = fast.multiply(&a, &b);
        assert_eq!(c, slow.multiply(&a, &b));
        if mode == 1 {
            for i in [0usize, 5, n - 1, n + m - 2] {
                assert_eq!(c[i], ModIntF::from((i + 1).min(n + m - 1 - i)));
            }
        }
        let mut f = a.clone();
        f[0] = ModIntF::one();
        assert_eq!(fast.inverse_series(&f, n), slow.inverse_series(&f, n));
    }
}

fn check_generic_fft<M: BaseModInt<u32> + std::fmt::Debug>(seed: u64) {
    let mut rng = Random::new_with_seed(seed);
    let mut fft = FFT::<M>::new();
    let mut conv = Convolution::<M>::new();
    let lens = ntt_lengths();
    for &n in &lens {
        for &m in &lens {
            if rng.gen_range(0..3u32) != 0 {
                continue;
            }
            let mode = rng.gen_range(0..4u32);
            let a: Vec<M> = rnd_vec(&mut rng, n, mode);
            let b: Vec<M> = rnd_vec(&mut rng, m, mode);
            let expected = naive_mul(&a, &b);
            assert_eq!(fft.multiply(&a, &b), expected, "fft n={n} m={m} mode={mode}");
            assert_eq!(conv.multiply(&a, &b), expected, "conv n={n} m={m} mode={mode}");
            let len = rng.gen_range(0..n + m + 5);
            let mut want = expected.clone();
            want.resize(len, M::zero());
            let mut res = vec![M::one(); len];
            fft.multiply_fix_len(&a, &b, &mut res);
            assert_eq!(res, want, "fft fix n={n} m={m} len={len}");
            let mut res = vec![M::one(); len];
            conv.multiply_fix_len(&a, &b, &mut res);
            assert_eq!(res, want, "conv fix n={n} m={m} len={len}");
            let mut res = vec![M::one(); len];
            fft.multiply_res(&a, &b, &mut res);
            let mut want = expected.clone();
            if want.len() < len {
                want.resize(len, M::zero());
            }
            assert_eq!(res, want, "fft res n={n} m={m} len={len}");
        }
    }
    let a: Vec<M> = rnd_vec(&mut rng, 3, 0);
    let mut e = vec![M::one()];
    for k in 0..70 {
        assert_eq!(fft.power(&a, k), e, "power {k}");
        assert_eq!(conv.power(&a, k), e, "cpower {k}");
        e = naive_mul(&e, &a);
    }
}

#[test]
fn generic_fft_moduli() {
    check_generic_fft::<ModInt7>(31);
    check_generic_fft::<ModIntF>(32);
    value!(C1: u32 = 1);
    check_generic_fft::<ModInt<C1>>(33);
    value!(C2: u32 = 2);
    check_generic_fft::<ModInt<C2>>(34);
    value!(C3: u32 = 2147483647);
    check_generic_fft::<ModInt<C3>>(35);
    value!(C4: u32 = 2147483646);
    check_generic_fft::<ModInt<C4>>(36);
    value!(C5: u32 = 257);
    check_generic_fft::<ModInt<C5>>(37);
    value!(C6: u32 = 1 << 30);
    check_generic_fft::<ModInt<C6>>(38);
    value!(C7: u32 = 769); // 769 - 1 = 768 = 256 * 3: prime with rank 8
    check_generic_fft::<ModInt<C7>>(39);
    value!(C8: u32 = 129 * 257); // composite with p - 1 divisible by 128
    check_generic_fft::<ModInt<C8>>(40);
}

#[test]
fn fft_max_values_big() {
    // max-size-ish max-value inputs through CRT
    value!(C3: u32 = 2147483647);
    type M = ModInt<C3>;
    let n = 1 << 19;
    let a = vec![M::from(2147483646u32); n];
    let mut fft = FFT::<M>::new();
    let c = fft.multiply(&a, &a);
    for i in [0usize, 1, n - 1, n, 2 * n - 2, 12345] {
        let cnt = (i + 1).min(2 * n - 1 - i);
        assert_eq!(c[i], M::from(cnt), "i={i}");
    }
    // u32 residues with ModInt64 storage
    value!(L: u64 = 4294967291);
    type W = ModInt64<L>;
    let a = vec![W::new(4294967290); n];
    let mut fft = FFT::<W, u64>::new();
    let c = fft.multiply(&a, &a);
    for i in [0usize, 1, n - 1, n, 2 * n - 2, 12345] {
        let cnt = (i + 1).min(2 * n - 1 - i);
        assert_eq!(c[i], W::from(cnt), "i={i}");
    }
}

#[test]
fn integer_convolution() {
    use algo_lib::numbers::mod_int::convolution::convolution;
    let mut rng = Random::new_with_seed(77);
    for _ in 0..300 {
        let n = rng.gen_range(0..150usize);
        let m = rng.gen_range(0..150usize);
        let mode = rng.gen_range(0..3u32);
        let gen = |rng: &mut Random| -> i32 {
            match mode {
                0 => rng.gen_range(i32::MIN..=i32::MAX),
                1 => if rng.gen_bool() { i32::MIN } else { i32::MAX },
                _ => i32::MIN,
            }
        };
        let a: Vec<i32> = (0..n).map(|_| gen(&mut rng)).collect();
        let b: Vec<i32> = (0..m).map(|_| gen(&mut rng)).collect();
        let mut e = if n == 0 || m == 0 { vec![] } else { vec![0i128; n + m - 1] };
        for i in 0..n {
            for j in 0..m {
                e[i + j] += a[i] as i128 * b[j] as i128;
            }
        }
        assert_eq!(convolution(&a, &b), e);
    }
    let n = 1 << 20;
    let a = vec![i32::MIN; n];
    let c = convolution(&a, &a);
    for i in [0usize, n - 1, 2 * n - 2, 777] {
        let cnt = (i + 1).min(2 * n - 1 - i) as i128;
        assert_eq!(c[i], cnt << 62);
    }
}

#[test]
fn bug_modint_new_signed_overflow_a() {
    value!(P: u32 = 2147483647);
    type M = ModInt<P>;
    assert_eq!(M::from(5i32).val(), 5);
}

#[test]
fn bug_modint_new_signed_overflow_b() {
    value!(P: u32 = 2013265921); // 15 * 2^27 + 1
    type M = ModInt<P>;
    assert_eq!(M::from(200_000_000i32).val(), 200_000_000);
}

#[test]
fn modint_big_modulus_other_ops() {
    // everything except new_signed for moduli in (2^30, 2^31)
    value!(P: u32 = 2147483647);
    type M = ModInt<P>;
    let mut rng = Random::new_with_seed(5);
    let m = 2147483647i128;
    for _ in 0..5000 {
        let a = rng.gen_range(0..2147483647u32);
        let b = 2147483646 - rng.gen_range(0..5u32);
        let (x, y) = (M::new(a), M::new(b));
        assert_eq!((x + y).val() as i128, (a as i128 + b as i128) % m);
        assert_eq!((x - y).val() as i128, (a as i128 - b as i128).rem_euclid(m));
        assert_eq!((y - x).val() as i128, (b as i128 - a as i128).rem_euclid(m));
        assert_eq!((x * y).val() as i128, (a as i128 * b as i128) % m);
        assert_eq!((y.inv().unwrap() * y).val(), 1);
        assert_eq!(M::from(-(a as i64)).val() as i128, (-(a as i128)).rem_euclid(m));
    }
}

// ---------------- mod_utils ----------------
use algo_lib::numbers::mod_int::mod_utils::{
    combinations, combinations_arr, inverse_factorials, inverses, mod_sqrt, Combinations,
};

fn is_prime_small(n: u32) -> bool {
    n >= 2 && (2..n).take_while(|d| d * d <= n).all(|d| n % d != 0)
}

#[test]
fn mod_utils_small_primes() {
    dynamic_value!(DP: u32);
    type M = ModInt<DP>;
    for p in (2..200u32).chain([65537, 998244353, 1_000_000_007, 1073741789]) {
        if !is_prime_small(p) {
            continue;
        }
        DP::set(p);
        // mod_sqrt over all residues for small p, random for large
        let mut squares = std::collections::HashSet::new();
        if p < 1000 {
            for x in 0..p {
                squares.insert((x as u64 * x as u64 % p as u64) as u32);
            }
        }
        let mut rng = Random::new_with_seed(p as u64);
        for it in 0..p.min(3000) {
            let a = if p < 1000 { it } else { rng.gen_range(0..p) };
            let r = mod_sqrt::<u32, M>(M::new(a));
            match r {
                Some(r) => assert_eq!((r * r).val(), a, "sqrt {a} mod {p}"),
                None => {
                    if p < 1000 {
                        assert!(!squares.contains(&a), "sqrt {a} mod {p} should exist");
                    } else {
                        assert_ne!(M::new(a).power((p - 1) / 2), M::one());
                    }
                }
            }
            if p >= 1000 {
                let sq = M::new(a) * M::new(a);
                let r = mod_sqrt::<u32, M>(sq).expect("square must have root");
                assert_eq!(r * r, sq);
            }
        }
        let len = (p as usize).min(300);
        let inv: Vec<M> = inverses(len);
        for i in 1..len {
            assert_eq!((inv[i] * M::from(i)).val(), 1, "inverse {i} mod {p}");
        }
        for l in 0..4 {
            let v: Vec<M> = inverses(l);
            assert_eq!(v.len(), l);
            let v: Vec<M> = inverse_factorials(l);
            assert_eq!(v.len(), l);
        }
        let ifact: Vec<M> = inverse_factorials(len);
        let mut f = M::one();
        for i in 0..len {
            if i > 0 {
                f *= M::from(i);
            }
            assert_eq!((ifact[i] * f).val(), 1);
        }
        // binomials
        let comb = Combinations::<M>::new(len);
        let n = len.min(60);
        let mut pas = vec![vec![0u64; n + 1]; n + 1];
        for i in 0..n {
            pas[i][0] = 1 % p as u64;
            for j in 1..=i {
                pas[i][j] = (pas[i - 1][j - 1] + pas[i - 1][j]) % p as u64;
            }
        }
        let arr = combinations_arr::<u32, M>(n);
        for i in 0..n {
            for j in 0..n {
                assert_eq!(comb.c(i, j).val() as u64, pas[i][j], "C({i},{j}) mod {p}");
                assert_eq!(arr[(i, j)].val() as u64, pas[i][j]);
                assert_eq!(combinations::<u32, M>(i, j).val() as u64, pas[i][j], "combinations({i},{j}) mod {p}");
                if j <= i {
                    assert_eq!((comb.c_inv(i, j) * comb.c(i, j)).val(), 1);
                }
            }
        }
        // comb_with_rep(n slots, k items) = number of multisets
        for slots in 0..8usize {
            for items in 0..8usize {
                if slots + items >= n.max(1) {
                    continue;
                }
                let e = if slots == 0 { (items == 0) as u64 } else { pas[slots + items - 1][items] };
                assert_eq!(comb.comb_with_rep(slots, items).val() as u64, e);
            }
        }
    }
}

// ---------------- fps ----------------
use algo_lib::numbers::polynomial::{evaluate, PolynomialOps, ProductTree};

trait TestMod<T>: BaseModInt<T> + std::fmt::Debug {
    fn rnd(rng: &mut Random) -> Self;
    fn modulus_u64() -> u64;
}
impl<V: algo_lib::misc::value::Value<u32>> TestMod<u32> for ModInt<V> {
    fn rnd(rng: &mut Random) -> Self {
        Self::new(rng.gen_range(0..V::val()))
    }
    fn modulus_u64() -> u64 {
        V::val() as u64
    }
}
impl<V: algo_lib::misc::value::Value<u64>> TestMod<u64> for ModInt64<V> {
    fn rnd(rng: &mut Random) -> Self {
        Self::new((rng.gen_u128() % V::val() as u128) as u64)
    }
    fn modulus_u64() -> u64 {
        V::val()
    }
}

fn nmul<T, M: BaseModInt<T>>(a: &[M], b: &[M], n: usize) -> Vec<M> {
    let mut r = vec![M::zero(); n];
    for (i, &x) in a.iter().enumerate().take(n) {
        for (j, &y) in b.iter().enumerate().take(n - i) {
            r[i + j] += x * y;
        }
    }
    r
}

fn rpoly<T, M: TestMod<T>>(rng: &mut Random, len: usize) -> Vec<M> {
    let mode = rng.gen_range(0..4u32);
    (0..len)
        .map(|_| match mode {
            0 | 1 => M::rnd(rng),
            2 => {
                if rng.gen_range(0..4u32) == 0 {
                    M::rnd(rng)
                } else {
                    M::zero()
                }
            }
            _ => -M::one(),
        })
        .collect()
}

fn npow<T, M: BaseModInt<T>>(f: &[M], mut k: u64, n: usize) -> Vec<M> {
    let mut res = vec![M::zero(); n];
    if n == 0 {
        return res;
    }
    res[0] = M::one();
    let mut base: Vec<M> = f.iter().take(n).copied().collect();
    while k > 0 {
        if k & 1 == 1 {
            res = nmul(&res, &base, n);
        }
        base = nmul(&base, &base, n);
        k >>= 1;
    }
    res
}

fn pick_n(rng: &mut Random, limit: usize) -> usize {
    match rng.gen_range(0..10u32) {
        0 => rng.gen_range(0..4usize).min(limit),
        1..=6 => rng.gen_range(0..40usize).min(limit),
        7 | 8 => rng.gen_range(55..140usize).min(limit),
        _ => rng.gen_range(100..400usize).min(limit),
    }
}

fn check_fps<T: Into<u64> + algo_lib::numbers::num_traits::algebra::IntegerSemiRing + Copy, M: TestMod<T>>(
    seed: u64,
    iters: usize,
) where
    M: From<usize>,
{
    let mut rng = Random::new_with_seed(seed);
    let mut ops = PolynomialOps::<T, M>::new();
    let p = M::modulus_u64();
    let limit = if p > u32::MAX as u64 { 60 } else if p > 100000 { 100000 } else { p as usize };
    for it in 0..iters {
        let n = pick_n(&mut rng, limit);
        let flen = match rng.gen_range(0..4u32) {
            0 => n,
            1 => rng.gen_range(0..n + 1),
            2 => n + rng.gen_range(0..20usize),
            _ => rng.gen_range(0..3usize),
        };
        let f: Vec<M> = rpoly(&mut rng, flen);
        let glen = pick_n(&mut rng, limit);
        let g: Vec<M> = rpoly(&mut rng, glen);
        let ctx = format!("it={it} n={n} flen={flen}");
        // mul_trunc
        assert_eq!(ops.mul_trunc(&f, &g, n), nmul(&f, &g, n), "mul_trunc {ctx}");
        // inverse
        if !f.is_empty() && f[0] != M::zero() {
            let inv = ops.inverse_series(&f, n);
            assert_eq!(inv.len(), n);
            let prod = nmul(&f, &inv, n);
            for i in 0..n {
                assert_eq!(prod[i], if i == 0 { M::one() } else { M::zero() }, "inverse {ctx}");
            }
            assert_eq!(ops.inverse_sparse(&f, n), inv, "inverse_sparse {ctx}");
        }
        // log / exp
        if !f.is_empty() {
            let mut u = f.clone();
            u[0] = M::one();
            let lg = ops.log(&u, n);
            assert_eq!(lg.len(), n);
            // check: lg' * u = u'
            if n > 0 {
                assert_eq!(lg[0], M::zero());
                let dl = ops.derivative(&lg);
                let ut: Vec<M> = u.iter().take(n).copied().collect();
                let mut du = ops.derivative(&ut);
                du.resize(n - 1, M::zero());
                assert_eq!(nmul(&dl, &u, n - 1), du, "log {ctx}");
            }
            assert_eq!(ops.log_sparse(&u, n), lg, "log_sparse {ctx}");
            let ex = ops.exp(&lg, n);
            let mut ut: Vec<M> = u.iter().take(n).copied().collect();
            ut.resize(n, M::zero());
            assert_eq!(ex, ut, "exp(log) {ctx}");
        }
        {
            let mut z = f.clone();
            if !z.is_empty() {
                z[0] = M::zero();
            }
            let ex = ops.exp(&z, n);
            assert_eq!(ex.len(), n);
            if n > 0 {
                // ex' = z' * ex
                assert_eq!(ex[0], M::one());
                let de = ops.derivative(&ex);
                let zt: Vec<M> = z.iter().take(n).copied().collect();
                let dz = ops.derivative(&zt);
                assert_eq!(nmul(&dz, &ex, n - 1), de, "exp {ctx}");
            }
            assert_eq!(ops.exp_sparse(&z, n), ex, "exp_sparse {ctx}");
        }
        // pow
        {
            let mut h = f.clone();
            let lead_zeros = rng.gen_range(0..4usize);
            if rng.gen_bool() {
                for x in h.iter_mut().take(lead_zeros) {
                    *x = M::zero();
                }
            }
            let k: u64 = match rng.gen_range(0..6u32) {
                0 => 0,
                1 => 1,
                2 => rng.gen_range(0..10u64),
                3 => rng.gen_range(0..1000u64),
                4 => rng.gen_range(0..p.min(1 << 40)),
                _ => p - 1,
            };
            let expected = npow(&h, k, n);
            assert_eq!(ops.pow(&h, k, n), expected, "pow k={k} {ctx}");
            assert_eq!(ops.pow_sparse(&h, k, n), expected, "pow_sparse k={k} {ctx}");
            // huge k with zero constant term
            if !h.is_empty() && h[0] == M::zero() && n > 0 {
                let big = u64::MAX - rng.gen_range(0..5u64);
                assert_eq!(ops.pow(&h, big, n), vec![M::zero(); n], "pow huge {ctx}");
                assert_eq!(ops.pow_sparse(&h, big, n), vec![M::zero(); n]);
            }
        }
        // sqrt
        {
            let r: Vec<M> = rpoly(&mut rng, flen);
            let mut r = r;
            let lz = rng.gen_range(0..3usize);
            for x in r.iter_mut().take(lz) {
                *x = M::zero();
            }
            let sq = nmul(&r, &r, 2 * r.len() + 1);
            for (name, res) in [("sqrt", ops.sqrt(&sq, n)), ("sqrt_sparse", ops.sqrt_sparse(&sq, n))] {
                let s = res.unwrap_or_else(|| panic!("{name} must exist {ctx}"));
                assert_eq!(s.len(), n);
                assert_eq!(nmul(&s, &s, n), nmul(&sq, &[M::one()], n), "{name} {ctx} r={r:?}");
            }
            // arbitrary f: if Some then must square to f; if None verify a reason
            for (name, res) in [("sqrt", ops.sqrt(&f, n)), ("sqrt_sparse", ops.sqrt_sparse(&f, n))] {
                match res {
                    Some(s) => assert_eq!(nmul(&s, &s, n), nmul(&f, &[M::one()], n), "{name} arbitrary {ctx}"),
                    None => {
                        let t = f.iter().take(n).position(|&c| c != M::zero()).unwrap();
                        assert!(t % 2 == 1 || f[t].power((p - 1) / 2) != M::one() && p > 2, "{name} none {ctx}");
                    }
                }
            }
        }
        // taylor shift
        {
            let c = if rng.gen_range(0..4u32) == 0 { M::zero() } else { M::rnd(&mut rng) };
            let ff: Vec<M> = f.iter().take(limit).copied().collect();
            let sh = ops.taylor_shift(&ff, c);
            assert_eq!(sh.len(), ff.len());
            for _ in 0..3 {
                let x = M::rnd(&mut rng);
                assert_eq!(evaluate(&sh, x), evaluate(&ff, x + c), "taylor {ctx}");
            }
        }
        // sampling shift
        {
            let ff: Vec<M> = f.iter().take(limit.min(150)).copied().collect();
            let ys: Vec<M> = (0..ff.len()).map(|i| evaluate(&ff, M::from(i))).collect();
            let start = match rng.gen_range(0..4u32) {
                0 => M::from(rng.gen_range(0..ff.len() + 3)),
                1 => -M::from(rng.gen_range(0..ff.len() + 3)),
                _ => M::rnd(&mut rng),
            };
            let count = rng.gen_range(0..200usize);
            let got = ops.shift_sampling_points(&ys, start, count);
            assert_eq!(got.len(), count);
            for k in 0..count {
                let e = if ff.is_empty() { M::zero() } else { evaluate(&ff, start + M::from(k)) };
                assert_eq!(got[k], e, "sampling shift {ctx} k={k} start={start:?} count={count}");
            }
        }
    }
}

#[test]
fn fps_f() {
    check_fps::<u32, ModIntF>(101, 1500);
}

#[test]
fn fps_7() {
    check_fps::<u32, ModInt7>(102, 1000);
}

#[test]
fn fps_small_ntt_prime() {
    value!(P: u32 = 7681);
    check_fps::<u32, ModInt<P>>(103, 1000);
    value!(Q: u32 = 769);
    check_fps::<u32, ModInt<Q>>(104, 1000);
}

#[test]
fn fps_small_primes() {
    value!(P: u32 = 101);
    check_fps::<u32, ModInt<P>>(105, 1000);
    value!(Q: u32 = 5);
    check_fps::<u32, ModInt<Q>>(106, 1000);
    value!(R: u32 = 3);
    check_fps::<u32, ModInt<R>>(107, 1000);
}

#[test]
fn fps_wide() {
    value!(P: u64 = 1_000_000_000_000_000_003);
    check_fps::<u64, ModInt64<P>>(108, 400);
}

#[test]
fn fps_u32_modulus_in_modint64() {
    value!(Q: u64 = 4294967291);
    check_fps::<u64, ModInt64<Q>>(109, 400);
}

#[test]
fn bug_fps_wide_modulus_mul_trunc_panics() {
    value!(P: u64 = 1_000_000_000_000_000_003);
    type M = ModInt64<P>;
    let mut ops = PolynomialOps::<u64, M>::new();
    let f: Vec<M> = (0..130usize).map(|i| M::from(i + 1)).collect();
    // PolynomialOps::multiply works (quadratic fallback) ...
    assert_eq!(ops.multiply(&f, &f), naive_mul(&f, &f));
    // ... but every fps routine that reaches mul_trunc panics
    let inv = ops.inverse_series(&f, 130);
    let prod = nmul(&f, &inv, 130);
    assert!(prod[0] == M::one() && prod[1..].iter().all(|&c| c == M::zero()));
}

// ---------------- polynomial / interpolation ----------------
use algo_lib::numbers::interpolation::Interpolation;

fn trimmed<T, M: BaseModInt<T>>(mut v: Vec<M>) -> Vec<M> {
    while v.last() == Some(&M::zero()) {
        v.pop();
    }
    v
}

fn check_poly<T: Into<u64> + algo_lib::numbers::num_traits::algebra::IntegerSemiRing + Copy, M: TestMod<T> + From<usize>>(
    seed: u64,
    iters: usize,
) {
    let mut rng = Random::new_with_seed(seed);
    let mut ops = PolynomialOps::<T, M>::new();
    let p = M::modulus_u64();
    for it in 0..iters {
        let big = it % 10 == 0;
        let alen = if big { rng.gen_range(0..500usize) } else { rng.gen_range(0..20usize) };
        let blen = if big { rng.gen_range(1..300usize) } else { rng.gen_range(1..12usize) };
        let mut a: Vec<M> = rpoly(&mut rng, alen);
        let mut b: Vec<M> = rpoly(&mut rng, blen);
        if b.iter().all(|&c| c == M::zero()) {
            b[0] = M::one();
        }
        // trailing zeros
        if rng.gen_bool() {
            let z = rng.gen_range(0..4usize);
            a.extend(vec![M::zero(); z]);
            b.extend(vec![M::zero(); rng.gen_range(0..4usize)]);
        }
        assert_eq!(ops.multiply(&a, &b), naive_mul(&a, &b));
        let (q, r) = ops.div_rem(&a, &b);
        let bt = trimmed(b.clone());
        assert!(r.len() < bt.len(), "rem degree it={it}");
        assert!(q.last() != Some(&M::zero()) && r.last() != Some(&M::zero()));
        let mut back = naive_mul(&q, &bt);
        back.resize(back.len().max(r.len()), M::zero());
        for (x, &y) in back.iter_mut().zip(&r) {
            *x += y;
        }
        assert_eq!(trimmed(back), trimmed(a.clone()), "div_rem it={it}");
        let at = trimmed(a.clone());
        assert_eq!(q.len(), if at.len() >= bt.len() { at.len() - bt.len() + 1 } else { 0 });

        // product tree
        let points = if big { rng.gen_range(1..300usize) } else { rng.gen_range(1..70usize) };
        let mut xs: Vec<M> = (0..points).map(|_| M::rnd(&mut rng)).collect();
        match rng.gen_range(0..5u32) {
            0 => xs[0] = M::zero(),
            1 => {
                for x in xs.iter_mut() {
                    *x = M::from(rng.gen_range(0..3usize));
                }
            }
            2 => {
                let v = xs[0];
                for x in xs.iter_mut() {
                    *x = v;
                }
            }
            _ => {}
        }
        let tree = ProductTree::new(&xs, &mut ops);
        let mut prod = vec![M::one()];
        for &x in &xs {
            prod = naive_mul(&prod, &[-x, M::one()]);
        }
        assert_eq!(tree.product(), &prod[..]);
        for poly in [&a, &b, &prod] {
            let got = tree.evaluate(poly, &mut ops);
            let expected: Vec<M> = xs.iter().map(|&x| evaluate(poly, x)).collect();
            assert_eq!(got, expected, "tree evaluate it={it} points={points} len={}", poly.len());
        }

        // interpolation through arbitrary distinct points
        let cnt = if big { rng.gen_range(0..200usize) } else { rng.gen_range(0..45usize) };
        let cnt = cnt.min(p as usize);
        let mut seen = std::collections::HashSet::new();
        let mut pts: Vec<(M, M)> = Vec::new();
        while pts.len() < cnt {
            let x = if p < 1000 || rng.gen_bool() { M::from(rng.gen_range(0..(p.min(400)) as usize)) } else { M::rnd(&mut rng) };
            if seen.insert(Into::<usize>::into(x)) {
                pts.push((x, M::rnd(&mut rng)));
            }
        }
        let ip = Interpolation::<T, M>::from_points(&pts);
        for &(x, y) in &pts {
            assert_eq!(ip.calculate(x), y, "from_points it={it}");
        }
        // Lagrange brute at random queries
        let qn = if rng.gen_bool() { rng.gen_range(0..10usize) } else { rng.gen_range(33..80usize) };
        let qs: Vec<M> = (0..qn).map(|_| if rng.gen_bool() { M::rnd(&mut rng) } else { M::from(rng.gen_range(0..5usize)) }).collect();
        let lagrange = |x: M| -> M {
            let mut res = M::zero();
            for (i, &(xi, yi)) in pts.iter().enumerate() {
                let mut num = yi;
                let mut den = M::one();
                for (j, &(xj, _)) in pts.iter().enumerate() {
                    if i != j {
                        num *= x - xj;
                        den *= xi - xj;
                    }
                }
                res += num / den;
            }
            res
        };
        let many = ip.calculate_many(&qs);
        for (i, &x) in qs.iter().enumerate() {
            let e = lagrange(x);
            assert_eq!(many[i], e, "calculate_many it={it}");
            assert_eq!(ip.calculate(x), e);
        }
        // consecutive
        let cnt = cnt.min(p as usize);
        let values: Vec<M> = (0..cnt).map(|_| M::rnd(&mut rng)).collect();
        let cpts: Vec<(M, M)> = values.iter().enumerate().map(|(i, &v)| (M::from(i), v)).collect();
        let lag2 = |x: M| -> M {
            let mut res = M::zero();
            for (i, &(xi, yi)) in cpts.iter().enumerate() {
                let mut num = yi;
                let mut den = M::one();
                for (j, &(xj, _)) in cpts.iter().enumerate() {
                    if i != j {
                        num *= x - xj;
                        den *= xi - xj;
                    }
                }
                res += num / den;
            }
            res
        };
        let ip = Interpolation::<T, M>::new(values.clone());
        // calculate before and after calculate_many (cached polynomial)
        for &x in qs.iter().take(5) {
            assert_eq!(ip.calculate(x), lag2(x), "consecutive calculate it={it} cnt={cnt}");
        }
        let many = ip.calculate_many(&qs);
        for (i, &x) in qs.iter().enumerate() {
            assert_eq!(many[i], lag2(x), "consecutive many it={it}");
            assert_eq!(ip.calculate(x), lag2(x));
        }
    }
}

#[test]
fn poly_f() {
    check_poly::<u32, ModIntF>(201, 400);
}

#[test]
fn poly_7() {
    check_poly::<u32, ModInt7>(202, 300);
}

#[test]
fn poly_small() {
    value!(P: u32 = 769);
    check_poly::<u32, ModInt<P>>(203, 300);
    value!(Q: u32 = 7);
    check_poly::<u32, ModInt<Q>>(204, 300);
    value!(R: u32 = 2);
    check_poly::<u32, ModInt<R>>(205, 300);
}

#[test]
fn poly_wide() {
    value!(P: u64 = 1_000_000_000_000_000_003);
    check_poly::<u64, ModInt64<P>>(206, 100);
}

// ---------------- linear recurrence ----------------
use algo_lib::numbers::linear_recurrence::{berlekamp_massey, guess_kth_term, kth_term};

fn unroll<T, M: BaseModInt<T>>(init: &[M], c: &[M], len: usize) -> Vec<M> {
    let mut s = init.to_vec();
    while s.len() < len {
        let i = s.len();
        let mut v = M::zero();
        for (j, &x) in c.iter().enumerate() {
            v += x * s[i - 1 - j];
        }
        s.push(v);
    }
    s
}

/// brute force minimal recurrence length: smallest L such that some c of length L works
fn brute_min_len(s: &[u32], p: u32) -> usize {
    // enumerate all c in p^L
    for l in 0..=s.len() {
        let total = (p as u64).pow(l as u32);
        'outer: for code in 0..total {
            let mut c = vec![0u32; l];
            let mut x = code;
            for v in c.iter_mut() {
                *v = (x % p as u64) as u32;
                x /= p as u64;
            }
            for i in l..s.len() {
                let mut v = 0u32;
                for j in 0..l {
                    v = (v + c[j] * s[i - 1 - j]) % p;
                }
                if v != s[i] {
                    continue 'outer;
                }
            }
            return l;
        }
    }
    unreachable!()
}

#[test]
fn berlekamp_massey_bruteforce_small_field() {
    dynamic_value!(BP: u32);
    type M = ModInt<BP>;
    let mut rng = Random::new_with_seed(301);
    for &p in &[2u32, 3, 5] {
        BP::set(p);
        for _ in 0..3000 {
            let n = rng.gen_range(0..8usize);
            let s: Vec<u32> = (0..n).map(|_| if rng.gen_range(0..3u32) == 0 { rng.gen_range(0..p) } else { rng.gen_range(0..2u32) }).collect();
            let sm: Vec<M> = s.iter().map(|&x| M::new(x)).collect();
            let c = berlekamp_massey(&sm);
            // must generate s
            for i in c.len()..n {
                let mut v = M::zero();
                for (j, &x) in c.iter().enumerate() {
                    v += x * sm[i - 1 - j];
                }
                assert_eq!(v, sm[i], "BM does not generate s={s:?} p={p} c={c:?}");
            }
            assert_eq!(c.len(), brute_min_len(&s, p), "BM not minimal s={s:?} p={p}");
        }
    }
}

fn check_recurrence<T: Into<u64> + Copy, M: TestMod<T>>(seed: u64, iters: usize) {
    let mut rng = Random::new_with_seed(seed);
    let mut ops = PolynomialOps::<T, M>::new();
    for it in 0..iters {
        let d = if it % 25 == 0 { rng.gen_range(60..130usize) } else { rng.gen_range(0..8usize) };
        let mut c: Vec<M> = rpoly(&mut rng, d);
        if rng.gen_range(0..4u32) == 0 && d > 0 {
            c[d - 1] = M::zero(); // trailing zero coefficient
        }
        let extra = rng.gen_range(0..4usize);
        let init_raw: Vec<M> = match rng.gen_range(0..4u32) {
            0 => vec![M::zero(); d + extra],
            _ => rpoly(&mut rng, d + extra),
        };
        let total = 2 * d + 12 + extra;
        // (a) init with extra terms that do NOT follow the recurrence: documented semantics
        // say the recurrence continues after init. Known bug -> check the wrapper fix only.
        let s_raw = unroll(&init_raw, &c, total);
        let off = init_raw.len() - d;
        for k in 0..total {
            let got = if k < init_raw.len() { init_raw[k] } else { kth_term(&init_raw[off..], &c, (k - off) as u64, &mut ops) };
            assert_eq!(got, s_raw[k], "kth (suffix) it={it} d={d} k={k}");
        }
        // (b) init consistent with the recurrence
        let s = unroll(&init_raw[..d], &c, total);
        let init: Vec<M> = s[..d + extra].to_vec();
        for k in 0..total {
            assert_eq!(kth_term(&init, &c, k as u64, &mut ops), s[k], "kth it={it} d={d} k={k}");
        }
        // BM on a sufficiently long prefix recovers a recurrence consistent with the rest
        let bm = berlekamp_massey(&s);
        assert!(bm.len() <= d.max(init.len()), "BM too long it={it}");
        for i in bm.len()..s.len() {
            let mut v = M::zero();
            for (j, &x) in bm.iter().enumerate() {
                v += x * s[i - 1 - j];
            }
            assert_eq!(v, s[i], "BM it={it}");
        }
        // big k: compare against kth_term through doubling consistency: s[k] via matrix-free check
        let big = (1u64 << 40) + rng.gen_range(0..1000u64);
        if d > 0 && d <= 8 {
            // window of d+1 consecutive terms must satisfy the recurrence
            let w: Vec<M> = (0..=d as u64).map(|t| kth_term(&init, &c, big + t, &mut ops)).collect();
            let mut v = M::zero();
            for (j, &x) in c.iter().enumerate() {
                v += x * w[d - 1 - j];
            }
            assert_eq!(v, w[d], "big k window it={it}");
            assert_eq!(guess_kth_term(&s, big, &mut ops), w[0], "guess it={it}");
        }
        assert_eq!(kth_term(&init, &c, u64::MAX, &mut ops), kth_term(&init, &c, u64::MAX, &mut ops));
    }
    // eventually zero / all zero / short sequences
    let z = vec![M::zero(); 10];
    assert!(berlekamp_massey(&z).is_empty());
    assert_eq!(guess_kth_term(&z, 1 << 50, &mut ops), M::zero());
    let e: Vec<M> = vec![];
    assert!(berlekamp_massey(&e).is_empty());
    assert_eq!(guess_kth_term(&e, 5, &mut ops), M::zero());
    let mut ez = vec![M::zero(); 12];
    ez[0] = M::one();
    ez[2] = -M::one();
    let c = berlekamp_massey(&ez);
    assert_eq!(c.len(), 3);
    assert_eq!(guess_kth_term(&ez, 1 << 50, &mut ops), M::zero());
    assert_eq!(guess_kth_term(&ez, 2, &mut ops), -M::one());
    assert_eq!(guess_kth_term(&[M::one()], 0, &mut ops), M::one());
}

#[test]
fn recurrence_f() {
    check_recurrence::<u32, ModIntF>(302, 600);
}

#[test]
fn recurrence_7() {
    check_recurrence::<u32, ModInt7>(303, 400);
}

#[test]
fn recurrence_small_and_wide() {
    value!(P: u32 = 3);
    check_recurrence::<u32, ModInt<P>>(304, 400);
    value!(R: u32 = 2);
    check_recurrence::<u32, ModInt<R>>(306, 400);
    value!(Q: u64 = 1_000_000_000_000_000_003);
    check_recurrence::<u64, ModInt64<Q>>(305, 200);
}

// ---------------- combinatorial series ----------------
#[test]
fn combinatorial_series_vs_dp() {
    fn run<M: TestMod<u32> + From<usize>>() {
        let mut ops = PolynomialOps::<u32, M>::new();
        let limit = 150usize;
        let mut c1 = vec![vec![M::zero(); limit + 2]; limit + 2];
        let mut s2 = vec![vec![M::zero(); limit + 2]; limit + 2];
        let mut binom = vec![vec![M::zero(); limit + 2]; limit + 2];
        c1[0][0] = M::one();
        s2[0][0] = M::one();
        for n in 0..=limit {
            binom[n][0] = M::one();
            for k in 1..=n {
                binom[n][k] = binom[n - 1][k - 1] + binom[n - 1][k];
                c1[n][k] = c1[n - 1][k - 1] + M::from(n - 1) * c1[n - 1][k];
                s2[n][k] = s2[n - 1][k - 1] + M::from(k) * s2[n - 1][k];
            }
        }
        for n in (0..70).chain([100, 127, 128, 129, 150]) {
            assert_eq!(ops.rising_factorial(n), c1[n][..=n].to_vec(), "rising {n}");
            let sf = ops.stirling_first(n);
            for k in 0..=n {
                let e = if (n - k) % 2 == 1 { -c1[n][k] } else { c1[n][k] };
                assert_eq!(sf[k], e);
            }
            assert_eq!(ops.stirling_second(n), s2[n][..=n].to_vec(), "stirling2 {n}");
        }
        // bell
        let mut bell = vec![M::zero(); limit + 1];
        bell[0] = M::one();
        for n in 1..=limit {
            for k in 0..n {
                let t = binom[n - 1][k] * bell[k];
                bell[n] += t;
            }
        }
        // partitions
        let mut part = vec![M::zero(); limit + 1];
        part[0] = M::one();
        for item in 1..=limit {
            for s in item..=limit {
                let t = part[s - item];
                part[s] += t;
            }
        }
        // bernoulli: sum_{k<=n} C(n+1,k) B_k = 0
        let mut bern = vec![M::zero(); limit + 1];
        bern[0] = M::one();
        for n in 1..limit {
            let mut acc = M::zero();
            for k in 0..n {
                acc += binom[n + 1][k] * bern[k];
            }
            bern[n] = -acc / M::from(n + 1);
        }
        for n in (0..70).chain([100, 121, 128, 129, 150]) {
            assert_eq!(ops.bell_numbers(n), bell[..n].to_vec(), "bell {n}");
            assert_eq!(ops.partition_numbers(n), part[..n].to_vec(), "partition {n}");
            assert_eq!(ops.bernoulli_numbers(n), bern[..n].to_vec(), "bernoulli {n}");
        }
        let mut rng = Random::new_with_seed(401);
        for _ in 0..300 {
            let n = rng.gen_range(0..200usize);
            let cnt = rng.gen_range(0..30usize);
            let hi = rng.gen_range(1..250usize);
            let items: Vec<usize> = (0..cnt).map(|_| rng.gen_range(0..hi)).collect();
            let mut dp = vec![M::zero(); n.max(1)];
            dp[0] = M::one();
            let mut dp = if n == 0 { vec![] } else { dp };
            for &s in &items {
                for t in (s..n).rev() {
                    let v = dp[t - s];
                    dp[t] += v;
                }
            }
            assert_eq!(ops.count_subset_sums(&items, n), dp, "subset sums n={n} items={items:?}");
        }
    }
    run::<ModIntF>();
    run::<ModInt7>();
}

#[test]
fn bug_kth_term_ignores_extra_init_terms() {
    type M = ModIntF;
    let mut ops = PolynomialOps::<u32, M>::new();
    // s = 5, 1, 1, then s[i] = s[i-1] + s[i-2]: 5 1 1 2 3 5 8
    let init = [M::new(5), M::new(1), M::new(1)];
    let c = [M::new(1), M::new(1)];
    assert_eq!(kth_term(&init, &c, 2, &mut ops), M::new(1));
    assert_eq!(kth_term(&init, &c, 3, &mut ops), M::new(2));
}

// ---------------- binomial_mod ----------------
use algo_lib::numbers::binomial_mod::BinomialMod;

#[test]
fn binomial_mod_pascal() {
    let limit = 130usize;
    for m in (1..=200u64).chain([256, 243, 625, 1024, 2187, 1000, 720720, 999, 1 << 20, 3 * (1 << 20), 1_000_000_007u64 * 0 + 65536 * 81]) {
        let b = BinomialMod::new(m);
        assert_eq!(b.modulus(), m);
        let mut pas = vec![vec![0u64; limit + 1]; limit + 1];
        for n in 0..=limit {
            pas[n][0] = 1 % m;
            for k in 1..=n {
                pas[n][k] = (pas[n - 1][k - 1] + pas[n - 1][k]) % m;
            }
        }
        for n in 0..=limit {
            for k in 0..=limit {
                assert_eq!(b.c(n as u64, k as u64), pas[n][k], "C({n},{k}) mod {m}");
            }
        }
    }
}

fn binom_mod_prime_power_brute(n: u64, k: u64, p: u64, q: u64) -> u64 {
    // legendre + product of units, O(k)
    assert!(k <= 200000);
    let mut v: i64 = 0;
    let mut unit: u128 = 1;
    let mut den: u128 = 1;
    for i in 0..k {
        let mut a = n - i;
        while a % p == 0 {
            a /= p;
            v += 1;
        }
        unit = unit * (a % q) as u128 % q as u128;
        let mut b = i + 1;
        while b % p == 0 {
            b /= p;
            v -= 1;
        }
        den = den * (b % q) as u128 % q as u128;
    }
    // inverse of den mod q via brute euler: den^(phi-1)
    let phi = q / p * (p - 1);
    let mut inv = 1u128;
    let mut base = den;
    let mut e = phi - 1;
    while e > 0 {
        if e & 1 == 1 {
            inv = inv * base % q as u128;
        }
        base = base * base % q as u128;
        e >>= 1;
    }
    let mut res = unit * inv % q as u128;
    for _ in 0..v {
        res = res * p as u128 % q as u128;
    }
    res as u64
}

#[test]
fn binomial_mod_huge_n() {
    let mut rng = Random::new_with_seed(501);
    for &(p, e) in &[(2u64, 1u32), (2, 3), (2, 10), (2, 20), (3, 1), (3, 5), (5, 4), (7, 2), (101, 2), (9973, 1), (1_000_003, 1)] {
        let q = p.pow(e);
        let b = BinomialMod::new(q);
        for _ in 0..200 {
            let n = match rng.gen_range(0..4u32) {
                0 => u64::MAX - rng.gen_range(0..100u64),
                1 => rng.gen_range(0..1u64 << 62),
                2 => q * rng.gen_range(1..1000u64) + rng.gen_range(0..3u64),
                _ => p.pow(rng.gen_range(0..(63.0 / (p as f64).log2()) as u32)) .saturating_add(rng.gen_range(0..5u64)),
            };
            let k = rng.gen_range(0..2000u64).min(n);
            let expected = binom_mod_prime_power_brute(n, k, p, q);
            assert_eq!(b.c(n, k), expected, "C({n},{k}) mod {p}^{e}");
            assert_eq!(b.c(n, n - k), expected, "C({n},n-{k}) mod {p}^{e}");
        }
        assert_eq!(b.c(5, 6), 0);
        assert_eq!(b.c(0, 0), 1 % q);
        assert_eq!(b.c(u64::MAX, u64::MAX), 1 % q);
        assert_eq!(b.c(u64::MAX, 0), 1 % q);
        assert_eq!(b.c(u64::MAX, 1), u64::MAX % q);
    }
    // composite with CRT
    let m = 2u64.pow(10) * 3u64.pow(5) * 7 * 101;
    let b = BinomialMod::new(m);
    for _ in 0..300 {
        let n = rng.gen_range(0..u64::MAX);
        let k = rng.gen_range(0..500u64);
        let r = b.c(n, k);
        for &(p, q) in &[(2u64, 1024u64), (3, 243), (7, 7), (101, 101)] {
            assert_eq!(r % q, binom_mod_prime_power_brute(n, k, p, q));
        }
    }
}

// ---------------- zeta / fwht ----------------
use algo_lib::numbers::fwht::FWHT;
use algo_lib::numbers::zeta::*;

#[test]
fn zeta_all() {
    type M = ModIntF;
    let mut rng = Random::new_with_seed(601);
    for it in 0..300 {
        let n = rng.gen_range(0..8usize);
        let size = 1usize << n;
        let a: Vec<M> = rpoly::<u32, M>(&mut rng, size);
        let b: Vec<M> = rpoly::<u32, M>(&mut rng, size);
        let mut z = a.clone();
        subset_zeta(&mut z);
        for mask in 0..size {
            let mut e = M::zero();
            for s in 0..size {
                if s & mask == s {
                    e += a[s];
                }
            }
            assert_eq!(z[mask], e);
        }
        subset_mobius(&mut z);
        assert_eq!(z, a);
        let mut z = a.clone();
        superset_zeta(&mut z);
        for mask in 0..size {
            let mut e = M::zero();
            for s in 0..size {
                if s & mask == mask {
                    e += a[s];
                }
            }
            assert_eq!(z[mask], e);
        }
        superset_mobius(&mut z);
        assert_eq!(z, a);
        let mut or = vec![M::zero(); size];
        let mut and = vec![M::zero(); size];
        let mut xor = vec![M::zero(); size];
        let mut sub = vec![M::zero(); size];
        for i in 0..size {
            for j in 0..size {
                or[i | j] += a[i] * b[j];
                and[i & j] += a[i] * b[j];
                xor[i ^ j] += a[i] * b[j];
                if i & j == 0 {
                    sub[i | j] += a[i] * b[j];
                }
            }
        }
        assert_eq!(or_convolution(&a, &b), or);
        assert_eq!(and_convolution(&a, &b), and);
        assert_eq!(xor_convolution(&a, &b), xor);
        assert_eq!(subset_convolution(&a, &b), sub, "subset conv it={it} n={n}");
        let mut w = a.clone();
        w.fwht(false);
        for i in 0..size {
            let mut e = M::zero();
            for j in 0..size {
                if (i & j).count_ones() % 2 == 0 {
                    e += a[j];
                } else {
                    e -= a[j];
                }
            }
            assert_eq!(w[i], e);
        }
        w.fwht(true);
        assert_eq!(w, a);
        // sps exp
        let mut f = a.clone();
        f[0] = M::zero();
        let ex = sps_exp(&f);
        // brute: res[S] = sum over partitions; recurrence: res[S] = sum over blocks T containing lowest bit of S: f[T]*res[S^T]
        let mut br = vec![M::zero(); size];
        br[0] = M::one();
        for s in 1..size {
            let low = s & s.wrapping_neg();
            let mut t = s;
            while t > 0 {
                if t & low != 0 {
                    let v = f[t] * br[s ^ t];
                    br[s] += v;
                }
                t = (t - 1) & s;
            }
        }
        assert_eq!(ex, br, "sps_exp n={n}");

        // divisor lattice
        let len = rng.gen_range(0..70usize);
        let a: Vec<M> = rpoly::<u32, M>(&mut rng, len);
        let b: Vec<M> = rpoly::<u32, M>(&mut rng, len);
        let mut z = a.clone();
        divisor_zeta(&mut z);
        for i in 1..len {
            let mut e = M::zero();
            for d in 1..=i {
                if i % d == 0 {
                    e += a[d];
                }
            }
            assert_eq!(z[i], e, "divisor zeta len={len} i={i}");
        }
        if len > 0 {
            assert_eq!(z[0], a[0]);
        }
        divisor_mobius(&mut z);
        assert_eq!(z, a);
        let mut z = a.clone();
        multiple_zeta(&mut z);
        for i in 1..len {
            let mut e = M::zero();
            for d in (i..len).step_by(i) {
                e += a[d];
            }
            assert_eq!(z[i], e, "multiple zeta len={len} i={i}");
        }
        multiple_mobius(&mut z);
        assert_eq!(z, a);
        let mut g = vec![M::zero(); len];
        let mut l = vec![M::zero(); len];
        for i in 1..len {
            for j in 1..len {
                let gg = gcd(i as u128, j as u128) as usize;
                g[gg] += a[i] * b[j];
                let ll = i / gg * j;
                if ll < len {
                    l[ll] += a[i] * b[j];
                }
            }
        }
        let gc = gcd_convolution(&a, &b);
        let lc = lcm_convolution(&a, &b);
        for i in 1..len {
            assert_eq!(gc[i], g[i], "gcd conv len={len} i={i}");
            assert_eq!(lc[i], l[i], "lcm conv len={len} i={i}");
        }
    }
    // signed integer instantiation
    let a: Vec<i64> = vec![1, -2, 3, -4, 5, -6, 7, -8];
    let mut z = a.clone();
    subset_zeta(&mut z);
    subset_mobius(&mut z);
    assert_eq!(z, a);
}

// ---------------- series / matrix_series / multiplicative_function ----------------
use algo_lib::numbers::matrix::Matrix;
use algo_lib::numbers::matrix_series::MatrixPowers;
use algo_lib::numbers::multiplicative_function::MulitplicativeFunction;
use algo_lib::numbers::series::{sum_arithmetic_series, sum_geometric_series};

#[test]
fn series_small() {
    type M = ModInt7;
    for first in -5i64..5 {
        for step in -5i64..5 {
            for len in 0i64..10 {
                let e: i64 = (0..len).map(|i| first + step * i).sum();
                assert_eq!(sum_arithmetic_series(first, step, len), e);
                assert_eq!(sum_arithmetic_series(M::from(first), M::from(step), M::from(len)), M::from(e));
            }
        }
    }
    for first in 0i64..5 {
        for ratio in [-3i64, -2, 0, 2, 3, 5] {
            for len in 0usize..10 {
                let mut e = M::zero();
                let mut cur = M::from(first);
                for _ in 0..len {
                    e += cur;
                    cur *= M::from(ratio);
                }
                assert_eq!(sum_geometric_series(M::from(first), M::from(ratio), len), e, "geo {first} {ratio} {len}");
            }
        }
    }
}

#[test]
fn bug_geometric_series_ratio_one() {
    type M = ModInt7;
    assert_eq!(sum_geometric_series(M::new(3), M::new(1), 5), M::new(15));
}

#[test]
fn matrix_powers() {
    type M = ModInt7;
    let mut rng = Random::new_with_seed(701);
    for _ in 0..200 {
        let n = rng.gen_range(1..5usize);
        let rows = rng.gen_range(1..4usize);
        let mut base = Matrix::zero(n, n);
        for i in 0..n {
            for j in 0..n {
                base[(i, j)] = M::new(rng.gen_range(0..1_000_000_007u32));
            }
        }
        let mut start = Matrix::zero(rows, n);
        for i in 0..rows {
            for j in 0..n {
                start[(i, j)] = M::new(rng.gen_range(0..1_000_000_007u32));
            }
        }
        let logs = rng.gen_range(0..8usize);
        let mp = MatrixPowers::new(base.clone(), logs);
        for _ in 0..5 {
            let power = rng.gen_range(0..(1usize << logs));
            let mut e = start.clone();
            for _ in 0..power {
                e = e.mult(&base);
            }
            let got = mp.calculate(start.clone(), power);
            for i in 0..rows {
                for j in 0..n {
                    assert_eq!(got[(i, j)], e[(i, j)]);
                }
            }
        }
    }
}

#[test]
fn multiplicative_functions() {
    let n = 3000usize;
    let fs: Vec<(&str, MulitplicativeFunction, Box<dyn Fn(u64) -> i64>)> = vec![
        ("d", MulitplicativeFunction::divisor_count(), Box::new(|x| (1..=x).filter(|d| x % d == 0).count() as i64)),
        ("sigma", MulitplicativeFunction::divisor_sum(), Box::new(|x| (1..=x).filter(|d| x % d == 0).sum::<u64>() as i64)),
        ("phi", MulitplicativeFunction::phi(), Box::new(|x| (1..=x).filter(|&d| gcd(d as u128, x as u128) == 1).count() as i64)),
        ("mu", MulitplicativeFunction::mobius(), Box::new(|x| {
            let mut x = x;
            let mut r = 1;
            let mut p = 2;
            while p * p <= x {
                if x % p == 0 {
                    x /= p;
                    if x % p == 0 {
                        return 0;
                    }
                    r = -r;
                }
                p += 1;
            }
            if x > 1 { -r } else { r }
        })),
    ];
    for (name, f, brute) in &fs {
        for len in [0usize, 1, 2, 3, 4, 5, 9, 10, 25, 26, n] {
            let table = f.calculate_up_to(len);
            assert_eq!(table.len(), len, "{name} len={len}");
            for i in 1..len {
                assert_eq!(table[i], brute(i as u64), "{name} table[{i}] len={len}");
            }
        }
        for i in 1..n as u64 {
            assert_eq!(f.call(i), brute(i), "{name} call({i})");
        }
    }
    // big arguments
    let phi = MulitplicativeFunction::phi();
    assert_eq!(phi.call(1_000_000_007u64 * 998_244_353), 1_000_000_006i64 * 998_244_352);
    assert_eq!(phi.call(1u64 << 62), 1i64 << 61);
    let sigma = MulitplicativeFunction::divisor_sum();
    assert_eq!(sigma.call(1u64 << 61), ((1u128 << 62) - 1) as i64);
}

// ---------------- misc mod_int ----------------
#[test]
fn modint_discrete_log() {
    dynamic_value!(LP: u32);
    type M = ModInt<LP>;
    for p in [2u32, 3, 5, 7, 11, 13, 17, 97, 101, 257, 65537, 10007] {
        LP::set(p);
        // find primitive root by brute force
        let mut g = 1u32;
        'outer: for cand in 1..p {
            let mut x = 1u64;
            for e in 1..p - 1 {
                x = x * cand as u64 % p as u64;
                if x == 1 && e < p - 1 {
                    continue 'outer;
                }
            }
            g = cand;
            break;
        }
        let step = (p / 300).max(1);
        let mut x = M::one();
        for e in 0..p - 1 {
            if e % step == 0 {
                let l = x.log(M::new(g));
                assert_eq!(l, e, "log p={p} g={g} e={e}");
            }
            x *= M::new(g);
        }
    }
}

#[test]
fn bug_modint_debug_composite_modulus() {
    value!(C: u32 = 1 << 30);
    type M = ModInt<C>;
    let s = format!("{:?}", M::new(123456789));
    assert!(!s.is_empty());
}

#[test]
fn bug_modint_read_large_value() {
    use algo_lib::io::input::Input;
    let data = b"3000000000 -5 1000000007 12345678901234567890";
    let mut input = Input::slice(data);
    let a: ModInt7 = input.read();
    assert_eq!(a.val(), (3000000000u64 % 1_000_000_007) as u32);
    let b: ModInt7 = input.read();
    assert_eq!(b.val(), 1_000_000_002);
    let c: ModInt7 = input.read();
    assert_eq!(c.val(), 0);
}

#[test]
fn prime_fft_power_of_empty() {
    let mut fft = PrimeFFT::<ModIntF>::new();
    assert!(fft.power(&[], 2).iter().all(|&x| x == ModIntF::zero()));
    assert_eq!(fft.power(&[], 0), vec![ModIntF::one()]);
}

#[test]
fn product_tree_fast_path_dense() {
    fn run<M: TestMod<u32> + From<usize>>(seed: u64) {
        let mut rng = Random::new_with_seed(seed);
        let mut ops = PolynomialOps::<u32, M>::new();
        for it in 0..150 {
            let points = rng.gen_range(33..200usize);
            let len = match it % 3 {
                0 => rng.gen_range(33..80usize),
                1 => rng.gen_range(33..600usize),
                _ => points + rng.gen_range(0..3usize) - 1,
            };
            let mut xs: Vec<M> = (0..points).map(|_| M::rnd(&mut rng)).collect();
            match rng.gen_range(0..5u32) {
                0 => {
                    for x in xs.iter_mut() {
                        *x = M::from(rng.gen_range(0..2usize));
                    }
                }
                1 => xs = vec![M::zero(); points],
                2 => xs = vec![xs[0]; points],
                _ => {}
            }
            let mut poly: Vec<M> = rpoly(&mut rng, len);
            if rng.gen_bool() {
                let l = poly.len();
                for x in poly[l - 5..].iter_mut() {
                    *x = M::zero();
                }
            }
            let tree = ProductTree::new(&xs, &mut ops);
            for _ in 0..2 {
                let got = tree.evaluate(&poly, &mut ops);
                let expected: Vec<M> = xs.iter().map(|&x| evaluate(&poly, x)).collect();
                assert_eq!(got, expected, "it={it} points={points} len={len}");
            }
        }
    }
    run::<ModIntF>(801);
    run::<ModInt7>(802);
    value!(P: u32 = 769);
    run::<ModInt<P>>(803);
    value!(Q: u32 = 2);
    run::<ModInt<Q>>(804);
}

#[test]
fn fps_large() {
    fn run<M: TestMod<u32> + From<usize>>(seed: u64) {
        let mut rng = Random::new_with_seed(seed);
        let mut ops = PolynomialOps::<u32, M>::new();
        for &n in &[1000usize, 2048, 3001] {
            let mut f: Vec<M> = (0..n).map(|_| M::rnd(&mut rng)).collect();
            f[0] = M::one();
            let inv = ops.inverse_series(&f, n);
            let mut e = vec![M::zero(); n];
            e[0] = M::one();
            assert_eq!(nmul(&f, &inv, n), e);
            let lg = ops.log(&f, n);
            assert_eq!(ops.exp(&lg, n), f);
            assert_eq!(lg, ops.log_sparse(&f, n));
            let sq = nmul(&f, &f, n);
            let r = ops.sqrt(&sq, n).unwrap();
            assert!(r == f || r == f.iter().map(|&x| -x).collect::<Vec<_>>());
            for k in [2u64, 5, 1_000_000_000_000] {
                let mut g = f.clone();
                g[0] = M::from(3usize);
                let sh = (n / 7) as usize;
                let mut h = vec![M::zero(); 2];
                h.extend_from_slice(&g);
                assert_eq!(ops.pow(&g, k, n), npow(&g, k, n), "pow {k}");
                assert_eq!(ops.pow(&h, k, n), npow(&h, k, n), "pow shifted {k}");
                let _ = sh;
            }
            let c = M::rnd(&mut rng);
            let sh = ops.taylor_shift(&f, c);
            let x = M::rnd(&mut rng);
            assert_eq!(evaluate(&sh, x), evaluate(&f, x + c));
            let ys: Vec<M> = (0..n).map(|i| evaluate(&f, M::from(i))).collect();
            for (start, count) in [(M::from(n / 2), 2 * n), (-M::from(5usize), 50), (M::rnd(&mut rng), n + 7)] {
                let got = ops.shift_sampling_points(&ys, start, count);
                for k in (0..count).step_by(37).chain(0..10.min(count)) {
                    assert_eq!(got[k], evaluate(&f, start + M::from(k)), "shift start={start:?} k={k}");
                }
            }
        }
    }
    run::<ModIntF>(901);
    run::<ModInt7>(902);
}

#[test]
fn fix_check_large_ntt_prime_via_crt() {
    // suggested fix for bug_fft_new_large_ntt_prime: route p >= 2^30 to Convolution
    value!(P: u32 = 2013265921);
    type M = ModInt<P>;
    let mut rng = Random::new_with_seed(5);
    let mut conv = Convolution::<M>::new();
    for _ in 0..50 {
        let n = rng.gen_range(61..300usize);
        let m = rng.gen_range(61..300usize);
        let a: Vec<M> = rnd_vec(&mut rng, n, 1);
        let b: Vec<M> = rnd_vec(&mut rng, m, 0);
        assert_eq!(conv.multiply(&a, &b), naive_mul(&a, &b));
    }
}

#[test]
fn combinations_table_small() {
    use algo_lib::numbers::combinations::combinations_table;
    for n in 0..20usize {
        let t = combinations_table::<u64>(n);
        for i in 0..=n {
            for j in 0..=n {
                let mut e = 1u64;
                if j > i {
                    e = 0;
                } else {
                    for k in 0..j {
                        e = e * (i - k) as u64 / (k + 1) as u64;
                    }
                }
                assert_eq!(t[(i, j)], e);
            }
        }
    }
    value!(P: u64 = 1_000_000_000_000_000_003);
    type W = ModInt64<P>;
    let c = Combinations::<W, u64>::new(100);
    let t = combinations_table::<u128>(99);
    for i in 0..100 {
        for j in 0..100 {
            assert_eq!(c.c(i, j).val() as u128, t[(i.min(99), j.min(99))] % 1_000_000_000_000_000_003u128 * ((j <= i) as u128));
        }
    }
}
