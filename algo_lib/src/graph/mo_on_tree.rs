use crate::graph::edges::edge_trait::BidirectionalEdgeTrait;
use crate::graph::lca::LCATrait;
use crate::graph::Graph;

pub trait MoOnTreeWorker {
    type T;
    type R;
    fn empty() -> Self;
    fn add(&mut self, val: &Self::T);
    fn remove(&mut self, val: &Self::T);
    fn result(&self) -> Self::R;
}

// Path queries (u, v), both endpoints inclusive, on an unweighted tree.
// Flattens the tree into a double Euler tour where a vertex is on the path
// iff it occurs an odd number of times in the query range, so the window
// toggles vertices; the LCA is added around result() when needed.
pub fn mo_on_tree<W: MoOnTreeWorker, E: BidirectionalEdgeTrait>(
    graph: &Graph<E>,
    values: &[W::T],
    queries: &[(usize, usize)],
) -> Vec<W::R> {
    debug_assert!(graph.is_tree());
    let n = graph.vertex_count();
    assert_eq!(values.len(), n);
    if queries.is_empty() {
        return Vec::new();
    }
    let mut tour = Vec::with_capacity(2 * n);
    let mut tin = vec![0; n];
    let mut tout = vec![0; n];
    let mut visited = vec![false; n];
    let mut stack = vec![(0usize, false)];
    visited[0] = true;
    while let Some((v, exit)) = stack.pop() {
        if exit {
            tout[v] = tour.len();
            tour.push(v);
            continue;
        }
        tin[v] = tour.len();
        tour.push(v);
        stack.push((v, true));
        for e in graph.adj(v).iter() {
            let to = e.to();
            if !visited[to] {
                visited[to] = true;
                stack.push((to, false));
            }
        }
    }
    let lca = graph.lca();

    // (from, to) inclusive in the tour, plus the lca when it is not inside.
    let ranges: Vec<(usize, usize, Option<usize>)> = queries
        .iter()
        .map(|&(u, v)| {
            let (a, b) = if tin[u] <= tin[v] { (u, v) } else { (v, u) };
            if tin[b] < tout[a] {
                (tin[a], tin[b], None)
            } else {
                (tout[a], tin[b], Some(lca.lca(a, b)))
            }
        })
        .collect();

    let block = (((2 * n) as f64 / (queries.len() as f64).sqrt()).ceil() as usize).max(1);
    let mut order: Vec<usize> = (0..queries.len()).collect();
    order.sort_by_key(|&i| (ranges[i].0 / block, ranges[i].1));

    let mut res: Vec<Option<W::R>> = (0..queries.len()).map(|_| None).collect();
    let mut worker = W::empty();
    let mut active = vec![false; n];
    let mut cur_block = usize::MAX;
    // Current range is [cur_left, cur_right), half-open.
    let mut cur_left = 0;
    let mut cur_right = 0;
    for i in order {
        let (from, to, extra) = ranges[i];
        if from / block != cur_block {
            cur_block = from / block;
            worker = W::empty();
            active.fill(false);
            cur_left = from;
            cur_right = from;
        }
        let mut toggle = |worker: &mut W, at: usize| {
            let v = tour[at];
            if active[v] {
                worker.remove(&values[v]);
            } else {
                worker.add(&values[v]);
            }
            active[v] = !active[v];
        };
        while cur_right <= to {
            toggle(&mut worker, cur_right);
            cur_right += 1;
        }
        while cur_left > from {
            cur_left -= 1;
            toggle(&mut worker, cur_left);
        }
        while cur_left < from {
            toggle(&mut worker, cur_left);
            cur_left += 1;
        }
        res[i] = Some(match extra {
            None => worker.result(),
            Some(x) => {
                worker.add(&values[x]);
                let r = worker.result();
                worker.remove(&values[x]);
                r
            }
        });
    }
    res.into_iter().map(Option::unwrap).collect()
}

#[cfg(test)]
mod tree_test {
    use super::{mo_on_tree, MoOnTreeWorker};
    use crate::graph::Graph;
    use crate::misc::random::{Random, RandomTrait};

    struct TreeDistinct {
        count: [u32; 8],
        distinct: u32,
        sum: u64,
    }

    impl MoOnTreeWorker for TreeDistinct {
        type T = u32;
        type R = (u32, u64);

        fn empty() -> Self {
            Self {
                count: [0; 8],
                distinct: 0,
                sum: 0,
            }
        }

        fn add(&mut self, val: &u32) {
            self.count[*val as usize] += 1;
            if self.count[*val as usize] == 1 {
                self.distinct += 1;
            }
            self.sum += *val as u64;
        }

        fn remove(&mut self, val: &u32) {
            self.count[*val as usize] -= 1;
            if self.count[*val as usize] == 0 {
                self.distinct -= 1;
            }
            self.sum -= *val as u64;
        }

        fn result(&self) -> (u32, u64) {
            (self.distinct, self.sum)
        }
    }

    fn path(u: usize, v: usize, parent: &[usize], depth: &[usize]) -> Vec<usize> {
        let (mut u, mut v) = (u, v);
        let mut left = Vec::new();
        let mut right = Vec::new();
        while u != v {
            if depth[u] >= depth[v] {
                left.push(u);
                u = parent[u];
            } else {
                right.push(v);
                v = parent[v];
            }
        }
        left.push(u);
        left.extend(right.into_iter().rev());
        left
    }

    #[test]
    fn paths_match_brute_force() {
        let mut rng = Random::new_with_seed(43);
        for _ in 0..100 {
            let n = 1 + rng.gen_bound(40usize);
            let mut parent = vec![0; n];
            let mut depth = vec![0; n];
            let mut edges = Vec::new();
            for v in 1..n {
                parent[v] = rng.gen_bound(v);
                depth[v] = depth[parent[v]] + 1;
                edges.push((parent[v], v));
            }
            let graph = Graph::with_biedges(n, &edges);
            let values: Vec<u32> = (0..n).map(|_| rng.gen_bound(8u32)).collect();
            let q = 1 + rng.gen_bound(50usize);
            let queries: Vec<(usize, usize)> = (0..q)
                .map(|_| (rng.gen_bound(n), rng.gen_bound(n)))
                .collect();
            let answers = mo_on_tree::<TreeDistinct, _>(&graph, &values, &queries);
            for (&(u, v), answer) in queries.iter().zip(answers.iter()) {
                let mut expected = TreeDistinct::empty();
                for w in path(u, v, &parent, &depth) {
                    expected.add(&values[w]);
                }
                assert_eq!(*answer, expected.result(), "path {u} - {v}, n={n}");
            }
        }
    }

    #[test]
    fn single_vertex_tree() {
        let graph = Graph::with_biedges(1, &[]);
        let answers = mo_on_tree::<TreeDistinct, _>(&graph, &[5u32], &[(0, 0), (0, 0)]);
        assert_eq!(answers, vec![(1, 5), (1, 5)]);
    }
}
