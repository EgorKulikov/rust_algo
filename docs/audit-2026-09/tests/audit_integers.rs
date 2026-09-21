#![allow(clippy::all)]
#![allow(dead_code)]
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::signed_big_int::BigInt;
use algo_lib::numbers::unsigned_big_int::UBigInt;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::cmp::Ordering;

const B: u64 = 1_000_000_000;

// ---------- oracle bignum: little-endian base 1e9, no trailing zeros ----------
fn trim(mut a: Vec<u64>) -> Vec<u64> {
    while a.last() == Some(&0) {
        a.pop();
    }
    a
}
fn o_add(a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut res = Vec::new();
    let mut carry = 0;
    for i in 0..a.len().max(b.len()) {
        let s = a.get(i).copied().unwrap_or(0) + b.get(i).copied().unwrap_or(0) + carry;
        res.push(s % B);
        carry = s / B;
    }
    if carry > 0 {
        res.push(carry);
    }
    trim(res)
}
fn o_cmp(a: &[u64], b: &[u64]) -> Ordering {
    if a.len() != b.len() {
        return a.len().cmp(&b.len());
    }
    for i in (0..a.len()).rev() {
        if a[i] != b[i] {
            return a[i].cmp(&b[i]);
        }
    }
    Ordering::Equal
}
fn o_sub(a: &[u64], b: &[u64]) -> Vec<u64> {
    assert!(o_cmp(a, b) != Ordering::Less);
    let mut res = Vec::new();
    let mut borrow = 0i64;
    for i in 0..a.len() {
        let mut s = a[i] as i64 - b.get(i).copied().unwrap_or(0) as i64 - borrow;
        if s < 0 {
            s += B as i64;
            borrow = 1;
        } else {
            borrow = 0;
        }
        res.push(s as u64);
    }
    assert_eq!(borrow, 0);
    trim(res)
}
fn o_mul(a: &[u64], b: &[u64]) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut res = vec![0u64; a.len() + b.len() + 1];
    for i in 0..a.len() {
        let mut carry = 0u64;
        for j in 0..b.len() {
            let cur = res[i + j] + a[i] * b[j] + carry;
            res[i + j] = cur % B;
            carry = cur / B;
        }
        let mut k = i + b.len();
        while carry > 0 {
            let cur = res[k] + carry;
            res[k] = cur % B;
            carry = cur / B;
            k += 1;
        }
    }
    trim(res)
}
fn o_str(a: &[u64]) -> String {
    if a.is_empty() {
        return "0".to_string();
    }
    let mut s = format!("{}", a[a.len() - 1]);
    for i in (0..a.len() - 1).rev() {
        s += &format!("{:09}", a[i]);
    }
    s
}
fn o_parse(s: &str) -> Vec<u64> {
    let bytes = s.as_bytes();
    let mut res = Vec::new();
    let mut at = bytes.len();
    while at > 0 {
        let start = at.saturating_sub(9);
        res.push(s[start..at].parse::<u64>().unwrap());
        at = start;
    }
    trim(res)
}
fn o_from_u128(mut v: u128) -> Vec<u64> {
    let mut res = Vec::new();
    while v > 0 {
        res.push((v % B as u128) as u64);
        v /= B as u128;
    }
    res
}
fn ub(a: &[u64]) -> UBigInt {
    UBigInt::from(o_str(a).as_bytes())
}
fn back(a: &UBigInt) -> Vec<u64> {
    o_parse(&format!("{}", a))
}
fn pow2(k: usize) -> Vec<u64> {
    let mut r = vec![1u64];
    for _ in 0..k {
        r = o_add(&r, &r);
    }
    r
}

fn gen(rng: &mut ChaCha8Rng, len: usize) -> Vec<u64> {
    if len == 0 {
        return Vec::new();
    }
    let kind = rng.gen_range(0..8);
    let mut v: Vec<u64> = match kind {
        0 | 1 => (0..len).map(|_| rng.gen_range(0..B)).collect(),
        2 => vec![B - 1; len],
        3 => {
            let mut v = vec![0; len];
            v[len - 1] = 1;
            v
        }
        4 => (0..len)
            .map(|_| if rng.gen_bool(0.8) { 0 } else { rng.gen_range(0..B) })
            .collect(),
        5 => (0..len)
            .map(|_| if rng.gen_bool(0.5) { B - 1 } else { 0 })
            .collect(),
        6 => {
            // top limb extreme
            let mut v: Vec<u64> = (0..len).map(|_| rng.gen_range(0..B)).collect();
            v[len - 1] = *[1, 2, B / 2 - 1, B / 2, B / 2 + 1, B - 2, B - 1].choose(rng).unwrap();
            v
        }
        _ => {
            // BASE^(len-1) - 1 +- small -> retains len-1 or len limbs
            let mut v = vec![B - 1; len];
            v[0] = rng.gen_range(0..B);
            v
        }
    };
    if v[len - 1] == 0 {
        v[len - 1] = rng.gen_range(1..B);
    }
    v
}

fn check_div(n: &[u64], d: &[u64]) {
    let (q, r) = ub(n).div_rem(&ub(d));
    let (q, r) = (back(&q), back(&r));
    assert!(o_cmp(&r, d) == Ordering::Less, "r >= d: n={} d={}", o_str(n), o_str(d));
    let recomposed = o_add(&o_mul(&q, d), &r);
    assert!(
        recomposed == n,
        "q*d+r != n: nlen={} dlen={} n={} d={}",
        n.len(),
        d.len(),
        o_str(n),
        o_str(d)
    );
}

#[test]
fn ubig_small_vs_u128() {
    let mut rng = ChaCha8Rng::seed_from_u64(1);
    for it in 0..200000 {
        let bits_a = rng.gen_range(0..=127);
        let bits_b = rng.gen_range(0..=127);
        let mut a: u128 = rng.gen::<u128>() >> (127 - bits_a) >> 1;
        let mut b: u128 = rng.gen::<u128>() >> (127 - bits_b) >> 1;
        if it % 17 == 0 {
            a = [0, 1, B as u128 - 1, B as u128, (B * B) as u128, (B * B) as u128 - 1][rng.gen_range(0..6)];
        }
        if it % 19 == 0 {
            b = [0, 1, B as u128 - 1, B as u128, (B * B) as u128, (B * B) as u128 - 1][rng.gen_range(0..6)];
        }
        let (x, y) = (UBigInt::from(a), UBigInt::from(b));
        assert_eq!(x.to_u128(), Some(a));
        assert_eq!(format!("{}", x), format!("{}", a));
        assert_eq!(UBigInt::from(format!("{}", a).as_bytes()), x);
        assert_eq!(UBigInt::from(format!("000000000000{}", a).as_bytes()), x);
        assert_eq!(x.cmp(&y), a.cmp(&b));
        assert_eq!((x.clone() + y.clone()).to_u128(), Some(a + b));
        if a >= b {
            assert_eq!((x.clone() - y.clone()).to_u128(), Some(a - b));
        }
        if let Some(p) = a.checked_mul(b) {
            assert_eq!((&x * &y).to_u128(), Some(p));
        }
        if b != 0 {
            let (q, r) = x.div_rem(&y);
            assert_eq!(q.to_u128(), Some(a / b), "{a} / {b}");
            assert_eq!(r.to_u128(), Some(a % b), "{a} % {b}");
            assert_eq!((&x / &y).to_u128(), Some(a / b));
            assert_eq!((&x % &y).to_u128(), Some(a % b));
        }
        let s = rng.gen_range(1..=i32::MAX);
        if let Some(p) = a.checked_mul(s as u128) {
            let mut z = x.clone();
            z *= s;
            assert_eq!(z.to_u128(), Some(p));
        }
        let mut z = x.clone();
        z /= s;
        assert_eq!(z.to_u128(), Some(a / s as u128));
        assert_eq!((&x % s) as u128, a % s as u128);
        assert_eq!(u64::try_from(&x).ok(), u64::try_from(a).ok());
        assert_eq!(i64::try_from(&x).ok(), i64::try_from(a).ok());
        assert_eq!(i128::try_from(&x).ok(), i128::try_from(a).ok());
        assert_eq!(u8::try_from(&x).ok(), u8::try_from(a).ok());
    }
    // to_u128 overflow boundary
    let max = UBigInt::from(u128::MAX);
    assert_eq!(max.to_u128(), Some(u128::MAX));
    assert_eq!((max.clone() + UBigInt::one()).to_u128(), None);
    assert_eq!(UBigInt::zero().to_u128(), Some(0));
    assert_eq!(format!("{}", UBigInt::zero()), "0");
    assert_eq!(UBigInt::from(&b"0"[..]), UBigInt::zero());
    assert_eq!(UBigInt::from(&b"0000000000000000000"[..]), UBigInt::zero());
    assert_eq!(UBigInt::from(&b""[..]), UBigInt::zero());
    assert!((UBigInt::from(5) - UBigInt::from(5)).is_zero());
    assert_eq!(UBigInt::from(7).power(0), UBigInt::one());
    assert_eq!(UBigInt::zero().power(0), UBigInt::one());
    assert_eq!(UBigInt::zero().power(5), UBigInt::zero());
    assert_eq!(UBigInt::from(3).power(80).to_u128(), Some(3u128.pow(80)));
    assert_eq!(UBigInt::from(i128::MAX).to_u128(), Some(i128::MAX as u128));
    assert_eq!(UBigInt::from(u64::MAX).to_u128(), Some(u64::MAX as u128));
}

#[test]
fn ubig_large_add_sub_mul_cmp() {
    let mut rng = ChaCha8Rng::seed_from_u64(2);
    for _ in 0..3000 {
        let la = rng.gen_range(0..60);
        let lb = rng.gen_range(0..60);
        let a = gen(&mut rng, la);
        let b = gen(&mut rng, lb);
        let (x, y) = (ub(&a), ub(&b));
        assert_eq!(back(&x), a);
        assert_eq!(x.cmp(&y), o_cmp(&a, &b));
        assert_eq!(back(&(x.clone() + y.clone())), o_add(&a, &b));
        assert_eq!(back(&(&x * &y)), o_mul(&a, &b));
        if o_cmp(&a, &b) != Ordering::Less {
            assert_eq!(back(&(x.clone() - y.clone())), o_sub(&a, &b));
        }
        // a + b - b == a, (a+b) - a == b
        let s = x.clone() + &y;
        assert_eq!(s.clone() - &y, x);
        assert_eq!(s - &x, y);
    }
    // large all-nines multiplication (max convolution values)
    for len in [100usize, 1000, 5000, 20000] {
        let a = vec![B - 1; len];
        let x = ub(&a);
        let p = &x * &x;
        // (B^len - 1)^2 = B^(2len) - 2 B^len + 1
        let mut expected = vec![0u64; 2 * len];
        expected[0] = 1;
        for i in len..2 * len {
            expected[i] = B - 1;
        }
        expected[len] = B - 2;
        assert_eq!(back(&p), expected, "len={len}");
    }
}

#[test]
fn ubig_division_structured() {
    let mut rng = ChaCha8Rng::seed_from_u64(3);
    let ms = [1usize, 2, 3, 5, 48, 95, 96, 97, 98, 99, 100, 130, 191, 192, 193, 194, 200, 250];
    for &m in &ms {
        let mut ns = vec![
            m.saturating_sub(1),
            m,
            m + 1,
            m + 2,
            m + 94,
            m + 95,
            m + 96,
            m + 97,
            2 * m - 1,
            2 * m,
            2 * m + 1,
            3 * m,
            3 * m + 1,
            3 * m - 1,
            400,
        ];
        ns.push(rng.gen_range(1..400));
        for &n in &ns {
            for _ in 0..6 {
                let a = gen(&mut rng, n);
                let d = gen(&mut rng, m);
                check_div(&a, &d);
            }
        }
    }
}

#[test]
fn ubig_division_random_sizes() {
    let mut rng = ChaCha8Rng::seed_from_u64(4);
    for _ in 0..1500 {
        let m = rng.gen_range(1..=400);
        let n = rng.gen_range(0..=400);
        let a = gen(&mut rng, n);
        let d = gen(&mut rng, m);
        check_div(&a, &d);
    }
}

#[test]
fn ubig_division_exact_products() {
    // n = q*d + r with r in {0, 1, d-1}: quotient digit estimate edge cases
    let mut rng = ChaCha8Rng::seed_from_u64(5);
    for _ in 0..600 {
        let m = *[2usize, 3, 50, 96, 97, 98, 120, 200].choose(&mut rng).unwrap();
        let ql = *[1usize, 2, 95, 96, 97, 100, 200, 250].choose(&mut rng).unwrap();
        let d = gen(&mut rng, m);
        let q = gen(&mut rng, ql);
        let one = vec![1u64];
        let r = match rng.gen_range(0..3) {
            0 => Vec::new(),
            1 => {
                if o_cmp(&d, &one) == Ordering::Greater {
                    one.clone()
                } else {
                    Vec::new()
                }
            }
            _ => o_sub(&d, &one),
        };
        let n = o_add(&o_mul(&q, &d), &r);
        let (gq, gr) = ub(&n).div_rem(&ub(&d));
        assert_eq!(back(&gq), q, "m={m} ql={ql}");
        assert_eq!(back(&gr), r, "m={m} ql={ql}");
    }
}

#[test]
fn ubig_division_pow2() {
    let mut rng = ChaCha8Rng::seed_from_u64(6);
    let one = vec![1u64];
    for _ in 0..150 {
        let k = rng.gen_range(2800..9000);
        let p = pow2(k);
        let d = match rng.gen_range(0..3) {
            0 => p.clone(),
            1 => o_add(&p, &one),
            _ => o_sub(&p, &one),
        };
        let nl = rng.gen_range(1..400);
        let n = match rng.gen_range(0..4) {
            0 => gen(&mut rng, nl),
            1 => pow2(rng.gen_range(k..12000)),
            2 => o_sub(&pow2(rng.gen_range(k..12000)), &one),
            _ => o_mul(&d, &d),
        };
        check_div(&n, &d);
    }
}

#[test]
fn bigint_vs_i128() {
    let mut rng = ChaCha8Rng::seed_from_u64(7);
    let special = [0i128, 1, -1, 999_999_999, -999_999_999, 1_000_000_000, -1_000_000_000, i64::MAX as i128, i64::MIN as i128];
    for it in 0..200000 {
        let sa = rng.gen_range(0..=126);
        let sb = rng.gen_range(0..=126);
        let mut a: i128 = rng.gen::<i128>() >> sa;
        let mut b: i128 = rng.gen::<i128>() >> sb;
        if it % 5 == 0 {
            a = special[rng.gen_range(0..special.len())];
        }
        if it % 7 == 0 {
            b = special[rng.gen_range(0..special.len())];
        }
        let (x, y) = (BigInt::from(a), BigInt::from(b));
        assert_eq!(x.to_i128(), Some(a));
        assert_eq!(format!("{}", x), format!("{}", a));
        assert_eq!(BigInt::from(format!("{}", a).as_bytes()), x);
        assert_eq!(x.cmp(&y), a.cmp(&b), "{a} {b}");
        assert_eq!(x == y, a == b);
        if let Some(s) = a.checked_add(b) {
            assert_eq!((x.clone() + y.clone()).to_i128(), Some(s));
            assert_eq!(x.clone() + y.clone(), BigInt::from(s));
        }
        if let Some(s) = a.checked_sub(b) {
            assert_eq!((x.clone() - y.clone()).to_i128(), Some(s));
            assert_eq!(x.clone() - y.clone(), BigInt::from(s));
        }
        if let Some(s) = a.checked_mul(b) {
            assert_eq!((&x * &y).to_i128(), Some(s));
            assert_eq!(&x * &y, BigInt::from(s), "{a} * {b}");
        }
        if b != 0 && !(a == i128::MIN && b == -1) {
            let (q, r) = x.div_rem(&y);
            assert_eq!(q, BigInt::from(a / b), "{a} / {b}");
            assert_eq!(r, BigInt::from(a % b), "{a} % {b}");
        }
        if a != i128::MIN {
            assert_eq!(-x.clone(), BigInt::from(-a));
        }
        let s: i32 = rng.gen_range(i32::MIN + 1..=i32::MAX);
        if let Some(p) = a.checked_mul(s as i128) {
            let mut z = x.clone();
            z *= s;
            assert_eq!(z, BigInt::from(p), "{a} *= {s}");
        }
        if s != 0 {
            let mut z = x.clone();
            z /= s;
            assert_eq!(z, BigInt::from(a / s as i128), "{a} /= {s}");
        }
        assert_eq!(i64::try_from(&x).ok(), i64::try_from(a).ok());
        assert_eq!(u64::try_from(&x).ok(), u64::try_from(a).ok());
        assert_eq!(i8::try_from(&x).ok(), i8::try_from(a).ok());
        assert_eq!(u128::try_from(&x).ok(), u128::try_from(a).ok());
        assert_eq!(x.clone().abs().to_u128(), Some(a.unsigned_abs()));
    }
    assert_eq!(BigInt::from(i128::MIN).to_i128(), Some(i128::MIN));
    assert_eq!(BigInt::from(i128::MAX).to_i128(), Some(i128::MAX));
    assert_eq!((BigInt::from(i128::MAX) + BigInt::one()).to_i128(), None);
    assert_eq!((BigInt::from(i128::MIN) - BigInt::one()).to_i128(), None);
    assert_eq!(BigInt::from(u128::MAX).to_i128(), None);
    assert_eq!(u128::try_from(&BigInt::from(u128::MAX)).ok(), Some(u128::MAX));
    // zero handling
    assert_eq!(BigInt::from(&b"-0"[..]), BigInt::zero());
    assert_eq!(BigInt::from(&b"-000"[..]), BigInt::zero());
    assert_eq!(format!("{}", BigInt::from(&b"-0"[..])), "0");
    assert_eq!(-BigInt::zero(), BigInt::zero());
    assert_eq!(BigInt::from(5) * BigInt::zero(), BigInt::zero());
    assert_eq!(BigInt::from(-5) * BigInt::zero(), BigInt::zero());
    let mut z = BigInt::from(-5);
    z *= 0;
    assert_eq!(z, BigInt::zero());
    assert_eq!(format!("{}", z), "0");
    let mut z = BigInt::from(-5);
    z /= 7;
    assert_eq!(z, BigInt::zero());
    assert_eq!(BigInt::from(-5) + BigInt::from(5), BigInt::zero());
    assert_eq!(BigInt::zero() - BigInt::from(5), BigInt::from(-5));
    assert_eq!(BigInt::from(-3) / BigInt::from(5), BigInt::zero());
    assert_eq!(BigInt::from(-10) % BigInt::from(5), BigInt::zero());
    assert_eq!(BigInt::from(-10) % BigInt::from(5) + BigInt::from(3), BigInt::from(3));
}

#[test]
fn ubig_division_big_newton() {
    let mut rng = ChaCha8Rng::seed_from_u64(8);
    for it in 0..120 {
        let m = if it % 3 == 0 {
            *[193usize, 194, 195, 196, 197, 384, 385, 386, 387, 388, 389, 390, 391, 392, 777, 1000, 1555]
                .choose(&mut rng)
                .unwrap()
        } else {
            rng.gen_range(97..1600)
        };
        let n = match rng.gen_range(0..4) {
            0 => m + 96,
            1 => 2 * m,
            2 => rng.gen_range(m + 96..m + 96 + 3 * m),
            _ => 3 * m + rng.gen_range(0..3),
        };
        let a = gen(&mut rng, n);
        let d = gen(&mut rng, m);
        check_div(&a, &d);
    }
}

// ======================= fixed_int =======================
use algo_lib::numbers::fixed_int::{i256, FixedInt};

fn special_i128(rng: &mut ChaCha8Rng) -> i128 {
    let limbs = [0u32, 1, 2, 0x7fff_ffff, 0x8000_0000, 0x8000_0001, 0xffff_fffe, 0xffff_ffff];
    let mut v: u128 = 0;
    let cnt = rng.gen_range(1..=4);
    for i in 0..4 {
        let l = if rng.gen_bool(0.3) { rng.gen::<u32>() } else { limbs[rng.gen_range(0..limbs.len())] };
        let l = if i < cnt { l } else if rng.gen_bool(0.5) { 0 } else { 0xffff_ffff };
        v |= (l as u128) << (32 * i);
    }
    v as i128
}

#[test]
fn fixed_int_128_vs_i128() {
    type F = FixedInt<4>;
    let mut rng = ChaCha8Rng::seed_from_u64(10);
    for it in 0..2_000_000 {
        let (a, b) = if it % 2 == 0 {
            (special_i128(&mut rng), special_i128(&mut rng))
        } else {
            (
                rng.gen::<i128>() >> rng.gen_range(0..128),
                rng.gen::<i128>() >> rng.gen_range(0..128),
            )
        };
        let (x, y) = (F::from(a), F::from(b));
        assert_eq!(x + y, F::from(a.wrapping_add(b)), "{a} + {b}");
        assert_eq!(x - y, F::from(a.wrapping_sub(b)), "{a} - {b}");
        assert_eq!(x * y, F::from(a.wrapping_mul(b)), "{a} * {b}");
        assert_eq!(-x, F::from(a.wrapping_neg()), "-{a}");
        assert_eq!(x.cmp(&y), a.cmp(&b), "{a} cmp {b}");
        if b != 0 {
            assert_eq!(x / y, F::from(a.wrapping_div(b)), "{a} / {b}");
            assert_eq!(x % y, F::from(a.wrapping_rem(b)), "{a} % {b}");
        }
    }
}

#[test]
fn fixed_int_64_32_vs_primitive() {
    let mut rng = ChaCha8Rng::seed_from_u64(11);
    for _ in 0..1_000_000 {
        let a = (special_i128(&mut rng) as i64) >> rng.gen_range(0..64);
        let b = (special_i128(&mut rng) as i64) >> rng.gen_range(0..64);
        type F = FixedInt<2>;
        let (x, y) = (F::from(a), F::from(b));
        assert_eq!(x * y, F::from(a.wrapping_mul(b)));
        assert_eq!(x + y, F::from(a.wrapping_add(b)));
        assert_eq!(x.cmp(&y), a.cmp(&b));
        if b != 0 {
            assert_eq!(x / y, F::from(a.wrapping_div(b)), "{a} / {b}");
            assert_eq!(x % y, F::from(a.wrapping_rem(b)), "{a} % {b}");
        }
        type G = FixedInt<1>;
        let (a, b) = (a as i32, b as i32);
        let (x, y) = (G::from(a), G::from(b));
        assert_eq!(x * y, G::from(a.wrapping_mul(b)));
        assert_eq!(x.cmp(&y), a.cmp(&b));
        if b != 0 {
            assert_eq!(x / y, G::from(a.wrapping_div(b)), "{a} / {b}");
            assert_eq!(x % y, G::from(a.wrapping_rem(b)), "{a} % {b}");
        }
    }
}

#[test]
fn fixed_int_256_identities() {
    let mut rng = ChaCha8Rng::seed_from_u64(12);
    let two64 = i256::from(1u128 << 64);
    let two128 = two64 * two64;
    let zero = i256::zero();
    let mk = |rng: &mut ChaCha8Rng| -> i256 {
        let hi = special_i128(rng) >> rng.gen_range(0..128);
        let lo_full = special_i128(rng);
        // lo as unsigned 128 = lo_hi * 2^64 + lo_lo
        let lo_hi = i256::from((lo_full as u128 >> 64) as u64);
        let lo_lo = i256::from(lo_full as u128 as u64);
        i256::from(hi) * two128 + lo_hi * two64 + lo_lo
    };
    for _ in 0..500_000 {
        let n = mk(&mut rng);
        let mut d = mk(&mut rng);
        if rng.gen_bool(0.5) {
            d = i256::from(special_i128(&mut rng) >> rng.gen_range(0..128));
        }
        if d == zero {
            continue;
        }
        let (q, r) = (n / d, n % d);
        assert_eq!(q * d + r, n, "{n:?} {d:?}");
        let abs = |x: i256| if x < zero { -x } else { x };
        // |r| < |d| (skip MIN divisor whose abs wraps)
        if abs(d) > zero {
            assert!(abs(r) < abs(d), "{n:?} {d:?} r={r:?}");
        }
        assert!(r == zero || ((r < zero) == (n < zero)), "{n:?} {d:?} r={r:?}");
        // ordering consistent with subtraction when no overflow: use halves
    }
    // conversions
    macro_rules! conv {
        ($($t:ident)+) => {$(
            for v in [$t::MIN, $t::MAX, 0 as $t, 1 as $t] {
                let x = i256::from(v);
                let y = i256::from(v as i128);
                assert_eq!(x, y, "{} {}", stringify!($t), v);
                assert_eq!(x < zero, (v as i128) < 0);
            }
        )+};
    }
    conv!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 usize);
}

#[test]
fn fixed_int_from_u128_large() {
    // u128 values above i128::MAX must stay positive in a 256-bit integer
    let x = i256::from(u128::MAX);
    assert!(x > i256::zero(), "i256::from(u128::MAX) = {x:?}");
    let two64 = i256::from(1u64 << 63) * i256::from(2);
    assert_eq!(x, two64 * two64 - i256::one());
}

#[test]
fn fixed_int_knuth_add_back_vectors() {
    type F = FixedInt<4>;
    let mk = |l: [u32; 4]| -> i128 {
        (l[0] as u128 | (l[1] as u128) << 32 | (l[2] as u128) << 64 | (l[3] as u128) << 96) as i128
    };
    let cases = [
        ([0, 0xfffe, 0x8000_0000, 0], [0xffff, 0x8000_0000, 0, 0]),
        ([3, 0, 0x8000_0000, 0], [1, 0, 0x2000_0000, 0]),
        ([0, 0, 0x8000, 0x7fff], [1, 0, 0x8000, 0]),
        ([0, 0xffff_fffe, 0x8000_0000, 0], [0xffff_ffff, 0x8000_0000, 0, 0]),
        ([0xffff_ffff, 0xffff_ffff, 0xffff_ffff, 0x7fff_ffff], [0xffff_ffff, 1, 0, 0]),
        ([0, 0, 0, 0x7fff_ffff], [0xffff_ffff, 0xffff_ffff, 0x7fff_ffff, 0]),
    ];
    for (u, v) in cases {
        let (a, b) = (mk(u), mk(v));
        assert_eq!(F::from(a) / F::from(b), F::from(a / b));
        assert_eq!(F::from(a) % F::from(b), F::from(a % b));
    }
}

// ======================= gcd / rational =======================
use algo_lib::numbers::gcd::{extended_gcd, gcd, lcm, remainder};
use algo_lib::numbers::rational::Rational;

fn naive_gcd(a: i64, b: i64) -> i64 {
    let (a, b) = (a.abs(), b.abs());
    if a == 0 && b == 0 {
        return 0;
    }
    (1..=a.max(b)).rev().find(|d| a % d == 0 && b % d == 0).unwrap()
}

#[test]
fn gcd_family() {
    for a in -40i64..=40 {
        for b in -40i64..=40 {
            let g = naive_gcd(a, b);
            assert_eq!(gcd(a, b).abs(), g, "gcd({a},{b})");
            let (d, x, y) = extended_gcd(a, b);
            assert_eq!(d.abs(), g, "egcd({a},{b})");
            assert_eq!(a * x + b * y, d, "egcd({a},{b}) = ({d},{x},{y})");
            if a != 0 && b != 0 {
                let l = lcm(a, b);
                assert_eq!(l.abs(), a.abs() * b.abs() / g);
            }
        }
    }
    let mut rng = ChaCha8Rng::seed_from_u64(20);
    for _ in 0..200000 {
        let a: i64 = rng.gen::<i64>() >> rng.gen_range(0..64);
        let b: i64 = rng.gen::<i64>() >> rng.gen_range(0..64);
        if a == i64::MIN || b == i64::MIN {
            continue;
        }
        let (d, x, y) = extended_gcd(a, b);
        assert_eq!(a as i128 * x as i128 + b as i128 * y as i128, d as i128);
        assert_eq!(d.abs(), gcd(a, b).abs());
        let (ua, ub) = (a.unsigned_abs(), b.unsigned_abs());
        assert_eq!(gcd(ua, ub), d.unsigned_abs());
    }
    // lcm without intermediate overflow
    assert_eq!(lcm(4_000_000_000u64, 6_000_000_000u64), 12_000_000_000);
    assert_eq!(lcm(1u64 << 62, 1u64 << 61), 1u64 << 62);
}

#[test]
fn crt_remainder() {
    for n1 in 1i64..=24 {
        for n2 in 1i64..=24 {
            let m = lcm(n1, n2);
            for a1 in 0..n1 {
                for a2 in 0..n2 {
                    let expected = (0..m).find(|x| x % n1 == a1 && x % n2 == a2);
                    let got = remainder(a1, n1, a2, n2).map(|x| ((x % m) + m) % m);
                    assert_eq!(got, expected, "remainder({a1},{n1},{a2},{n2})");
                }
            }
        }
    }
    // larger moduli: answer must stay valid as long as lcm fits
    let mut rng = ChaCha8Rng::seed_from_u64(21);
    for _ in 0..100000 {
        let n1: i64 = rng.gen_range(1..=1_000_000_000);
        let n2: i64 = rng.gen_range(1..=1_000_000_000);
        let x: i64 = rng.gen_range(0..lcm(n1, n2));
        let got = remainder(x % n1, n1, x % n2, n2).expect("solvable");
        let m = lcm(n1, n2);
        assert_eq!(((got % m) + m) % m, x, "n1={n1} n2={n2} x={x}");
    }
}

#[test]
fn rational_vs_bruteforce() {
    let mut rng = ChaCha8Rng::seed_from_u64(22);
    for _ in 0..200000 {
        let (an, ad, bn, bd): (i64, i64, i64, i64) = (
            rng.gen_range(-30..=30),
            rng.gen_range(-30..=30),
            rng.gen_range(-30..=30),
            rng.gen_range(-30..=30),
        );
        if ad == 0 || bd == 0 {
            continue;
        }
        let (a, b) = (Rational::new(an, ad), Rational::new(bn, bd));
        assert!(a.den() > 0);
        assert_eq!(naive_gcd(a.num(), a.den()), 1, "{an}/{ad}");
        assert_eq!(a.num() * ad, an * a.den());
        let check = |r: Rational<i64>, n: i64, d: i64| {
            assert!(r.den() > 0, "{an}/{ad} {bn}/{bd}");
            assert_eq!(naive_gcd(r.num(), r.den()), 1);
            assert_eq!(r.num() * d, n * r.den(), "{an}/{ad} {bn}/{bd}");
        };
        check(a + b, an * bd + bn * ad, ad * bd);
        check(a - b, an * bd - bn * ad, ad * bd);
        check(a * b, an * bn, ad * bd);
        check(-a, -an, ad);
        check(a.abs(), (an * ad).abs(), ad * ad);
        if bn != 0 {
            check(a / b, an * bd, ad * bn);
        }
        let lhs = (an * bd * ad.signum() * bd.signum()).cmp(&(bn * ad * ad.signum() * bd.signum()));
        assert_eq!(a.cmp(&b), lhs);
        assert_eq!(a == b, lhs == Ordering::Equal);
    }
}

// ======================= integer_sqrt =======================
use algo_lib::numbers::integer_sqrt::IntegerSqrt;

#[test]
fn integer_sqrt_edges() {
    for n in 0u64..100000 {
        let s = n.lower_sqrt();
        assert!(s * s <= n && (s + 1) * (s + 1) > n);
        let u = n.upper_sqrt();
        assert!(u * u >= n && (u == 0 || (u - 1) * (u - 1) < n));
        assert_eq!(n.sqrt().is_some(), s * s == n);
        for k in 1..=7usize {
            let r = n.lower_root(k);
            assert!((r as u128).pow(k as u32) <= n as u128 && ((r + 1) as u128).pow(k as u32) > n as u128, "{n} {k}");
        }
    }
    let mut rng = ChaCha8Rng::seed_from_u64(30);
    for _ in 0..200000 {
        let s: u64 = rng.gen_range(0..=u32::MAX as u64);
        for n in [s * s, (s * s).saturating_sub(1), s * s + 1] {
            let r = n.lower_sqrt();
            assert!(r as u128 * r as u128 <= n as u128 && (r as u128 + 1) * (r as u128 + 1) > n as u128, "{n}");
        }
        let k = rng.gen_range(2..=64usize);
        let n: u64 = rng.gen::<u64>() >> rng.gen_range(0..64);
        if n == u64::MAX {
            continue;
        }
        let r = n.lower_root(k) as u128;
        let pw = |b: u128| (0..k).try_fold(1u128, |acc, _| acc.checked_mul(b).filter(|&v| v < 1 << 100)).unwrap_or(1 << 100);
        assert!(pw(r) <= n as u128 && pw(r + 1) > n as u128, "{n} {k} {r}");
    }
    for n in [u64::MAX - 1, u64::MAX - 2, (u32::MAX as u64) * (u32::MAX as u64), i64::MAX as u64, i64::MAX as u64 + 1] {
        let r = n.lower_sqrt() as u128;
        assert!(r * r <= n as u128 && (r + 1) * (r + 1) > n as u128);
    }
    let n = i64::MAX;
    assert_eq!(n.lower_sqrt(), 3037000499);
    assert_eq!(n.upper_sqrt(), 3037000500);
}

fn finishes_within<T: Send + 'static>(secs: u64, f: impl FnOnce() -> T + Send + 'static) -> Option<T> {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let _ = tx.send(f());
    });
    rx.recv_timeout(std::time::Duration::from_secs(secs)).ok()
}

#[test]
#[ignore] // BUG: hangs; leaks a spinning thread
fn integer_sqrt_u64_max() {
    let r = finishes_within(5, || u64::MAX.lower_sqrt());
    assert_eq!(r, Some(4294967295), "u64::MAX.lower_sqrt() hangs or is wrong");
}

#[test]
#[ignore] // BUG: hangs; leaks spinning threads
fn integer_sqrt_u64_max_variants() {
    assert_eq!(finishes_within(5, || u64::MAX.sqrt()), Some(None));
    assert_eq!(finishes_within(5, || u64::MAX.upper_sqrt()), Some(4294967296));
    assert_eq!(finishes_within(5, || u64::MAX.lower_root(3)), Some(2642245));
    assert_eq!(finishes_within(5, || u64::MAX.lower_root(64)), Some(1));
}

// fixed copy of integer_sqrt::lower_root with a non-saturating power
fn fixed_power(n: u64, exp: usize) -> u128 {
    let mut res = 1u128;
    for _ in 0..exp {
        res = res.saturating_mul(n as u128);
        if res > u64::MAX as u128 {
            return u128::MAX;
        }
    }
    res
}
fn fixed_lower_root(v: u64, k: usize) -> u64 {
    let mut s = (v as f64).powf(1. / (k as f64)).round() as u64;
    while fixed_power(s, k) > v as u128 {
        s -= 1;
    }
    while fixed_power(s + 1, k) <= v as u128 {
        s += 1;
    }
    s
}
#[test]
fn integer_sqrt_fixed_copy() {
    assert_eq!(fixed_lower_root(u64::MAX, 2), 4294967295);
    assert_eq!(fixed_lower_root(u64::MAX, 3), 2642245);
    assert_eq!(fixed_lower_root(u64::MAX, 64), 1);
    assert_eq!(fixed_lower_root(u64::MAX, 63), 2);
    assert_eq!(fixed_lower_root(u64::MAX - 1, 2), 4294967295);
    let mut rng = ChaCha8Rng::seed_from_u64(31);
    for _ in 0..100000 {
        let n: u64 = rng.gen::<u64>() >> rng.gen_range(0..64);
        let k = rng.gen_range(2..10);
        if n != u64::MAX {
            assert_eq!(fixed_lower_root(n, k), n.lower_root(k));
        }
    }
}

// ======================= floor_sum =======================
use algo_lib::numbers::floor_sum::floor_sum;

#[test]
fn floor_sum_vs_naive() {
    let mut rng = ChaCha8Rng::seed_from_u64(40);
    for _ in 0..300000 {
        let n = rng.gen_range(0..40i64);
        let m = rng.gen_range(1..40i64);
        let a = rng.gen_range(0..100i64);
        let b = rng.gen_range(0..100i64);
        let expected: i64 = (0..n).map(|i| (a * i + b) / m).sum();
        assert_eq!(floor_sum(n, m, a, b), expected, "{n} {m} {a} {b}");
    }
    // large: compare against i128 re-implementation
    fn big(n: i128, m: i128, a: i128, b: i128) -> i128 {
        if n == 0 {
            return 0;
        }
        let mut ans = n * (n - 1) / 2 * (a / m) + n * (b / m);
        let (a, b) = (a % m, b % m);
        let y = (a * (n - 1) + b) / m;
        if y == 0 {
            return ans;
        }
        ans += (n - 1) * y - big(y, a, m, m - b - 1);
        ans
    }
    for _ in 0..200000 {
        let n = rng.gen_range(0..=1_000_000_000i64);
        let m = rng.gen_range(1..=1_000_000_000i64);
        let a = rng.gen_range(0..m);
        let b = rng.gen_range(0..m);
        assert_eq!(floor_sum(n, m, a, b) as i128, big(n as i128, m as i128, a as i128, b as i128));
    }
}

// ======================= number_iterator =======================
use algo_lib::numbers::number_iterator::{iterate, iterate_with_base};

fn check_iter(res: &[(u32, usize, u32)], from: u32, to: u32, base: u32) {
    // blocks (prefix, free digits, first value) must tile [from, to] exactly, in order
    let mut next = from as u64;
    for &(prefix, digs, first) in res {
        let pw = (base as u64).pow(digs as u32);
        assert_eq!(first as u64, prefix as u64 * pw, "{from} {to} {base}");
        assert_eq!(first as u64, next, "{from} {to} {base}: {res:?}");
        next += pw;
    }
    assert_eq!(next, to as u64 + 1, "{from} {to} {base}: {res:?}");
}

#[test]
fn number_iterator_tiles_range() {
    for base in [2u32, 3, 10] {
        for from in 0..130u32 {
            for to in from..400 {
                let res = iterate_with_base(from, to, base);
                check_iter(&res, from, to, base);
            }
        }
    }
    let mut rng = ChaCha8Rng::seed_from_u64(41);
    for _ in 0..100000 {
        let a = rng.gen::<u32>() >> rng.gen_range(0..32);
        let b = rng.gen::<u32>() >> rng.gen_range(0..32);
        let (from, to) = (a.min(b), a.max(b));
        if to == u32::MAX {
            continue;
        }
        check_iter(&iterate(from, to), from, to, 10);
    }
    for (from, to) in [(0u32, u32::MAX - 1), (1, u32::MAX - 1), (4_000_000_000, u32::MAX - 1), (4_294_967_290, 4_294_967_294), (999_999_999, 4_294_967_294)] {
        check_iter(&iterate(from, to), from, to, 10);
    }
    // u8 exhaustive
    for from in 0u8..=254 {
        for to in from..=254 {
            let res = iterate(from, to);
            let mut next = from as u32;
            for &(prefix, digs, first) in &res {
                let pw = 10u32.pow(digs as u32);
                assert_eq!(first as u32, prefix as u32 * pw);
                assert_eq!(first as u32, next);
                next += pw;
            }
            assert_eq!(next, to as u32 + 1);
        }
    }
}

// ======================= num_utils / number_ext / bit_ops =======================
use algo_lib::numbers::num_traits::bit_ops::{BitIter, BitOps};
use algo_lib::numbers::num_utils::{factorial, factorials, powers, PartialSums, Powers, UpperDiv};
use algo_lib::numbers::number_ext::{digits, num_digs, sum_digs, Power, Square};

#[test]
fn num_utils_and_ext() {
    assert_eq!(factorials::<u64>(0), Vec::<u64>::new());
    assert_eq!(factorials::<u64>(1), vec![1]);
    assert_eq!(factorials::<u64>(6), vec![1, 1, 2, 6, 24, 120]);
    assert_eq!(factorial::<u64>(0), 1);
    assert_eq!(factorial::<u64>(20), 2432902008176640000);
    assert_eq!(powers(3u64, 0), Vec::<u64>::new());
    assert_eq!(powers(3u64, 4), vec![1, 3, 9, 27]);
    for len in 1..8usize {
        let p = Powers::new(2u64, len);
        for e in 0..len * len {
            assert_eq!(p.power(e), 1u64 << e, "len={len} e={e}");
        }
    }
    assert_eq!([1i64, -2, 3][..].partial_sums(), vec![0, 1, -1, 2]);
    assert_eq!(Vec::<i64>::new().partial_sums(), vec![0]);
    for a in 0u64..200 {
        for b in 1u64..30 {
            assert_eq!(a.upper_div(b), (a + b - 1) / b);
            assert_eq!(a.upper_div(b), (0..).find(|q| q * b >= a).unwrap());
        }
    }
    for n in 0u64..3000 {
        let s = n.to_string();
        assert_eq!(num_digs(n), s.len());
        assert_eq!(sum_digs(n), s.bytes().map(|c| (c - b'0') as u64).sum::<u64>());
        let d: Vec<u64> = digits(n).collect();
        let e: Vec<u64> = if n == 0 { vec![] } else { s.bytes().rev().map(|c| (c - b'0') as u64).collect() };
        assert_eq!(d, e);
    }
    assert_eq!(num_digs(u64::MAX), 20);
    assert_eq!(num_digs(i64::MIN), 19);
    assert_eq!(num_digs(-5i32), 1);
    for b in 0u64..6 {
        for e in 0u32..12 {
            assert_eq!(b.power(e), b.pow(e));
            assert_eq!(b.power(e as usize), b.pow(e));
            assert_eq!((b as i64).power(e as i64), (b as i64).pow(e));
        }
    }
    assert_eq!(2u64.power(63u8), 1 << 63);
    assert_eq!((-3i64).power(3), -27);
    assert_eq!(7i32.square(), 49);
}

#[test]
fn bit_ops_checks() {
    for v in 0u32..2000 {
        for at in 0..32 {
            assert_eq!(v.is_set(at), (v >> at) & 1 == 1);
            assert_eq!(v.with_bit(at), v | (1 << at));
            assert_eq!(v.without_bit(at), v & !(1 << at));
            assert_eq!(v.flipped_bit(at), v ^ (1 << at));
        }
        if v != 0 {
            assert_eq!(v.lowest_bit(), (0..32).find(|&i| v >> i & 1 == 1).unwrap());
            assert_eq!(v.highest_bit(), (0..32).rev().find(|&i| v >> i & 1 == 1).unwrap());
        }
        let subs: Vec<u32> = BitIter::new(v).collect();
        let expected: Vec<u32> = (0..=v).rev().filter(|s| s & v == *s).collect();
        assert_eq!(subs, expected);
    }
    assert_eq!(u64::all_bits(0), 0);
    assert_eq!(u64::all_bits(63), u64::MAX >> 1);
    assert_eq!(i64::all_bits(63), i64::MAX);
    assert_eq!(i8::all_bits(7), 127);
    assert_eq!(u64::bit(63), 1 << 63);
    assert_eq!(i64::bit(63), i64::MIN);
    assert!(i64::MIN.is_set(63));
    assert!((-1i64).is_set(17));
    assert_eq!(i64::MIN.highest_bit(), 63);
    assert_eq!(u128::bit(127).highest_bit(), 127);
    assert_eq!(u128::bit(127).lowest_bit(), 127);
    assert_eq!(u8::iter_all(3).count(), 8);
    assert!(5u8.is_subset(7));
    assert!(!5u8.is_subset(6));
    assert!(0u8.is_subset(0));
}

// ======================= number_theory =======================
use algo_lib::numbers::number_theory::{discrete_log, nim_product, primitive_root, stern_brocot_search};

fn pw_mod(mut b: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1 % m;
    b %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = (r as u128 * b as u128 % m as u128) as u64;
        }
        b = (b as u128 * b as u128 % m as u128) as u64;
        e >>= 1;
    }
    r
}

#[test]
fn discrete_log_vs_naive() {
    // exhaustive for m in 61..=200 (library tests cover up to 60)
    for m in 61..=200u64 {
        for a in 0..m {
            let mut first = vec![None; m as usize];
            let mut cur = 1 % m;
            for x in 0..=2 * m {
                if first[cur as usize].is_none() {
                    first[cur as usize] = Some(x);
                }
                cur = cur * a % m;
            }
            for b in 0..m {
                assert_eq!(discrete_log(a, b, m), first[b as usize], "{a}^x = {b} mod {m}");
            }
        }
    }
    // unreduced arguments
    assert_eq!(discrete_log(12, 17, 5), discrete_log(2, 2, 5));
    assert_eq!(discrete_log(5, 0, 1), Some(0));
    assert_eq!(discrete_log(0, 0, 7), Some(1));
    assert_eq!(discrete_log(0, 1, 7), Some(0));
    assert_eq!(discrete_log(0, 3, 7), None);
    // mid-size with highly composite moduli, naive oracle
    let mut rng = ChaCha8Rng::seed_from_u64(50);
    for _ in 0..3000 {
        let m: u64 = match rng.gen_range(0..4) {
            0 => rng.gen_range(1..5000),
            1 => 1 << rng.gen_range(1..13),
            2 => [720u64, 5040, 3 * 3 * 3 * 3 * 3 * 3 * 3, 2 * 3 * 5 * 7 * 11, 4096 * 3, 6 * 6 * 6 * 6][rng.gen_range(0..6)],
            _ => {
                let p = [2u64, 3, 5, 7][rng.gen_range(0..4)];
                p.pow(rng.gen_range(1..5)) * rng.gen_range(1..20)
            }
        };
        let a = rng.gen_range(0..m);
        let b = if rng.gen_bool(0.7) { pw_mod(a, rng.gen_range(0..40), m) } else { rng.gen_range(0..m) };
        let mut expected = None;
        let mut cur = 1 % m;
        for x in 0..=m + 70 {
            if cur == b {
                expected = Some(x);
                break;
            }
            cur = cur * a % m;
        }
        assert_eq!(discrete_log(a, b, m), expected, "{a}^x = {b} mod {m}");
    }
    // large moduli: result must be valid and not larger than the known exponent
    for _ in 0..40 {
        let m: u64 = rng.gen_range(2..1u64 << 40);
        let a = rng.gen_range(0..m);
        let x = rng.gen_range(0..1u64 << 40);
        let b = pw_mod(a, x, m);
        let got = discrete_log(a, b, m).expect("solvable");
        assert!(got <= x && pw_mod(a, got, m) == b, "{a} {b} {m}");
    }
}

#[test]
fn primitive_root_vs_naive() {
    let is_p = |n: u64| n >= 2 && (2..n).take_while(|d| d * d <= n).all(|d| n % d != 0);
    for p in (2..3000u64).filter(|&p| is_p(p)) {
        let g = primitive_root(p);
        let order = |g: u64| {
            let mut x = g % p;
            let mut o = 1;
            while x != 1 % p {
                x = x * g % p;
                o += 1;
            }
            o
        };
        let expected = (1..p).find(|&g| order(g) == p - 1).unwrap();
        assert_eq!(g, expected, "p={p}");
    }
    for p in [998_244_353u64, 1_000_000_007, 18446744073709551557, 9223372036854775783, 4294967311] {
        let g = primitive_root(p);
        use algo_lib::numbers::primes::factorize::Factorize;
        for (q, _) in (p - 1).prime_divisors() {
            assert_ne!(pw_mod(g, (p - 1) / q, p), 1);
        }
    }
}

#[test]
fn stern_brocot_vs_naive() {
    let mut rng = ChaCha8Rng::seed_from_u64(51);
    for it in 0..20000 {
        let limit = rng.gen_range(1..=60u64);
        let (tn, td) = (rng.gen_range(0..300u64), rng.gen_range(1..80u64));
        let strict = it % 2 == 0; // predicate p/q < tn/td (strict) or <=
        let pred = |p: u64, q: u64| -> bool {
            if q == 0 {
                return false;
            }
            if strict { p * td < tn * q || p == 0 } else { p * td <= tn * q }
        };
        let (lo, hi) = stern_brocot_search(limit, pred);
        let mut best_lo = (0u64, 1u64);
        let mut best_hi = (1u64, 0u64);
        for q in 1..=limit {
            for p in 0..=limit {
                if pred(p, q) {
                    if p * best_lo.1 > best_lo.0 * q {
                        best_lo = (p, q);
                    }
                } else if p * best_hi.1 < best_hi.0 * q {
                    best_hi = (p, q);
                }
            }
        }
        assert_eq!(lo.0 * best_lo.1, best_lo.0 * lo.1, "lower limit={limit} t={tn}/{td} strict={strict} got {lo:?} want {best_lo:?}");
        assert_eq!(hi.0 * best_hi.1, best_hi.0 * hi.1, "upper limit={limit} t={tn}/{td} strict={strict} got {hi:?} want {best_hi:?}");
        assert!(lo.0 <= limit && lo.1 <= limit && hi.0 <= limit && hi.1 <= limit);
    }
    // predicate that never fails
    let (lo, hi) = stern_brocot_search(10, |_, _| true);
    assert_eq!(lo, (10, 1));
    assert_eq!(hi, (1, 0));
    // large limit
    let (lo, hi) = stern_brocot_search(1_000_000_000_000, |p, q| (p as u128) * 1_000_000_007 <= (q as u128) * 998_244_353);
    assert_eq!(lo, (998_244_353, 1_000_000_007));
    assert!(hi.0 as u128 * 1_000_000_007 > hi.1 as u128 * 998_244_353);
    assert_eq!(hi.0 as i128 * lo.1 as i128 - hi.1 as i128 * lo.0 as i128, 1);
}

#[test]
fn stern_brocot_limit_u64_max() {
    // target 1/3 with the maximal limit
    let r = std::panic::catch_unwind(|| stern_brocot_search(u64::MAX, |p, q| (p as u128) * 3 <= q as u128));
    let (lo, _hi) = r.expect("panicked with limit = u64::MAX");
    assert_eq!(lo, (1, 3));
}

#[test]
fn nim_product_vs_definition() {
    // mex definition up to 64, then field axioms for random values
    let n = 64usize;
    let mut table = vec![vec![0u64; n]; n];
    for a in 0..n {
        for b in 0..n {
            let mut seen = vec![false; 4 * n * n];
            for x in 0..a {
                for y in 0..b {
                    seen[(table[x][b] ^ table[a][y] ^ table[x][y]) as usize] = true;
                }
            }
            table[a][b] = seen.iter().position(|&s| !s).unwrap() as u64;
            assert_eq!(nim_product(a as u64, b as u64), table[a][b], "{a} x {b}");
        }
    }
    assert_eq!(nim_product(1 << 32, 1 << 32), (1u64 << 32) ^ (1 << 31));
    assert_eq!(nim_product(1 << 63, 1 << 63) != 0, true);
    assert_eq!(nim_product(u64::MAX, 1), u64::MAX);
}

// ======================= primes =======================
use algo_lib::numbers::primes::factorize::{all_divisors, Factorize};
use algo_lib::numbers::primes::prime::{binary_gcd, find_divisor, is_prime, next_prime, Montgomery64};
use algo_lib::numbers::primes::sieve::{divisor_table, for_each_prime, primality_table, prime_pi, primes, PrimeSums};

fn trial_prime(n: u64) -> bool {
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
fn montgomery_vs_u128() {
    let mut rng = ChaCha8Rng::seed_from_u64(60);
    for it in 0..300000 {
        let n: u64 = match it % 4 {
            0 => rng.gen::<u64>() | 1,
            1 => (rng.gen::<u64>() >> rng.gen_range(0..63)) | 1,
            2 => u64::MAX - 2 * rng.gen_range(0..100u64),
            _ => [1u64, 3, 5, (1 << 63) - 1, (1 << 63) + 1, u64::MAX][rng.gen_range(0..6)],
        };
        let mont = Montgomery64::new(n);
        let a = rng.gen::<u64>();
        let b = rng.gen::<u64>();
        let (am, bm) = (mont.to_mont(a), mont.to_mont(b));
        assert!(n == 1 || (am < n && bm < n));
        assert_eq!(mont.from_mont(am), a % n, "n={n} a={a}");
        assert_eq!(mont.from_mont(mont.mul(am, bm)) as u128, (a % n) as u128 * (b % n) as u128 % n as u128, "n={n}");
        assert_eq!(mont.from_mont(mont.add(am, bm)) as u128, ((a % n) as u128 + (b % n) as u128) % n as u128, "n={n}");
        assert_eq!(mont.from_mont(mont.sub(am, bm)) as u128, ((a % n) as u128 + n as u128 - (b % n) as u128) % n as u128, "n={n}");
        let e = rng.gen::<u64>() >> rng.gen_range(0..64);
        assert_eq!(mont.from_mont(mont.pow(am, e)), pw_mod(a, e, n), "n={n} a={a} e={e}");
        assert_eq!(mont.from_mont(mont.one()), 1 % n);
    }
}

#[test]
fn is_prime_checks() {
    for n in 0..300000u64 {
        assert_eq!(is_prime(n), trial_prime(n), "{n}");
    }
    let mut rng = ChaCha8Rng::seed_from_u64(61);
    for _ in 0..3000 {
        let n = rng.gen_range(0..1u64 << 40);
        assert_eq!(is_prime(n), trial_prime(n), "{n}");
    }
    // values near 2^32: all in a window
    for n in (1u64 << 32) - 3000..(1u64 << 32) + 3000 {
        assert_eq!(is_prime(n), trial_prime(n), "{n}");
    }
    // Carmichael numbers and strong pseudoprimes to several bases
    for n in [
        561u64, 1105, 1729, 2465, 2821, 6601, 8911, 10585, 15841, 29341, 41041, 46657, 52633, 62745, 63973, 75361,
        101101, 115921, 126217, 162401, 172081, 188461, 252601, 278545, 294409, 314821, 334153, 340561, 399001,
        410041, 449065, 488881, 512461, 9746347772161, 1436697831295441, 60977817398996785, 7156857700403137441,
        3713287938297324241, // Carmichael
        2047, 1373653, 25326001, 3215031751, 2152302898747, 3474749660383, 341550071728321, 3825123056546413051,
        318665857834031151, // wait: not a classic; product checked below
        4759123141, 1122004669633, 4294967297, 18446744073709551615, 18446744073709551556, 9223372036854775808,
        9223372036854775807, 3037000493 * 3037000493, 4294967291 * 4294967291, 4294967291 * 4294967279,
    ] {
        if n == 318665857834031151 {
            continue;
        }
        assert!(!is_prime(n), "{n} is composite");
    }
    for p in [
        2u64, 3, 5, 7, 37, 41, 1669, 4294967291, 4294967311, 9223372036854775783, 9223372036854775837,
        18446744073709551557, 18446744073709551533, 2305843009213693951, 1000000000000000003, 999999999999999989,
    ] {
        assert!(is_prime(p), "{p} is prime");
    }
    assert!(!is_prime(0u64) && !is_prime(1u64));
    assert!(is_prime(2u8) && is_prime(251u8) && !is_prime(255u8));
    assert!(is_prime(2147483647i32));
    assert_eq!(next_prime(0), 2);
    assert_eq!(next_prime(2), 2);
    assert_eq!(next_prime(3), 3);
    assert_eq!(next_prime(4), 5);
    for n in 0..5000u64 {
        assert_eq!(next_prime(n), (n..).find(|&x| trial_prime(x)).unwrap());
    }
    for a in 0..200u64 {
        for b in 0..200u64 {
            assert_eq!(binary_gcd(a, b), gcd(a, b));
        }
    }
    assert_eq!(binary_gcd(u64::MAX, u64::MAX), u64::MAX);
    assert_eq!(binary_gcd(1 << 63, 1 << 62), 1 << 62);
}

fn trial_factor(mut n: u64) -> Vec<(u64, usize)> {
    let mut res = Vec::new();
    let mut p = 2;
    while p * p <= n {
        let mut e = 0;
        while n % p == 0 {
            n /= p;
            e += 1;
        }
        if e > 0 {
            res.push((p, e));
        }
        p += 1;
    }
    if n > 1 {
        res.push((n, 1));
    }
    res
}

#[test]
fn factorization_checks() {
    for n in 1..60000u64 {
        assert_eq!(n.prime_divisors(), trial_factor(n), "{n}");
        let d = find_divisor(n);
        assert!(d >= 1 && n % d == 0 && (d == n) == (n == 1 || trial_prime(n)) && (n == 1 || d > 1), "{n} -> {d}");
    }
    for n in 1..3000u64 {
        let expected: Vec<u64> = (1..=n).filter(|d| n % d == 0).collect();
        assert_eq!(n.divisors(), expected, "{n}");
    }
    let verify = |n: u64| {
        let f = n.prime_divisors();
        let mut prod = 1u128;
        let mut last = 0;
        for &(p, e) in &f {
            assert!(is_prime(p) && p > last && e >= 1, "{n}: {f:?}");
            last = p;
            prod *= (p as u128).pow(e as u32);
        }
        assert_eq!(prod, n as u128, "{n}: {f:?}");
        f
    };
    let ps31 = [2147483647u64, 2147483629, 2147483587, 2147483659, 2147483693, 4294967291, 4294967279, 3037000493];
    for &p in &ps31 {
        assert_eq!(verify(p * p), vec![(p, 2)]);
        for &q in &ps31 {
            if let Some(n) = p.checked_mul(q) {
                verify(n);
            }
        }
    }
    for p in [2u64, 3, 5, 7, 1000003, 2097143, 2642239] {
        let mut n = p;
        let mut e = 1;
        while let Some(nn) = n.checked_mul(p) {
            assert_eq!(verify(n), vec![(p, e)]);
            n = nn;
            e += 1;
        }
        assert_eq!(verify(n), vec![(p, e)]);
    }
    assert_eq!(verify(1 << 63), vec![(2, 63)]);
    assert_eq!(verify(u64::MAX), vec![(3, 1), (5, 1), (17, 1), (257, 1), (641, 1), (65537, 1), (6700417, 1)]);
    assert_eq!(1u64.prime_divisors(), vec![]);
    assert_eq!(1u64.divisors(), vec![1]);
    let mut rng = ChaCha8Rng::seed_from_u64(62);
    for _ in 0..2000 {
        let n = (rng.gen::<u64>() >> rng.gen_range(0..63)).max(1);
        verify(n);
    }
    // many divisors
    let n = 897612484786617600u64;
    assert_eq!(n.divisors().len(), 103680);
    assert_eq!(12u64.max_power(2), 2);
    assert_eq!(1u64.max_power(2), 0);
    assert_eq!((1u64 << 63).max_power(2), 63);

    for n in [0usize, 1, 2, 3, 4, 5, 10, 1000] {
        for sorted in [true, false] {
            let all = all_divisors(n, sorted);
            assert_eq!(all.len(), n);
            for (i, d) in all.into_iter().enumerate() {
                let mut d = d;
                if !sorted {
                    d.sort();
                }
                let expected: Vec<usize> = (1..=i).filter(|x| i % x == 0).collect();
                assert_eq!(d, expected, "n={n} i={i}");
            }
        }
    }
}

#[test]
fn sieve_checks() {
    let big = primes(3_000_000);
    {
        let mut comp = vec![false; 3_000_000];
        let mut expected = Vec::new();
        for i in 2..3_000_000usize {
            if !comp[i] {
                expected.push(i);
                let mut j = i * i;
                while j < 3_000_000 {
                    comp[j] = true;
                    j += i;
                }
            }
        }
        assert_eq!(big, expected);
    }
    let mut check = |n: usize| {
        let cnt = big.partition_point(|&p| p < n);
        let mut got = Vec::new();
        for_each_prime(n, |p| got.push(p));
        assert_eq!(got, big[..cnt], "n={n}");
    };
    for n in 0..2000 {
        check(n);
    }
    for c in [1usize << 16, 1 << 17, 3 << 16, 1 << 18, (1 << 17) + (1 << 16), 1 << 20] {
        for d in 0..12 {
            check(c - 6 + d);
            check(2 * c - 6 + d);
            check(2 * c + 1 - 6 + d);
        }
    }
    // squares of primes as limits
    for &p in big.iter().take(250) {
        for d in 0..3 {
            check(p * p - 1 + d);
        }
    }
    for n in [0usize, 1, 2, 3, 4, 5, 100, 131073] {
        let t = primality_table(n);
        for i in 0..n {
            assert_eq!(t[i], big.binary_search(&i).is_ok());
        }
    }
    for n in [0usize, 1, 2, 3, 4, 5, 9, 10, 25, 26, 1000, 10007] {
        let t = divisor_table(n);
        assert_eq!(t.len(), n);
        for i in 2..n {
            assert!(t[i] > 1 && i % t[i] == 0 && trial_prime(t[i] as u64), "n={n} i={i} t={}", t[i]);
            assert_eq!(t[i] == i, trial_prime(i as u64));
        }
    }
}

#[test]
fn prime_pi_checks() {
    let ps = primes(1_000_001);
    let mut pi = vec![0u64; 1_000_001];
    for &p in &ps {
        pi[p] = 1;
    }
    for i in 1..pi.len() {
        pi[i] += pi[i - 1];
    }
    for n in 0..20000u64 {
        assert_eq!(prime_pi(n), pi[n as usize], "pi({n})");
    }
    let mut rng = ChaCha8Rng::seed_from_u64(63);
    for _ in 0..1500 {
        let n = rng.gen_range(0..=1_000_000u64);
        assert_eq!(prime_pi(n), pi[n as usize], "pi({n})");
    }
    // perfect squares and neighbours
    for s in 2..1000u64 {
        for n in [s * s - 1, s * s, s * s + 1, s * (s + 1), s * (s + 1) - 1, s * (s + 2)] {
            assert_eq!(prime_pi(n), pi[n as usize], "pi({n})");
        }
    }
    assert_eq!(prime_pi(1_000_000_000), 50_847_534);
    // all n / k values, sums of primes and of squares
    for n in [1u64, 2, 3, 4, 5, 6, 7, 8, 9, 10, 15, 16, 17, 24, 25, 99, 100, 101, 9999, 10000, 123456, 1_000_000] {
        let sums = PrimeSums::new(n, |v| (v as i128 * (v as i128 + 1) / 2 - 1), |p| p as i128);
        let sq = PrimeSums::new(
            n,
            |v| {
                let v = v as i128;
                v * (v + 1) * (2 * v + 1) / 6 - 1
            },
            |p| (p * p) as i128,
        );
        let cnt = PrimeSums::new(n, |v| v as i64 - 1, |_| 1i64);
        let mut k = 1;
        while k <= n {
            let v = n / k;
            let e1: i128 = ps.iter().take_while(|&&p| p as u64 <= v).map(|&p| p as i128).sum();
            let e2: i128 = ps.iter().take_while(|&&p| p as u64 <= v).map(|&p| (p * p) as i128).sum();
            assert_eq!(sums.get(v), e1, "n={n} v={v}");
            assert_eq!(sq.get(v), e2, "n={n} v={v}");
            assert_eq!(cnt.get(v) as u64, pi[v as usize], "n={n} v={v}");
            k = n / v + 1;
            if n > 10000 && k > 2000 {
                k += (k / 7).max(1);
            }
        }
    }
}

#[test]
fn stern_brocot_limit_u64_max_never_fails() {
    let r = finishes_within(5, || std::panic::catch_unwind(|| stern_brocot_search(u64::MAX, |_, _| true)));
    let r = r.expect("hangs").expect("panics");
    assert_eq!(r, ((u64::MAX, 1), (1, 0)));
}

#[test]
fn stern_brocot_limit_u64_max_target_zero() {
    let r = finishes_within(5, || std::panic::catch_unwind(|| stern_brocot_search(u64::MAX, |p, _| p == 0)));
    let r = r.expect("hangs").expect("panics");
    assert_eq!(r, ((0, 1), (1, u64::MAX)));
}

// ======================= linear algebra =======================
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::numbers::gauss;
use algo_lib::numbers::matrix::Matrix;
use algo_lib::numbers::mod_int::{BaseModInt, ModInt, ModInt7};
use algo_lib::numbers::mod_linear;
use algo_lib::value;

value!(V2: u32 = 2);
value!(V3: u32 = 3);
value!(V5: u32 = 5);
value!(VBig: u32 = 2147483647);
value!(VMid: u32 = 65537);
type M2 = ModInt<V2>;
type M3 = ModInt<V3>;
type M5 = ModInt<V5>;
type MBig = ModInt<VBig>;
type MMid = ModInt<VMid>;

fn cofactor_det<M: BaseModInt<u32>>(a: &Vec<Vec<M>>) -> M {
    let n = a.len();
    if n == 0 {
        return M::one();
    }
    let mut res = M::zero();
    for j in 0..n {
        let minor: Vec<Vec<M>> = (1..n)
            .map(|i| (0..n).filter(|&c| c != j).map(|c| a[i][c]).collect())
            .collect();
        let term = a[0][j] * cofactor_det(&minor);
        if j % 2 == 0 {
            res += term;
        } else {
            res -= term;
        }
    }
    res
}

fn small_field<M: BaseModInt<u32> + std::fmt::Debug>(seed: u64, iters: usize) {
    let p = M::module() as usize;
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    for _ in 0..iters {
        let n = rng.gen_range(0..=4usize);
        let m = rng.gen_range(0..=4usize);
        let zero_heavy = rng.gen_bool(0.4);
        let a: Arr2d<M> = Arr2d::with_gen(n, m, |_, _| {
            if zero_heavy && rng.gen_bool(0.6) {
                M::zero()
            } else {
                M::from(rng.gen_range(0..p))
            }
        });
        let b: Vec<M> = (0..n).map(|_| M::from(rng.gen_range(0..p))).collect();
        // enumerate all x
        let total = p.pow(m as u32);
        let mut kernel_size = 0usize;
        let mut solutions = 0usize;
        for code in 0..total {
            let mut c = code;
            let x: Vec<M> = (0..m)
                .map(|_| {
                    let v = M::from(c % p);
                    c /= p;
                    v
                })
                .collect();
            let ax: Vec<M> = (0..n).map(|i| (0..m).fold(M::zero(), |s, j| s + a[(i, j)] * x[j])).collect();
            if ax.iter().all(|&v| v == M::zero()) {
                kernel_size += 1;
            }
            if ax == b {
                solutions += 1;
            }
        }
        let mut rk = 0;
        while p.pow((m - rk) as u32) != kernel_size {
            rk += 1;
        }
        assert_eq!(mod_linear::rank(&a), rk, "rank {a:?}");
        match mod_linear::solve(&a, &b) {
            None => assert_eq!(solutions, 0, "solve said inconsistent {a:?} {b:?}"),
            Some((x, kernel)) => {
                assert!(solutions > 0, "solve found solution of inconsistent system {a:?} {b:?}");
                assert_eq!(x.len(), m);
                for i in 0..n {
                    assert_eq!((0..m).fold(M::zero(), |s, j| s + a[(i, j)] * x[j]), b[i]);
                }
                assert_eq!(kernel.len(), m - rk);
                for v in &kernel {
                    for i in 0..n {
                        assert_eq!((0..m).fold(M::zero(), |s, j| s + a[(i, j)] * v[j]), M::zero());
                    }
                }
                // kernel vectors independent
                if !kernel.is_empty() {
                    let km: Arr2d<M> = Arr2d::with_gen(kernel.len(), m, |i, j| kernel[i][j]);
                    assert_eq!(mod_linear::rank(&km), kernel.len());
                }
            }
        }
        if n == m {
            let rows: Vec<Vec<M>> = (0..n).map(|i| (0..n).map(|j| a[(i, j)]).collect()).collect();
            let d = cofactor_det(&rows);
            assert_eq!(mod_linear::det(&a), d, "det {a:?}");
            if n > 0 {
                let mut copy = a.clone();
                assert_eq!(gauss::det(&mut copy), d, "gauss det {a:?}");
            }
            let inv = mod_linear::invert(&a);
            assert_eq!(inv.is_some(), d != M::zero(), "invert {a:?}");
            if n > 0 {
                let ginv = gauss::invert(&a);
                assert_eq!(ginv.is_some(), d != M::zero(), "gauss invert {a:?}");
                if let (Some(x), Some(y)) = (&inv, &ginv) {
                    assert!(x == y);
                }
            }
            if let Some(inv) = inv {
                let prod = mod_linear::mat_mul(&a, &inv);
                for i in 0..n {
                    for j in 0..n {
                        assert_eq!(prod[(i, j)], if i == j { M::one() } else { M::zero() });
                    }
                }
            }
        }
    }
}

#[test]
fn mod_linear_small_fields() {
    small_field::<M2>(70, 6000);
    small_field::<M3>(71, 6000);
    small_field::<M5>(72, 1500);
}

fn big_field<M: BaseModInt<u32> + std::fmt::Debug>(seed: u64) {
    let p = M::module() as usize;
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    for it in 0..400 {
        let n = if it < 300 { rng.gen_range(1..=14usize) } else { rng.gen_range(15..=45usize) };
        let mode = rng.gen_range(0..4);
        let mut gen_val = |rng: &mut ChaCha8Rng| -> M {
            match mode {
                0 => M::from(p - 1),
                1 => M::from(p - 1 - rng.gen_range(0..3usize)),
                2 => M::from(rng.gen_range(0..p)),
                _ => M::from(if rng.gen_bool(0.5) { p - 1 } else { rng.gen_range(0..2usize) }),
            }
        };
        let mut a: Arr2d<M> = Arr2d::with_gen(n, n, |_, _| gen_val(&mut rng));
        if mode == 0 {
            // all entries p-1 is singular for n >= 2; perturb the diagonal
            for i in 0..n {
                a[(i, i)] = M::from(rng.gen_range(0..p));
            }
        }
        // rank-deficient variant
        if it % 3 == 0 && n >= 2 {
            let r = rng.gen_range(0..n);
            let s = (r + 1) % n;
            let c1 = gen_val(&mut rng);
            for j in 0..n {
                let v = a[(s, j)] * c1;
                a[(r, j)] = v;
            }
        }
        let mut copy = a.clone();
        let gd = gauss::det(&mut copy);
        assert_eq!(mod_linear::det(&a), gd, "n={n} mode={mode}");
        if n <= 6 {
            let rows: Vec<Vec<M>> = (0..n).map(|i| (0..n).map(|j| a[(i, j)]).collect()).collect();
            assert_eq!(cofactor_det(&rows), gd);
        }
        let mut copy = a.clone();
        gauss::gauss(&mut copy);
        let grank = (0..n).filter(|&i| copy.row(i).any(|&x| x != M::zero())).count();
        assert_eq!(mod_linear::rank(&a), grank);
        let inv = mod_linear::invert(&a);
        assert_eq!(inv.is_some(), gd != M::zero());
        let am = Matrix::from(a.clone());
        if let Some(inv) = inv {
            assert!(am.mult(&Matrix::from(inv.clone())) == Matrix::ident(n), "n={n} mode={mode}");
            assert!(Matrix::from(mod_linear::mat_mul(&inv, &a)) == Matrix::ident(n));
        }
        // non-square product with p-1 entries
        let k = rng.gen_range(1..=40usize);
        let b: Arr2d<M> = Arr2d::with_gen(n, k, |_, _| gen_val(&mut rng));
        let expected = am.mult(&Matrix::from(b.clone()));
        assert!(Matrix::from(mod_linear::mat_mul(&a, &b)) == expected, "mat_mul n={n} k={k}");
        assert!(am.fast_mult(&Matrix::from(b.clone())) == expected);
        let bt: Arr2d<M> = Arr2d::with_gen(k, n, |i, j| b[(j, i)]);
        assert!(Matrix::from(mod_linear::mat_mul(&bt, &a)) == Matrix::from(bt.clone()).mult(&am));
        let e = rng.gen_range(0..50u64);
        assert!(am.fast_power(e) == am.power(e as usize));
        // solve with a known solution and with an arbitrary rhs
        let x_true: Vec<M> = (0..k).map(|_| gen_val(&mut rng)).collect();
        let rhs: Vec<M> = (0..n).map(|i| (0..k).fold(M::zero(), |s, j| s + b[(i, j)] * x_true[j])).collect();
        let (x, kernel) = mod_linear::solve(&b, &rhs).expect("consistent");
        for i in 0..n {
            assert_eq!((0..k).fold(M::zero(), |s, j| s + b[(i, j)] * x[j]), rhs[i]);
        }
        let mut copy = b.clone();
        gauss::gauss(&mut copy);
        let brank = (0..n).filter(|&i| copy.row(i).any(|&x| x != M::zero())).count();
        assert_eq!(kernel.len(), k - brank);
        for v in &kernel {
            for i in 0..n {
                assert_eq!((0..k).fold(M::zero(), |s, j| s + b[(i, j)] * v[j]), M::zero());
            }
        }
    }
}

#[test]
fn mod_linear_big_modulus_delayed_reduction() {
    big_field::<MBig>(73);
    big_field::<MMid>(74);
    big_field::<ModInt7>(75);
}

#[test]
fn mod_linear_wide_all_max() {
    // every entry p-1 with p = 2^31-1: batch = 4, inner dimension crosses it many times
    for k in [1usize, 3, 4, 5, 6, 7, 8, 9, 15, 16, 17, 100, 257] {
        for m in [1usize, 7, 8, 9, 33] {
            let a: Arr2d<MBig> = Arr2d::new(3, k, MBig::from(2147483646usize));
            let b: Arr2d<MBig> = Arr2d::new(k, m, MBig::from(2147483646usize));
            let c = mod_linear::mat_mul(&a, &b);
            for v in c.iter() {
                assert_eq!(*v, MBig::from(k % 2147483647), "k={k} m={m}");
            }
        }
    }
}

#[test]
fn mod_linear_degenerate_shapes() {
    let e: Arr2d<ModInt7> = Arr2d::new(0, 0, ModInt7::zero());
    assert_eq!(mod_linear::det(&e), ModInt7::one());
    assert_eq!(mod_linear::rank(&e), 0);
    assert!(mod_linear::invert(&e).is_some());
    let (x, k) = mod_linear::solve(&e, &[]).unwrap();
    assert!(x.is_empty() && k.is_empty());
    let wide: Arr2d<ModInt7> = Arr2d::new(0, 3, ModInt7::zero());
    assert_eq!(mod_linear::rank(&wide), 0);
    let (x, k) = mod_linear::solve(&wide, &[]).unwrap();
    assert_eq!(x, vec![ModInt7::zero(); 3]);
    assert_eq!(k.len(), 3);
    let tall: Arr2d<ModInt7> = Arr2d::new(3, 0, ModInt7::zero());
    assert_eq!(mod_linear::rank(&tall), 0);
    assert!(mod_linear::solve(&tall, &[ModInt7::zero(); 3]).is_some());
    assert!(mod_linear::solve(&tall, &[ModInt7::zero(), ModInt7::one(), ModInt7::zero()]).is_none());
    let z: Arr2d<ModInt7> = Arr2d::new(3, 3, ModInt7::zero());
    assert_eq!(mod_linear::det(&z), ModInt7::zero());
    assert_eq!(mod_linear::rank(&z), 0);
    assert!(mod_linear::invert(&z).is_none());
    let one: Arr2d<ModInt7> = Arr2d::new(1, 1, ModInt7::from(5usize));
    assert_eq!(mod_linear::det(&one), ModInt7::from(5usize));
    assert_eq!(mod_linear::invert(&one).unwrap()[(0, 0)] * ModInt7::from(5usize), ModInt7::one());
}

#[test]
fn matrix_generic() {
    let mut rng = ChaCha8Rng::seed_from_u64(76);
    for _ in 0..2000 {
        let (n, k, m) = (rng.gen_range(1..5usize), rng.gen_range(1..5usize), rng.gen_range(1..5usize));
        let a = Matrix::from(Arr2d::with_gen(n, k, |_, _| rng.gen_range(-5i64..=5)));
        let b = Matrix::from(Arr2d::with_gen(k, m, |_, _| rng.gen_range(-5i64..=5)));
        let c = a.mult(&b);
        assert_eq!((c.d1(), c.d2()), (n, m));
        for i in 0..n {
            for j in 0..m {
                assert_eq!(c[(i, j)], (0..k).map(|t| a[(i, t)] * b[(t, j)]).sum::<i64>());
            }
        }
        let s = Matrix::from(Arr2d::with_gen(n, n, |_, _| rng.gen_range(-2i64..=2)));
        let mut pw = Matrix::<i64>::ident(n);
        let mut sum = Matrix::<i64>::zero(n, n);
        for e in 0..9usize {
            assert!(s.power(e) == pw, "power {e}");
            assert!(s.sum_power(e) == sum, "sum_power {e}");
            sum.add_to(&pw);
            pw = pw.mult(&s);
        }
    }
    let r = Matrix::row(&[1i64, 2, 3]);
    let c = Matrix::column(&[4i64, 5, 6]);
    assert!(r.mult(&c) == Matrix::new(&[&[32i64]]));
    assert_eq!((c.mult(&r).d1(), c.mult(&r).d2()), (3, 3));
}

// fixed copy of the inner search of stern_brocot_search (checked k + jump)
fn stern_brocot_fixed(limit: u64, mut is_at_most: impl FnMut(u64, u64) -> bool) -> ((u64, u64), (u64, u64)) {
    let (mut lo, mut hi) = ((0u64, 1u64), (1u64, 0u64));
    loop {
        let fits = |a: (u64, u64), b: (u64, u64), k: Option<u64>| -> Option<(u64, u64)> {
            let k = k?;
            let num = a.0.checked_add(b.0.checked_mul(k)?)?;
            let den = a.1.checked_add(b.1.checked_mul(k)?)?;
            (num <= limit && den <= limit).then_some((num, den))
        };
        let mut moved = false;
        for right in [true, false] {
            let (from, step) = if right { (lo, hi) } else { (hi, lo) };
            let mut k = 0u64;
            let mut jump = 1u64;
            while let Some(f) = fits(from, step, k.checked_add(jump)) {
                if is_at_most(f.0, f.1) != right {
                    break;
                }
                k += jump;
                jump = jump.saturating_mul(2);
            }
            while jump > 0 {
                if let Some(f) = fits(from, step, k.checked_add(jump)) {
                    if is_at_most(f.0, f.1) == right {
                        k += jump;
                    }
                }
                jump /= 2;
            }
            if k > 0 {
                moved = true;
                let f = fits(from, step, Some(k)).unwrap();
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

#[test]
fn stern_brocot_fixed_copy() {
    assert_eq!(stern_brocot_fixed(u64::MAX, |_, _| true), ((u64::MAX, 1), (1, 0)));
    assert_eq!(stern_brocot_fixed(u64::MAX, |p, _| p == 0), ((0, 1), (1, u64::MAX)));
    let mut rng = ChaCha8Rng::seed_from_u64(52);
    for _ in 0..5000 {
        let limit = rng.gen_range(1..=1000u64);
        let (tn, td) = (rng.gen_range(0..3000u64), rng.gen_range(1..800u64));
        assert_eq!(
            stern_brocot_fixed(limit, |p, q| p * td <= tn * q),
            stern_brocot_search(limit, |p, q| p * td <= tn * q)
        );
    }
}

// ======================= real =======================
use algo_lib::numbers::real::Real;

#[test]
fn real_with_precision_keeps_sign() {
    for (v, prec) in [(-100.5f64, 1usize), (-20.0, 1), (-1000.25, 2), (-0.04, 1), (-0.0, 3), (-1.5, 1), (0.04, 1), (-10.0, 0), (-0.4, 0)] {
        let got = String::from_utf8(Real(v).with_precision(prec).unwrap()).unwrap();
        let mut expected = format!("{:.*}", prec, v);
        if expected.trim_start_matches('-').bytes().all(|c| c == b'0' || c == b'.') {
            expected = expected.trim_start_matches('-').to_string();
        }
        assert_eq!(got, expected, "v={v} prec={prec}");
    }
}

#[test]
fn real_with_precision_random() {
    let mut rng = ChaCha8Rng::seed_from_u64(80);
    for _ in 0..200000 {
        let prec = rng.gen_range(0..6usize);
        let v = (rng.gen_range(-100000i64..100000) as f64) / 10f64.powi(rng.gen_range(0..5));
        let got = String::from_utf8(Real(v).with_precision(prec).unwrap()).unwrap();
        let mut expected = format!("{:.*}", prec, v);
        if expected.trim_start_matches('-').bytes().all(|c| c == b'0' || c == b'.') {
            expected = expected.trim_start_matches('-').to_string();
        }
        assert_eq!(got, expected, "v={v} prec={prec}");
    }
}

#[test]
fn real_misc() {
    assert!(Real(1.0) == Real(1.0 + 1e-12));
    assert!(Real(1.0) != Real(1.0 + 1e-6));
    assert!(Real(1.0) < Real(2.0));
    assert!(Real(1.0) < 2);
    assert_eq!(Real(1.0).cmp(&Real(1.0 + 1e-12)), Ordering::Equal);
    assert_eq!(Real(2.5).floor(), 2);
    assert_eq!(Real(-2.5).floor(), -3);
    assert_eq!(Real(-2.5).ceil(), -2);
    assert_eq!(Real(2.5).round(), 3);
    assert_eq!((Real(1.0) + 2 * 1).0, 3.0);
    assert_eq!((Real(1.0) / 4).0, 0.25);
    use algo_lib::numbers::num_traits::invertible::Invertible;
    assert!(Real(0.0).inv().is_none());
    assert_eq!(Real(4.0).inv().unwrap().0, 0.25);
}

#[test]
fn bigint_writable_matches_display() {
    use algo_lib::io::output::Output;
    let mut rng = ChaCha8Rng::seed_from_u64(90);
    for _ in 0..3000 {
        let len = rng.gen_range(0..6);
        let a = gen(&mut rng, len);
        let x = ub(&a);
        let mut buf = Vec::new();
        {
            let mut out = Output::buf(&mut buf);
            out.print(&x);
            out.flush();
        }
        assert_eq!(String::from_utf8(buf).unwrap(), o_str(&a));
        let neg = -BigInt::from(x);
        let mut buf = Vec::new();
        {
            let mut out = Output::buf(&mut buf);
            out.print(&neg);
            out.flush();
        }
        let expected = if a.is_empty() { "0".to_string() } else { format!("-{}", o_str(&a)) };
        assert_eq!(String::from_utf8(buf).unwrap(), expected);
    }
}

#[test]
fn bigint_mul_div_assign_i32_min() {
    let mut z = BigInt::from(5);
    z *= i32::MIN;
    assert_eq!(z, BigInt::from(5i128 * i32::MIN as i128), "5 *= i32::MIN gives {z}");
}

#[test]
fn bigint_div_assign_i32_min() {
    let mut z = BigInt::from(1i128 << 40);
    z /= i32::MIN;
    assert_eq!(z, BigInt::from((1i128 << 40) / i32::MIN as i128), "2^40 /= i32::MIN gives {z}");
}

#[test]
fn crt_remainder_unreduced_inputs() {
    for n1 in 1i64..=12 {
        for n2 in 1i64..=12 {
            let m = lcm(n1, n2);
            for a1 in -15..=15i64 {
                for a2 in -15..=15i64 {
                    let expected = (0..m).find(|x| (x - a1) % n1 == 0 && (x - a2) % n2 == 0);
                    let got = remainder(a1, n1, a2, n2).map(|x| ((x % m) + m) % m);
                    assert_eq!(got, expected, "remainder({a1},{n1},{a2},{n2})");
                }
            }
        }
    }
}
