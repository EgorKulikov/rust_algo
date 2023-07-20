pub trait IsSigned {
    const SIGNED: bool;
}

pub trait Signed: IsSigned {}

pub trait Unsigned: IsSigned {
    fn distance(self, other: Self) -> Self;
}

macro_rules! unsigned_impl {
    ($($t: ident)+) => {$(
        impl Unsigned for $t {
            fn distance(self, other: Self) -> Self {
                if self > other {
                    self - other
                } else {
                    other - self
                }
            }
        }

        impl IsSigned for $t {
            const SIGNED: bool = false;
        }
    )+};
}

unsigned_impl!(u128 u64 u32 u16 u8 usize);

macro_rules! signed_impl {
    ($($t: ident)+) => {$(
        impl Signed for $t {}

        impl IsSigned for $t {
            const SIGNED: bool = true;
        }
    )+};
}

signed_impl!(i128 i64 i32 i16 i8 isize);
