use std::collections::VecDeque;

/// Maximum bipartite matching by push-relabel with periodic global relabeling.
///
/// Right vertices are active: each grabs its lowest-level left neighbour,
/// displacing that neighbour's current mate, which is requeued. A BFS from the
/// free left vertices recomputes levels every `left + right` pops.
#[derive(Clone)]
pub struct BipartiteMatching {
    left_adj: Vec<Vec<u32>>,
    right_adj: Vec<Vec<u32>>,
    left_mate: Vec<Option<usize>>,
    right_mate: Vec<Option<usize>>,
    size: Option<usize>,
}

impl BipartiteMatching {
    pub fn new(left: usize, right: usize) -> Self {
        Self {
            left_adj: vec![Vec::new(); left],
            right_adj: vec![Vec::new(); right],
            left_mate: vec![None; left],
            right_mate: vec![None; right],
            size: None,
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.left_adj[u].push(v as u32);
        self.right_adj[v].push(u as u32);
        self.size = None;
    }

    /// Right mate of each left vertex, valid after `run`.
    pub fn left_mate(&self) -> &[Option<usize>] {
        &self.left_mate
    }

    /// Left mate of each right vertex, valid after `run`.
    pub fn right_mate(&self) -> &[Option<usize>] {
        &self.right_mate
    }

    /// Computes the maximum matching and returns its size.
    pub fn run(&mut self) -> usize {
        if let Some(size) = self.size {
            return size;
        }
        let (l, r) = (self.left_adj.len(), self.right_adj.len());
        self.left_mate.fill(None);
        self.right_mate.fill(None);
        // Levels of left vertices: a lower bound on the alternating-path
        // distance to a free left vertex; `inf` means no such path.
        let inf = (l + r) as u32;
        let period = l + r;
        let mut level = vec![inf; l];
        let mut seen = vec![false; r];
        let mut queue: VecDeque<usize> = (0..r).collect();
        let mut bfs = VecDeque::new();
        let mut since_relabel = period;
        let mut size = 0;
        while let Some(v) = queue.pop_front() {
            if since_relabel == period {
                since_relabel = 0;
                level.fill(inf);
                seen.fill(false);
                for (u, mate) in self.left_mate.iter().enumerate() {
                    if mate.is_none() {
                        level[u] = 0;
                        bfs.push_back(u);
                    }
                }
                while let Some(u) = bfs.pop_front() {
                    for &w in &self.left_adj[u] {
                        let w = w as usize;
                        if !seen[w] {
                            seen[w] = true;
                            if let Some(x) = self.right_mate[w] {
                                level[x] = level[u] + 2;
                                bfs.push_back(x);
                            }
                        }
                    }
                }
            }
            since_relabel += 1;
            let Some(&u) = self.right_adj[v].iter().min_by_key(|&&u| level[u as usize]) else {
                continue;
            };
            let u = u as usize;
            if level[u] >= inf {
                continue;
            }
            match self.left_mate[u].replace(v) {
                Some(w) => {
                    self.right_mate[w] = None;
                    queue.push_back(w);
                }
                None => size += 1,
            }
            self.right_mate[v] = Some(u);
            level[u] += 2;
        }
        self.size = Some(size);
        size
    }
}

#[cfg(test)]
mod tests {
    use super::BipartiteMatching;
    use crate::graph::edges::flow_edge::FlowEdge;
    use crate::graph::max_flow::MaxFlow;
    use crate::graph::Graph;
    use crate::misc::random::{Random, RandomTrait};

    fn check(left: usize, right: usize, edges: &[(usize, usize)]) -> usize {
        let mut bm = BipartiteMatching::new(left, right);
        for &(u, v) in edges {
            bm.add_edge(u, v);
        }
        let size = bm.run();
        assert_eq!(bm.run(), size);
        let mut count = 0;
        for (u, &m) in bm.left_mate().iter().enumerate() {
            if let Some(v) = m {
                assert!(edges.contains(&(u, v)), "({u}, {v}) is not an edge");
                assert_eq!(bm.right_mate()[v], Some(u));
                count += 1;
            }
        }
        for (v, &m) in bm.right_mate().iter().enumerate() {
            if let Some(u) = m {
                assert_eq!(bm.left_mate()[u], Some(v));
            }
        }
        assert_eq!(count, size);
        let (s, t) = (left + right, left + right + 1);
        let mut g = Graph::new_linked(left + right + 2);
        for i in 0..left {
            g.add_edge(FlowEdge::new(s, i, 1i32));
        }
        for j in 0..right {
            g.add_edge(FlowEdge::new(left + j, t, 1i32));
        }
        for &(u, v) in edges {
            g.add_edge(FlowEdge::new(u, left + v, 1i32));
        }
        assert_eq!(size as i32, g.max_flow(s, t));
        size
    }

    #[test]
    fn edge_cases() {
        assert_eq!(check(0, 0, &[]), 0);
        assert_eq!(check(3, 0, &[]), 0);
        assert_eq!(check(0, 3, &[]), 0);
        assert_eq!(check(3, 3, &[]), 0);
        assert_eq!(check(1, 1, &[(0, 0), (0, 0), (0, 0)]), 1);
        assert_eq!(check(2, 2, &[(0, 0), (1, 0)]), 1);
        assert_eq!(check(2, 2, &[(0, 0), (0, 1), (1, 0)]), 2);
        assert_eq!(check(3, 1, &[(0, 0), (1, 0), (2, 0)]), 1);
        assert_eq!(check(1, 3, &[(0, 0), (0, 1), (0, 2)]), 1);
    }

    #[test]
    fn complete_and_chains() {
        let complete: Vec<_> = (0..7).flat_map(|u| (0..5).map(move |v| (u, v))).collect();
        assert_eq!(check(7, 5, &complete), 5);
        // path 0-0-1-1-2-2-...: perfect matching exists only through the chain
        let n = 50;
        let mut chain = Vec::new();
        for i in 0..n {
            chain.push((i, i));
            if i + 1 < n {
                chain.push((i + 1, i));
            }
        }
        assert_eq!(check(n, n, &chain), n);
        // hard case for greedy: every left i is adjacent to right i and right 0
        let mut star = Vec::new();
        for i in 0..n {
            star.push((i, 0));
            star.push((i, i));
        }
        assert_eq!(check(n, n, &star), n);
    }

    #[test]
    fn random_against_max_flow() {
        let mut rng = Random::new_with_seed(11);
        for _ in 0..300 {
            let left = rng.gen_range(1..=12usize);
            let right = rng.gen_range(1..=12usize);
            let m = rng.gen_range(0..=left * right);
            let edges: Vec<_> = (0..m)
                .map(|_| (rng.gen_range(0..left), rng.gen_range(0..right)))
                .collect();
            check(left, right, &edges);
        }
        for _ in 0..20 {
            let left = rng.gen_range(50..=200usize);
            let right = rng.gen_range(50..=200usize);
            let m = rng.gen_range(0..=4 * (left + right));
            let edges: Vec<_> = (0..m)
                .map(|_| (rng.gen_range(0..left), rng.gen_range(0..right)))
                .collect();
            check(left, right, &edges);
        }
    }

    #[test]
    fn requeue_after_add_edge() {
        let mut bm = BipartiteMatching::new(2, 2);
        bm.add_edge(0, 0);
        assert_eq!(bm.run(), 1);
        bm.add_edge(1, 1);
        assert_eq!(bm.run(), 2);
    }
}
