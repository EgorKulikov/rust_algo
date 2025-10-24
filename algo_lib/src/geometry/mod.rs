use crate::numbers::num_traits::algebra::Ring;

pub mod angle;
pub mod circle;
pub mod geometry_utils;
pub mod line;
pub mod point;
pub mod polygon;
pub mod ray;
pub mod segment;

pub trait Base: Copy + Ring {}

impl<T: Copy + Ring> Base for T {}
