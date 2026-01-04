use crate::collections::indexed_heap::IndexedHeap;
use crate::collections::min_max::MinimMaxim;
use crate::graph::edges::edge_id::EdgeId;
use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::edges::flow_edge_trait::FlowEdgeTrait;
use crate::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use crate::graph::edges::weighted_flow_edge::WeightedFlowEdgeRaw;
use crate::graph::flow_graph::FlowGraph;
use crate::graph::{CostAndFlow, Graph};
use crate::numbers::num_traits::bit_ops::BitOps;

pub trait MinCostFlow<C> {
    fn min_cost_flow(&mut self, source: usize, sink: usize) -> CostAndFlow<C>;
    fn min_cost_max_flow(&mut self, source: usize, sink: usize) -> CostAndFlow<C>;
}

macro_rules! min_cost_flow {
    ($($t: ty: $imp: ident)+) => {
        $(
            fn $imp<Id, P: Clone + Default>(
                or_graph: &mut Graph<WeightedFlowEdgeRaw<$t, $t, Id, P>>,
                source: usize,
                sink: usize,
                take_positive_cycles: bool,
            ) -> CostAndFlow<$t>
            where
                Id: EdgeId,
            {
                type C = $t;
                let inf = C::MAX >> 1;

                let n = or_graph.vertex_count();
                let mut p = vec![0; n + 1];
                p[n] = inf;

                let mut graph = Graph::new(n + 1);
                let mut corresponding = Vec::new();
                let mut max_capacity = 0;
                let mut sum_weight = 0;
                for i in 0..n {
                    for (j, e) in or_graph[i].iter().enumerate() {
                        if e.capacity() > 0 {
                            max_capacity.maxim(e.capacity());
                            sum_weight += e.weight().max(-e.weight());
                            corresponding.push((
                                i,
                                j,
                                graph.add_edge(WeightedFlowEdgeRaw::new(i, e.to(), e.weight(), 0)),
                            ));
                        }
                    }
                }
                let bits = max_capacity.highest_bit() + 1;
                let back = graph.add_edge(WeightedFlowEdgeRaw::new(
                    sink,
                    source,
                    if take_positive_cycles {
                        -sum_weight - 1
                    } else {
                        0
                    },
                    0,
                ));
                for i in 0..n {
                    graph.add_edge(WeightedFlowEdgeRaw::new(n, i, 0, 1));
                }

                let mut dis = vec![0; n + 1];
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
                    dis.fill(inf);
                    dis[s] = 0;
                    assert!(heap.is_empty());
                    heap.add_or_adjust(s, 0);

                    while let Some((u, w)) = heap.pop() {
                        assert!(w == dis[u]);
                        for (i, e) in graph[u].iter().enumerate() {
                            let v = e.to();

                            debug_assert!(e.capacity() <= 0 || c(u, e, p) >= 0);

                            if e.capacity() > 0 && dis[v] > w + c(u, e, p) {
                                dis[v] = w + c(u, e, p);
                                pre[v] = (u, i);
                                heap.add_or_adjust(v, dis[v]);
                            }
                        }
                    }
                };

                let mut add_one =
                    |graph: &mut Graph<WeightedFlowEdgeRaw<C, C, Id, ()>>, from: usize, id: usize| {
                        if graph[from][id].capacity() > 0 {
                            *graph[from][id].capacity_mut() += 1;
                            return;
                        }
                        let mut u = from;
                        let e = &graph[from][id];
                        let v = e.to();
                        let cur_len = c(u, e, &p);
                        dijkstra(graph, &mut dis, &mut pre, &mut heap, &p, v);
                        let e = &graph[from][id];
                        if dis[u] < inf && dis[u] + c(u, e, &p) < 0 {
                            let rev_id = e.reverse_id();
                            *graph[v][rev_id].capacity_mut() += 1;
                            while u != v {
                                graph.push_flow(graph[pre[u].0][pre[u].1].push_flow(1));
                                u = pre[u].0;
                            }
                        } else {
                            *graph[from][id].capacity_mut() += 1;
                        }
                        let mut max_dis = 0;
                        for i in dis.iter().take(n) {
                            if *i != inf {
                                max_dis.maxim(*i);
                            }
                        }
                        for i in 0..n {
                            p[i] += if dis[i] < inf {
                                dis[i]
                            } else {
                                max_dis + cur_len.abs()
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
                            if graph[sink][back].capacity() == 1 {
                                *graph[sink][back].capacity_mut() += 1;
                            }
                        }
                    }
                }

                let mut min_cost = 0;
                let max_flow = graph[sink][back].flow(&graph);
                for (from, self_edge_id, graph_edge_id) in corresponding {
                    let x = &graph[from][graph_edge_id];
                    min_cost += x.flow(&graph) * x.weight();
                    or_graph.push_flow(or_graph[from][self_edge_id].push_flow(x.flow(&graph)));
                }
                CostAndFlow {
                    cost: min_cost,
                    flow: max_flow,
                }
            }

            impl<Id, P: Clone + Default> MinCostFlow<$t> for Graph<WeightedFlowEdgeRaw<$t, $t, Id, P>>
            where
                Id: EdgeId,
            {
                fn min_cost_flow(&mut self, source: usize, sink: usize) -> CostAndFlow<$t> {
                    $imp(self, source, sink, false)
                }

                fn min_cost_max_flow(&mut self, source: usize, sink: usize) -> CostAndFlow<$t> {
                    $imp(self, source, sink, true)
                }
            }
        )+
    }
}

min_cost_flow!(i32: impl_32 i64: impl_64 i128: impl_128);
