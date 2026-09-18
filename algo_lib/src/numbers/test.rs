use crate::numbers::primes::factorize::Factorize;

mod mod_int {
    use crate::numbers::mod_int::ModInt7;
    use crate::numbers::num_traits::algebra::{One, Zero};

    type Mod = ModInt7;

    #[test]
    fn add() {
        let x = Mod::new(1);
        let y = Mod::new(2);
        assert_eq!(format!("{}", x + y), "3");
    }

    #[test]
    fn sub() {
        let x = Mod::new(1);
        let y = Mod::new(2);
        assert_eq!(format!("{}", x - y), "1000000006");
        assert_eq!(format!("{:?}", x - y), "-1");
    }

    #[test]
    fn mul() {
        let x = Mod::new(3);
        let y = Mod::new(5);
        assert_eq!(format!("{}", x * y), "15");
    }

    #[test]
    fn div() {
        let x = Mod::new(3);
        let y = Mod::new(5);
        assert_eq!(format!("{}", x / y), "200000002");
        assert_eq!(format!("{:?}", x / y), "3/5");
    }

    #[test]
    fn div_assign() {
        let mut x = Mod::new(3);
        let y = Mod::new(5);
        x /= y;
        assert_eq!(format!("{}", x), "200000002");
        assert_eq!(format!("{:?}", x), "3/5");
    }

    #[test]
    fn dbg_format() {
        let x = Mod::new(1) / Mod::new(2);
        let y = Mod::new(1) / Mod::new(3);
        assert_eq!(format!("{}", x + y), "833333340");
        assert_eq!(format!("{:?}", x + y), "5/6");
    }

    #[test]
    fn dbg_format_big() {
        let x = Mod::new(123) / Mod::new(457);
        assert_eq!(format!("{:?}", x), "(?? 262582059 ??)");
    }

    #[test]
    fn dbg_format_more() {
        assert_eq!(format!("{:?}", Mod::new(1)), "1");
        assert_eq!(format!("{:?}", Mod::new(3)), "3");
        assert_eq!(format!("{:?}", Mod::new_signed(-5)), "-5");
    }

    #[test]
    fn consts() {
        let one = Mod::one() - Mod::zero();
        assert_eq!(format!("{:?}", one), "1");
    }
}

#[test]
fn test_divisors() {
    for i in 1..100000 {
        i.prime_divisors();
    }
}

mod big_int {
    use crate::io::output::{Output, Writable};
    use crate::misc::random::{Random, RandomTrait};
    use crate::numbers::num_traits::algebra::{IntegerRing, IntegerSemiRing, One, Zero};
    use crate::numbers::signed_big_int::BigInt;
    use crate::numbers::unsigned_big_int::UBigInt;

    fn ub(s: &str) -> UBigInt {
        UBigInt::from(s.as_bytes())
    }

    fn sb(s: &str) -> BigInt {
        BigInt::from(s.as_bytes())
    }

    #[test]
    fn test_mul() {
        let a = UBigInt::from(b"18347" as &[u8]);
        let res = a.clone() * a.clone();
        let res2 = res.clone() * res.clone();
        assert_eq!(res2.to_string(), "113307913892783281");
        let b = UBigInt::from(b"10365" as &[u8]);
        let res = b.clone() * b.clone() * b.clone() * b.clone() * b.clone();
        assert_eq!(res.to_string(), "119631771048379978125");
        let res = res + res2;
        assert_eq!(res.to_string(), "119745078962272761406");
    }

    #[test]
    fn zero_has_one_representation() {
        assert_eq!(ub("0"), UBigInt::zero());
        assert_eq!(ub("000000000000"), UBigInt::zero());
        assert_eq!(ub("0000000000007"), UBigInt::from(7));
        assert_eq!(sb("00"), BigInt::zero());
        assert_eq!(sb("-0"), BigInt::zero());
        assert_eq!(sb("-000000000012"), BigInt::from(-12));
        let d = BigInt::from(5) - BigInt::from(5);
        assert_eq!(d, BigInt::zero());
        assert_eq!(d.cmp(&BigInt::zero()), std::cmp::Ordering::Equal);
        assert_eq!(d + BigInt::from(3), BigInt::from(3));
        let d = sb("-123456789012345678901234567890") + sb("123456789012345678901234567890");
        assert_eq!(d, BigInt::zero());
        assert_eq!(-BigInt::zero(), BigInt::zero());
    }

    #[test]
    fn division_matches_i128() {
        let mut rng = Random::new_with_seed(7);
        for _ in 0..3000 {
            let bits = rng.gen_range(0..127usize);
            let a = (rng.gen_u128() >> (127 - bits)) as i128 * if rng.gen_bool() { -1 } else { 1 };
            let bits = rng.gen_range(0..127usize);
            let b = (rng.gen_u128() >> (127 - bits)) as i128 * if rng.gen_bool() { -1 } else { 1 };
            if b == 0 {
                continue;
            }
            let (fa, fb) = (BigInt::from(a), BigInt::from(b));
            assert_eq!((&fa / &fb).to_string(), (a / b).to_string(), "{a} / {b}");
            assert_eq!((&fa % &fb).to_string(), (a % b).to_string(), "{a} % {b}");
            assert_eq!((fa.clone() / fb.clone()).to_string(), (a / b).to_string());
            assert_eq!((fa.clone() % fb.clone()).to_string(), (a % b).to_string());
            let (mut q, mut r) = (fa.clone(), fa.clone());
            q /= &fb;
            r %= fb.clone();
            assert_eq!(q.to_string(), (a / b).to_string());
            assert_eq!(r.to_string(), (a % b).to_string());
            let (ua, ub) = (
                UBigInt::from(a.unsigned_abs()),
                UBigInt::from(b.unsigned_abs()),
            );
            assert_eq!(
                (&ua / &ub).to_string(),
                (a.unsigned_abs() / b.unsigned_abs()).to_string()
            );
            assert_eq!(
                (ua.clone() % ub.clone()).to_string(),
                (a.unsigned_abs() % b.unsigned_abs()).to_string()
            );
        }
    }

    fn rand_ubig(rng: &mut Random, limbs: usize) -> UBigInt {
        let mut res = UBigInt::zero();
        for _ in 0..limbs {
            res *= 1_000_000_000;
            res += UBigInt::from(rng.gen_range(0..1_000_000_000u32));
        }
        res
    }

    #[test]
    fn division_identities() {
        let mut rng = Random::new_with_seed(8);
        for _ in 0..300 {
            let al = rng.gen_range(0..40usize);
            let a = rand_ubig(&mut rng, al);
            let bl = rng.gen_range(1..25usize);
            let b = rand_ubig(&mut rng, bl);
            if b == UBigInt::zero() {
                continue;
            }
            let (q, r) = a.div_rem(&b);
            assert_eq!(&q * &b + &r, a);
            assert!(r < b);
            let sa = if rng.gen_bool() {
                -BigInt::from(a.clone())
            } else {
                BigInt::from(a.clone())
            };
            let sbb = if rng.gen_bool() {
                -BigInt::from(b.clone())
            } else {
                BigInt::from(b.clone())
            };
            let (q, r) = sa.div_rem(&sbb);
            assert_eq!(&q * &sbb + &r, sa);
            assert!(r.clone().abs() < sbb.clone().abs());
            assert!(r == BigInt::zero() || (r < BigInt::zero()) == (sa < BigInt::zero()));
        }
        // divisor with top limb forcing quotient-digit corrections
        let a = ub("999999999999999999999999999999999999999999999999999999");
        let b = ub("1000000000000000000000000000000000000000000000000000001");
        assert_eq!(a.div_rem(&b), (UBigInt::zero(), a.clone()));
        let b = ub("500000000499999999");
        let (q, r) = a.div_rem(&b);
        assert_eq!(&q * &b + &r, a);
        assert!(r < b);
    }

    #[test]
    fn newton_division() {
        let mut rng = Random::new_with_seed(9);
        let base_pow = |limbs: usize| {
            let mut x = UBigInt::one();
            for _ in 0..limbs {
                x *= 1_000_000_000;
            }
            x
        };
        let mut cases: Vec<(UBigInt, UBigInt)> = Vec::new();
        for &(n, m) in &[
            (300usize, 100usize),
            (200, 100),
            (196, 100),
            (1000, 97),
            (500, 250),
            (2000, 130),
            (400, 399),
        ] {
            cases.push((rand_ubig(&mut rng, n), rand_ubig(&mut rng, m)));
        }
        // divisors with extreme top limbs, exact multiples, remainder b - 1
        let b_max = base_pow(120) - &UBigInt::one();
        let b_min = base_pow(119);
        let a = rand_ubig(&mut rng, 400);
        cases.push((a.clone(), b_max.clone()));
        cases.push((a.clone(), b_min.clone()));
        let b = rand_ubig(&mut rng, 150);
        let qq = rand_ubig(&mut rng, 300);
        cases.push((&qq * &b, b.clone()));
        cases.push((&qq * &b + &(b.clone() - &UBigInt::one()), b.clone()));
        cases.push((&b_max * &b_max, b_max.clone()));
        cases.push((base_pow(500), b_min));
        for (a, b) in cases {
            if b == UBigInt::zero() {
                continue;
            }
            let (q, r) = a.div_rem(&b);
            assert!(r < b);
            assert_eq!(&q * &b + &r, a);
        }
    }

    #[test]
    #[should_panic]
    fn division_by_zero() {
        let _ = ub("5") / UBigInt::zero();
    }

    #[test]
    fn conversions() {
        assert_eq!(UBigInt::from(u64::MAX).to_string(), u64::MAX.to_string());
        assert_eq!(UBigInt::from(u128::MAX).to_string(), u128::MAX.to_string());
        assert_eq!(UBigInt::from(12usize).to_string(), "12");
        assert_eq!(UBigInt::from(0u8), UBigInt::zero());
        assert_eq!(BigInt::from(i64::MIN).to_string(), i64::MIN.to_string());
        assert_eq!(BigInt::from(i128::MIN).to_string(), i128::MIN.to_string());
        assert_eq!(BigInt::from(u128::MAX).to_string(), u128::MAX.to_string());
        assert_eq!(BigInt::from(-1i8).to_string(), "-1");
        assert_eq!(BigInt::from(0i64), BigInt::zero());
        assert_eq!(BigInt::from(ub("123")), sb("123"));
        assert_eq!(sb("-123").abs(), ub("123"));
        assert_eq!(sb("123").abs(), ub("123"));
        assert_eq!(u64::try_from(&ub("18446744073709551615")), Ok(u64::MAX));
        assert_eq!(u64::try_from(&ub("18446744073709551616")), Err(()));
        assert_eq!(u128::try_from(&UBigInt::from(u128::MAX)), Ok(u128::MAX));
        assert_eq!(
            u128::try_from(&(UBigInt::from(u128::MAX) + UBigInt::one())),
            Err(())
        );
        assert_eq!(usize::try_from(&UBigInt::zero()), Ok(0));
        assert_eq!(i32::try_from(&sb("-2147483648")), Ok(i32::MIN));
        assert_eq!(i32::try_from(&sb("-2147483649")), Err(()));
        assert_eq!(i128::try_from(&BigInt::from(i128::MIN)), Ok(i128::MIN));
        assert_eq!(
            i128::try_from(&(BigInt::from(i128::MIN) - BigInt::one())),
            Err(())
        );
        assert_eq!(u8::try_from(&sb("-1")), Err(()));
        assert_eq!(u8::try_from(&sb("255")), Ok(255));
    }

    #[test]
    #[should_panic]
    fn negative_into_unsigned() {
        let _ = UBigInt::from(-1);
    }

    #[test]
    fn writable_matches_display() {
        for s in [
            "0",
            "7",
            "1000000000",
            "123456789012345678901234567890",
            "-5",
            "-1000000000000000000",
        ] {
            let mut buf = Vec::new();
            {
                let mut out = Output::buf(&mut buf);
                sb(s).write(&mut out);
                out.flush();
            }
            assert_eq!(String::from_utf8(buf).unwrap(), s);
            assert_eq!(sb(s).to_string(), s);
            if !s.starts_with('-') {
                let mut buf = Vec::new();
                {
                    let mut out = Output::buf(&mut buf);
                    ub(s).write(&mut out);
                    out.flush();
                }
                assert_eq!(String::from_utf8(buf).unwrap(), s);
            }
        }
    }

    #[test]
    fn traits() {
        fn semi<T: IntegerSemiRing>() -> T {
            T::ten()
        }
        fn ring<T: IntegerRing>() -> T {
            -T::ten()
        }
        assert_eq!(semi::<UBigInt>(), UBigInt::from(10));
        assert_eq!(semi::<BigInt>(), BigInt::from(10));
        assert_eq!(ring::<BigInt>(), BigInt::from(-10));
    }
}

mod fixed_int {
    use crate::misc::random::{Random, RandomTrait};
    use crate::numbers::fixed_int::{i1024, i256, i512, FixedInt};
    use crate::numbers::num_traits::algebra::{IntegerRing, IntegerSemiRing, One, Zero};

    fn rand_i128(rng: &mut Random) -> i128 {
        let bits = rng.gen_range(0..127usize);
        let v = (rng.gen_u128() >> (127 - bits)) as i128;
        if rng.gen_bool() {
            -v
        } else {
            v
        }
    }

    fn check<const N: usize>(seed: u64) {
        let mut rng = Random::new_with_seed(seed);
        for _ in 0..2000 {
            let a = rand_i128(&mut rng);
            let b = rand_i128(&mut rng);
            let fa = FixedInt::<N>::from(a);
            let fb = FixedInt::<N>::from(b);
            assert_eq!(fa + fb, FixedInt::from(a.wrapping_add(b)), "{a} + {b}");
            assert_eq!(fa - fb, FixedInt::from(a.wrapping_sub(b)), "{a} - {b}");
            assert_eq!(-fa, FixedInt::from(a.wrapping_neg()), "-{a}");
            if let Some(p) = a.checked_mul(b) {
                assert_eq!(fa * fb, FixedInt::from(p), "{a} * {b}");
            }
            assert_eq!(fa.cmp(&fb), a.cmp(&b), "{a} cmp {b}");
            if b != 0 {
                assert_eq!(fa / fb, FixedInt::from(a / b), "{a} / {b}");
                assert_eq!(fa % fb, FixedInt::from(a % b), "{a} % {b}");
            }
            let (mut x, mut y, mut z, mut w) = (fa, fa, fa, fa);
            x += fb;
            y -= fb;
            z *= fb;
            assert_eq!(x, fa + fb);
            assert_eq!(y, fa - fb);
            assert_eq!(z, fa * fb);
            if b != 0 {
                let mut r = fa;
                w /= fb;
                r %= fb;
                assert_eq!(w, fa / fb);
                assert_eq!(r, fa % fb);
            }
        }
    }

    #[test]
    fn matches_i128() {
        check::<8>(1);
        check::<16>(2);
        check::<32>(3);
    }

    fn rand_big<const N: usize>(rng: &mut Random, limbs: usize) -> FixedInt<N> {
        let mut res = FixedInt::<N>::zero();
        for _ in 0..limbs {
            res = res * FixedInt::from(1u128 << 32) + FixedInt::from(rng.gen_u128() as u32);
        }
        if rng.gen_bool() {
            -res
        } else {
            res
        }
    }

    fn check_big<const N: usize>(seed: u64) {
        let mut rng = Random::new_with_seed(seed);
        let zero = FixedInt::<N>::zero();
        for _ in 0..500 {
            let al = rng.gen_range(1..=N);
            let a: FixedInt<N> = rand_big(&mut rng, al);
            let bl = rng.gen_range(1..=N);
            let b: FixedInt<N> = rand_big(&mut rng, bl);
            if b == zero || -b == b {
                continue;
            }
            let (q, r) = (a / b, a % b);
            assert_eq!(q * b + r, a);
            // remainder has the sign of the dividend (or is zero) and |r| < |b|
            let (abs_r, abs_b) = (if r < zero { -r } else { r }, if b < zero { -b } else { b });
            assert!(abs_r < abs_b);
            assert!(r == zero || (r < zero) == (a < zero));
            // multiplication without overflow round-trips through division
            let cl = rng.gen_range(1..N - 1);
            let c: FixedInt<N> = rand_big(&mut rng, cl);
            let sbl = rng.gen_range(1..=N - 1 - cl);
            let small_b: FixedInt<N> = rand_big(&mut rng, sbl);
            if small_b != zero {
                let cb = c * small_b;
                assert_eq!(cb / small_b, c);
                assert_eq!(cb % small_b, zero);
            }
        }
    }

    #[test]
    fn big_identities() {
        check_big::<8>(4);
        check_big::<16>(5);
        check_big::<32>(6);
    }

    #[test]
    fn wrapping_and_constants() {
        let one = i256::one();
        let two = one + one;
        let mut max = one;
        for _ in 0..254 {
            max *= two;
        }
        max = max - one + max; // 2^255 - 1
        assert!(max > i256::zero());
        assert!(max + one < i256::zero()); // wraps to -2^255
        assert_eq!(max + one, -(max + one));
        assert_eq!(i256::ten(), i256::from(10));
        assert_eq!(i512::ten() * i512::ten(), i512::from(100u8));
        assert_eq!(i1024::from(-7i64) / i1024::from(2usize), i1024::from(-3));
        assert_eq!(i1024::from(-7i64) % i1024::from(2usize), i1024::from(-1));
        assert!(i256::from(i128::MIN) - one < i256::from(i128::MIN));
        assert!(i256::from(i128::MAX) + one > i256::from(i128::MAX));
        assert_eq!(
            i256::from(-1) * i256::from(i128::MIN),
            -i256::from(i128::MIN)
        );
        fn ring<T: IntegerRing + Ord>(_: T) {}
        ring(i256::zero());
        ring(i512::zero());
        ring(i1024::zero());
    }
}
