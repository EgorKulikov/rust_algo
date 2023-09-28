use crate::collections::iter_ext::min_max::IterMinMaxPos;
use crate::graph::edges::edge_trait::{BidirectionalEdgeTrait, EdgeTrait};
use crate::graph::graph::Graph;
use std::collections::VecDeque;

pub trait EdgeAlgos {
    fn edge_distances(&self, source: usize) -> Vec<u32>;
}

pub trait BiEdgeAlgos: EdgeAlgos {
    fn centers(&self) -> Vec<usize>;
}

impl<E: EdgeTrait> EdgeAlgos for Graph<E> {
    fn edge_distances(&self, source: usize) -> Vec<u32> {
        // 1.43
        let mut dist = vec![std::u32::MAX; self.vertex_count()];
        dist[source] = 0;
        let mut q = VecDeque::new();
        q.push_back(source);
        while !q.is_empty() {
            let cur = q.pop_front().unwrap();
            for e in self[cur].iter() {
                let next = e.to();
                // 1.43
                if dist[next] == std::u32::MAX {
                    dist[next] = dist[cur] + 1;
                    q.push_back(next);
                }
            }
        }
        dist
    }
}

impl<E: BidirectionalEdgeTrait> BiEdgeAlgos for Graph<E> {
    fn centers(&self) -> Vec<usize> {
        debug_assert!(self.is_tree());
        if self.vertex_count() == 0 {
            return Vec::new();
        }
        let d0 = self.edge_distances(0);
        let first = d0.iter().max_position().unwrap();
        let d1 = self.edge_distances(first);
        let second = d1.iter().max_position().unwrap();
        let d2 = self.edge_distances(second);
        let mut res = Vec::new();
        let r1 = d1[second] / 2;
        let r2 = (d1[second] + 1) / 2;
        for (i, (d1, d2)) in d1.iter().zip(d2.iter()).enumerate() {
            if *d1 == r1 && *d2 == r2 || *d1 == r2 && *d2 == r1 {
                res.push(i);
            }
        }
        res
    }
}
