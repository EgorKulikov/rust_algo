use crate::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use crate::graph::edges::edge_trait::BidirectionalEdgeTrait;
use crate::graph::hl_decomposition::{HLDecomposition, HLDecompositionTrait, HLPart};
use crate::graph::Graph;
use std::ops::RangeInclusive;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PathDirection {
    Up,
    Down,
}

/// Segment tree over the paths of a tree, built on heavy-light decomposition.
///
/// With `include_lca == true` nodes represent vertices and a path range
/// `a..=b` covers every vertex on the path, lca included once. With
/// `include_lca == false` the node of vertex `v` represents the edge
/// `(v, parent(v))`, the lca is excluded from every range (so `a..=b` covers
/// exactly the path's edges), and the root's slot stays unused.
pub struct PathSegmentTree<Node> {
    hld: HLDecomposition,
    trees: Vec<SegmentTree<Node>>,
    include_lca: bool,
}

impl<Node: SegmentTreeNode> PathSegmentTree<Node> {
    pub fn new<E: BidirectionalEdgeTrait>(graph: &Graph<E>, include_lca: bool) -> Self {
        Self::new_with_root(graph, 0, include_lca)
    }

    pub fn new_with_root<E: BidirectionalEdgeTrait>(
        graph: &Graph<E>,
        root: usize,
        include_lca: bool,
    ) -> Self {
        Self::with_gen_with_root(graph, root, include_lca, |_| Node::default())
    }

    pub fn with_gen<E: BidirectionalEdgeTrait>(
        graph: &Graph<E>,
        include_lca: bool,
        g: impl FnMut(usize) -> Node,
    ) -> Self {
        Self::with_gen_with_root(graph, 0, include_lca, g)
    }

    pub fn with_gen_with_root<E: BidirectionalEdgeTrait>(
        graph: &Graph<E>,
        root: usize,
        include_lca: bool,
        mut g: impl FnMut(usize) -> Node,
    ) -> Self {
        let hld = graph.hl_decomposition_with_root(root);
        let trees = hld
            .paths
            .iter()
            .map(|path| SegmentTree::with_gen(path.len(), |i| g(path[i])))
            .collect();
        Self {
            hld,
            trees,
            include_lca,
        }
    }

    pub fn hld(&self) -> &HLDecomposition {
        &self.hld
    }

    pub fn lca(&self, u: usize, v: usize) -> usize {
        self.hld.lca(u, v)
    }

    pub fn point_query(&mut self, vertex: usize) -> &Node {
        self.trees[self.hld.id[vertex]].point_query(self.hld.pos[vertex])
    }

    pub fn point_update(&mut self, vertex: usize, val: Node) {
        self.trees[self.hld.id[vertex]].point_update(self.hld.pos[vertex], val);
    }

    pub fn point_through_update(&mut self, vertex: usize, f: impl FnMut(&mut Node)) {
        self.trees[self.hld.id[vertex]].point_through_update(self.hld.pos[vertex], f);
    }

    // (up, down): up parts in a→lca travel order, down parts in lca→b travel
    // order. Within any part, positions are ancestor-first, so up parts are
    // reversed relative to travel — callers compensate (Up flag, RTL search,
    // join-on-the-left + swap).
    fn parts(&self, range: RangeInclusive<usize>) -> (Vec<HLPart>, Vec<HLPart>) {
        let (a, b) = range.into_inner();
        let l = self.hld.lca(a, b);
        let up = if self.include_lca {
            self.hld.iter(a..=l).collect()
        } else {
            self.hld.iter(a..l).collect()
        };
        let mut down: Vec<HLPart> = self.hld.iter(b..l).collect();
        down.reverse();
        (up, down)
    }

    pub fn for_each<R: Default>(
        &mut self,
        range: RangeInclusive<usize>,
        f: impl FnMut(R, &Node, PathDirection) -> R,
    ) -> R {
        self.for_each_with_init(range, f, R::default())
    }

    /// Folds over the maximal covered segment-tree nodes of the path in a→b
    /// part order. Up-leg nodes (a→lca) come with `PathDirection::Up`; within
    /// that leg nodes are enumerated ancestor-first, i.e. opposite to travel.
    /// Down-leg nodes (lca→b) come with `PathDirection::Down` in travel order.
    pub fn for_each_with_init<R>(
        &mut self,
        range: RangeInclusive<usize>,
        mut f: impl FnMut(R, &Node, PathDirection) -> R,
        init: R,
    ) -> R {
        let (up, down) = self.parts(range);
        let mut res = init;
        for part in up {
            res = self.trees[part.id].for_each_with_init(
                part.pos_from..=part.pos_to,
                |r, node| f(r, node, PathDirection::Up),
                res,
            );
        }
        for part in down {
            res = self.trees[part.id].for_each_with_init(
                part.pos_from..=part.pos_to,
                |r, node| f(r, node, PathDirection::Down),
                res,
            );
        }
        res
    }

    pub fn for_each_mut<R: Default>(
        &mut self,
        range: RangeInclusive<usize>,
        f: impl FnMut(R, &mut Node, PathDirection) -> R,
    ) -> R {
        self.for_each_mut_with_init(range, f, R::default())
    }

    /// See [`Self::for_each_with_init`] for ordering and direction semantics.
    pub fn for_each_mut_with_init<R>(
        &mut self,
        range: RangeInclusive<usize>,
        mut f: impl FnMut(R, &mut Node, PathDirection) -> R,
        init: R,
    ) -> R {
        let (up, down) = self.parts(range);
        let mut res = init;
        for part in up {
            res = self.trees[part.id].for_each_mut_with_init(
                part.pos_from..=part.pos_to,
                |r, node| f(r, node, PathDirection::Up),
                res,
            );
        }
        for part in down {
            res = self.trees[part.id].for_each_mut_with_init(
                part.pos_from..=part.pos_to,
                |r, node| f(r, node, PathDirection::Down),
                res,
            );
        }
        res
    }

    /// `val` is oriented a→b. Down-leg parts (orientation aligned with
    /// travel) receive it as is; then it is swapped in place and applied to
    /// up-leg parts (orientation opposite to travel). Per-leaf accumulation
    /// order across parts does not matter.
    pub fn update(&mut self, range: RangeInclusive<usize>, mut val: Node) {
        let (up, down) = self.parts(range);
        for part in down {
            self.trees[part.id].update(part.pos_from..=part.pos_to, &val);
        }
        val.swap();
        for part in up {
            self.trees[part.id].update(part.pos_from..=part.pos_to, &val);
        }
    }

    /// Searches for the first leaf in a→b travel order satisfying `accept`.
    ///
    /// Leaves are considered in a→b travel order (up-leg from `a` to lca,
    /// then down-leg from lca to `b`). `accept` follows the same stateful
    /// contract as `SegmentTree::binary_search_in`: return `true` to descend
    /// into a subtree, mutate your accumulator and return `false` to skip it.
    /// When the leaf is found `calc` receives the node and its **vertex id**
    /// and its return value is wrapped in `Some`. Returns `None` if no leaf
    /// satisfies the predicate.
    pub fn binary_search<R>(
        &mut self,
        range: RangeInclusive<usize>,
        mut accept: impl FnMut(&Node) -> bool,
        calc: impl FnOnce(&Node, usize) -> R,
    ) -> Option<R> {
        let (up, down) = self.parts(range);
        let mut calc = Some(calc);
        let hld = &self.hld;
        for part in up {
            if let Some(res) = self.trees[part.id].binary_search_in_rtl(
                part.pos_from..=part.pos_to,
                &mut accept,
                |node, pos| (calc.take().unwrap())(node, hld.paths[part.id][pos]),
            ) {
                return Some(res);
            }
        }
        for part in down {
            if let Some(res) = self.trees[part.id].binary_search_in(
                part.pos_from..=part.pos_to,
                &mut accept,
                |node, pos| (calc.take().unwrap())(node, hld.paths[part.id][pos]),
            ) {
                return Some(res);
            }
        }
        None
    }

    /// Searches for the first leaf in a→b travel order satisfying `accept`,
    /// then calls `calc` with a mutable reference to the leaf node.
    ///
    /// Leaves are considered in a→b travel order (up-leg from `a` to lca,
    /// then down-leg from lca to `b`). `accept` follows the same stateful
    /// contract as `SegmentTree::binary_search_in`: return `true` to descend
    /// into a subtree, mutate your accumulator and return `false` to skip it.
    /// When the leaf is found `calc` receives a mutable reference to the node
    /// and its **vertex id**; the segment tree is updated on the way back.
    /// Returns `None` if no leaf satisfies the predicate.
    pub fn binary_search_mut<R>(
        &mut self,
        range: RangeInclusive<usize>,
        mut accept: impl FnMut(&Node) -> bool,
        calc: impl FnOnce(&mut Node, usize) -> R,
    ) -> Option<R> {
        let (up, down) = self.parts(range);
        let mut calc = Some(calc);
        let hld = &self.hld;
        for part in up {
            if let Some(res) = self.trees[part.id].binary_search_in_mut_rtl(
                part.pos_from..=part.pos_to,
                &mut accept,
                |node, pos| (calc.take().unwrap())(node, hld.paths[part.id][pos]),
            ) {
                return Some(res);
            }
        }
        for part in down {
            if let Some(res) = self.trees[part.id].binary_search_in_mut(
                part.pos_from..=part.pos_to,
                &mut accept,
                |node, pos| (calc.take().unwrap())(node, hld.paths[part.id][pos]),
            ) {
                return Some(res);
            }
        }
        None
    }

    /// Joined node of the path, oriented a→b. `Node::default()` must be a
    /// join identity — it is joined into every result, not only empty paths.
    pub fn query(&mut self, range: RangeInclusive<usize>) -> Node
    where
        Node: Clone,
    {
        let (up, down) = self.parts(range);
        let mut res = Node::default();
        for part in up {
            res = Node::join(
                &self.trees[part.id].query(part.pos_from..=part.pos_to),
                &res,
            );
        }
        res.swap();
        for part in down {
            res = Node::join(
                &res,
                &self.trees[part.id].query(part.pos_from..=part.pos_to),
            );
        }
        res
    }
}

pub trait PathSegmentTreeTrait {
    fn path_segment_tree_with_gen_with_root<Node: SegmentTreeNode>(
        &self,
        root: usize,
        include_lca: bool,
        g: impl FnMut(usize) -> Node,
    ) -> PathSegmentTree<Node>;

    fn path_segment_tree_with_gen<Node: SegmentTreeNode>(
        &self,
        include_lca: bool,
        g: impl FnMut(usize) -> Node,
    ) -> PathSegmentTree<Node> {
        self.path_segment_tree_with_gen_with_root(0, include_lca, g)
    }

    fn path_segment_tree_with_root<Node: SegmentTreeNode>(
        &self,
        root: usize,
        include_lca: bool,
    ) -> PathSegmentTree<Node> {
        self.path_segment_tree_with_gen_with_root(root, include_lca, |_| Node::default())
    }

    fn path_segment_tree<Node: SegmentTreeNode>(&self, include_lca: bool) -> PathSegmentTree<Node> {
        self.path_segment_tree_with_gen_with_root(0, include_lca, |_| Node::default())
    }
}

impl<E: BidirectionalEdgeTrait> PathSegmentTreeTrait for Graph<E> {
    fn path_segment_tree_with_gen_with_root<Node: SegmentTreeNode>(
        &self,
        root: usize,
        include_lca: bool,
        g: impl FnMut(usize) -> Node,
    ) -> PathSegmentTree<Node> {
        PathSegmentTree::with_gen_with_root(self, root, include_lca, g)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::graph::edges::bi_edge::BiEdge;

    //        0
    //       / \
    //      1   2
    //     / \   \
    //    3   4   5
    //   /       / \
    //  6       7   8
    const EDGES: [(usize, usize); 8] = [
        (0, 1),
        (0, 2),
        (1, 3),
        (1, 4),
        (3, 6),
        (2, 5),
        (5, 7),
        (5, 8),
    ];
    const N: usize = 9;

    fn graph() -> Graph<BiEdge<()>> {
        Graph::with_biedges(N, &EDGES)
    }

    // parent/level computed by BFS, independent of the HLD under test
    fn naive_parents() -> (Vec<usize>, Vec<usize>) {
        let mut adj = vec![Vec::new(); N];
        for &(u, v) in EDGES.iter() {
            adj[u].push(v);
            adj[v].push(u);
        }
        let mut parent = vec![usize::MAX; N];
        let mut level = vec![0; N];
        let mut queue = vec![0];
        parent[0] = 0;
        let mut i = 0;
        while i < queue.len() {
            let v = queue[i];
            i += 1;
            for &u in adj[v].iter() {
                if parent[u] == usize::MAX {
                    parent[u] = v;
                    level[u] = level[v] + 1;
                    queue.push(u);
                }
            }
        }
        (parent, level)
    }

    // vertices of the a..=b path, in order from a to b
    fn naive_path(a: usize, b: usize) -> Vec<usize> {
        let (parent, level) = naive_parents();
        let mut left = vec![a];
        let mut right = vec![b];
        let (mut x, mut y) = (a, b);
        while level[x] > level[y] {
            x = parent[x];
            left.push(x);
        }
        while level[y] > level[x] {
            y = parent[y];
            right.push(y);
        }
        while x != y {
            x = parent[x];
            left.push(x);
            y = parent[y];
            right.push(y);
        }
        right.pop();
        right.reverse();
        left.extend(right);
        left
    }

    fn naive_lca(a: usize, b: usize) -> usize {
        let (_, level) = naive_parents();
        *naive_path(a, b).iter().min_by_key(|&&v| level[v]).unwrap()
    }

    #[derive(Clone, Default)]
    struct SumNode {
        sum: i64,
    }

    impl SegmentTreeNode for SumNode {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.sum = left_val.sum + right_val.sum;
        }
    }

    #[derive(Clone, Default)]
    struct SeqNode {
        seq: Vec<usize>,
    }

    impl SegmentTreeNode for SeqNode {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.seq.clear();
            self.seq.extend_from_slice(&left_val.seq);
            self.seq.extend_from_slice(&right_val.seq);
        }

        fn swap(&mut self) {
            self.seq.reverse();
        }
    }

    #[test]
    fn query_sum_all_pairs() {
        let g = graph();
        let mut tree =
            g.path_segment_tree_with_gen(true, |v| SumNode { sum: 1 + (v as i64) * 10 });
        for a in 0..N {
            for b in 0..N {
                let expected: i64 = naive_path(a, b).iter().map(|&v| 1 + (v as i64) * 10).sum();
                assert_eq!(tree.query(a..=b).sum, expected, "path {}..={}", a, b);
            }
        }
    }

    #[test]
    fn query_orientation_all_pairs() {
        let g = graph();
        let mut tree = g.path_segment_tree_with_gen(true, |v| SeqNode { seq: vec![v] });
        for a in 0..N {
            for b in 0..N {
                assert_eq!(tree.query(a..=b).seq, naive_path(a, b), "path {}..={}", a, b);
            }
        }
    }

    #[test]
    fn query_edge_mode_all_pairs() {
        let g = graph();
        let mut tree = g.path_segment_tree_with_gen(false, |v| SeqNode { seq: vec![v] });
        for a in 0..N {
            for b in 0..N {
                // each non-lca vertex stands for the edge to its parent
                let lca = naive_lca(a, b);
                let expected: Vec<usize> = naive_path(a, b)
                    .into_iter()
                    .filter(|&v| v != lca)
                    .collect();
                assert_eq!(tree.query(a..=b).seq, expected, "path {}..={}", a, b);
            }
        }
    }

    #[test]
    fn point_ops() {
        let g = graph();
        let mut tree = g.path_segment_tree_with_gen(true, |v| SumNode { sum: v as i64 });
        for v in 0..N {
            assert_eq!(tree.point_query(v).sum, v as i64);
        }
        tree.point_update(4, SumNode { sum: 100 });
        assert_eq!(tree.point_query(4).sum, 100);
        tree.point_through_update(4, |node| node.sum += 1);
        assert_eq!(tree.point_query(4).sum, 101);
        assert_eq!(tree.point_query(3).sum, 3);
    }

    #[test]
    fn for_each_directions() {
        let g = graph();
        let mut tree =
            g.path_segment_tree_with_gen(true, |v| SumNode { sum: 1 + (v as i64) * 10 });
        let val = |v: usize| 1 + (v as i64) * 10;
        for a in 0..N {
            for b in 0..N {
                let path = naive_path(a, b);
                let lca = naive_lca(a, b);
                let lca_idx = path.iter().position(|&v| v == lca).unwrap();
                let up_expected: i64 = path[..=lca_idx].iter().map(|&v| val(v)).sum();
                let down_expected: i64 = path[lca_idx + 1..].iter().map(|&v| val(v)).sum();
                let (up, down) = tree.for_each(a..=b, |acc: (i64, i64), node, dir| match dir {
                    PathDirection::Up => (acc.0 + node.sum, acc.1),
                    PathDirection::Down => (acc.0, acc.1 + node.sum),
                });
                assert_eq!((up, down), (up_expected, down_expected), "path {}..={}", a, b);
            }
        }
    }

    #[test]
    fn for_each_mut_matches_for_each() {
        let g = graph();
        let mut tree = g.path_segment_tree_with_gen(true, |v| SumNode { sum: v as i64 });
        for a in 0..N {
            for b in 0..N {
                let immut: i64 = tree.for_each(a..=b, |acc: i64, node, _| acc + node.sum);
                let muts: i64 = tree.for_each_mut(a..=b, |acc: i64, node, _| acc + node.sum);
                assert_eq!(immut, muts, "path {}..={}", a, b);
            }
        }
    }

    #[derive(Clone, Default)]
    struct DirNode {
        fwd: i64,
        bwd: i64,
    }

    impl SegmentTreeNode for DirNode {
        fn accumulate(&mut self, value: &Self) {
            self.fwd += value.fwd;
            self.bwd += value.bwd;
        }

        fn reset_delta(&mut self) {
            self.fwd = 0;
            self.bwd = 0;
        }

        fn swap(&mut self) {
            std::mem::swap(&mut self.fwd, &mut self.bwd);
        }
    }

    #[test]
    fn update_swaps_on_up_leg() {
        let g = graph();
        for a in 0..N {
            for b in 0..N {
                let mut tree: PathSegmentTree<DirNode> = g.path_segment_tree(true);
                tree.update(a..=b, DirNode { fwd: 1, bwd: 100 });
                let path = naive_path(a, b);
                let lca = naive_lca(a, b);
                let lca_idx = path.iter().position(|&v| v == lca).unwrap();
                for v in 0..N {
                    let expected = match path.iter().position(|&u| u == v) {
                        None => (0, 0),
                        Some(i) if i <= lca_idx => (100, 1), // up-leg: swapped
                        Some(_) => (1, 100),                 // down-leg: as given
                    };
                    let node = tree.point_query(v);
                    assert_eq!(
                        (node.fwd, node.bwd),
                        expected,
                        "path {}..={}, vertex {}",
                        a, b, v
                    );
                }
            }
        }
    }

    #[test]
    fn update_edge_mode() {
        let g = graph();
        for a in 0..N {
            for b in 0..N {
                let mut tree: PathSegmentTree<DirNode> = g.path_segment_tree(false);
                tree.update(a..=b, DirNode { fwd: 1, bwd: 100 });
                let path = naive_path(a, b);
                let lca = naive_lca(a, b);
                let lca_idx = path.iter().position(|&v| v == lca).unwrap();
                for v in 0..N {
                    let expected = match path.iter().position(|&u| u == v) {
                        Some(i) if i < lca_idx => (100, 1), // up-leg edge: swapped
                        Some(i) if i > lca_idx => (1, 100), // down-leg edge: as given
                        _ => (0, 0),                        // off path or the lca itself
                    };
                    let node = tree.point_query(v);
                    assert_eq!(
                        (node.fwd, node.bwd),
                        expected,
                        "path {}..={}, vertex {}",
                        a, b, v
                    );
                }
            }
        }
    }

    #[test]
    fn binary_search_kth_vertex() {
        let g = graph();
        let mut tree = g.path_segment_tree_with_gen(true, |_| SumNode { sum: 1 });
        for a in 0..N {
            for b in 0..N {
                let path = naive_path(a, b);
                for k in 1..=(path.len() + 1) {
                    let mut cnt = 0i64;
                    let res = tree.binary_search(
                        a..=b,
                        |node| {
                            if cnt + node.sum >= k as i64 {
                                true
                            } else {
                                cnt += node.sum;
                                false
                            }
                        },
                        |_, vertex| vertex,
                    );
                    let expected = path.get(k - 1).copied();
                    assert_eq!(res, expected, "path {}..={}, k {}", a, b, k);
                }
            }
        }
    }

    #[test]
    fn binary_search_mut_kth_vertex() {
        let g = graph();
        let mut tree = g.path_segment_tree_with_gen(true, |_| SumNode { sum: 1 });
        let path = naive_path(6, 8);
        assert_eq!(path, vec![6, 3, 1, 0, 2, 5, 8]);
        let k = 5; // path[4] == 2, first vertex of the down-leg
        let mut cnt = 0i64;
        let res = tree.binary_search_mut(
            6..=8,
            |node| {
                if cnt + node.sum >= k {
                    true
                } else {
                    cnt += node.sum;
                    false
                }
            },
            |node, vertex| {
                node.sum = 10;
                vertex
            },
        );
        assert_eq!(res, Some(2));
        assert_eq!(tree.point_query(2).sum, 10);
        assert_eq!(tree.query(6..=8).sum, 16);

        // hit on the up-leg, exercising the mut RTL descent
        let k = 2; // path[1] == 3
        let mut cnt = 0i64;
        let res = tree.binary_search_mut(
            6..=8,
            |node| {
                if cnt + node.sum >= k {
                    true
                } else {
                    cnt += node.sum;
                    false
                }
            },
            |node, vertex| {
                node.sum = 20;
                vertex
            },
        );
        assert_eq!(res, Some(3));
        assert_eq!(tree.point_query(3).sum, 20);
        assert_eq!(tree.query(6..=8).sum, 35);
    }

    #[test]
    fn binary_search_edge_mode_empty() {
        let g = graph();
        let mut tree = g.path_segment_tree_with_gen(false, |_| SumNode { sum: 1 });
        for a in 0..N {
            let res = tree.binary_search(a..=a, |_| true, |_, vertex| vertex);
            assert_eq!(res, None, "vertex {}", a);
        }
    }
}
