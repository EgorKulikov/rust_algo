use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::graph::Graph;
use crate::numbers::num_traits::add_sub::AddSub;
use crate::numbers::num_traits::zero_one::ZeroOne;

pub trait FlowEdgeTrait<C: AddSub + PartialOrd + Copy + ZeroOne>: EdgeTrait {
    fn capacity(&self) -> C;
    fn capacity_mut(&mut self) -> &mut C;
    fn flow(&self, graph: &Graph<Self>) -> C;
    fn push_flow(&self, flow: C) -> (usize, usize, C) {
        (self.to(), self.reverse_id(), flow)
    }
}
