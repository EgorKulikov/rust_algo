use crate::collections::bit_set::BitSet;
use crate::collections::min_max::MinimMaxim;
use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::Graph;

pub trait CutPointSearch {
    fn cut_points(&self) -> Vec<usize>;
}

impl<E: EdgeTrait> CutPointSearch for Graph<E> {
    /// Articulation points, each reported once.
    fn cut_points(&self) -> Vec<usize> {
        assert!(E::REVERSABLE);
        let n = self.vertex_count();
        let mut timer = 0u32;
        let mut tin = vec![0u32; n];
        let mut fup = vec![0u32; n];
        let mut used = BitSet::new(n);
        let mut reported = BitSet::new(n);
        let mut ans = Vec::new();
        const NO_PARENT: u32 = u32::MAX;
        // Frame: (vertex, parent, edge cursor, tree children so far).
        let mut frames: Vec<(u32, u32, u32, u32)> = Vec::new();
        for root in 0..n {
            if used[root] {
                continue;
            }
            used.set(root);
            tin[root] = timer;
            fup[root] = timer;
            timer += 1;
            frames.push((root as u32, NO_PARENT, self.head_edge(root), 0));
            while let Some(frame) = frames.last_mut() {
                let (vert, prev, cursor) = (frame.0 as usize, frame.1, frame.2);
                if cursor == u32::MAX {
                    let children = frame.3;
                    frames.pop();
                    if prev == NO_PARENT && children > 1 && !reported[vert] {
                        reported.set(vert);
                        ans.push(vert);
                    }
                    if let Some(parent) = frames.last_mut() {
                        let up = parent.0 as usize;
                        parent.3 += 1;
                        let cand = fup[vert];
                        fup[up].minim(cand);
                        if fup[vert] >= tin[up] && parent.1 != NO_PARENT && !reported[up] {
                            reported.set(up);
                            ans.push(up);
                        }
                    }
                    continue;
                }
                frame.2 = self.step_edge(vert, cursor);
                let to = self.edge_at(vert, cursor).to();
                if prev != NO_PARENT && to == prev as usize {
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
                    frames.push((to as u32, vert as u32, self.head_edge(to), 0));
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::CutPointSearch;
    use crate::graph::Graph;

    #[test]
    fn path_interior_cut_points() {
        let graph = Graph::with_biedges(4, &[(0, 1), (1, 2), (2, 3)]);
        let mut cp = graph.cut_points();
        cp.sort();
        assert_eq!(cp, vec![1, 2]);
    }

    #[test]
    fn triangle_no_cut_points() {
        let graph = Graph::with_biedges(3, &[(0, 1), (1, 2), (2, 0)]);
        assert!(graph.cut_points().is_empty());
    }

    #[test]
    fn star_center() {
        let graph = Graph::with_biedges(4, &[(0, 1), (0, 2), (0, 3)]);
        assert_eq!(graph.cut_points(), vec![0]);
    }

    fn components_without(n: usize, edges: &[(usize, usize)], removed: Option<usize>) -> usize {
        let mut comp: Vec<usize> = (0..n).collect();
        for &(a, b) in edges {
            if Some(a) == removed || Some(b) == removed {
                continue;
            }
            let (ca, cb) = (comp[a], comp[b]);
            for c in comp.iter_mut() {
                if *c == cb {
                    *c = ca;
                }
            }
        }
        let mut distinct: Vec<usize> = (0..n)
            .filter(|&v| Some(v) != removed)
            .map(|v| comp[v])
            .collect();
        distinct.sort();
        distinct.dedup();
        distinct.len()
    }

    #[test]
    fn random_multigraphs_match_brute_force() {
        use crate::misc::random::{Random, RandomTrait};
        let mut rng = Random::new_with_seed(152);
        for _ in 0..400 {
            let n = rng.gen_range(1..=8usize);
            let m = rng.gen_range(0..=10usize);
            let edges: Vec<(usize, usize)> = (0..m)
                .map(|_| (rng.gen_range(0..n), rng.gen_range(0..n)))
                .filter(|&(a, b)| a != b)
                .collect();
            let graph = Graph::with_biedges(n, &edges);
            let base = components_without(n, &edges, None);
            // Removing v leaves base - 1 components if v was isolated, base
            // if it was not a cut vertex, and more if it was.
            let expected: Vec<usize> = (0..n)
                .filter(|&v| components_without(n, &edges, Some(v)) > base)
                .collect();
            let mut got = graph.cut_points();
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
                Graph::with_biedges(n, &edges).cut_points().len()
            })
            .unwrap();
        assert_eq!(handle.join().unwrap(), n - 2);
    }
}
