use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign,
};

pub trait BitOps:
    Copy
    + BitAnd<Output = Self>
    + BitAndAssign
    + BitOr<Output = Self>
    + BitOrAssign
    + BitXor<Output = Self>
    + BitXorAssign
    + Not<Output = Self>
    + Shl<usize, Output = Self>
    + ShlAssign<usize>
    + Shr<usize, Output = Self>
    + ShrAssign<usize>
{
}

impl<
        T: Copy
            + BitAnd<Output = Self>
            + BitAndAssign
            + BitOr<Output = Self>
            + BitOrAssign
            + BitXor<Output = Self>
            + BitXorAssign
            + Not<Output = Self>
            + Shl<usize, Output = Self>
            + ShlAssign<usize>
            + Shr<usize, Output = Self>
            + ShrAssign<usize>,
    > BitOps for T
{
}
