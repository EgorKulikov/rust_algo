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
    use crate::numbers::unsigned_big_int::UBigInt;

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
