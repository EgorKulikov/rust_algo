mod memoization_test {
    use crate::misc::memo::memoization::Memoization2;
    use crate::misc::memo::memoization_2d::Memoization2d;
    use crate::misc::recursive_function::Callable2;
    use crate::numbers::mod_int::ModIntF;
    use crate::numbers::num_traits::zero_one::ZeroOne;

    #[test]
    fn test() {
        type Mod = ModIntF;
        let mut mem2d = Memoization2d::new(1001, 1001, |f, n, m| {
            if n == 0 || m == 0 {
                Mod::one()
            } else {
                f.call(n - 1, m) + f.call(n, m - 1)
            }
        });
        let mut mem = Memoization2::new(|f, n, m| {
            if n == 0 || m == 0 {
                Mod::one()
            } else {
                f.call(n - 1, m) + f.call(n, m - 1)
            }
        });
        assert_eq!(mem.call(1000, 1000), mem2d.call(1000, 1000));
    }
}
