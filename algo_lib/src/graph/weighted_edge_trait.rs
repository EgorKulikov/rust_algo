use crate::graph::edge_trait::EdgeTrait;
use crate::numbers::integer::Integer;

pub trait WeightedEdgeTrait<W: Integer>: EdgeTrait {
    fn weight(&self) -> W;
    fn weight_mut(&mut self) -> &mut W;
}
