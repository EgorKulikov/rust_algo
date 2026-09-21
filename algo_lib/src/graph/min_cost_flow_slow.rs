use crate::collections::indexed_heap::IndexedHeap;
use crate::collections::min_max::MinimMaxim;
use crate::graph::edges::flow_edge_trait::FlowEdgeTrait;
use crate::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use crate::graph::flow_graph::FlowGraph;
use crate::graph::negative_distances::{Distance, NegativeDistances};
use crate::graph::{CostAndFlow, Graph};
use crate::numbers::num_traits::algebra::{AdditionMonoidWithSub, MultiplicationMonoid};

/// Successive shortest paths with potentials.
///
/// "Slow" is meant literally once a cost is negative. The initial potentials
/// come from a Bellman-Ford over all stored edges, zero-capacity reverse edges
/// included, so any negative edge with an alternative route looks like a
/// negative cycle and the potentials stay zero. Dijkstra then runs on negative
/// reduced costs as a label-correcting search: the answer is still right, but
/// the running time has no polynomial bound (it is exponential on adversarial
/// graphs). With non-negative costs it is the usual `O(flow * E log V)`.
///
/// A real negative cycle among positive-capacity edges is not supported (the
/// search never ends); use `MinCostFlow` for that.
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
    // prev[v] = Some((from, edge_id)) where edge_id is the global edge id.
    let mut prev: Vec<Option<(usize, usize)>> = vec![None; graph.vertex_count()];
    let mut flow = C::zero();
    let mut cost = C::zero();
    loop {
        prev.fill(None);
        d[source] = C::zero();
        heap.add_or_adjust(source, C::zero());
        while let Some((cur, dist)) = heap.pop() {
            for (i, e) in graph.adj(cur).iter_with_id() {
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
        if prev[sink].is_none() || !take_positive && d[sink] + adj[sink] >= adj[source] {
            break;
        }
        let mut cur_flow = None;
        let mut v = sink;
        while v != source {
            let (from, edge) = prev[v].unwrap();
            let e = graph.edge_at(from, edge as u32);
            cur_flow.minim(e.capacity());
            v = from;
        }
        let cur_flow = cur_flow.unwrap();
        flow += cur_flow;
        cost += (d[sink] + adj[sink] - adj[source]) * cur_flow;
        let mut v = sink;
        while v != source {
            let (from, edge) = prev[v].unwrap();
            let push_data = graph.edge_at(from, edge as u32).push_flow(cur_flow);
            graph.push_flow(push_data);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::edges::weighted_flow_edge::WeightedFlowEdge;

    // Chain of 5 split vertices; every vertex may emit one free unit and
    // absorb one unit for a gain. Once flow leaves the source, reverse edges
    // lead back into it and its potential becomes positive, which the stop
    // condition used to ignore.
    fn chain() -> (Graph<WeightedFlowEdge<i64, i64, ()>>, usize, usize) {
        let gain = [10i64, 89, 30, 27, 0];
        let num = gain.len();
        let mut graph = Graph::new_2d(2 * num + 2);
        let source = 2 * num;
        let sink = source + 1;
        for i in 0..num {
            graph.add_edge(WeightedFlowEdge::new(2 * i, sink, -gain[i], 1));
            graph.add_edge(WeightedFlowEdge::new(2 * i, 2 * i + 1, 0, num as i64));
            graph.add_edge(WeightedFlowEdge::new(source, 2 * i + 1, 0, 1));
        }
        for i in 0..num - 1 {
            graph.add_edge(WeightedFlowEdge::new(2 * i + 1, 2 * i + 2, 0, num as i64));
        }
        (graph, source, sink)
    }

    #[test]
    fn negative_paths_after_source_potential_grows() {
        let (mut graph, source, sink) = chain();
        let res = graph.min_cost_flow_slow(source, sink);
        assert_eq!(res.cost, -146);
        assert_eq!(res.flow, 3);
    }

    #[test]
    fn max_flow_variant_agrees_on_cost() {
        let (mut graph, source, sink) = chain();
        let res = graph.min_cost_max_flow_slow(source, sink);
        assert_eq!(res.cost, -146);
        assert_eq!(res.flow, 4);
    }
}
