# PathSegmentTree Design

Segment-tree-like structure over tree paths (HLD-based), abstracting the manual
pattern from `archive/2026/06/2026.06.12 - Codeforces Round 1103 (Div. 3)/g_criterion_in_burlandia.rs`.

## 1. `SegmentTreeNode` trait change (`algo_lib/src/collections/segment_tree.rs`)

Add to the trait:

```rust
fn swap(&mut self) {}
```

Semantics: reverse the node's left↔right orientation (a node aggregating a
directed segment flips which end is "left"; e.g. the contest `Node` swaps
`paths_to_left`/`paths_to_right`). Empty default — existing nodes unaffected.

## 2. `HLDecomposition` additions (`algo_lib/src/graph/hl_decomposition.rs`)

New method:

```rust
pub fn lca(&self, mut u: usize, mut v: usize) -> usize
```

HLD lca without levels: path ids are assigned in DFS preorder, so the parent
of path `i`'s head always lies on a path with id `< i` — climbing strictly
decreases the path id. Hence while `id[u] != id[v]`, the endpoint with the
*greater* path id can never have the lca on its current path and safely jumps
to `parent[paths[id][0]]`; once on the same path, the endpoint with smaller
`pos` is the lca. O(log n) per query, no extra fields needed.

Rationale: lets `PathSegmentTree` depend only on `HLDecomposition` +
`SegmentTree`, cutting the sparse-table `LCA` dependency. The extra log factor
is acceptable.

## 3. New module `algo_lib/src/graph/path_segment_tree.rs`

```rust
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PathDirection { Up, Down }

pub struct PathSegmentTree<Node> {
    hld: HLDecomposition,
    trees: Vec<SegmentTree<Node>>,   // one per heavy path
    include_lca: bool,
}
```

Registered in `graph/mod.rs`.

### Constructors

- `new<E: BidirectionalEdgeTrait>(graph: &Graph<E>, include_lca: bool)` — all
  nodes `Default`.
- `with_gen(graph, include_lca, gen: impl FnMut(usize /* vertex */) -> Node)`.
- `new_with_root` / `with_gen_with_root` variants taking an explicit root.

`include_lca = true`: nodes represent vertices; a path query `a..=b` covers
every vertex on the path including the lca (once).

`include_lca = false` (edge mode): the node of vertex `v` represents the edge
`(v, parent(v))`; the lca vertex is excluded from every path range, so
`a..=b` covers exactly the edges of the path. The root's slot exists but stays
`Default`/unused. `query(a..=a)` is empty → `Node::default()`.

### Path decomposition (internal)

All range operations take `RangeInclusive<usize>` (vertices `a..=b`) — no
other range forms. Internally: `l = hld.lca(a, b)`;
up-leg = `hld.iter(a..=l)` (vertex mode) or `hld.iter(a..l)` (edge mode);
down-leg = `hld.iter(b..l)` collected into a `Vec` and reversed.
Traversal order is a→b: up-leg parts first (direction `Up`), then down-leg
parts (direction `Down`). Within an up-leg part the underlying segment tree
still works left-to-right in path coordinates (ancestor first), i.e. reversed
relative to travel — the `Up` flag is what signals this to the caller.

### Operations

- `point_query(v) -> &Node`, `point_update(v, node)`,
  `point_through_update(v, f)` — delegate to `trees[id[v]]` at `pos[v]`.
- `query(a..=b) -> Node` (requires `Node: Clone`): fold up-leg parts as
  `res = join(part, res)` (orients res l→a), then `res.swap()` (a→l), then
  fold reversed down-leg parts as `res = join(res, part)`. Result oriented
  a→b; correct for non-commutative joins. `Node::default()` is assumed to be
  a join identity (same assumption as `SegmentTree::query`).
- `for_each(a..=b, f)` / `for_each_with_init` / `for_each_mut` /
  `for_each_mut_with_init` with callback `f(R, &Node, PathDirection) -> R`
  (`&mut Node` for the mut variants). Parts visited in a→b traversal order.
- `update(a..=b, mut val: Node)`: `val` is oriented a→b. Accumulate `&val`
  into down-leg parts first, then `val.swap()` in place, then accumulate
  `&val` into up-leg parts (per-leaf accumulation order across parts does not
  matter). No `Clone` needed.
- `binary_search(a..=b, accept, calc) -> Option<R>` and `binary_search_mut`:
  walk parts in a→b traversal order; up-leg parts use
  `binary_search_in_rtl` / `binary_search_in_mut_rtl`, down-leg parts the LTR
  variants. First part that yields a result wins. `calc` receives
  `(&Node, vertex)` — leaf path-position translated back to the vertex via
  `hld.paths[id][pos]`. The `FnOnce` calc is held in an `Option` and `take()`n
  inside per-part adapter closures so it survives rejected parts. `accept` is
  a stateful `FnMut(&Node) -> bool` exactly as in `SegmentTree`. There is no
  whole-tree `binary_search` (the unranged seg-tree search has no meaningful
  path analogue).

## 4. Tests (in-module `#[cfg(test)]`)

- Vertex-mode path sum `query` vs naive `lca.path()`-style enumeration on a
  small fixed tree, all vertex pairs.
- Non-commutative node (e.g. `(first, last)` endpoints + concatenation order,
  with `swap`) verifying a→b orientation through the lca.
- Edge mode: path sums over edges, all pairs; `query(a..=a)` empty.
- `binary_search`: first vertex on the path with running prefix-sum ≥ k,
  including cases where the answer lies on the up-leg, at the lca, and on the
  down-leg.
- `update(a..=b, val)` + point queries vs naive; a direction-sensitive
  update exercising the swapped-clone behavior on the up-leg.
- `HLDecomposition::lca` vs the existing `LCA` struct on a random tree.
