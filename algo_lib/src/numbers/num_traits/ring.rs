use crate::numbers::num_traits::add_sub::AddSub;
use crate::numbers::num_traits::mul_div_rem::Multable;
use crate::numbers::num_traits::zero_one::ZeroOne;

pub trait Ring: AddSub + Multable + ZeroOne {}

impl<T: AddSub + Multable + ZeroOne> Ring for T {}
