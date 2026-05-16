use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::Graph;
use crate::numbers::num_traits::algebra::AdditionMonoidWithSub;

pub trait FlowEdgeTrait<C: AdditionMonoidWithSub + PartialOrd + Copy>: EdgeTrait {
    fn capacity(&self) -> C;
    fn capacity_mut(&mut self) -> &mut C;
    fn flow(&self, graph: &Graph<Self>) -> C;
    /// Returns `(rev_edge_id, flow)` — the global edge id of this edge's reverse,
    /// plus the amount of flow to push.
    fn push_flow(&self, flow: C) -> (usize, C) {
        (self.reverse_id(), flow)
    }
}
