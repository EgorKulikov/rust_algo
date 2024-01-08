use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::graph::Graph;
use crate::numbers::num_traits::algebra::AdditionMonoidWithSub;

pub trait FlowEdgeTrait<C: AdditionMonoidWithSub + PartialOrd + Copy>: EdgeTrait {
    fn capacity(&self) -> C;
    fn capacity_mut(&mut self) -> &mut C;
    fn flow(&self, graph: &Graph<Self>) -> C;
    fn push_flow(&self, flow: C) -> (usize, usize, C) {
        (self.to(), self.reverse_id(), flow)
    }
}
