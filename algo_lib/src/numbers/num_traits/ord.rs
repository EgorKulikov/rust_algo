pub trait MinMax: PartialOrd {
    fn min_val() -> Self;
    fn max_val() -> Self;
    #[must_use]
    fn minimum(self, other: Self) -> Self;
    #[must_use]
    fn maximum(self, other: Self) -> Self;
}

macro_rules! min_max_integer_impl {
    ($t: ident) => {
        impl MinMax for $t {
            fn min_val() -> Self {
                // 1.43
                std::$t::MIN
            }

            fn max_val() -> Self {
                // 1.43
                std::$t::MAX
            }

            fn minimum(self, other: Self) -> Self {
                Self::min(self, other)
            }

            fn maximum(self, other: Self) -> Self {
                Self::max(self, other)
            }
        }
    };
}

min_max_integer_impl!(i128);
min_max_integer_impl!(i64);
min_max_integer_impl!(i32);
min_max_integer_impl!(i16);
min_max_integer_impl!(i8);
min_max_integer_impl!(isize);
min_max_integer_impl!(u128);
min_max_integer_impl!(u64);
min_max_integer_impl!(u32);
min_max_integer_impl!(u16);
min_max_integer_impl!(u8);
min_max_integer_impl!(usize);

macro_rules! min_max_float_impl {
    ($t: ident) => {
        impl MinMax for $t {
            fn min_val() -> Self {
                // 1.43
                -std::$t::INFINITY
            }

            fn max_val() -> Self {
                // 1.43
                std::$t::INFINITY
            }

            fn minimum(self, other: Self) -> Self {
                $t::min(self, other)
            }

            fn maximum(self, other: Self) -> Self {
                $t::max(self, other)
            }
        }
    };
}

min_max_float_impl!(f32);
min_max_float_impl!(f64);
