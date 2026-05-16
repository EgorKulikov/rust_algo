use crate::graph::edges::flow_edge_trait::FlowEdgeTrait;
use crate::graph::Graph;
use crate::numbers::num_traits::algebra::AdditionMonoidWithSub;

pub trait FlowGraph<C: AdditionMonoidWithSub + PartialOrd + Copy, E: FlowEdgeTrait<C>> {
    /// `(to, reverse_id, flow)`: increase capacity of the reverse edge (which
    /// lives at `(to, reverse_id)`) by `flow`, decrease capacity of the
    /// direct edge (the reverse-of-the-reverse) by `flow`.
    fn push_flow(&mut self, push_data: (usize, usize, C));
}

impl<C: AdditionMonoidWithSub + PartialOrd + Copy, E: FlowEdgeTrait<C>> FlowGraph<C, E>
    for Graph<E>
{
    fn push_flow(&mut self, (to, reverse_id, flow): (usize, usize, C)) {
        let rev_cursor = reverse_id as u32;
        let direct_to;
        let direct_cursor;
        {
            let rev = self.edge_at_mut(to, rev_cursor);
            *rev.capacity_mut() += flow;
            direct_to = rev.to();
            direct_cursor = rev.reverse_id() as u32;
        }
        let direct = self.edge_at_mut(direct_to, direct_cursor);
        assert!(flow >= C::zero() && flow <= direct.capacity());
        *direct.capacity_mut() -= flow;
    }
}
