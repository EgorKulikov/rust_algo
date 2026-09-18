use crate::numbers::num_traits::algebra::Ring;

pub mod angle;
pub mod arg_sort;
pub mod circle;
pub mod geometry_utils;
pub mod line;
pub mod min_enclosing_circle;
pub mod point;
pub mod point_pairs;
pub mod polygon;
pub mod ray;
pub mod segment;

pub trait Base: Copy + Ring {}

impl<T: Copy + Ring> Base for T {}
