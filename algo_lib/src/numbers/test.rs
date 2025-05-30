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
