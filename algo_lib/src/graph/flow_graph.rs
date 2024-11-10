use crate::graph::edges::flow_edge_trait::FlowEdgeTrait;
use crate::graph::Graph;
use crate::numbers::num_traits::algebra::AdditionMonoidWithSub;

pub trait FlowGraph<C: AdditionMonoidWithSub + PartialOrd + Copy, E: FlowEdgeTrait<C>> {
    fn push_flow(&mut self, push_data: (usize, usize, C));
}

impl<C: AdditionMonoidWithSub + PartialOrd + Copy, E: FlowEdgeTrait<C>> FlowGraph<C, E>
    for Graph<E>
{
    fn push_flow(&mut self, (to, reverse_id, flow): (usize, usize, C)) {
        *self.edges[to][reverse_id].capacity_mut() += flow;
        let from = self.edges[to][reverse_id].to();
        let direct_id = self.edges[to][reverse_id].reverse_id();
        let direct = &mut self.edges[from][direct_id];
        assert!(flow >= C::zero() && flow <= direct.capacity());
        *direct.capacity_mut() -= flow;
    }
}
