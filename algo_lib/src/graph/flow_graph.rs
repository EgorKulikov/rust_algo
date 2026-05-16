use crate::graph::edges::flow_edge_trait::FlowEdgeTrait;
use crate::graph::Graph;
use crate::numbers::num_traits::algebra::AdditionMonoidWithSub;

pub trait FlowGraph<C: AdditionMonoidWithSub + PartialOrd + Copy, E: FlowEdgeTrait<C>> {
    /// `(rev_edge_id, flow)`: increase capacity of `edges[rev_edge_id]` by `flow`,
    /// decrease capacity of its reverse (the original direct edge) by `flow`.
    fn push_flow(&mut self, push_data: (usize, C));
}

impl<C: AdditionMonoidWithSub + PartialOrd + Copy, E: FlowEdgeTrait<C>> FlowGraph<C, E>
    for Graph<E>
{
    fn push_flow(&mut self, (rev_edge_id, flow): (usize, C)) {
        *self.edge_mut(rev_edge_id).capacity_mut() += flow;
        let direct_id = self.edge(rev_edge_id).reverse_id();
        let direct = self.edge_mut(direct_id);
        assert!(flow >= C::zero() && flow <= direct.capacity());
        *direct.capacity_mut() -= flow;
    }
}
