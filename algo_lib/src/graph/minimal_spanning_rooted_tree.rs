//! Minimal spanning rooted tree (minimum arborescence) of a directed graph.
//!
//! Tarjan's version of the Chu-Liu/Edmonds algorithm: every super-vertex
//! keeps its incoming edges in a lazily-adjusted skew heap, cycles are
//! contracted with a rollback DSU and expanded afterwards to recover the
//! actual edges. Runs in O((V + E) log V).

use crate::collections::dsu_rollback::DSURollback;
use crate::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use crate::graph::minimal_spanning_tree::{MstTarget, WeightSum};
use crate::graph::Graph;
use crate::numbers::num_traits::algebra::AdditionGroup;

const NONE: u32 = u32::MAX;

/// Skew heap over edge ids with lazy addition of a delta to whole subtrees.
struct SkewHeap<W> {
    left: Vec<u32>,
    right: Vec<u32>,
    weight: Vec<W>,
    delta: Vec<W>,
    merge_stack: Vec<u32>,
}

impl<W: Ord + AdditionGroup + Copy> SkewHeap<W> {
    fn new(weights: Vec<W>) -> Self {
        let len = weights.len();
        Self {
            left: vec![NONE; len],
            right: vec![NONE; len],
            weight: weights,
            delta: vec![W::zero(); len],
            merge_stack: Vec::new(),
        }
    }

    fn push_delta(&mut self, v: u32) {
        let v = v as usize;
        let d = self.delta[v];
        if d != W::zero() {
            self.weight[v] += d;
            for c in [self.left[v], self.right[v]] {
                if c != NONE {
                    self.delta[c as usize] += d;
                }
            }
            self.delta[v] = W::zero();
        }
    }

    fn merge(&mut self, mut a: u32, mut b: u32) -> u32 {
        let mut stack = std::mem::take(&mut self.merge_stack);
        while a != NONE && b != NONE {
            self.push_delta(a);
            self.push_delta(b);
            if self.weight[a as usize] > self.weight[b as usize] {
                std::mem::swap(&mut a, &mut b);
            }
            stack.push(a);
            a = self.right[a as usize];
        }
        let mut res = if a == NONE { b } else { a };
        while let Some(v) = stack.pop() {
            let v = v as usize;
            self.right[v] = self.left[v];
            self.left[v] = res;
            res = v as u32;
        }
        self.merge_stack = stack;
        res
    }

    fn min_weight(&mut self, root: u32) -> W {
        self.push_delta(root);
        self.weight[root as usize]
    }

    /// Removes the minimum and lazily subtracts its weight from the rest.
    fn pop_reduced(&mut self, root: u32) -> u32 {
        let w = self.min_weight(root);
        let (l, r) = (self.left[root as usize], self.right[root as usize]);
        let merged = self.merge(l, r);
        if merged != NONE {
            self.delta[merged as usize] -= w;
        }
        merged
    }
}

fn msrt<W, E, T>(graph: &Graph<E>, root: usize) -> Option<T>
where
    W: Ord + AdditionGroup + Copy,
    E: WeightedEdgeTrait<W>,
    T: MstTarget<W, E>,
{
    let n = graph.vertex_count();
    let mut from = Vec::with_capacity(graph.edge_count());
    let mut edges = Vec::with_capacity(graph.edge_count());
    let mut weights = Vec::with_capacity(graph.edge_count());
    for v in 0..n {
        for e in graph.adj(v).iter() {
            from.push(v);
            weights.push(e.weight());
            edges.push(e);
        }
    }
    let mut heap = SkewHeap::new(weights);
    let mut heap_root = vec![NONE; n];
    for (j, e) in edges.iter().enumerate() {
        heap_root[e.to()] = heap.merge(heap_root[e.to()], j as u32);
    }

    const UNSEEN: usize = usize::MAX;
    let mut seen = vec![UNSEEN; n];
    seen[root] = n;
    let mut dsu = DSURollback::new(n);
    let mut in_edge = vec![NONE; n];
    let mut path = Vec::new();
    let mut chosen = Vec::new();
    let mut contractions = Vec::new();
    for s in 0..n {
        let mut u = s;
        while seen[u] == UNSEEN {
            if heap_root[u] == NONE {
                return None;
            }
            let e = heap_root[u];
            heap_root[u] = heap.pop_reduced(e);
            chosen.push(e);
            path.push(u);
            seen[u] = s;
            u = dsu.find(from[e as usize]);
            if seen[u] == s {
                // Walked into the current path (or popped an edge internal to
                // this super-vertex) - contract the cycle into one node.
                let checkpoint = dsu.save();
                let mut merged_heap = NONE;
                let mut cycle = Vec::new();
                loop {
                    let w = path.pop().unwrap();
                    cycle.push(chosen.pop().unwrap());
                    merged_heap = heap.merge(merged_heap, heap_root[w]);
                    if !dsu.union(u, w) {
                        break;
                    }
                }
                u = dsu.find(u);
                heap_root[u] = merged_heap;
                seen[u] = UNSEEN;
                contractions.push((u, checkpoint, cycle));
            }
        }
        for &e in &chosen {
            in_edge[dsu.find(edges[e as usize].to())] = e;
        }
        chosen.clear();
        path.clear();
    }

    // Expand contractions, most recent first: the edge entering a contracted
    // node stays, the cycle edge entering the same vertex is discarded.
    for (rep, checkpoint, cycle) in contractions.into_iter().rev() {
        dsu.rollback(checkpoint);
        let real_in = in_edge[rep];
        for &e in &cycle {
            in_edge[dsu.find(edges[e as usize].to())] = e;
        }
        in_edge[dsu.find(edges[real_in as usize].to())] = real_in;
    }
    let mut res = T::new(n);
    for (v, &e) in in_edge.iter().enumerate() {
        if v != root {
            res.add_edge(from[e as usize], E::clone(edges[e as usize]));
        }
    }
    Some(res)
}

pub trait MinimalSpanningRootedTree<W: Ord + AdditionGroup + Copy, E: WeightedEdgeTrait<W>> {
    fn minimal_spanning_rooted_tree(&self, root: usize) -> Option<Graph<E>>;
    fn minimal_spanning_rooted_tree_weight(&self, root: usize) -> Option<W>;
}

impl<W: Ord + AdditionGroup + Copy, E: WeightedEdgeTrait<W>> MinimalSpanningRootedTree<W, E>
    for Graph<E>
{
    fn minimal_spanning_rooted_tree(&self, root: usize) -> Option<Graph<E>> {
        msrt(self, root)
    }

    fn minimal_spanning_rooted_tree_weight(&self, root: usize) -> Option<W> {
        msrt::<W, E, WeightSum<W>>(self, root).map(|res| res.0)
    }
}
