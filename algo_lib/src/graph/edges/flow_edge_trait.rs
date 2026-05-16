use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::Graph;
use crate::numbers::num_traits::algebra::AdditionMonoidWithSub;

pub trait FlowEdgeTrait<C: AdditionMonoidWithSub + PartialOrd + Copy>: EdgeTrait {
    fn capacity(&self) -> C;
    fn capacity_mut(&mut self) -> &mut C;
    fn flow(&self, graph: &Graph<Self>) -> C;
    /// Returns `(to, reverse_id, flow)` — destination vertex of the direct
    /// edge, the cursor at which the reverse lives in the destination's
    /// adjacency (under the current Linked storage this is a global edge id),
    /// and the amount of flow to push.
    fn push_flow(&self, flow: C) -> (usize, usize, C) {
        (self.to(), self.reverse_id(), flow)
    }
}
