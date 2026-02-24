use crate::collections::bit_set::BitSet;
use crate::collections::indexed_heap::IndexedHeap;
use crate::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use crate::graph::Graph;
use crate::numbers::num_traits::algebra::{AdditionMonoid, One};
use std::mem::swap;

pub trait Distances<W: AdditionMonoid + Ord + Copy> {
    fn distances_from(&self, source: usize) -> Vec<Option<(W, usize, usize)>>;

    fn distance(&self, source: usize, mut destination: usize) -> Option<(W, Vec<(usize, usize)>)> {
        let dist = self.distances_from(source);
        dist[destination].map(|(w, ..)| {
            let mut path = Vec::new();
            while destination != source {
                let (_, from, edge) = dist[destination].unwrap();
                path.push((from, edge));
                destination = from;
            }
            path.reverse();
            (w, path)
        })
    }

    fn zero_one_distances_from(&self, source: usize) -> Vec<Option<(W, usize, usize)>>
    where
        W: One;
}

impl<W: AdditionMonoid + Ord + Copy, E: WeightedEdgeTrait<W>> Distances<W> for Graph<E> {
    fn distances_from(&self, source: usize) -> Vec<Option<(W, usize, usize)>> {
        let n = self.vertex_count();
        let mut res = vec![None; n];
        let mut heap = IndexedHeap::new(n);
        heap.add_or_adjust(source, (W::zero(), source, self[source].len()));
        while let Some((cur, dist)) = heap.pop() {
            res[cur] = Some(dist);
            let dist = dist.0;
            for (i, e) in self[cur].iter().enumerate() {
                let next = e.to();
                if res[next].is_some() {
                    continue;
                }
                let total = dist + e.weight();
                let next_dist = (total, cur, i);
                heap.add_or_relax(next, next_dist);
            }
        }
        res
    }

    fn zero_one_distances_from(&self, source: usize) -> Vec<Option<(W, usize, usize)>>
    where
        W: One,
    {
        let n = self.vertex_count();
        let mut res = vec![None; n];
        res[source] = Some((W::zero(), 0, 0));
        let mut cur = vec![source];
        let mut next = Vec::new();
        let mut added_cur = BitSet::new(n);
        added_cur.set(source);
        let mut added_next = BitSet::new(n);
        let mut processed = BitSet::new(n);
        let mut cur_dist = W::zero();
        while !cur.is_empty() {
            while let Some(v) = cur.pop() {
                if processed[v] {
                    continue;
                }
                processed.set(v);
                for (i, e) in self[v].iter().enumerate() {
                    let to = e.to();
                    let w = e.weight();
                    if added_cur[to] {
                        continue;
                    }
                    if w == W::zero() {
                        added_cur.set(to);
                        res[to] = Some((cur_dist, v, i));
                        cur.push(to);
                        continue;
                    }
                    if added_next[to] {
                        continue;
                    }
                    assert!(w == W::one());
                    added_next.set(to);
                    res[to] = Some((cur_dist + W::one(), v, i));
                    next.push(to);
                }
            }
            for &v in &next {
                added_cur.set(v);
            }
            cur_dist += W::one();
            swap(&mut cur, &mut next);
        }
        res
    }
}
