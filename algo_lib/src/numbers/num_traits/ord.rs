pub trait MinMax: PartialOrd {
    fn min_val() -> Self;
    fn max_val() -> Self;
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
}

macro_rules! min_max_integer_impl {
    ($t: ident) => {
        impl MinMax for $t {
            fn min_val() -> Self {
                $t::MIN
            }

            fn max_val() -> Self {
                $t::MAX
            }

            fn min(self, other: Self) -> Self {
                <Self as Ord>::min(self, other)
            }

            fn max(self, other: Self) -> Self {
                <Self as Ord>::max(self, other)
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
                -$t::INFINITY
            }

            fn max_val() -> Self {
                $t::INFINITY
            }

            fn min(self, other: Self) -> Self {
                if self < other {
                    self
                } else {
                    other
                }
            }

            fn max(self, other: Self) -> Self {
                if self > other {
                    self
                } else {
                    other
                }
            }
        }
    };
}

min_max_float_impl!(f32);
min_max_float_impl!(f64);
