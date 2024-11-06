use crate::numbers::num_traits::algebra::Ring;

pub trait Base: Copy + Ring + 'static {}

impl<T: Copy + Ring + 'static> Base for T {}
