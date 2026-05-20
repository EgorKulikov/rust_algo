# Graph Storage Enum (Linked / 2D)

## Goal

Make `algo_lib`'s `Graph<E>` support both the existing linked-list adjacency
storage (good at sparse) and the older `Vec<Vec<E>>` storage (good at dense)
under one runtime-selected enum. Provide explicit storage-selecting
constructors and an auto-selecting constructor that picks based on an
expected-edge-count hint. Tune the auto-select threshold `K` empirically with
the existing `cargo bench --bench graph` infrastructure.

## Motivation

The earlier refactor to linked-list-only storage shipped big wins on sparse
algorithms (−30% to −55%) but caused major regressions on dense flow benches
(+100% to +300%). The root cause is iteration cost on high-degree vertices:
linked-list traversal jumps in memory, defeating the CPU prefetcher; the
old `Vec<Vec<E>>` per-vertex contiguous layout was prefetcher-perfect.
Reintroducing the 2D variant as an option lets callers pay the right cost
for their graph's density.

## Non-Goals

- A wholesale rewrite of algorithm files. Most of the 24 algorithms in
  `graph/*.rs` use only `g.adj(v)` and `g.adj_mut(v)`, which become trivial
  enum forwards. The files that DO need mechanical updates (`(id)` →
  `(vertex, cursor)` on edge-access methods) are:
    - `max_flow.rs`, `fast_max_flow.rs` (Dinic per-vertex cursor)
    - `flow_graph.rs`, `min_cost_flow.rs`, `min_cost_flow_slow.rs`
      (`push_flow` and edge-by-id access)
    - `flow_edge.rs`, `weighted_flow_edge.rs` (the `flow(&self, graph)`
      trait method)
    - `dfs_order.rs`, `lca.rs`, `euler_path.rs` (iterative DFS with
      persistent per-vertex cursor — `next_edge(eid)` →
      `next_edge(v, cursor)`)
  Mechanical, ~10 files. Everything else is unchanged.
- Allowing storage migration on a live graph (e.g. `g.to_2d()`). Storage is
  picked at construction time and never changes.

## Storage

```rust
enum Storage<E: EdgeTrait> {
    Linked {
        first: Vec<u32>,
        next: Vec<u32>,
        edges: Vec<E>,
        degree: Vec<u32>,
    },
    TwoD {
        edges: Vec<Vec<E>>,
        edge_count: usize,
    },
}

pub struct Graph<E: EdgeTrait> {
    storage: Storage<E>,
}
```

`Linked` is exactly the current layout. `TwoD` mirrors the pre-refactor
`Vec<Vec<E>>` directly — no auxiliary tables. Each variant uses its own
natural edge handle (see "Edge cursors" below); the API is unified over a
`u32` cursor whose meaning is variant-dependent.

## Constructors

```rust
impl<E: EdgeTrait> Graph<E> {
    /// Auto-select storage from an expected edge count.
    /// 2D if `m >= K * n`, else Linked. `K` is a tuned constant (see below).
    pub fn new(vertex_count: usize, expected_edge_count: usize) -> Self;

    /// Explicit Linked storage.
    pub fn new_linked(vertex_count: usize) -> Self;

    /// Explicit 2D storage.
    pub fn new_2d(vertex_count: usize) -> Self;
}
```

The bare `Graph::new(n)` constructor is **removed**. Existing call sites
must convert to one of the three forms above. The least-thought-required
default is `new_linked(n)` (matches current behaviour after the LIFO refactor).

## `K` (auto-select threshold) tuning

After the refactor lands and compiles cleanly, a separate sweep adds bench
variants over `(storage, n, m)` for three representative algorithms:

- Dijkstra (sparse-iteration heavy)
- Max-flow Dinic (dense-iteration heavy)
- SCC (sparse, recursion + DFS)

For each, run with `n = 10⁵` (Dinic: 500) and `m / n ∈ {1, 2, 4, 8, 16, 32, 64}`
against both storages. Identify the crossover `m/n` ratio where 2D
consistently beats Linked. Set `K` to the **smallest** crossover across the
three algorithms (be conservative — prefer Linked unless 2D clearly wins
across the board). Hard-code `K` in `Graph::new`.

The bench file gets a new section dedicated to this sweep; the new benches
join `criterion_group!` so they're part of the standard bench run.

## Edge cursors and `reverse_id`

`EdgeTrait::reverse_id` / `set_reverse_id` become `pub(crate)`. The value is
**context-dependent** and means different things per storage variant:

- `Linked`: `reverse_id` is a global edge id (an index into `edges`).
- `TwoD`: `reverse_id` is a position within the destination vertex's
  inner `Vec<E>` (i.e., exactly the pre-refactor semantics).

Algorithm code never reads `reverse_id` directly. Edge access goes through
a small set of `Graph` methods that take `(vertex, cursor)` everywhere —
the `cursor` is a variant-agnostic `u32` whose semantics match the
variant's `reverse_id`:

```rust
impl<E: EdgeTrait> Graph<E> {
    /// First edge cursor for `v`; `None` if the vertex has no edges.
    pub fn head_edge(&self, v: usize) -> Option<u32>;

    /// Step to the next edge after `cursor` in `v`'s adjacency.
    pub fn next_edge(&self, v: usize, cursor: u32) -> Option<u32>;

    /// Edge access by cursor. For `Linked` the vertex is ignored (the
    /// cursor is a global id); for `TwoD` both are used.
    pub fn edge_at(&self, v: usize, cursor: u32) -> &E;
    pub fn edge_at_mut(&mut self, v: usize, cursor: u32) -> &mut E;
}
```

Internally:

- `Linked::head_edge(v)` returns `Some(first[v])` if not `u32::MAX`.
- `Linked::next_edge(_, cursor)` returns `Some(next[cursor])` if not `MAX`.
- `Linked::edge_at(_, cursor)` returns `&edges[cursor]`.
- `TwoD::head_edge(v)` returns `Some(0)` if `!edges[v].is_empty()`.
- `TwoD::next_edge(v, cursor)` returns `Some(cursor + 1)` if `cursor + 1 < edges[v].len()`.
- `TwoD::edge_at(v, cursor)` returns `&edges[v][cursor as usize]`.

The global-id-based `Graph::edge(id) -> &E` from the prior refactor is
**removed**; all access goes through `edge_at(v, cursor)`.

### Flow push path

`FlowEdgeTrait::push_flow(&self, flow) -> (usize, usize, C)` reverts to its
pre-refactor 3-tuple shape: `(to, reverse_id, flow)`. `Graph::push_flow`
matches on storage:

- `Linked` body: ignores `to`; uses `reverse_id` as a global cursor with
  `edge_at_mut(_, rev_id)`. (Recovering the direct edge after mutating the
  reverse uses the reverse's `reverse_id()` field as another global id.)
- `TwoD` body: uses `to` and `reverse_id` as `(vertex, position)`. To find
  the direct edge after mutating the reverse, read the reverse's stored
  `(reverse_id_into_vertex_to)` — but in `TwoD`, the reverse edge's
  `reverse_id` field points at the direct's position within the
  **direct's** vertex (`from`). So the direct lookup is
  `edge_at_mut(direct_from_vertex, direct_pos)` where `direct_from_vertex`
  is recoverable from the reverse edge's own `to()` (the reverse points
  back at the original source).

`FlowEdge::flow(&self, graph)` similarly reads the reverse's capacity
via `graph.edge_at(self.to(), self.reverse_id() as u32).capacity()` —
same call shape for both variants since `to()` and `reverse_id()` are on
the edge.

This keeps the storage-aware logic confined to (a) the small set of
`Graph` cursor methods, (b) `Graph::push_flow`'s body, and (c) two
trait-method bodies (`flow`). Algorithm files everywhere else remain
storage-agnostic.

## `AdjView` / `AdjIter`

```rust
pub struct AdjView<'a, E: EdgeTrait> {
    inner: AdjViewInner<'a, E>,
}

enum AdjViewInner<'a, E: EdgeTrait> {
    Linked { graph: &'a Graph<E>, head: u32, len: u32 },
    Slice(&'a [E]),
}

impl<'a, E: EdgeTrait> AdjView<'a, E> {
    pub fn iter(&self) -> AdjIter<'a, E> { /* match inner */ }
    pub fn len(&self) -> usize { /* match inner */ }
    pub fn is_empty(&self) -> bool { /* match inner */ }
    pub fn get(&self, i: usize) -> Option<&'a E> { /* match inner */ }
}

pub enum AdjIter<'a, E: EdgeTrait> {
    Linked(LinkedAdjIter<'a, E>),
    Slice(std::slice::Iter<'a, E>),
}

impl<'a, E: EdgeTrait> Iterator for AdjIter<'a, E> {
    type Item = &'a E;
    fn next(&mut self) -> Option<Self::Item> { /* match self */ }
}
```

One `match` per method/`next()` — a single, well-predicted branch. Same
shape applies to `AdjViewMut` / `AdjIterMut` (the existing `unsafe` linked-
list mut iterator becomes the `Linked` arm; `slice::IterMut` becomes the
`Slice` arm).

## `add_edge`

Per variant:

- `Linked`: identical to today (LIFO prepend; `reverse_id` is a global id).
- `TwoD`: push to `edges[from]`; if `E::REVERSABLE`, push the reverse to
  `edges[to]`. `reverse_id` is the position in the destination vertex's
  `Vec` — exactly the pre-LL semantics.

`add_edge` returns a `u32` cursor that follows the variant's convention
(global id for Linked, position-within-`from` for TwoD). Callers that
previously took the return value as a "global id" must be re-read as
"opaque cursor passed back into `Graph::edge_at(from, cursor)`".

## `clear`, `add_vertices`, `edge_count`, `vertex_count`, `degrees`

Per-variant bodies, same external semantics. `clear` does NOT change the
variant — once a graph is constructed Linked, it stays Linked.

`edge_count`: derived from `edges.len() [/ 2 if REVERSABLE]` for `Linked`
(as today); read from `edge_count: usize` for `TwoD` (explicit, per the
user's request — matches the inner-Vec model where summing
`edges.iter().map(|v| v.len()).sum() / 2` would be O(n)).

## Migration

### Inside `algo_lib`

- `algo_lib/src/graph/mod.rs` — full rewrite to the enum design.
- `algo_lib/src/graph/flow_graph.rs` — `push_flow` body splits per variant.
- `algo_lib/src/graph/edges/flow_edge.rs` and
  `algo_lib/src/graph/edges/weighted_flow_edge.rs` — `flow(&self, graph)`
  splits per variant (or asks the graph via a new helper).
- All other `algo_lib/src/graph/*.rs` algos — unchanged.
- Internal `#[cfg(test)]` modules inside `graph/*.rs` — convert
  `Graph::new(n)` to `Graph::new_linked(n)`.

### `algo_lib/tests/`

Every integration test that calls `Graph::new(n)` becomes
`Graph::new_linked(n)`. Mechanical search and replace. The algorithm output
doesn't depend on storage choice, so Linked is a safe universal target —
no per-test thinking required.

Count of files to touch: about 25 in `algo_lib/tests/`. Verified by:
```
grep -lr "Graph::new(" algo_lib/tests/ | sort -u
```

### `algo_lib/benches/graph.rs`

Bench file's `Graph::new(n)` calls become `Graph::new_linked(n)` for
baseline parity. Add the K-tuning sweep here (see "K tuning" above).

### Out-of-tree code

- `tasks/*` and `archive/*` — convert their `Graph::new(n)` call sites to
  `Graph::new_linked(n)` as well, so the workspace builds cleanly. These
  aren't part of `cargo test -p algo_lib`, so they're a separate sweep
  but easy.

## Risk / Out-of-scope hazards

- **Branch overhead in `AdjView` / `AdjIter`** — adds one well-predicted
  branch per iteration step. Inside a tight algorithm loop running ~10⁶
  iterations, the impact should be ≤ 2% (single branch in a hot loop
  where the variant is fixed across the call site). The bench will
  measure this directly.
- **Cursor semantics drift** — `reverse_id` and `add_edge`'s return value
  changed meaning between Linked (global id) and TwoD (position). Code
  that treats either as an arithmetic value (rather than an opaque cursor
  passed back into `Graph::edge_at`) will break on the other variant.
  This is enforced softly via `pub(crate)` on `EdgeTrait::reverse_id`; a
  brief sweep of `algo_lib/src/graph/*.rs` confirms only the flow code
  touches it.
- **`d_maxflow` test** currently fails after the LIFO refactor because it
  pins a specific maxflow routing. It will continue to fail under
  `new_linked`. The fix is either (a) restore that test under the `new_2d`
  storage variant (which preserves the FIFO order the test depended on),
  or (b) loosen the test to compare only the flow value. Out of scope for
  this spec; flagged for follow-up.

## Done When

- `Graph::new`, `Graph::new_linked`, `Graph::new_2d` exist; bare
  `Graph::new(n)` is gone.
- Every `Graph::new(n)` call site in `algo_lib/src/`, `algo_lib/tests/`,
  `algo_lib/benches/`, `tasks/`, `archive/` is converted.
- `cargo test -p algo_lib --tests` passes (with the known `d_maxflow`
  caveat unchanged).
- `cargo bench --bench graph --no-run` compiles.
- `cargo bench --bench graph -- --baseline pre` runs and produces
  numbers comparable to today's Linked-only baseline.
- A K-tuning bench section exists, sweep numbers are inspected, and `K`
  is set to a documented value (in a constant near `Graph::new`).
