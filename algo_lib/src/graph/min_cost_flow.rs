// use crate::graph::edges::flow_edge_trait::FlowEdgeTrait;
// use crate::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
// use crate::graph::graph::Graph;
// use crate::numbers::num_traits::add_sub::{AddSub, Addable};
// use crate::numbers::num_traits::bit_ops::Bits;
// use crate::numbers::num_traits::ord::MinMax;
// use crate::numbers::num_traits::zero_one::ZeroOne;
//
// pub trait MinCostFlow<C, W>
// where
//     C: AddSub + PartialOrd + Copy + ZeroOne + MinMax + Bits,
//     W: Addable + PartialOrd + Copy + ZeroOne,
// {
//     fn min_cost_flow_impl(&mut self, source: usize, sink: usize, only_negative: bool) -> (W, C);
//
//     fn min_cost_flow(&mut self, source: usize, sink: usize) -> (W, C) {
//         self.min_cost_flow_impl(source, sink, false)
//     }
// }
//
// impl<C, W, E> MinCostFlow<C, W> for Graph<E>
// where
//     C: AddSub + PartialOrd + Copy + ZeroOne + MinMax + Bits,
//     W: Addable + PartialOrd + Copy + ZeroOne,
//     E: WeightedEdgeTrait<W> + FlowEdgeTrait<C>,
// {
//     fn min_cost_flow_impl(&mut self, source: usize, sink: usize, only_negative: bool) -> (W, C) {}
// }
