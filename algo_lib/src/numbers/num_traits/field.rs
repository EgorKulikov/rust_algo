use crate::numbers::num_traits::mul_div_rem::MulDiv;
use crate::numbers::num_traits::ring::Ring;

pub trait Field: Ring + MulDiv {}

impl<T: Ring + MulDiv> Field for T {}
