pub trait Primitive<T>: Copy {
    fn to(self) -> T;
}

macro_rules! primitive_one {
    ($t: ident, $($u: ident)+) => {$(
        impl Primitive<$u> for $t {
            fn to(self) -> $u {
                self as $u
            }
        }
    )+};
}

macro_rules! primitive {
    ($($t: ident)+) => {$(
        primitive_one!($t, u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
    )+}
}

primitive!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
