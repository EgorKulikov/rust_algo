use crate::numbers::num_traits::algebra::Ring;

pub mod angle;
pub mod circle;
pub mod geometry_utils;
pub mod line;
pub mod point;
pub mod polygon;
pub mod ray;
pub mod segment;

pub trait Base: Copy + Ring + 'static {}

impl<T: Copy + Ring + 'static> Base for T {}
