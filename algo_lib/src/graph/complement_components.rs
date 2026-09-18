use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::Graph;

pub trait ComplementComponents {
    /// Component id of every vertex in the complement of an undirected
    /// graph, in O(n + m): a vertex leaves the list of unvisited vertices the
    /// first time it is scanned from a non-neighbour, and every failed scan
    /// is paid for by an edge.
    fn complement_components(&self) -> Vec<usize>;
}

impl<E: EdgeTrait> ComplementComponents for Graph<E> {
    fn complement_components(&self) -> Vec<usize> {
        assert!(E::REVERSABLE);
        let n = self.vertex_count();
        let mut color = vec![usize::MAX; n];
        let mut remaining: Vec<usize> = (0..n).collect();
        let mut stamp = vec![usize::MAX; n];
        let mut queue = Vec::new();
        let mut components = 0;
        while let Some(start) = remaining.pop() {
            color[start] = components;
            queue.push(start);
            while let Some(v) = queue.pop() {
                for e in self.adj(v).iter() {
                    stamp[e.to()] = v;
                }
                let mut kept = 0;
                for i in 0..remaining.len() {
                    let u = remaining[i];
                    if stamp[u] == v {
                        remaining[kept] = u;
                        kept += 1;
                    } else {
                        color[u] = components;
                        queue.push(u);
                    }
                }
                remaining.truncate(kept);
            }
            components += 1;
        }
        color
    }
}

#[cfg(test)]
mod tests {
    use super::ComplementComponents;
    use crate::graph::Graph;
    use crate::misc::random::{Random, RandomTrait};

    #[test]
    fn matches_brute_force() {
        let mut rng = Random::new_with_seed(241);
        for _ in 0..300 {
            let n = rng.gen_range(1..=10usize);
            let m = rng.gen_range(0..=n * n);
            let edges: Vec<(usize, usize)> = (0..m)
                .map(|_| (rng.gen_range(0..n), rng.gen_range(0..n)))
                .filter(|&(a, b)| a != b)
                .collect();
            let mut adjacent = vec![vec![false; n]; n];
            for &(a, b) in &edges {
                adjacent[a][b] = true;
                adjacent[b][a] = true;
            }
            let mut comp: Vec<usize> = (0..n).collect();
            for a in 0..n {
                for b in 0..n {
                    if a != b && !adjacent[a][b] {
                        let (ca, cb) = (comp[a], comp[b]);
                        for c in comp.iter_mut() {
                            if *c == cb {
                                *c = ca;
                            }
                        }
                    }
                }
            }
            let color = Graph::with_biedges(n, &edges).complement_components();
            for a in 0..n {
                for b in 0..n {
                    assert_eq!(
                        color[a] == color[b],
                        comp[a] == comp[b],
                        "n={n} edges={edges:?}"
                    );
                }
            }
            let count = color.iter().max().unwrap() + 1;
            assert!(color.iter().all(|&c| c < count));
        }
    }
}
