use crate::graph::edges::flow_edge_trait::FlowEdgeTrait;
use crate::graph::flow_graph::FlowGraph;
use crate::graph::Graph;
use crate::misc::recursive_function::{Callable2, RecursiveFunction2};
use crate::numbers::num_traits::algebra::AdditionMonoidWithSub;
use crate::numbers::num_traits::ord::MinMax;
use crate::when;
use std::collections::VecDeque;

pub trait MaxFlow<C: AdditionMonoidWithSub + Ord + Copy + MinMax> {
    fn max_flow(&mut self, source: usize, destination: usize) -> C;
}

impl<C: AdditionMonoidWithSub + Ord + Copy + MinMax, E: FlowEdgeTrait<C>> MaxFlow<C> for Graph<E> {
    fn max_flow(&mut self, source: usize, destination: usize) -> C {
        let n = self.vertex_count();
        let mut dist = vec![0u32; n];
        // `next_edge[v]` is the current edge id at vertex `v` (u32::MAX = exhausted).
        let mut next_edge = vec![u32::MAX; n];
        let inf = C::max_val();
        let mut total_flow = C::zero();
        let mut q = VecDeque::new();
        loop {
            dist.fill(u32::MAX);
            dist[source] = 0;
            q.push_back(source);
            while !q.is_empty() {
                let cur = q.pop_front().unwrap();
                for e in self.adj(cur).iter() {
                    if e.capacity() != C::zero() {
                        let next = e.to();
                        if dist[next] == u32::MAX {
                            dist[next] = dist[cur] + 1;
                            q.push_back(next);
                        }
                    }
                }
            }
            if dist[destination] == u32::MAX {
                break;
            }
            for v in 0..n {
                next_edge[v] = self.head_edge(v);
            }
            let mut dinic_impl = RecursiveFunction2::new(|f, source, mut flow| -> C {
                when! {
                    source == destination => flow,
                    flow == C::zero() || dist[source] == dist[destination] => C::zero(),
                    else => {
                        let mut total_pushed = C::zero();
                        while next_edge[source] != u32::MAX {
                            let eid = next_edge[source];
                            let edge = self.edge_at(source, eid);
                            if edge.capacity() != C::zero() && dist[edge.to()] == dist[source] + 1 {
                                let pushed = f.call(edge.to(), flow.min(edge.capacity()));
                                if pushed != C::zero() {
                                    let push_data = self.edge_at(source, eid).push_flow(pushed);
                                    self.push_flow(push_data);
                                    flow -= pushed;
                                    total_pushed += pushed;
                                    if flow == C::zero() {
                                        return total_pushed;
                                    }
                                }
                            }
                            next_edge[source] = self.step_edge(source, eid);
                        }
                        total_pushed
                    },
                }
            });
            total_flow += dinic_impl.call(source, inf);
        }
        total_flow
    }
}
