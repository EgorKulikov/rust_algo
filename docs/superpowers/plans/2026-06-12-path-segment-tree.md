# PathSegmentTree Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** A segment-tree-like structure over tree paths (`a..=b` between any two vertices), built on heavy-light decomposition, supporting point/range updates, `for_each(_mut)` with an Up/Down direction flag, oriented `query`, and ranged `binary_search`.

**Architecture:** One `SegmentTree<Node>` per heavy path of an `HLDecomposition`. Every path operation decomposes `a..=b` at the lca into an up-leg (`a→lca`, parts reversed relative to seg-tree order) and a down-leg (`lca→b`, parts aligned). A new `SegmentTreeNode::swap` hook reverses a node's left↔right orientation, letting `query` and `update` handle non-commutative semantics. `HLDecomposition` gains its own O(log n) `lca` so no sparse-table `LCA` dependency is needed.

**Tech Stack:** Rust, `algo_lib` workspace crate. No external deps. Tests are plain `#[cfg(test)]` modules; run with `cargo test -p algo_lib <filter>`.

**Spec:** `docs/superpowers/specs/2026-06-12-path-segment-tree-design.md`

---

## File map

- Modify `algo_lib/src/collections/segment_tree.rs` — add `swap` to the `SegmentTreeNode` trait (Task 1).
- Modify `algo_lib/src/graph/hl_decomposition.rs` — add `level` field + `lca` method + tests (Task 2).
- Create `algo_lib/src/graph/path_segment_tree.rs` — the new structure + all its tests (Tasks 3–7).
- Modify `algo_lib/src/graph/mod.rs` — register the new module (Task 3).

Background you need:

- `SegmentTree` (in `collections/segment_tree.rs`) is bottom-up-allocated; all read APIs take `&mut self` (lazy push-down). Key methods used here: `point_query`, `point_update`, `point_through_update`, `query(range) -> Node` (needs `Node: Clone`), `for_each_with_init`, `for_each_mut_with_init`, `update(range, &Node)` (accumulate), `binary_search_in`, `binary_search_in_rtl`, `binary_search_in_mut`, `binary_search_in_mut_rtl`. All range methods accept `impl RangeBounds<usize>`.
- `HLDecomposition` (in `graph/hl_decomposition.rs`) has `pub paths: Vec<Vec<usize>>` (each heavy path, ancestor first), `pub id: Vec<usize>` (path id per vertex), `pub pos: Vec<usize>` (position within path), `pub parent: Vec<usize>` (`parent[root] == root`), `pub root`. `hld.iter(range)` walks from a source vertex up to an ancestor destination, yielding `HLPart { id, pos_from, pos_to }` per heavy path touched, source-side part first; `iter(a..=l)` includes the destination `l`, `iter(a..l)` excludes it. Within a part, `pos_from` is the ancestor side.

---

### Task 1: `SegmentTreeNode::swap`

**Files:**
- Modify: `algo_lib/src/collections/segment_tree.rs` (trait at top of file)

- [ ] **Step 1: Add the trait method**

In the `SegmentTreeNode` trait, after `reset_delta`, add:

```rust
    fn swap(&mut self) {}
```

The trait becomes:

```rust
#[allow(unused_variables)]
pub trait SegmentTreeNode: Default {
    fn join(left: &Self, right: &Self) -> Self {
        let mut node = Self::default();
        node.update(left, right);
        node
    }
    fn update(&mut self, left_val: &Self, right_val: &Self) {}
    fn accumulate(&mut self, value: &Self) {}
    fn reset_delta(&mut self) {}
    fn swap(&mut self) {}
}
```

Semantics (worth a doc comment on the method): reverse the node's left↔right orientation — a node aggregating a directed segment flips which end is "left". Default is a no-op (correct for symmetric aggregates).

- [ ] **Step 2: Verify everything still compiles and passes**

Run: `cargo test -p algo_lib collections::segment_tree`
Expected: all existing segment tree tests PASS (default method, no impl changes needed).

- [ ] **Step 3: Commit**

```bash
git add algo_lib/src/collections/segment_tree.rs
git commit -m "(feature) swap on SegmentTreeNode"
```

---

### Task 2: `HLDecomposition::lca` + `level`

**Files:**
- Modify: `algo_lib/src/graph/hl_decomposition.rs`

- [ ] **Step 1: Write the failing test**

`hl_decomposition.rs` has no test module yet. Add at the end of the file:

```rust
#[cfg(test)]
mod test {
    use super::*;
    use crate::graph::lca::LCATrait;

    #[test]
    fn lca_matches_lca_struct() {
        // deterministic pseudo-random tree (no rand dependency)
        let n = 50;
        let mut seed = 12345u64;
        let mut edges = Vec::new();
        for v in 1..n {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let parent = (seed >> 33) as usize % v;
            edges.push((parent, v));
        }
        let graph = Graph::with_biedges(n, &edges);
        let hld = graph.hl_decomposition();
        let lca = graph.lca();
        for u in 0..n {
            assert_eq!(hld.level[u], lca.level(u), "level of {}", u);
            for v in 0..n {
                assert_eq!(hld.lca(u, v), lca.lca(u, v), "lca of {} and {}", u, v);
            }
        }
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p algo_lib graph::hl_decomposition`
Expected: COMPILE ERROR — no field `level`, no method `lca` on `HLDecomposition`.

- [ ] **Step 3: Implement `level` and `lca`**

Add the field to the struct:

```rust
pub struct HLDecomposition {
    pub paths: Vec<Vec<usize>>,
    pub id: Vec<usize>,
    pub pos: Vec<usize>,
    pub parent: Vec<usize>,
    pub level: Vec<usize>,
    pub root: usize,
}
```

Add the method inside `impl HLDecomposition` (next to `iter`):

```rust
    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
        while self.id[u] != self.id[v] {
            let head_u = self.paths[self.id[u]][0];
            let head_v = self.paths[self.id[v]][0];
            if self.level[head_u] >= self.level[head_v] {
                u = self.parent[head_u];
            } else {
                v = self.parent[head_v];
            }
        }
        if self.pos[u] <= self.pos[v] {
            u
        } else {
            v
        }
    }
```

(Safe at the root: if one endpoint's path is the root path, its head is the root with level 0, so the other endpoint always jumps.)

In `hl_decomposition_with_root`, declare `level` next to the other vecs and fill it during the existing `calc_size` DFS (a child's level is set before recursing into it; `level[root]` stays 0):

```rust
        let mut level = vec![0; n];
        let mut calc_size = RecursiveFunction2::new(|f, vert, last| {
            size[vert] = 1;
            parent[vert] = last;
            for e in self.adj(vert).iter() {
                let next = e.to();
                if next == last {
                    continue;
                }
                level[next] = level[vert] + 1;
                size[vert] += f.call(next, vert);
            }
            size[vert]
        });
```

And include it in the returned struct:

```rust
        HLDecomposition {
            paths,
            id,
            pos,
            parent,
            level,
            root,
        }
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p algo_lib graph::hl_decomposition`
Expected: PASS (1 test).

- [ ] **Step 5: Commit**

```bash
git add algo_lib/src/graph/hl_decomposition.rs
git commit -m "(feature) lca and level on HLDecomposition"
```

---

### Task 3: `PathSegmentTree` skeleton — constructors + point ops

**Files:**
- Create: `algo_lib/src/graph/path_segment_tree.rs`
- Modify: `algo_lib/src/graph/mod.rs` (module list)

- [ ] **Step 1: Register the module**

In `algo_lib/src/graph/mod.rs`, in the alphabetical `pub mod` list, after `pub mod negative_distances;` add:

```rust
pub mod path_segment_tree;
```

- [ ] **Step 2: Write the failing test**

Create `algo_lib/src/graph/path_segment_tree.rs` containing, for now, only the test module scaffolding (fixture tree, naive helpers, first test). The fixture tree used by all tasks:

```text
        0
       / \
      1   2
     / \   \
    3   4   5
   /       / \
  6       7   8
```

```rust
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

    #[test]
    fn point_ops() {
        let g = graph();
        let mut tree = PathSegmentTree::with_gen(&g, true, |v| SumNode { sum: v as i64 });
        for v in 0..N {
            assert_eq!(tree.point_query(v).sum, v as i64);
        }
        tree.point_update(4, SumNode { sum: 100 });
        assert_eq!(tree.point_query(4).sum, 100);
        tree.point_through_update(4, |node| node.sum += 1);
        assert_eq!(tree.point_query(4).sum, 101);
        assert_eq!(tree.point_query(3).sum, 3);
    }
}
```

(`naive_lca` is used from Task 4 onward; `#[allow(dead_code)]` is not needed inside test modules only if used — if the compiler warns at this task, that's fine, it's used two commits later.)

- [ ] **Step 3: Run test to verify it fails**

Run: `cargo test -p algo_lib graph::path_segment_tree`
Expected: COMPILE ERROR — `PathSegmentTree` does not exist.

- [ ] **Step 4: Implement the skeleton**

At the top of `path_segment_tree.rs`, above the test module:

```rust
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
        gen: impl FnMut(usize) -> Node,
    ) -> Self {
        Self::with_gen_with_root(graph, 0, include_lca, gen)
    }

    pub fn with_gen_with_root<E: BidirectionalEdgeTrait>(
        graph: &Graph<E>,
        root: usize,
        include_lca: bool,
        mut gen: impl FnMut(usize) -> Node,
    ) -> Self {
        let hld = graph.hl_decomposition_with_root(root);
        let trees = hld
            .paths
            .iter()
            .map(|path| SegmentTree::with_gen(path.len(), |i| gen(path[i])))
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
}
```

(`HLPart` and `RangeInclusive` become used in Task 4; if unused-import warnings bother the build at this step, add them in Task 4 instead.)

- [ ] **Step 5: Run test to verify it passes**

Run: `cargo test -p algo_lib graph::path_segment_tree`
Expected: PASS (`point_ops`).

- [ ] **Step 6: Commit**

```bash
git add algo_lib/src/graph/path_segment_tree.rs algo_lib/src/graph/mod.rs
git commit -m "(feature) PathSegmentTree skeleton and point operations"
```

---

### Task 4: path decomposition + `query`

**Files:**
- Modify: `algo_lib/src/graph/path_segment_tree.rs`

- [ ] **Step 1: Write the failing tests**

Add to the test module. `SeqNode` is the non-commutative orientation probe: it aggregates the literal vertex sequence, and `swap` reverses it.

```rust
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
            PathSegmentTree::with_gen(&g, true, |v| SumNode { sum: 1 + (v as i64) * 10 });
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
        let mut tree = PathSegmentTree::with_gen(&g, true, |v| SeqNode { seq: vec![v] });
        for a in 0..N {
            for b in 0..N {
                assert_eq!(tree.query(a..=b).seq, naive_path(a, b), "path {}..={}", a, b);
            }
        }
    }

    #[test]
    fn query_edge_mode_all_pairs() {
        let g = graph();
        let mut tree = PathSegmentTree::with_gen(&g, false, |v| SeqNode { seq: vec![v] });
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
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p algo_lib graph::path_segment_tree`
Expected: COMPILE ERROR — no method `query`.

- [ ] **Step 3: Implement `parts` and `query`**

Add inside `impl<Node: SegmentTreeNode> PathSegmentTree<Node>`:

```rust
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
```

How `query` orients: each part query is ancestor-first. Folding up-leg parts with the new part on the **left** yields the lca→a sequence; `swap()` flips it to a→lca; down-leg parts (already in lca→b order, internally aligned with travel) are appended on the right. `Node::default()` is assumed to be a join identity, same as in `SegmentTree::query`; `swap` is also called when the path is pure-up (b == lca), which must be a no-op-compatible flip of a complete result — that is exactly the contract of `swap`.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p algo_lib graph::path_segment_tree`
Expected: PASS (4 tests).

- [ ] **Step 5: Commit**

```bash
git add algo_lib/src/graph/path_segment_tree.rs
git commit -m "(feature) PathSegmentTree path query"
```

---

### Task 5: `for_each` family

**Files:**
- Modify: `algo_lib/src/graph/path_segment_tree.rs`

- [ ] **Step 1: Write the failing tests**

```rust
    #[test]
    fn for_each_directions() {
        let g = graph();
        let mut tree =
            PathSegmentTree::with_gen(&g, true, |v| SumNode { sum: 1 + (v as i64) * 10 });
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
        let mut tree = PathSegmentTree::with_gen(&g, true, |v| SumNode { sum: v as i64 });
        for a in 0..N {
            for b in 0..N {
                let immut: i64 = tree.for_each(a..=b, |acc: i64, node, _| acc + node.sum);
                let muts: i64 = tree.for_each_mut(a..=b, |acc: i64, node, _| acc + node.sum);
                assert_eq!(immut, muts, "path {}..={}", a, b);
            }
        }
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p algo_lib graph::path_segment_tree`
Expected: COMPILE ERROR — no method `for_each` / `for_each_mut`.

- [ ] **Step 3: Implement the four methods**

```rust
    pub fn for_each<R: Default>(
        &mut self,
        range: RangeInclusive<usize>,
        f: impl FnMut(R, &Node, PathDirection) -> R,
    ) -> R {
        self.for_each_with_init(range, f, R::default())
    }

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
```

Parts are visited in a→b travel order; the `Up` flag tells the callback that inside the current part the underlying seg tree still enumerates ancestor-first, i.e. reversed relative to travel.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p algo_lib graph::path_segment_tree`
Expected: PASS (6 tests).

- [ ] **Step 5: Commit**

```bash
git add algo_lib/src/graph/path_segment_tree.rs
git commit -m "(feature) PathSegmentTree for_each with direction"
```

---

### Task 6: `update` (by value, swapped for the up-leg)

**Files:**
- Modify: `algo_lib/src/graph/path_segment_tree.rs`

- [ ] **Step 1: Write the failing test**

`DirNode` is a delta-only lazy node with an asymmetric delta, so a missing `swap` on the up-leg is visible. `reset_delta` clears the node because the whole node is its delta (internal nodes' aggregate is unused).

```rust
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
                let mut tree: PathSegmentTree<DirNode> = PathSegmentTree::new(&g, true);
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
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p algo_lib graph::path_segment_tree`
Expected: COMPILE ERROR — no method `update`.

- [ ] **Step 3: Implement `update`**

```rust
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
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p algo_lib graph::path_segment_tree`
Expected: PASS (7 tests).

- [ ] **Step 5: Commit**

```bash
git add algo_lib/src/graph/path_segment_tree.rs
git commit -m "(feature) PathSegmentTree range update"
```

---

### Task 7: `binary_search` + `binary_search_mut`

**Files:**
- Modify: `algo_lib/src/graph/path_segment_tree.rs`

- [ ] **Step 1: Write the failing tests**

The k-th-vertex-on-path search over all pairs and all k exercises answers on the up-leg, at the lca, and on the down-leg, plus the not-found case.

```rust
    #[test]
    fn binary_search_kth_vertex() {
        let g = graph();
        let mut tree = PathSegmentTree::with_gen(&g, true, |_| SumNode { sum: 1 });
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
        let mut tree = PathSegmentTree::with_gen(&g, true, |_| SumNode { sum: 1 });
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
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p algo_lib graph::path_segment_tree`
Expected: COMPILE ERROR — no method `binary_search` / `binary_search_mut`.

- [ ] **Step 3: Implement both methods**

Up-leg parts are searched right-to-left (deepest position first — that's the a-side), down-leg parts left-to-right, so leaves are considered in a→b travel order. `accept` is stateful exactly as in `SegmentTree`: return `true` to descend toward the answer, mutate your accumulator and return `false` to skip a subsegment. The `FnOnce` `calc` is parked in an `Option` and `take()`n inside the per-part adapter, so parts that reject (their adapter never runs) don't consume it. The adapter also translates the leaf's path position back to the vertex id.

```rust
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
```

(Borrow note: `let hld = &self.hld;` alongside `&mut self.trees[...]` is fine — disjoint fields borrowed within one method body. Do not turn `parts`/lookups into `self.`-method calls inside these loops or the whole-`self` borrow will conflict.)

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p algo_lib graph::path_segment_tree`
Expected: PASS (9 tests).

- [ ] **Step 5: Run the full library test suite**

Run: `cargo test -p algo_lib`
Expected: everything PASSES (no regressions from the trait/struct changes).

- [ ] **Step 6: Commit**

```bash
git add algo_lib/src/graph/path_segment_tree.rs
git commit -m "(feature) PathSegmentTree binary search"
```
