pub trait AsIndex {
    fn from_index(idx: usize) -> Self;
    fn to_index(&self) -> usize;
}

macro_rules! from_index_impl {
    ($t: ident) => {
        impl AsIndex for $t {
            fn from_index(idx: usize) -> Self {
                idx as $t
            }

            fn to_index(&self) -> usize {
                *self as usize
            }
        }
    };
}

from_index_impl!(i128);
from_index_impl!(i64);
from_index_impl!(i32);
from_index_impl!(i16);
from_index_impl!(i8);
from_index_impl!(isize);
from_index_impl!(u128);
from_index_impl!(u64);
from_index_impl!(u32);
from_index_impl!(u16);
from_index_impl!(u8);
from_index_impl!(usize);
