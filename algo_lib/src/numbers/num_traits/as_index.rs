pub trait AsIndex {
    fn from_index(idx: usize) -> Self;
    fn to_index(self) -> usize;
}

macro_rules! from_index_impl {
    ($($t: ident)+) => {$(
        impl AsIndex for $t {
            fn from_index(idx: usize) -> Self {
                idx as $t
            }

            fn to_index(self) -> usize {
                self as usize
            }
        }
    )+};
}

from_index_impl!(i128 i64 i32 i16 i8 isize u128 u64 u32 u16 u8 usize);
