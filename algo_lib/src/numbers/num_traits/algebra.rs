use crate::numbers::num_traits::invertible::Invertible;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

pub trait Zero {
    fn zero() -> Self;
}

pub trait One {
    fn one() -> Self;
}

// Defines a marker trait as an alias for its bounds, with a blanket impl.
macro_rules! alias {
    ($name: ident: $($bound: tt)+) => {
        pub trait $name: $($bound)+ {}
        impl<T: $($bound)+> $name for T {}
    };
}

alias!(AdditionMonoid: Add<Output = Self> + AddAssign + Zero + Eq + Sized);
alias!(AdditionMonoidWithSub: AdditionMonoid + Sub<Output = Self> + SubAssign);
alias!(AdditionGroup: AdditionMonoidWithSub + Neg<Output = Self>);
alias!(MultiplicationMonoid: Mul<Output = Self> + MulAssign + One + Eq + Sized);
alias!(IntegerMultiplicationMonoid: MultiplicationMonoid + Div<Output = Self> + Rem<Output = Self> + DivAssign + RemAssign);
alias!(MultiplicationGroup: MultiplicationMonoid + Div<Output = Self> + DivAssign + Invertible<Output = Self>);
alias!(SemiRing: AdditionMonoid + MultiplicationMonoid);
alias!(SemiRingWithSub: AdditionMonoidWithSub + SemiRing);
alias!(Ring: SemiRing + AdditionGroup);

pub trait IntegerSemiRing: SemiRing + IntegerMultiplicationMonoid {
    fn ten() -> Self {
        Self::one()
            + Self::one()
            + Self::one()
            + Self::one()
            + Self::one()
            + Self::one()
            + Self::one()
            + Self::one()
            + Self::one()
            + Self::one()
    }
}

impl<T: SemiRing + IntegerMultiplicationMonoid> IntegerSemiRing for T {}

alias!(IntegerSemiRingWithSub: SemiRingWithSub + IntegerSemiRing);
alias!(IntegerRing: IntegerSemiRing + Ring);
alias!(Field: Ring + MultiplicationGroup);

macro_rules! zero_one_integer_impl {
    ($($t: ident)+) => {$(
        impl Zero for $t {
            fn zero() -> Self {
                0
            }
        }

        impl One for $t {
            fn one() -> Self {
                1
            }
        }
    )+};
}

zero_one_integer_impl!(i128 i64 i32 i16 i8 isize u128 u64 u32 u16 u8 usize);
