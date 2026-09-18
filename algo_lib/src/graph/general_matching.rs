use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::Graph;
use std::collections::VecDeque;

pub trait GeneralMatching {
    /// Maximum matching in an undirected graph (Edmonds' blossom algorithm,
    /// O(V^3) worst case with a greedy start): the mate of every vertex.
    fn general_matching(&self) -> Vec<Option<usize>>;
}

const NONE: usize = usize::MAX;

struct Blossom<'a, E: EdgeTrait> {
    graph: &'a Graph<E>,
    mate: Vec<usize>,
    parent: Vec<usize>,
    base: Vec<usize>,
    used: Vec<bool>,
    in_blossom: Vec<bool>,
    lca_mark: Vec<usize>,
    lca_stamp: usize,
    queue: VecDeque<usize>,
}

impl<E: EdgeTrait> Blossom<'_, E> {
    fn lca(&mut self, mut a: usize, mut b: usize) -> usize {
        self.lca_stamp += 1;
        loop {
            a = self.base[a];
            self.lca_mark[a] = self.lca_stamp;
            if self.mate[a] == NONE {
                break;
            }
            a = self.parent[self.mate[a]];
        }
        loop {
            b = self.base[b];
            if self.lca_mark[b] == self.lca_stamp {
                return b;
            }
            b = self.parent[self.mate[b]];
        }
    }

    fn mark_path(&mut self, mut v: usize, b: usize, mut child: usize) {
        while self.base[v] != b {
            self.in_blossom[self.base[v]] = true;
            self.in_blossom[self.base[self.mate[v]]] = true;
            self.parent[v] = child;
            child = self.mate[v];
            v = self.parent[self.mate[v]];
        }
    }

    /// BFS for an augmenting path from `root`; returns its free endpoint.
    fn find_path(&mut self, root: usize) -> usize {
        let n = self.mate.len();
        self.used.fill(false);
        self.parent.fill(NONE);
        for (i, b) in self.base.iter_mut().enumerate() {
            *b = i;
        }
        self.queue.clear();
        self.queue.push_back(root);
        self.used[root] = true;
        while let Some(v) = self.queue.pop_front() {
            let mut cursor = self.graph.head_edge(v);
            while cursor != u32::MAX {
                let to = self.graph.edge_at(v, cursor).to();
                cursor = self.graph.step_edge(v, cursor);
                if self.base[v] == self.base[to] || self.mate[v] == to {
                    continue;
                }
                if to == root || (self.mate[to] != NONE && self.parent[self.mate[to]] != NONE) {
                    // Odd cycle: contract the blossom.
                    let cur = self.lca(v, to);
                    self.in_blossom.fill(false);
                    self.mark_path(v, cur, to);
                    self.mark_path(to, cur, v);
                    for i in 0..n {
                        if self.in_blossom[self.base[i]] {
                            self.base[i] = cur;
                            if !self.used[i] {
                                self.used[i] = true;
                                self.queue.push_back(i);
                            }
                        }
                    }
                } else if self.parent[to] == NONE {
                    self.parent[to] = v;
                    if self.mate[to] == NONE {
                        return to;
                    }
                    let next = self.mate[to];
                    self.used[next] = true;
                    self.queue.push_back(next);
                }
            }
        }
        NONE
    }
}

impl<E: EdgeTrait> GeneralMatching for Graph<E> {
    fn general_matching(&self) -> Vec<Option<usize>> {
        assert!(E::REVERSABLE);
        let n = self.vertex_count();
        let mut state = Blossom {
            graph: self,
            mate: vec![NONE; n],
            parent: vec![NONE; n],
            base: (0..n).collect(),
            used: vec![false; n],
            in_blossom: vec![false; n],
            lca_mark: vec![0; n],
            lca_stamp: 0,
            queue: VecDeque::new(),
        };
        // Greedy start.
        for v in 0..n {
            if state.mate[v] == NONE {
                for e in self.adj(v).iter() {
                    let to = e.to();
                    if to != v && state.mate[to] == NONE {
                        state.mate[v] = to;
                        state.mate[to] = v;
                        break;
                    }
                }
            }
        }
        for root in 0..n {
            if state.mate[root] != NONE {
                continue;
            }
            let mut v = state.find_path(root);
            while v != NONE {
                let pv = state.parent[v];
                let next = state.mate[pv];
                state.mate[v] = pv;
                state.mate[pv] = v;
                v = next;
            }
        }
        state
            .mate
            .into_iter()
            .map(|m| (m != NONE).then_some(m))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::GeneralMatching;
    use crate::graph::Graph;
    use crate::misc::random::{Random, RandomTrait};

    fn brute(n: usize, adjacent: &[Vec<bool>]) -> usize {
        // best[mask] = maximum matching inside the vertex set `mask`
        let mut best = vec![0usize; 1 << n];
        for mask in 1usize..1 << n {
            let v = mask.trailing_zeros() as usize;
            let rest = mask & (mask - 1);
            let mut res = best[rest];
            for u in 0..n {
                if rest >> u & 1 == 1 && adjacent[v][u] {
                    res = res.max(1 + best[rest & !(1 << u)]);
                }
            }
            best[mask] = res;
        }
        best[(1 << n) - 1]
    }

    #[test]
    fn matches_brute_force() {
        let mut rng = Random::new_with_seed(271);
        for _ in 0..500 {
            let n = rng.gen_range(1..=11usize);
            let m = rng.gen_range(0..=2 * n);
            let edges: Vec<(usize, usize)> = (0..m)
                .map(|_| (rng.gen_range(0..n), rng.gen_range(0..n)))
                .filter(|&(a, b)| a != b)
                .collect();
            let mut adjacent = vec![vec![false; n]; n];
            for &(a, b) in &edges {
                adjacent[a][b] = true;
                adjacent[b][a] = true;
            }
            let mate = Graph::with_biedges(n, &edges).general_matching();
            let mut size = 0;
            for v in 0..n {
                if let Some(u) = mate[v] {
                    assert!(adjacent[v][u], "{v}-{u} is not an edge");
                    assert_eq!(mate[u], Some(v));
                    size += 1;
                }
            }
            assert_eq!(size / 2, brute(n, &adjacent), "n={n} edges={edges:?}");
        }
    }

    #[test]
    fn odd_cycles_and_petersen() {
        // two triangles joined by an edge: perfect matching through blossoms
        let g = Graph::with_biedges(6, &[(0, 1), (1, 2), (2, 0), (2, 3), (3, 4), (4, 5), (5, 3)]);
        assert!(g.general_matching().iter().all(|m| m.is_some()));
        let petersen = [
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 4),
            (4, 0),
            (0, 5),
            (1, 6),
            (2, 7),
            (3, 8),
            (4, 9),
            (5, 7),
            (7, 9),
            (9, 6),
            (6, 8),
            (8, 5),
        ];
        let g = Graph::with_biedges(10, &petersen);
        assert!(g.general_matching().iter().all(|m| m.is_some()));
    }
}
