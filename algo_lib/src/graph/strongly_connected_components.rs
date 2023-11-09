use crate::collections::bit_set::BitSet;
use crate::graph::edges::edge::Edge;
use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::graph::Graph;
use crate::misc::recursive_function::{Callable, RecursiveFunction};

pub trait StronglyConnectedComponents {
    fn strongly_connected_components(&self) -> (Vec<usize>, Graph<Edge<()>>);
}

impl<E: EdgeTrait> StronglyConnectedComponents for Graph<E> {
    fn strongly_connected_components(&self) -> (Vec<usize>, Graph<Edge<()>>) {
        assert!(!E::REVERSABLE);
        let n = self.vertex_count();
        let mut order = Vec::with_capacity(n);
        let mut color = vec![0; n];
        let mut visited = BitSet::new(n);
        for i in 0..n {
            if !visited[i] {
                let mut first_dfs = RecursiveFunction::new(|f, vert| {
                    if visited[vert] {
                        return;
                    }
                    visited.set(vert);
                    for e in self[vert].iter() {
                        f.call(e.to());
                    }
                    order.push(vert);
                });
                first_dfs.call(i);
            }
        }
        visited.fill(false);
        let mut res = Graph::new(0);
        let mut index = 0usize;
        let mut next = vec![n; n];
        let mut queue = Vec::with_capacity(n);
        let mut gt = Graph::new(n);
        for i in 0..n {
            for e in self[i].iter() {
                gt.add_edge(e.to(), Edge::new(i));
            }
        }
        for i in (0..n).rev() {
            if !visited[order[i]] {
                let key = i;
                let mut second_dfs = RecursiveFunction::new(|f, vert| {
                    if visited[vert] {
                        if color[vert] != index && next[color[vert]] != key {
                            next[color[vert]] = key;
                            queue.push(color[vert]);
                        }
                        return;
                    }
                    color[vert] = index;
                    visited.set(vert);
                    for e in gt[vert].iter() {
                        f.call(e.to());
                    }
                });
                second_dfs.call(order[i]);
                res.add_vertices(1);
                for j in queue.drain(..) {
                    res.add_edge(j, Edge::new(index));
                }
                index += 1;
            }
        }
        (color, res)
    }
}
