use crate::collections::bit_set::BitSet;
use crate::graph::edges::bi_edge_trait::BiEdgeTrait;
use crate::graph::Graph;
use crate::misc::recursive_function::{Callable, Callable2, RecursiveFunction, RecursiveFunction2};

pub trait Decompose<F: FnMut(usize, &BitSet)> {
    fn decompose(&self, f: F);
}

impl<E: BiEdgeTrait, F: FnMut(usize, &BitSet)> Decompose<F> for Graph<E> {
    fn decompose(&self, mut f: F) {
        let n = self.vertex_count();
        let mut sz = vec![0; n];
        let mut closed = BitSet::new(n);
        let mut center_decomp = RecursiveFunction::new(|center_decomp, vert: usize| {
            let mut dfs = RecursiveFunction2::new(|dfs, vert: usize, prev: usize| {
                if closed[vert] {
                    sz[vert] = 0;
                    return 0;
                }
                sz[vert] = 1;
                for e in &self[vert] {
                    if e.to() == prev {
                        continue;
                    }
                    sz[vert] += dfs.call(e.to(), vert);
                }
                sz[vert]
            });
            let total = dfs.call(vert, vert);
            let mut cur = vert;
            let mut last = vert;
            loop {
                let mut found = false;
                for e in &self[cur] {
                    if e.to() != last && 2 * sz[e.to()] >= total {
                        found = true;
                        last = cur;
                        cur = e.to();
                        break;
                    }
                }
                if !found {
                    break;
                }
            }
            f(cur, &closed);
            closed.set(cur);
            for e in &self[cur] {
                if !closed[e.to()] {
                    center_decomp.call(e.to());
                }
            }
        });
        center_decomp.call(0);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::graph::edges::edge_trait::EdgeTrait;

    #[test]
    fn centroid_of_path() {
        // Path: 0-1-2-3-4-5-6 (centroid should be 3)
        let graph = Graph::with_biedges(7, &[(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6)]);
        let mut first_centroid = None;
        graph.decompose(|v, _closed| {
            if first_centroid.is_none() {
                first_centroid = Some(v);
                assert_eq!(v, 3, "centroid of 7-node path should be 3");
            }
        });
        assert!(first_centroid.is_some());
    }

    #[test]
    fn centroid_deep_tree() {
        // Star graph: 0 connected to 1,2,3,4,5 (centroid is 0)
        let graph = Graph::with_biedges(6, &[(0, 1), (0, 2), (0, 3), (0, 4), (0, 5)]);
        let mut first_centroid = None;
        graph.decompose(|v, _closed| {
            if first_centroid.is_none() {
                first_centroid = Some(v);
                assert_eq!(v, 0, "centroid of star should be the center");
            }
        });
    }

    #[test]
    fn centroid_caterpillar() {
        // Caterpillar: spine 0-1-2-3-4, with leaves 5,6 on node 2
        // Total 7 nodes. Centroid should be 2.
        let graph = Graph::with_biedges(
            7,
            &[(0, 1), (1, 2), (2, 3), (3, 4), (2, 5), (2, 6)],
        );
        let mut first_centroid = None;
        graph.decompose(|v, _closed| {
            if first_centroid.is_none() {
                first_centroid = Some(v);
                assert_eq!(v, 2, "centroid of caterpillar should be 2");
            }
        });
    }

    #[test]
    fn decompose_visits_all_vertices() {
        // Verify all n vertices are visited as centroids exactly once
        let n = 10;
        let edges: Vec<(usize, usize)> = (0..n - 1).map(|i| (i, i + 1)).collect();
        let graph = Graph::with_biedges(n, &edges);
        let mut visited = vec![false; n];
        graph.decompose(|v, _| {
            assert!(!visited[v], "vertex {} visited twice", v);
            visited[v] = true;
        });
        for i in 0..n {
            assert!(visited[i], "vertex {} not visited", i);
        }
    }

    #[test]
    fn centroid_is_valid() {
        // Build a larger path and verify the centroid's subtrees are balanced
        // Path: 0-1-2-...-14 (15 nodes)
        let n = 15;
        let edges: Vec<(usize, usize)> = (0..n - 1).map(|i| (i, i + 1)).collect();
        let graph = Graph::with_biedges(n, &edges);
        let mut first_centroid = None;
        graph.decompose(|v, closed| {
            if first_centroid.is_none() {
                first_centroid = Some(v);
                // Count subtree sizes by BFS from centroid, excluding closed vertices
                let mut max_subtree = 0usize;
                for e in &graph[v] {
                    if closed[e.to()] {
                        continue;
                    }
                    let mut size = 0;
                    let mut stack = vec![(e.to(), v)];
                    while let Some((u, prev)) = stack.pop() {
                        if closed[u] {
                            continue;
                        }
                        size += 1;
                        for e2 in &graph[u] {
                            if e2.to() != prev && !closed[e2.to()] {
                                stack.push((e2.to(), u));
                            }
                        }
                    }
                    if size > max_subtree {
                        max_subtree = size;
                    }
                }
                // Centroid property: no subtree has more than n/2 nodes
                assert!(
                    max_subtree <= n / 2,
                    "centroid {} has subtree of size {}, exceeds n/2={}",
                    v, max_subtree, n / 2
                );
            }
        });
    }
}
