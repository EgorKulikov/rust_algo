pub trait MinMax: PartialOrd {
    fn min_val() -> Self;
    fn max_val() -> Self;
}

macro_rules! min_max_integer_impl {
    ($($t: ident)+) => {$(
        impl MinMax for $t {
            fn min_val() -> Self {
                // 1.43
                std::$t::MIN
            }

            fn max_val() -> Self {
                // 1.43
                std::$t::MAX
            }
        }
    )+};
}

min_max_integer_impl!(i128 i64 i32 i16 i8 isize u128 u64 u32 u16 u8 usize);
