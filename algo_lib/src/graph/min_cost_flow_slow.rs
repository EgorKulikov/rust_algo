use crate::collections::indexed_heap::IndexedHeap;
use crate::collections::min_max::MinimMaxim;
use crate::graph::edges::flow_edge_trait::FlowEdgeTrait;
use crate::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use crate::graph::flow_graph::FlowGraph;
use crate::graph::negative_distances::{Distance, NegativeDistances};
use crate::graph::{CostAndFlow, Graph};
use crate::numbers::num_traits::algebra::{AdditionMonoidWithSub, MultiplicationMonoid};

pub trait MinCostFlowSlow<C> {
    fn min_cost_flow_slow(&mut self, source: usize, sink: usize) -> CostAndFlow<C>;
    fn min_cost_max_flow_slow(&mut self, source: usize, sink: usize) -> CostAndFlow<C>;
}

impl<C, E> MinCostFlowSlow<C> for Graph<E>
where
    C: Copy + AdditionMonoidWithSub + MultiplicationMonoid + Ord,
    E: WeightedEdgeTrait<C> + FlowEdgeTrait<C>,
{
    fn min_cost_flow_slow(&mut self, source: usize, sink: usize) -> CostAndFlow<C> {
        min_cost_flow_slow_impl(self, source, sink, false)
    }

    fn min_cost_max_flow_slow(&mut self, source: usize, sink: usize) -> CostAndFlow<C> {
        min_cost_flow_slow_impl(self, source, sink, true)
    }
}

fn min_cost_flow_slow_impl<
    C: Copy + AdditionMonoidWithSub + MultiplicationMonoid + Ord,
    E: WeightedEdgeTrait<C> + FlowEdgeTrait<C>,
>(
    graph: &mut Graph<E>,
    source: usize,
    sink: usize,
    take_positive: bool,
) -> CostAndFlow<C> {
    let dist = graph.negative_distances_from(source);
    let mut adj = vec![C::zero(); graph.vertex_count()];
    for i in 0..graph.vertex_count() {
        if let Distance::Finite { distance, .. } = dist[i] {
            adj[i] = distance;
        }
    }
    let mut heap = IndexedHeap::new(graph.vertex_count());
    let mut d = vec![C::zero(); graph.vertex_count()];
    let mut prev = vec![None; graph.vertex_count()];
    let mut flow = C::zero();
    let mut cost = C::zero();
    loop {
        prev.fill(None);
        d[source] = C::zero();
        heap.add_or_adjust(source, C::zero());
        while let Some((cur, dist)) = heap.pop() {
            for (i, e) in graph[cur].iter().enumerate() {
                if e.capacity() != C::zero() {
                    let next = e.to();
                    let cost = e.weight() + adj[cur] - adj[next];
                    let total = dist + cost;
                    if prev[next].is_none() || total < d[next] {
                        d[next] = total;
                        prev[next] = Some((cur, i));
                        heap.add_or_relax(next, total);
                    }
                }
            }
        }
        if prev[sink].is_none() || !take_positive && d[sink] + adj[sink] >= C::zero() {
            break;
        }
        let mut cur_flow = None;
        let mut v = sink;
        while v != source {
            let (from, edge) = prev[v].unwrap();
            let e = &graph[from][edge];
            cur_flow.minim(e.capacity());
            v = from;
        }
        let cur_flow = cur_flow.unwrap();
        flow += cur_flow;
        cost += (d[sink] + adj[sink] - adj[source]) * cur_flow;
        let mut v = sink;
        while v != source {
            let (from, edge) = prev[v].unwrap();
            graph.push_flow(graph[from][edge].push_flow(cur_flow));
            v = from;
        }
        for i in 0..graph.vertex_count() {
            if prev[i].is_some() {
                adj[i] += d[i];
            }
        }
    }
    CostAndFlow { cost, flow }
}
