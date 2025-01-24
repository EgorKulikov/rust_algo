use crate::collections::link_cut::LinkCutNode;
use crate::collections::payload::Payload;
use crate::collections::vec_ext::gen_vec::VecGen;
use crate::graph::edges::flow_edge_trait::FlowEdgeTrait;
use crate::graph::flow_graph::FlowGraph;
use crate::graph::Graph;
use crate::numbers::num_traits::algebra::AdditionMonoidWithSub;
use crate::numbers::num_traits::ord::MinMax;
use std::collections::VecDeque;

// is it really fast though?
pub trait FastMaxFlow<C: AdditionMonoidWithSub + Ord + Copy + MinMax> {
    fn fast_max_flow(&mut self, source: usize, destination: usize) -> C;
}

impl<C: AdditionMonoidWithSub + Ord + Copy + MinMax, E: FlowEdgeTrait<C>> FastMaxFlow<C>
    for Graph<E>
{
    fn fast_max_flow(&mut self, source: usize, destination: usize) -> C {
        struct Node<C: AdditionMonoidWithSub + Ord + Copy + MinMax> {
            id: u32,
            self_val: C,
            val: C,
            val_id: u32,
            delta: C,
            pushed: C,
        }
        impl<C: AdditionMonoidWithSub + Ord + Copy + MinMax> Payload for Node<C> {
            const NEED_UPDATE: bool = true;
            const NEED_ACCUMULATE: bool = true;
            fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
                self.val_id = self.id;
                self.val = self.self_val;
                if let Some(left) = left {
                    if left.val < self.val {
                        self.val = left.val;
                        self.val_id = left.val_id;
                    }
                }
                if let Some(right) = right {
                    if right.val < self.val {
                        self.val = right.val;
                        self.val_id = right.val_id;
                    }
                }
            }

            fn accumulate(&mut self, delta: &Self) {
                self.self_val += delta.delta;
                self.val += delta.delta;
                self.delta += delta.delta;
                self.pushed -= delta.delta;
            }

            fn reset_delta(&mut self) {
                self.delta = C::zero();
            }
        }

        let n = self.vertex_count();
        let nodes = Vec::with_gen(n, |i| {
            LinkCutNode::new(Node {
                id: i as u32,
                self_val: C::zero(),
                val: C::zero(),
                val_id: i as u32,
                delta: C::zero(),
                pushed: C::zero(),
            })
        });
        let mut incomming = vec![Vec::new(); n];
        let mut queue = Vec::new();
        let mut dist = vec![0u32; n];
        let mut next_edge = vec![0u32; n];
        let inf = C::max_val();
        let mut total_flow = C::zero();
        let mut q = VecDeque::new();
        loop {
            nodes[destination].with_payload_mut(|p| p.self_val = inf);
            dist.fill(u32::MAX);
            dist[source] = 0;
            q.push_back(source);
            while !q.is_empty() {
                let cur = q.pop_front().unwrap();
                for e in self[cur].iter() {
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
            next_edge.fill(0);
            for i in 0..n {
                nodes[i].cut();
                if dist[i] >= dist[destination] {
                    next_edge[i] = self[i].len() as u32;
                    continue;
                }
                while (next_edge[i] as usize) < self[i].len() {
                    let edge = &self[i][next_edge[i] as usize];
                    if edge.capacity() != C::zero() && dist[edge.to()] == dist[i] + 1 {
                        nodes[i].with_payload_mut(|p| p.self_val = edge.capacity());
                        nodes[i].link(nodes[edge.to()]);
                        incomming[edge.to()].push(i);
                        break;
                    }
                    next_edge[i] += 1;
                }
            }
            loop {
                let (cur_val, cur_id) = if nodes[source].find_root() == nodes[destination] {
                    nodes[source].with_payload(|p| (p.val, p.val_id as usize))
                } else {
                    if nodes[source].parent().is_none() {
                        break;
                    }
                    (
                        C::zero(),
                        nodes[source]
                            .find_root()
                            .with_payload(|p| p.val_id as usize),
                    )
                };
                total_flow += cur_val;
                nodes[source].with_payload_mut(|p| {
                    p.delta -= cur_val;
                    p.pushed += cur_val;
                    p.self_val -= cur_val;
                    p.val -= cur_val;
                });
                queue.push(cur_id);
                while let Some(cur_id) = queue.pop() {
                    if (next_edge[cur_id] as usize) < self[cur_id].len() {
                        let edge = &self[cur_id][next_edge[cur_id] as usize];
                        let push_data = edge.push_flow(nodes[cur_id].with_payload(|p| p.pushed));
                        self.push_flow(push_data);
                        nodes[cur_id].cut();
                        next_edge[cur_id] += 1;
                    }
                    nodes[cur_id].with_payload_mut(|p| {
                        p.pushed = C::zero();
                        p.self_val = C::zero();
                    });
                    let mut found = false;
                    while (next_edge[cur_id] as usize) < self[cur_id].len() {
                        let edge = &self[cur_id][next_edge[cur_id] as usize];
                        if edge.capacity() != C::zero() && dist[edge.to()] == dist[cur_id] + 1 {
                            nodes[cur_id].with_payload_mut(|p| p.self_val = edge.capacity());
                            nodes[cur_id].link(nodes[edge.to()]);
                            incomming[edge.to()].push(cur_id);
                            found = true;
                            break;
                        }
                        next_edge[cur_id] += 1;
                    }
                    if found {
                        continue;
                    }
                    for p in incomming[cur_id].drain(..) {
                        if nodes[p].parent() == Some(nodes[cur_id]) {
                            queue.push(p);
                        }
                    }
                }
            }

            for i in 0..n {
                if next_edge[i] < self[i].len() as u32 {
                    let edge = &self[i][next_edge[i] as usize];
                    let push_data = edge.push_flow(nodes[i].with_payload(|p| p.pushed));
                    self.push_flow(push_data);
                    nodes[i].cut();
                    nodes[i].with_payload_mut(|p| {
                        p.pushed = C::zero();
                        p.self_val = C::zero();
                    });
                    incomming[i].clear();
                }
            }
        }
        total_flow
    }
}
