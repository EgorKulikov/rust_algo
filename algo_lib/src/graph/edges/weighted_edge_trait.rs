use crate::graph::edges::edge_trait::EdgeTrait;

pub trait WeightedEdgeTrait<W: Copy>: EdgeTrait {
    fn weight(&self) -> W;
    fn weight_mut(&mut self) -> &mut W;
}
