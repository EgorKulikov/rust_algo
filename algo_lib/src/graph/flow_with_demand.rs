use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::edges::flow_edge::FlowEdge;
use crate::graph::edges::flow_edge_trait::FlowEdgeTrait;
use crate::graph::flow_graph::FlowGraph;
use crate::graph::graph::Graph;
use crate::graph::max_flow::MaxFlow;
use crate::numbers::num_traits::add_sub::AddSub;
use crate::numbers::num_traits::ord::MinMax;
use crate::numbers::num_traits::zero_one::ZeroOne;

pub trait FlowWithDemand<C: AddSub + PartialOrd + Copy + ZeroOne + MinMax> {
    fn flow_with_demand(&mut self, source: usize, destination: usize) -> bool;
}

impl<C: AddSub + PartialOrd + Copy + ZeroOne + MinMax, E: FlowEdgeTrait<C, Payload = C>>
    FlowWithDemand<C> for Graph<E>
{
    fn flow_with_demand(&mut self, source: usize, destination: usize) -> bool {
        let mut flow_graph = Graph::new(self.vertex_count() + 2);
        let mut demand = vec![C::zero(); self.vertex_count()];
        for i in 0..self.vertex_count() {
            for (j, e) in self[i].iter().enumerate() {
                if e.capacity() != C::zero() {
                    flow_graph.add_edge(
                        i,
                        FlowEdge::with_payload(e.to(), e.capacity() - *e.payload(), (i, j)),
                    );
                    demand[e.to()] += *e.payload();
                    demand[i] -= *e.payload();
                }
            }
        }
        let mut total_demand = C::zero();
        for (i, d) in demand.into_iter().enumerate() {
            if d > C::zero() {
                flow_graph.add_edge(
                    self.vertex_count(),
                    FlowEdge::with_payload(i, d, (self.vertex_count(), self.vertex_count())),
                );
                total_demand += d;
            } else if d < C::zero() {
                flow_graph.add_edge(
                    i,
                    FlowEdge::with_payload(
                        self.vertex_count() + 1,
                        C::zero() - d,
                        (self.vertex_count(), self.vertex_count()),
                    ),
                );
            }
        }
        flow_graph.add_edge(
            destination,
            FlowEdge::with_payload(
                source,
                C::max_val(),
                (self.vertex_count(), self.vertex_count()),
            ),
        );
        let res = flow_graph.max_flow(self.vertex_count(), self.vertex_count() + 1) == total_demand;
        if !res {
            return false;
        }
        for i in 0..self.vertex_count() {
            for e in &flow_graph[i] {
                let (from, id) = *e.payload();
                if from == i {
                    self.push_flow(
                        self[from][id].push_flow(*self[from][id].payload() + e.flow(&flow_graph)),
                    );
                }
            }
        }
        true
    }
}
