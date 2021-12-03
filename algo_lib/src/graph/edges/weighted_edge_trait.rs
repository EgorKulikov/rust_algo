use crate::graph::edges::edge_trait::EdgeTrait;
use crate::numbers::num_traits::add_sub::Addable;
use crate::numbers::num_traits::zero_one::ZeroOne;

pub trait WeightedEdgeTrait<W: Addable + PartialOrd + Copy + ZeroOne>: EdgeTrait {
    fn weight(&self) -> W;
    fn weight_mut(&mut self) -> &mut W;
}
