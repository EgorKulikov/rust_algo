pub trait Wideable: Sized {
    type W: From<Self>;

    fn downcast(w: Self::W) -> Self;
}

macro_rules! wideable_impl {
    ($t: ident, $w: ident) => {
        impl Wideable for $t {
            type W = $w;

            fn downcast(w: Self::W) -> Self {
                w as $t
            }
        }
    };
}

wideable_impl!(i128, i128);
wideable_impl!(i64, i128);
wideable_impl!(i32, i64);
wideable_impl!(i16, i32);
wideable_impl!(i8, i16);
wideable_impl!(isize, isize);
wideable_impl!(u128, u128);
wideable_impl!(u64, u128);
wideable_impl!(u32, u64);
wideable_impl!(u16, u32);
wideable_impl!(u8, u16);
wideable_impl!(usize, usize);
wideable_impl!(f64, f64);
wideable_impl!(f32, f64);
