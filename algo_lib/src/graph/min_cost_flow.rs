use crate::collections::indexed_heap::IndexedHeap;
use crate::collections::min_max::MinimMaxim;
use crate::collections::slice_ext::legacy_fill::LegacyFill;
use crate::graph::edges::edge_id::EdgeId;
use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::edges::flow_edge_trait::FlowEdgeTrait;
use crate::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use crate::graph::edges::weighted_flow_edge::WeightedFlowEdgeRaw;
use crate::graph::flow_graph::FlowGraph;
use crate::graph::graph::Graph;
use crate::numbers::num_traits::add_sub::AddSub;
use crate::numbers::num_traits::bit_ops::Bits;
use crate::numbers::num_traits::mul_div_rem::Multable;
use crate::numbers::num_traits::ord::MinMax;
use crate::numbers::num_traits::zero_one::ZeroOne;
use std::fmt::Display;
use std::ops::Neg;

pub trait MinCostFlow<C> {
    fn min_cost_flow(&mut self, source: usize, sink: usize) -> (C, C);
    fn min_cost_max_flow(&mut self, source: usize, sink: usize) -> (C, C);
}

fn min_cost_flow_impl<C, Id, P: Clone>(
    or_graph: &mut Graph<WeightedFlowEdgeRaw<C, C, Id, P>>,
    source: usize,
    sink: usize,
    take_positive_cycles: bool,
) -> (C, C)
where
    C: AddSub + Neg<Output = C> + Multable + PartialOrd + Copy + ZeroOne + MinMax + Bits + Display,
    Id: EdgeId,
{
    let inf = C::max_val() >> 1;

    let n = or_graph.vertex_count();
    let mut p = vec![C::zero(); n + 1];
    p[n] = inf;

    let mut graph = Graph::new(n + 1);
    let mut corresponding = Vec::new();
    let mut max_capacity = C::zero();
    let mut sum_weight = C::zero();
    for i in 0..n {
        for (j, e) in or_graph[i].iter().enumerate() {
            if e.capacity() > C::zero() {
                max_capacity.maxim(e.capacity());
                sum_weight += e.weight().maximum(C::zero() - e.weight());
                corresponding.push((
                    i,
                    j,
                    graph.add_edge(i, WeightedFlowEdgeRaw::new(e.to(), e.weight(), C::zero())),
                ));
            }
        }
    }
    let mut bits = 0usize;
    while (C::one() << bits) <= max_capacity {
        bits += 1;
    }
    let back = graph.add_edge(
        sink,
        WeightedFlowEdgeRaw::new(
            source,
            if take_positive_cycles {
                C::zero() - sum_weight - C::one()
            } else {
                C::zero()
            },
            C::zero(),
        ),
    );
    for i in 0..n {
        graph.add_edge(n, WeightedFlowEdgeRaw::new(i, C::zero(), C::one()));
    }

    let mut dis = vec![C::zero(); n + 1];
    let mut pre = vec![(0usize, 0usize); n + 1];
    let mut heap = IndexedHeap::new(n + 1);

    let c = |from: usize, e: &WeightedFlowEdgeRaw<C, C, Id, ()>, p: &Vec<C>| -> C {
        p[from] + e.weight() - p[e.to()]
    };

    let dijkstra = |graph: &mut Graph<WeightedFlowEdgeRaw<C, C, Id, ()>>,
                    dis: &mut Vec<C>,
                    pre: &mut Vec<(usize, usize)>,
                    heap: &mut IndexedHeap<C>,
                    p: &Vec<C>,
                    s: usize| {
        dis.legacy_fill(inf);
        dis[s] = C::zero();
        assert!(heap.is_empty());
        heap.add_or_adjust(s, C::zero());

        while let Some((u, w)) = heap.pop() {
            assert!(w == dis[u]);
            for (i, e) in graph[u].iter().enumerate() {
                let v = e.to();

                debug_assert!(e.capacity() <= C::zero() || c(u, e, p) >= C::zero());

                if e.capacity() > C::zero() && dis[v] > w + c(u, e, p) {
                    dis[v] = w + c(u, e, p);
                    pre[v] = (u, i);
                    heap.add_or_adjust(v, dis[v]);
                }
            }
        }
    };

    let mut add_one =
        |graph: &mut Graph<WeightedFlowEdgeRaw<C, C, Id, ()>>, from: usize, id: usize| {
            if graph[from][id].capacity() > C::zero() {
                *graph[from][id].capacity_mut() += C::one();
                return;
            }
            let mut u = from;
            let e = &graph[from][id];
            let v = e.to();
            let cur_len = c(u, e, &p);
            dijkstra(graph, &mut dis, &mut pre, &mut heap, &p, v);
            let e = &graph[from][id];
            if dis[u] < inf && dis[u] + c(u, e, &p) < C::zero() {
                let rev_id = e.reverse_id();
                *graph[v][rev_id].capacity_mut() += C::one();
                while u != v {
                    graph.push_flow(graph[pre[u].0][pre[u].1].push_flow(C::one()));
                    u = pre[u].0;
                }
            } else {
                *graph[from][id].capacity_mut() += C::one();
            }
            let mut max_dis = C::zero();
            for i in dis.iter().take(n) {
                if *i != inf {
                    max_dis.maxim(*i);
                }
            }
            for i in 0..n {
                p[i] += if dis[i] < inf {
                    dis[i]
                } else {
                    max_dis
                        + if cur_len > C::zero() {
                            cur_len
                        } else {
                            C::zero() - cur_len
                        }
                };
            }
            dijkstra(graph, &mut dis, &mut pre, &mut heap, &p, n);
            for i in 0..n {
                let npi = dis[i] - p[n];
                p[i] += npi;
            }
        };

    add_one(&mut graph, sink, back);
    for i in (0..bits).rev() {
        for j in 0..=n {
            for e in graph[j].iter_mut() {
                *e.capacity_mut() <<= 1;
            }
        }
        for (from, self_edge_id, graph_edge_id) in corresponding.iter() {
            if or_graph[*from][*self_edge_id].capacity().is_set(i) {
                add_one(&mut graph, *from, *graph_edge_id);
                if graph[sink][back].capacity() == C::one() {
                    *graph[sink][back].capacity_mut() += C::one();
                }
            }
        }
    }

    let mut min_cost = C::zero();
    let max_flow = graph[sink][back].flow(&graph);
    for (from, self_edge_id, graph_edge_id) in corresponding {
        let x = &graph[from][graph_edge_id];
        min_cost += x.flow(&graph) * x.weight();
        or_graph.push_flow(or_graph[from][self_edge_id].push_flow(x.flow(&graph)));
    }
    (min_cost, max_flow)
}

impl<C, Id, P: Clone> MinCostFlow<C> for Graph<WeightedFlowEdgeRaw<C, C, Id, P>>
where
    C: AddSub + Neg<Output = C> + Multable + PartialOrd + Copy + ZeroOne + MinMax + Bits + Display,
    Id: EdgeId,
{
    fn min_cost_flow(&mut self, source: usize, sink: usize) -> (C, C) {
        min_cost_flow_impl(self, source, sink, false)
    }

    fn min_cost_max_flow(&mut self, source: usize, sink: usize) -> (C, C) {
        min_cost_flow_impl(self, source, sink, true)
    }
}
