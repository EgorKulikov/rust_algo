use crate::collections::bit_set::BitSet;
use crate::collections::min_max::MinimMaxim;
use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::Graph;

pub trait BridgeSearch {
    fn bridges(&self) -> Vec<(usize, usize)>;
}

impl<E: EdgeTrait> BridgeSearch for Graph<E> {
    fn bridges(&self) -> Vec<(usize, usize)> {
        assert!(E::REVERSABLE);
        let n = self.vertex_count();
        let mut timer = 0u32;
        let mut tin = vec![0u32; n];
        let mut fup = vec![0u32; n];
        let mut used = BitSet::new(n);
        let mut ans = Vec::new();
        // Frame: (vertex, parent, edge cursor, parent edge already skipped).
        let mut frames: Vec<(u32, u32, u32, bool)> = Vec::new();
        for root in 0..n {
            if used[root] {
                continue;
            }
            used.set(root);
            tin[root] = timer;
            fup[root] = timer;
            timer += 1;
            frames.push((root as u32, root as u32, self.head_edge(root), false));
            while let Some(frame) = frames.last_mut() {
                let (vert, prev, cursor) = (frame.0 as usize, frame.1 as usize, frame.2);
                if cursor == u32::MAX {
                    frames.pop();
                    if let Some(parent) = frames.last() {
                        let up = parent.0 as usize;
                        let cand = fup[vert];
                        fup[up].minim(cand);
                        if fup[vert] > tin[up] {
                            ans.push((up, vert));
                        }
                    }
                    continue;
                }
                frame.2 = self.step_edge(vert, cursor);
                let to = self.edge_at(vert, cursor).to();
                // Only the first edge back to the parent is the tree edge; a
                // parallel edge still counts as a back edge.
                if to == prev && !frame.3 {
                    frame.3 = true;
                    continue;
                }
                if used[to] {
                    let cand = tin[to];
                    fup[vert].minim(cand);
                } else {
                    used.set(to);
                    tin[to] = timer;
                    fup[to] = timer;
                    timer += 1;
                    frames.push((to as u32, vert as u32, self.head_edge(to), false));
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::BridgeSearch;
    use crate::graph::Graph;

    #[test]
    fn path_all_bridges() {
        let graph = Graph::with_biedges(4, &[(0, 1), (1, 2), (2, 3)]);
        assert_eq!(graph.bridges().len(), 3);
    }

    #[test]
    fn cycle_no_bridges() {
        let graph = Graph::with_biedges(3, &[(0, 1), (1, 2), (2, 0)]);
        assert_eq!(graph.bridges().len(), 0);
    }

    #[test]
    fn mixed() {
        // Triangle 0-1-2 plus pendant 2-3
        let graph = Graph::with_biedges(4, &[(0, 1), (1, 2), (2, 0), (2, 3)]);
        let bridges = graph.bridges();
        assert_eq!(bridges.len(), 1);
        let (a, b) = bridges[0];
        assert!((a == 2 && b == 3) || (a == 3 && b == 2));
    }

    fn components(n: usize, edges: &[(usize, usize)], skip_edge: Option<usize>) -> usize {
        let mut comp: Vec<usize> = (0..n).collect();
        for (i, &(a, b)) in edges.iter().enumerate() {
            if Some(i) == skip_edge {
                continue;
            }
            let (ca, cb) = (comp[a], comp[b]);
            for c in comp.iter_mut() {
                if *c == cb {
                    *c = ca;
                }
            }
        }
        let mut distinct = comp.clone();
        distinct.sort();
        distinct.dedup();
        distinct.len()
    }

    #[test]
    fn random_multigraphs_match_brute_force() {
        use crate::misc::random::{Random, RandomTrait};
        let mut rng = Random::new_with_seed(151);
        for _ in 0..400 {
            let n = rng.gen_range(1..=8usize);
            let m = rng.gen_range(0..=10usize);
            let edges: Vec<(usize, usize)> = (0..m)
                .map(|_| (rng.gen_range(0..n), rng.gen_range(0..n)))
                .filter(|&(a, b)| a != b)
                .collect();
            let graph = Graph::with_biedges(n, &edges);
            let base = components(n, &edges, None);
            let mut expected: Vec<(usize, usize)> = (0..edges.len())
                .filter(|&i| components(n, &edges, Some(i)) > base)
                .map(|i| (edges[i].0.min(edges[i].1), edges[i].0.max(edges[i].1)))
                .collect();
            expected.sort();
            let mut got: Vec<(usize, usize)> = graph
                .bridges()
                .into_iter()
                .map(|(a, b)| (a.min(b), a.max(b)))
                .collect();
            got.sort();
            assert_eq!(got, expected, "n={n} edges={edges:?}");
        }
    }

    #[test]
    fn long_path_needs_no_deep_stack() {
        let n = 200_000;
        let handle = std::thread::Builder::new()
            .stack_size(256 * 1024)
            .spawn(move || {
                let edges: Vec<(usize, usize)> = (1..n).map(|v| (v - 1, v)).collect();
                Graph::with_biedges(n, &edges).bridges().len()
            })
            .unwrap();
        assert_eq!(handle.join().unwrap(), n - 1);
    }
}
