use crate::collections::bit_set::BitSet;
use crate::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use crate::graph::Graph;
use crate::numbers::num_traits::algebra::{AdditionMonoid, One};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::mem::swap;

fn build_path_from_dist<W: Copy>(
    source: usize,
    mut destination: usize,
    dist: Vec<Option<(W, usize, usize)>>,
) -> Option<(W, Vec<(usize, usize)>)> {
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

pub trait Distances<W: AdditionMonoid + Ord + Copy> {
    fn distances_from(&self, source: usize) -> Vec<Option<(W, usize, usize)>>;

    fn distance(&self, source: usize, destination: usize) -> Option<(W, Vec<(usize, usize)>)> {
        build_path_from_dist(source, destination, self.distances_from(source))
    }

    fn zero_one_distances_from(&self, source: usize) -> Vec<Option<(W, usize, usize)>>
    where
        W: One;

    fn zero_one_distance(
        &self,
        source: usize,
        destination: usize,
    ) -> Option<(W, Vec<(usize, usize)>)>
    where
        W: One,
    {
        build_path_from_dist(source, destination, self.zero_one_distances_from(source))
    }
}

impl<W: AdditionMonoid + Ord + Copy, E: WeightedEdgeTrait<W>> Distances<W> for Graph<E> {
    fn distances_from(&self, source: usize) -> Vec<Option<(W, usize, usize)>> {
        // Lazy deletion: every improvement is pushed, stale entries are
        // skipped on pop. Entries are ordered by (distance, from, edge), so
        // ties resolve exactly as they did with the indexed heap.
        let n = self.vertex_count();
        let mut res: Vec<Option<(W, usize, usize)>> = vec![None; n];
        let mut done = BitSet::new(n);
        let mut heap = BinaryHeap::new();
        res[source] = Some((W::zero(), source, usize::MAX));
        heap.push(Reverse((W::zero(), source as u32, u32::MAX, source as u32)));
        while let Some(Reverse((dist, from, edge, cur))) = heap.pop() {
            let cur = cur as usize;
            if done[cur] {
                continue;
            }
            let edge = if edge == u32::MAX {
                usize::MAX
            } else {
                edge as usize
            };
            if res[cur] != Some((dist, from as usize, edge)) {
                continue;
            }
            done.set(cur);
            for (i, e) in self.adj(cur).iter_with_id() {
                let next = e.to();
                if done[next] {
                    continue;
                }
                let candidate = (dist + e.weight(), cur, i);
                if res[next].map_or(true, |best| candidate < best) {
                    res[next] = Some(candidate);
                    heap.push(Reverse((candidate.0, cur as u32, i as u32, next as u32)));
                }
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
                for (i, e) in self.adj(v).iter_with_id() {
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

#[cfg(test)]
mod tests {
    use super::Distances;
    use crate::graph::edges::edge_trait::EdgeTrait;
    use crate::graph::edges::weighted_edge::WeightedEdge;
    use crate::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
    use crate::graph::Graph;
    use crate::misc::random::{Random, RandomTrait};

    #[test]
    fn dijkstra_matches_floyd_warshall() {
        let mut rng = Random::new_with_seed(231);
        for _ in 0..200 {
            let n = rng.gen_range(1..=12usize);
            let m = rng.gen_range(0..=40usize);
            let mut graph: Graph<WeightedEdge<i64, ()>> = if rng.gen_bool() {
                Graph::new_linked(n)
            } else {
                Graph::new_2d(n)
            };
            const INF: i64 = i64::MAX / 4;
            let mut d = vec![vec![INF; n]; n];
            for (i, row) in d.iter_mut().enumerate() {
                row[i] = 0;
            }
            for _ in 0..m {
                let (a, b, w) = (
                    rng.gen_range(0..n),
                    rng.gen_range(0..n),
                    rng.gen_range(0..=5i64),
                );
                graph.add_edge(WeightedEdge::new(a, b, w));
                d[a][b] = d[a][b].min(w);
            }
            for k in 0..n {
                for i in 0..n {
                    for j in 0..n {
                        d[i][j] = d[i][j].min(d[i][k] + d[k][j]);
                    }
                }
            }
            let source = rng.gen_range(0..n);
            let res = graph.distances_from(source);
            for v in 0..n {
                match res[v] {
                    None => assert_eq!(d[source][v], INF),
                    Some((dist, from, edge)) => {
                        assert_eq!(dist, d[source][v]);
                        if v != source {
                            let e = graph.edge_at(from, edge as u32);
                            assert_eq!(e.to(), v);
                            assert_eq!(res[from].unwrap().0 + e.weight(), dist);
                        }
                    }
                }
                if let Some((total, path)) = graph.distance(source, v) {
                    assert_eq!(total, d[source][v]);
                    let mut at = source;
                    let mut sum = 0;
                    for (from, edge) in path {
                        assert_eq!(from, at);
                        let e = graph.edge_at(from, edge as u32);
                        sum += e.weight();
                        at = e.to();
                    }
                    assert_eq!((at, sum), (v, total));
                }
            }
        }
    }
}
