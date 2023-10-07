use crate::collections::slice_ext::legacy_fill::LegacyFill;
use crate::graph::edges::flow_edge_trait::FlowEdgeTrait;
use crate::graph::flow_graph::FlowGraph;
use crate::graph::graph::Graph;
use crate::misc::recursive_function::{Callable2, RecursiveFunction2};
use crate::numbers::num_traits::add_sub::AddSub;
use crate::numbers::num_traits::ord::MinMax;
use crate::numbers::num_traits::zero_one::ZeroOne;
use crate::when;
use std::collections::VecDeque;

pub trait MaxFlow<C: AddSub + PartialOrd + Copy + ZeroOne + MinMax> {
    fn max_flow(&mut self, source: usize, destination: usize) -> C;
}

impl<C: AddSub + PartialOrd + Copy + ZeroOne + MinMax, E: FlowEdgeTrait<C>> MaxFlow<C>
    for Graph<E>
{
    fn max_flow(&mut self, source: usize, destination: usize) -> C {
        let n = self.vertex_count();
        let mut dist = vec![0u32; n];
        let mut next_edge = vec![0u32; n];
        let inf = C::max_val();
        let mut total_flow = C::zero();
        let mut q = VecDeque::new();
        loop {
            // 1.43
            dist.legacy_fill(std::u32::MAX);
            dist[source] = 0;
            q.push_back(source);
            while !q.is_empty() {
                let cur = q.pop_front().unwrap();
                for e in self[cur].iter() {
                    if e.capacity() != C::zero() {
                        let next = e.to();
                        // 1.43
                        if dist[next] == std::u32::MAX {
                            dist[next] = dist[cur] + 1;
                            q.push_back(next);
                        }
                    }
                }
            }
            // 1.43
            if dist[destination] == std::u32::MAX {
                break;
            }
            next_edge.legacy_fill(0);
            let mut dinic_impl = RecursiveFunction2::new(|f, source, mut flow| -> C {
                when! {
                    source == destination => flow,
                    flow == C::zero() || dist[source] == dist[destination] => C::zero(),
                    else => {
                        let mut total_pushed = C::zero();
                        while (next_edge[source] as usize) < self[source].len() {
                            let edge = &self[source][next_edge[source] as usize];
                            if edge.capacity() != C::zero() && dist[edge.to()] == dist[source] + 1 {
                                let pushed = f.call(edge.to(), flow.minimum(edge.capacity()));
                                if pushed != C::zero() {
                                    let push_data = edge.push_flow(pushed);
                                    self.push_flow(push_data);
                                    flow -= pushed;
                                    total_pushed += pushed;
                                    if flow == C::zero() {
                                        return total_pushed;
                                    }
                                }
                            }
                            next_edge[source] += 1;
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
