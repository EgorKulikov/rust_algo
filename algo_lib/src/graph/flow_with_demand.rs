use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::edges::flow_edge::FlowEdge;
use crate::graph::edges::flow_edge_trait::FlowEdgeTrait;
use crate::graph::flow_graph::FlowGraph;
use crate::graph::max_flow::MaxFlow;
use crate::graph::Graph;
use crate::numbers::num_traits::algebra::AdditionMonoidWithSub;
use crate::numbers::num_traits::ord::MinMax;
use std::cmp::Ordering;

pub trait FlowWithDemand<C: AdditionMonoidWithSub + Ord + Copy + MinMax> {
    fn flow_with_demand(&mut self, source: usize, destination: usize) -> bool;
}

impl<C: AdditionMonoidWithSub + Ord + Copy + MinMax, E: FlowEdgeTrait<C, Payload = C>>
    FlowWithDemand<C> for Graph<E>
{
    #[allow(clippy::needless_range_loop)]
    fn flow_with_demand(&mut self, source: usize, destination: usize) -> bool {
        let mut flow_graph = Graph::new_linked(self.vertex_count() + 2);
        // Lower bounds entering and leaving each vertex, kept separately so
        // unsigned capacity types never go below zero.
        let mut incoming = vec![C::zero(); self.vertex_count()];
        let mut outgoing = vec![C::zero(); self.vertex_count()];
        for i in 0..self.vertex_count() {
            for (j, e) in self.adj(i).iter_with_id() {
                // Checked for zero-capacity edges as well: a demand on one of
                // them cannot be met (reverse edges carry a zero payload).
                if *e.payload() > e.capacity() {
                    return false;
                }
                if e.capacity() != C::zero() {
                    flow_graph.add_edge(FlowEdge::with_payload(
                        i,
                        e.to(),
                        e.capacity() - *e.payload(),
                        Some((i, j)),
                    ));
                    incoming[e.to()] += *e.payload();
                    outgoing[i] += *e.payload();
                }
            }
        }
        let mut total_demand = C::zero();
        // Helper edges (and the automatically added reverse edges, whose
        // payload is the default) carry `None`; original edges carry their
        // position in `self`.
        let none: Option<(usize, usize)> = None;
        for i in 0..self.vertex_count() {
            match incoming[i].cmp(&outgoing[i]) {
                Ordering::Greater => {
                    let d = incoming[i] - outgoing[i];
                    flow_graph.add_edge(FlowEdge::with_payload(self.vertex_count(), i, d, none));
                    total_demand += d;
                }
                Ordering::Less => {
                    let d = outgoing[i] - incoming[i];
                    flow_graph.add_edge(FlowEdge::with_payload(
                        i,
                        self.vertex_count() + 1,
                        d,
                        none,
                    ));
                }
                Ordering::Equal => {}
            }
        }
        flow_graph.add_edge(FlowEdge::with_payload(
            destination,
            source,
            C::max_val(),
            none,
        ));
        let res = flow_graph.max_flow(self.vertex_count(), self.vertex_count() + 1) == total_demand;
        if !res {
            return false;
        }
        for i in 0..self.vertex_count() {
            for e in flow_graph.adj(i).iter() {
                if let Some((from, id)) = *e.payload() {
                    let push = *self.edge_at(from, id as u32).payload() + e.flow(&flow_graph);
                    let push_data = self.edge_at(from, id as u32).push_flow(push);
                    self.push_flow(push_data);
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::FlowWithDemand;
    use crate::graph::edges::edge_trait::EdgeTrait;
    use crate::graph::edges::flow_edge::FlowEdge;
    use crate::graph::edges::flow_edge_trait::FlowEdgeTrait;
    use crate::graph::Graph;
    use crate::misc::random::{Random, RandomTrait};

    #[test]
    fn unsigned_feasible_circulations() {
        let mut rng = Random::new_with_seed(141);
        for _ in 0..200 {
            let n = rng.gen_range(2..=8usize);
            // A known feasible flow: a few random walks from 0 to n - 1.
            let mut flow = std::collections::BTreeMap::<(usize, usize), u64>::new();
            for _ in 0..rng.gen_range(1..=4usize) {
                let amount = rng.gen_range(1..=5u64);
                let mut v = 0;
                while v != n - 1 {
                    let to = rng.gen_range(v + 1..n);
                    *flow.entry((v, to)).or_default() += amount;
                    v = to;
                }
            }
            let mut graph: Graph<FlowEdge<u64, u64>> = Graph::new_linked(n);
            let mut edges = Vec::new();
            for (&(a, b), &f) in &flow {
                let low = rng.gen_range(0..=f);
                let cap = f + rng.gen_range(0..=3u64);
                graph.add_edge(FlowEdge::with_payload(a, b, cap, low));
                edges.push((a, b, low, cap));
            }
            assert!(graph.flow_with_demand(0, n - 1));
            let mut balance = vec![0i64; n];
            for &(a, b, low, cap) in &edges {
                let e = graph
                    .adj(a)
                    .iter()
                    .find(|e| e.to() == b && *e.payload() == low)
                    .unwrap();
                let f = e.flow(&graph);
                assert!(low <= f && f <= cap, "{low} <= {f} <= {cap}");
                balance[a] -= f as i64;
                balance[b] += f as i64;
            }
            assert!(balance[1..n - 1].iter().all(|&x| x == 0));
        }
    }

    #[test]
    fn infeasible_cases() {
        // lower bound above capacity
        let mut graph: Graph<FlowEdge<u64, u64>> = Graph::new_linked(2);
        graph.add_edge(FlowEdge::with_payload(0, 1, 3, 5));
        assert!(!graph.flow_with_demand(0, 1));
        // demand into a dead end
        let mut graph: Graph<FlowEdge<u64, u64>> = Graph::new_linked(3);
        graph.add_edge(FlowEdge::with_payload(0, 1, 5, 2));
        graph.add_edge(FlowEdge::with_payload(0, 2, 5, 0));
        assert!(!graph.flow_with_demand(0, 2));
    }

    #[test]
    fn demand_on_an_edge_without_capacity_is_infeasible() {
        let mut g: Graph<FlowEdge<u64, u64>> = Graph::new_linked(2);
        g.add_edge(FlowEdge::with_payload(0, 1, 0, 5));
        assert!(!g.flow_with_demand(0, 1));
        let mut g: Graph<FlowEdge<u64, u64>> = Graph::new_linked(2);
        g.add_edge(FlowEdge::with_payload(0, 1, 0, 0));
        g.add_edge(FlowEdge::with_payload(0, 1, 5, 5));
        assert!(g.flow_with_demand(0, 1));
    }
}
