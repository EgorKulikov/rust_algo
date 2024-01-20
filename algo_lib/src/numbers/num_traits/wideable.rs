use std::convert::From;

pub trait Wideable: Sized {
    type W: From<Self>;

    fn downcast(w: Self::W) -> Self;
}

macro_rules! wideable_impl {
    ($($t: ident $w: ident),+) => {$(
        impl Wideable for $t {
            type W = $w;

            fn downcast(w: Self::W) -> Self {
                w as $t
            }
        }
    )+};
}

wideable_impl!(i64 i128, i32 i64, i16 i32, i8 i16, u64 u128, u32 u64, u16 u32, u8 u16);
