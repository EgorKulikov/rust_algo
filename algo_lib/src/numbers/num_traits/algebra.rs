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

pub trait AdditionMonoid: Add<Output = Self> + AddAssign + Zero + Eq + Sized {}

impl<T: Add<Output = Self> + AddAssign + Zero + Eq> AdditionMonoid for T {}

pub trait AdditionMonoidWithSub: AdditionMonoid + Sub<Output = Self> + SubAssign {}

impl<T: AdditionMonoid + Sub<Output = Self> + SubAssign> AdditionMonoidWithSub for T {}

pub trait AdditionGroup: AdditionMonoidWithSub + Neg<Output = Self> {}

impl<T: AdditionMonoidWithSub + Neg<Output = Self>> AdditionGroup for T {}

pub trait MultiplicationMonoid: Mul<Output = Self> + MulAssign + One + Eq + Sized {}

impl<T: Mul<Output = Self> + MulAssign + One + Eq> MultiplicationMonoid for T {}

pub trait IntegerMultiplicationMonoid:
    MultiplicationMonoid + Div<Output = Self> + Rem<Output = Self> + DivAssign + RemAssign
{
}

impl<T: MultiplicationMonoid + Div<Output = Self> + Rem<Output = Self> + DivAssign + RemAssign>
    IntegerMultiplicationMonoid for T
{
}

pub trait MultiplicationGroup:
    MultiplicationMonoid + Div<Output = Self> + DivAssign + Invertible<Output = Self>
{
}

impl<T: MultiplicationMonoid + Div<Output = Self> + DivAssign + Invertible<Output = Self>>
    MultiplicationGroup for T
{
}

pub trait SemiRing: AdditionMonoid + MultiplicationMonoid {}

impl<T: AdditionMonoid + MultiplicationMonoid> SemiRing for T {}

pub trait SemiRingWithSub: AdditionMonoidWithSub + SemiRing {}

impl<T: AdditionMonoidWithSub + SemiRing> SemiRingWithSub for T {}

pub trait Ring: SemiRing + AdditionGroup {}

impl<T: SemiRing + AdditionGroup> Ring for T {}

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

pub trait IntegerSemiRingWithSub: SemiRingWithSub + IntegerSemiRing {}

impl<T: SemiRingWithSub + IntegerSemiRing> IntegerSemiRingWithSub for T {}

pub trait IntegerRing: IntegerSemiRing + Ring {}

impl<T: IntegerSemiRing + Ring> IntegerRing for T {}

pub trait Field: Ring + MultiplicationGroup {}

impl<T: Ring + MultiplicationGroup> Field for T {}

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
