# Graph Storage Enum Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make `algo_lib`'s `Graph<E>` storage runtime-selectable between a linked-list adjacency (current) and `Vec<Vec<E>>` (pre-LL), exposed via three constructors (`new_linked`, `new_2d`, `new(n, m)` auto-select), so callers can pick the right layout for their density.

**Architecture:** Migration is staged so the workspace compiles and passes the existing test suite after every task. First we add a new `(vertex, cursor)` API to today's Linked-only graph. Then we migrate the ~10 algorithm files that touch edges by raw id (flow, MCMF, iterative-DFS) onto that API. Then we wrap the storage in an enum and add the TwoD variant. Finally we sweep all `Graph::new(n)` call sites and tune the auto-select threshold `K` empirically with the criterion bench suite.

**Tech Stack:** Rust workspace. `algo_lib` crate. Cursor convention `u32` everywhere (`u32::MAX` sentinel for "no edge"). Criterion bench harness already in place from prior work.

**Reference:** `docs/superpowers/specs/2026-05-16-graph-storage-enum-design.md`.

---

### Task 1: Add `(vertex, cursor)` accessor API on the current Graph

Add three new methods on `Graph<E>` that take a vertex *and* a cursor: `edge_at`, `edge_at_mut`, `step_edge`. They're storage-agnostic by signature; in the current Linked-only world they ignore the vertex and delegate to existing impls. This makes the new API available for callers to migrate to without changing existing callers.

**Files:**
- Modify: `algo_lib/src/graph/mod.rs`

- [ ] **Step 1: Add the three new methods**

Open `algo_lib/src/graph/mod.rs`. In the main `impl<E: EdgeTrait> Graph<E>` block (just below the existing `next_edge` method around line 153–155), append:

```rust
    /// Edge access by (vertex, cursor). In the current Linked storage the
    /// cursor is a global edge id and the vertex is ignored; the parameter
    /// is there so future storage variants can use it.
    pub fn edge_at(&self, _v: usize, cursor: u32) -> &E {
        &self.edges[cursor as usize]
    }

    /// Mutable counterpart of `edge_at`.
    pub fn edge_at_mut(&mut self, _v: usize, cursor: u32) -> &mut E {
        &mut self.edges[cursor as usize]
    }

    /// Advance a cursor in `v`'s adjacency list. Returns the next cursor or
    /// `u32::MAX` at the end. As with `edge_at`, the vertex is currently
    /// unused.
    pub fn step_edge(&self, _v: usize, cursor: u32) -> u32 {
        self.next[cursor as usize]
    }
```

- [ ] **Step 2: Verify compile**

Run: `cargo build -p algo_lib`
Expected: no errors. Warnings about unused parameters are silenced by the `_v` prefix.

- [ ] **Step 3: Run existing tests**

Run: `cargo test -p algo_lib --tests 2>&1 | tail -5`
Expected: `d_maxflow` fails (pre-existing); everything else passes. The pre-existing `d_maxflow` failure was logged in commit `470119a`.

- [ ] **Step 4: Commit**

```bash
git add algo_lib/src/graph/mod.rs
git commit -m "feat(algo_lib/graph): add (vertex, cursor) accessor API

Adds edge_at(v, cursor), edge_at_mut(v, cursor), step_edge(v, cursor)
alongside the existing (id)-based accessors. The vertex parameter is
currently unused (Linked storage uses the cursor as a global edge id),
but the signature is ready for an upcoming TwoD storage variant where
cursor is a position within the vertex's adjacency."
```

---

### Task 2a: Migrate flow files (`max_flow`, `fast_max_flow`, `flow_graph`, `flow_edge`, `weighted_flow_edge`)

Switch these files from `(id)` to `(vertex, cursor)`. Also update `FlowEdgeTrait::push_flow` to return the 3-tuple `(to, reverse_id, flow)`, and update `Graph::push_flow` (in `flow_graph.rs`) to consume that shape.

**Files:**
- Modify: `algo_lib/src/graph/edges/flow_edge_trait.rs`
- Modify: `algo_lib/src/graph/edges/flow_edge.rs`
- Modify: `algo_lib/src/graph/edges/weighted_flow_edge.rs`
- Modify: `algo_lib/src/graph/flow_graph.rs`
- Modify: `algo_lib/src/graph/max_flow.rs`
- Modify: `algo_lib/src/graph/fast_max_flow.rs`

- [ ] **Step 1: Update `FlowEdgeTrait::push_flow` return type**

Open `algo_lib/src/graph/edges/flow_edge_trait.rs`. Replace the trait body with:

```rust
pub trait FlowEdgeTrait<C: AdditionMonoidWithSub + PartialOrd + Copy>: EdgeTrait {
    fn capacity(&self) -> C;
    fn capacity_mut(&mut self) -> &mut C;
    fn flow(&self, graph: &Graph<Self>) -> C;
    /// Returns `(to, reverse_id, flow)` — destination vertex of the direct
    /// edge, the cursor at which the reverse lives in the destination's
    /// adjacency (under the current Linked storage this is a global edge id),
    /// and the amount of flow to push.
    fn push_flow(&self, flow: C) -> (usize, usize, C) {
        (self.to(), self.reverse_id(), flow)
    }
}
```

- [ ] **Step 2: Update `FlowEdge::flow` to use `edge_at`**

In `algo_lib/src/graph/edges/flow_edge.rs`, find the `flow` method (around line 93) inside `impl FlowEdgeTrait` and replace:

```rust
    fn flow(&self, graph: &Graph<Self>) -> C {
        graph.edge(self.reverse_id as usize).capacity
    }
```

with:

```rust
    fn flow(&self, graph: &Graph<Self>) -> C {
        graph.edge_at(self.to as usize, self.reverse_id).capacity
    }
```

- [ ] **Step 3: Update `WeightedFlowEdge::flow` the same way**

In `algo_lib/src/graph/edges/weighted_flow_edge.rs`, find the `flow` method (around line 130) and replace:

```rust
    fn flow(&self, graph: &Graph<Self>) -> C {
        graph.edge(self.reverse_id as usize).capacity
    }
```

with:

```rust
    fn flow(&self, graph: &Graph<Self>) -> C {
        graph.edge_at(self.to as usize, self.reverse_id).capacity
    }
```

- [ ] **Step 4: Update `Graph::push_flow` to consume `(to, reverse_id, flow)`**

In `algo_lib/src/graph/flow_graph.rs`, replace the trait `FlowGraph` and its `impl` with:

```rust
pub trait FlowGraph<C: AdditionMonoidWithSub + PartialOrd + Copy, E: FlowEdgeTrait<C>> {
    /// `(to, reverse_id, flow)`: increase capacity of the reverse edge (which
    /// lives at `(to, reverse_id)`) by `flow`, decrease capacity of the
    /// direct edge (the reverse-of-the-reverse) by `flow`.
    fn push_flow(&mut self, push_data: (usize, usize, C));
}

impl<C: AdditionMonoidWithSub + PartialOrd + Copy, E: FlowEdgeTrait<C>> FlowGraph<C, E>
    for Graph<E>
{
    fn push_flow(&mut self, (to, reverse_id, flow): (usize, usize, C)) {
        let rev_cursor = reverse_id as u32;
        let direct_to;
        let direct_cursor;
        {
            let rev = self.edge_at_mut(to, rev_cursor);
            *rev.capacity_mut() += flow;
            direct_to = rev.to();
            direct_cursor = rev.reverse_id() as u32;
        }
        let direct = self.edge_at_mut(direct_to, direct_cursor);
        assert!(flow >= C::zero() && flow <= direct.capacity());
        *direct.capacity_mut() -= flow;
    }
}
```

- [ ] **Step 5: Update `max_flow.rs`'s Dinic loop**

Open `algo_lib/src/graph/max_flow.rs`. Replace the inner `dinic_impl` body's edge access (around lines 51–66) so the cursor walk uses `edge_at` / `step_edge`:

```rust
                        while next_edge[source] != u32::MAX {
                            let eid = next_edge[source];
                            let edge = self.edge_at(source, eid);
                            if edge.capacity() != C::zero() && dist[edge.to()] == dist[source] + 1 {
                                let pushed = f.call(edge.to(), flow.min(edge.capacity()));
                                if pushed != C::zero() {
                                    let push_data = self.edge_at(source, eid).push_flow(pushed);
                                    self.push_flow(push_data);
                                    flow -= pushed;
                                    total_pushed += pushed;
                                    if flow == C::zero() {
                                        return total_pushed;
                                    }
                                }
                            }
                            next_edge[source] = self.step_edge(source, eid);
                        }
```

Two key changes: `self.edge(eid)` → `self.edge_at(source, eid)`, `self.next_edge(eid)` → `self.step_edge(source, eid)`. Keep `next_edge[source] = self.head_edge(v)` unchanged at the phase reset (`head_edge` already takes a vertex).

- [ ] **Step 6: Update `fast_max_flow.rs` similarly**

Open `algo_lib/src/graph/fast_max_flow.rs`. Wherever the file uses `self.edge(eid)`, replace with `self.edge_at(vertex, eid)` — the vertex is the loop variable in scope at each call site (it's named `cur`, `i`, or `cur_id` depending on which loop). Wherever `self.next_edge(eid)` appears, replace with `self.step_edge(vertex, eid)`. The relevant lines per the earlier grep are 100, 105, 108–117, 143–149, 156–166, 180–184. Read each site, identify which vertex variable is in scope, and pass it through.

For example, at lines 108–117 (the BFS loop in the file):

```rust
                while next_edge[i] != u32::MAX {
                    let eid = next_edge[i];
                    // ... use self.edge_at(i, eid) ...
                    next_edge[i] = self.step_edge(i, eid);
                }
```

The vertex variable is `i` because the surrounding `for i in 0..n` is iterating vertices.

Also update the `push_data` extraction sites:

```rust
                            let push_data = self.edge_at(cur_id, eid).push_flow(/* ... */);
                            self.push_flow(push_data);
```

- [ ] **Step 7: Verify compile and tests**

Run: `cargo build -p algo_lib && cargo test -p algo_lib --tests 2>&1 | grep -E "^test result|FAILED" | tail -10`
Expected: only `d_maxflow` fails; rest pass.

- [ ] **Step 8: Commit**

```bash
git add algo_lib/src/graph/edges/flow_edge_trait.rs \
        algo_lib/src/graph/edges/flow_edge.rs \
        algo_lib/src/graph/edges/weighted_flow_edge.rs \
        algo_lib/src/graph/flow_graph.rs \
        algo_lib/src/graph/max_flow.rs \
        algo_lib/src/graph/fast_max_flow.rs
git commit -m "refactor(algo_lib/graph): migrate flow code to (vertex, cursor) API

FlowEdgeTrait::push_flow now returns (to, reverse_id, flow). Graph::push_flow
consumes that triple. Max-flow Dinic and LCT-Dinic use edge_at(v, cursor)
and step_edge(v, cursor) for all per-vertex cursor walks. No behaviour
change — Linked storage still ignores the vertex parameter."
```

---

### Task 2b: Migrate MCMF files (`min_cost_flow`, `min_cost_flow_slow`)

These also use edge-by-id access. Migrate them in the same shape as Task 2a.

**Files:**
- Modify: `algo_lib/src/graph/min_cost_flow.rs`
- Modify: `algo_lib/src/graph/min_cost_flow_slow.rs`

- [ ] **Step 1: Update `min_cost_flow.rs` edge-access sites**

Open `algo_lib/src/graph/min_cost_flow.rs`. The file uses `graph.edge(id)` and `graph.edge_mut(id)` in several places (the `add_one` closure, the iter_mut loop, and the final accounting loop). Replace each occurrence with `graph.edge_at(v, cursor)` / `graph.edge_at_mut(v, cursor)` where `v` is the vertex that owns the edge.

The vertex source varies per call site. Pattern to use:
- When the file already tracks `(from, id)` as `corresponding.push((i, j, graph_edge_id))`, the first element `i` is the owning vertex; use it for `edge_at` calls.
- Inside `add_one`, the `id` parameter comes alongside `from`; pass `from` as the vertex.
- Inside the `dijkstra` closure, `pre[v] = (u, i)` already stores the owning vertex `u`; use `pre[u].0` as the vertex for `edge_at` of `pre[u].1`.

After the migration, the final accounting loop reads:

```rust
                let mut min_cost = 0;
                let max_flow = graph.edge_at(sink, back as u32).flow(&graph);
                for (from, self_edge_id, graph_edge_id) in corresponding {
                    let x_flow;
                    let x_weight;
                    {
                        let x = graph.edge_at(from, graph_edge_id as u32);
                        x_flow = x.flow(&graph);
                        x_weight = x.weight();
                    }
                    min_cost += x_flow * x_weight;
                    let push_data = or_graph.edge_at(from, self_edge_id as u32).push_flow(x_flow);
                    or_graph.push_flow(push_data);
                }
```

(Note: the `corresponding` tuple's first field was already `from`. The earlier loop discards it via `_`; reuse it now.)

Inside the `add_one` closure, the loop that pushes flow along the predecessor path:

```rust
                            while u != v {
                                let push_data = graph.edge_at(pre[u].0, pre[u].1 as u32).push_flow(1);
                                graph.push_flow(push_data);
                                u = pre[u].0;
                            }
```

And the iter_mut bit-scaling block:

```rust
                    for j in 0..=n {
                        for e in graph.adj_mut(j).iter_mut() {
                            *e.capacity_mut() <<= 1;
                        }
                    }
```

stays unchanged (it's already `adj_mut`-based).

The capacity increments on `graph.edge_mut(id)` inside `add_one` become `graph.edge_at_mut(from, id as u32)`. The `back` edge accounting at the top of `add_one` operates on the "back" edge whose owner is `sink`, so `graph.edge_at_mut(sink, id as u32)`.

- [ ] **Step 2: Update `min_cost_flow_slow.rs` similarly**

`min_cost_flow_slow.rs` follows the same pattern but smaller. Read each `graph.edge(...)` / `graph.edge_mut(...)` call site, identify the owning vertex (typically `from` is tracked alongside), and switch to `edge_at` / `edge_at_mut`.

- [ ] **Step 3: Build and test**

Run: `cargo build -p algo_lib && cargo test -p algo_lib --tests --test e_min_cost_flow --test minimum_cost_maximum_flow --test minimum_spanning_tree 2>&1 | tail -10`
Expected: pass. (These are the MCMF tests.)

- [ ] **Step 4: Commit**

```bash
git add algo_lib/src/graph/min_cost_flow.rs algo_lib/src/graph/min_cost_flow_slow.rs
git commit -m "refactor(algo_lib/graph): migrate MCMF to (vertex, cursor) API"
```

---

### Task 2c: Migrate cursor-DFS files (`dfs_order`, `lca`, `euler_path`)

These use `head_edge(v)` (already taking a vertex) and `next_edge(id)` (NOT taking a vertex). Migrate the `next_edge` calls.

**Files:**
- Modify: `algo_lib/src/graph/dfs_order.rs`
- Modify: `algo_lib/src/graph/lca.rs`
- Modify: `algo_lib/src/graph/euler_path.rs`

- [ ] **Step 1: Update `dfs_order.rs`**

In `algo_lib/src/graph/dfs_order.rs` at line 55, replace `self.next_edge(eid)` with `self.step_edge(current, eid)`. The vertex `current` is already in scope (the file's iterative-DFS state).

- [ ] **Step 2: Update `lca.rs`**

In `algo_lib/src/graph/lca.rs` at lines 187 and 198, replace `self.next_edge(...)` with `self.step_edge(vertex, ...)`. The vertex variable in scope is `vertex` at each site.

- [ ] **Step 3: Update `euler_path.rs`**

In `algo_lib/src/graph/euler_path.rs` at line 34, replace `self.next_edge(id[v] as usize)` with `self.step_edge(v, id[v])`. The vertex `v` is already the surrounding loop's variable.

Note: line 34 currently passes `id[v] as usize` because the old `next_edge` took a `usize`. The new `step_edge` takes `u32` directly, so the cast is no longer needed — just `id[v]`. Also adjust the `id[v] = self.next_edge(id[v] as usize);` shape if the type still mismatches; the new method returns `u32`.

- [ ] **Step 4: Build and test**

Run: `cargo build -p algo_lib && cargo test -p algo_lib --tests 2>&1 | grep -E "^test result|FAILED" | tail -10`
Expected: only `d_maxflow` fails.

- [ ] **Step 5: Commit**

```bash
git add algo_lib/src/graph/dfs_order.rs algo_lib/src/graph/lca.rs algo_lib/src/graph/euler_path.rs
git commit -m "refactor(algo_lib/graph): migrate cursor-DFS callers to step_edge(v, c)"
```

---

### Task 3: Remove old by-id API and tighten `reverse_id` visibility

Drop the now-unused `Graph::edge(id)`, `Graph::edge_mut(id)`, `Graph::next_edge(id)` methods. Mark `EdgeTrait::reverse_id` / `set_reverse_id` as `pub(crate)`.

**Files:**
- Modify: `algo_lib/src/graph/mod.rs`
- Modify: `algo_lib/src/graph/edges/edge_trait.rs`

- [ ] **Step 1: Remove old single-arg methods from `Graph`**

In `algo_lib/src/graph/mod.rs`, delete the three blocks (lines ~137–155 in the current file):

```rust
    pub fn edge(&self, id: usize) -> &E { ... }
    pub fn edge_mut(&mut self, id: usize) -> &mut E { ... }
    pub fn next_edge(&self, id: usize) -> u32 { ... }
```

Keep `head_edge(v)` (already correct shape).

- [ ] **Step 2: Make `EdgeTrait::reverse_id` `pub(crate)`**

Open `algo_lib/src/graph/edges/edge_trait.rs`. Find the trait `EdgeTrait`. Change the visibility of `reverse_id` and `set_reverse_id`:

Old (assumed shape — verify in the file):

```rust
    fn reverse_id(&self) -> usize { 0 }
    fn set_reverse_id(&mut self, _reverse_id: usize) {}
```

These are trait methods; they can't literally be `pub(crate)` because trait methods inherit the trait's visibility. Instead, make the **trait** still `pub` (so it's usable from algo_lib's algorithm files), but rename the methods or hide them. The simplest mechanical move:

a. Keep the trait `pub`.
b. Rename methods to `_reverse_id` / `_set_reverse_id` (leading underscore is a soft convention that other crates won't typo-stumble onto). Document them as crate-internal.

OR simpler — just leave the names alone but add a `#[doc(hidden)]` attribute on each method:

```rust
    #[doc(hidden)]
    fn reverse_id(&self) -> usize { 0 }
    #[doc(hidden)]
    fn set_reverse_id(&mut self, _reverse_id: usize) {}
```

Use `#[doc(hidden)]`. It signals "internal API" to other crates without breaking compilation.

- [ ] **Step 3: Build and test**

Run: `cargo build -p algo_lib && cargo test -p algo_lib --tests 2>&1 | grep -E "^test result|FAILED" | tail -5`
Expected: only `d_maxflow` fails.

- [ ] **Step 4: Commit**

```bash
git add algo_lib/src/graph/mod.rs algo_lib/src/graph/edges/edge_trait.rs
git commit -m "refactor(algo_lib/graph): drop old by-id API, hide reverse_id

Removes Graph::edge, Graph::edge_mut, Graph::next_edge (the (id)-only
forms); all callers were migrated to the (v, cursor) API in earlier
commits. EdgeTrait::reverse_id and set_reverse_id are marked
#[doc(hidden)] to discourage external use."
```

---

### Task 4: Wrap storage in `Storage<E>` enum (Linked-only single variant)

Convert the inline fields on `Graph<E>` into a `Storage<E>` enum with one variant `Linked`. Public API is unchanged — every method matches on the variant and delegates to a private helper. This sets up Task 5 (adding TwoD) as an additive change.

**Files:**
- Modify: `algo_lib/src/graph/mod.rs`

- [ ] **Step 1: Introduce the enum and move fields into it**

In `algo_lib/src/graph/mod.rs`, replace the `Graph<E>` struct definition and the `new` method:

```rust
#[derive(Clone)]
enum Storage<E: EdgeTrait> {
    Linked {
        first: Vec<u32>,
        next: Vec<u32>,
        edges: Vec<E>,
        degree: Vec<u32>,
    },
}

#[derive(Clone)]
pub struct Graph<E: EdgeTrait> {
    storage: Storage<E>,
}

impl<E: EdgeTrait> Graph<E> {
    pub fn new(vertex_count: usize) -> Self {
        Self {
            storage: Storage::Linked {
                first: vec![u32::MAX; vertex_count],
                next: Vec::new(),
                edges: Vec::new(),
                degree: vec![0u32; vertex_count],
            },
        }
    }
```

- [ ] **Step 2: Adapt every method body to `match self.storage`**

Every existing method on `Graph<E>` that previously accessed `self.first`, `self.next`, `self.edges`, `self.degree` now needs a `match` on `self.storage`. With only one variant, every match has a single arm. Example for `vertex_count`:

```rust
    pub fn vertex_count(&self) -> usize {
        match &self.storage {
            Storage::Linked { first, .. } => first.len(),
        }
    }
```

Apply the same shape to: `edge_count`, `degrees`, `adj`, `adj_mut`, `edge_at`, `edge_at_mut`, `step_edge`, `head_edge`, `add_edge`, `add_vertices`, `clear`, the internal `push_one`, and `edges()` (the iterator).

`AdjView` and `AdjIter` currently hold a `&'a Graph<E>` — they index into `self.next` and `self.edges` directly. Update them to borrow the Linked fields directly:

```rust
pub struct AdjView<'a, E: EdgeTrait> {
    storage: &'a Storage<E>,  // for now always Linked
    head: u32,
    len: u32,
}

impl<'a, E: EdgeTrait> AdjView<'a, E> {
    pub fn iter(&self) -> AdjIter<'a, E> {
        AdjIter { storage: self.storage, cur: self.head }
    }
    pub fn iter_with_id(&self) -> AdjIterWithId<'a, E> {
        AdjIterWithId { storage: self.storage, cur: self.head }
    }
    pub fn len(&self) -> usize { self.len as usize }
    pub fn is_empty(&self) -> bool { self.len == 0 }
    pub fn get(&self, i: usize) -> Option<&'a E> { self.iter().nth(i) }
    pub fn head_id(&self) -> u32 { self.head }
}

pub struct AdjIter<'a, E: EdgeTrait> {
    storage: &'a Storage<E>,
    cur: u32,
}

impl<'a, E: EdgeTrait> Iterator for AdjIter<'a, E> {
    type Item = &'a E;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur == u32::MAX { return None; }
        match self.storage {
            Storage::Linked { edges, next, .. } => {
                let id = self.cur as usize;
                let edge = &edges[id];
                self.cur = next[id];
                Some(edge)
            }
        }
    }
}
```

`AdjIterWithId` follows the same shape; only `Some((id, edge))` differs.

`AdjViewMut` and `AdjIterMut` similarly hold `&'a mut Storage<E>`. The `unsafe` linked-list walk for the mutable iterator splits per variant; the Linked arm keeps the current `*const`/`*mut` pointer dance.

- [ ] **Step 3: Build and test**

Run: `cargo build -p algo_lib && cargo test -p algo_lib --tests 2>&1 | grep -E "^test result|FAILED" | tail -5`
Expected: only `d_maxflow` fails.

- [ ] **Step 4: Commit**

```bash
git add algo_lib/src/graph/mod.rs
git commit -m "refactor(algo_lib/graph): wrap storage in Storage::Linked enum

Pure refactor — every public method now matches on the storage variant.
Currently the only variant is Linked, so every match has a single arm.
This sets up the upcoming TwoD storage variant as a pure additive change."
```

---

### Task 5: Add `Storage::TwoD` variant + enum-dispatched views + new constructors

Add the second storage variant, implement all the dispatch arms, and introduce `new_linked`, `new_2d`, `new(n, m)`. The bare `Graph::new(n)` constructor is kept temporarily (to avoid breaking 30+ call sites in this task) and routed to `new_linked`; Task 6 removes it.

**Files:**
- Modify: `algo_lib/src/graph/mod.rs`

- [ ] **Step 1: Add the TwoD variant to `Storage`**

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
```

- [ ] **Step 2: Add each public method's TwoD arm**

For each `match self.storage { Storage::Linked { ... } => ... }` block introduced in Task 4, add a `Storage::TwoD { ... } => ...` arm:

`vertex_count`:
```rust
            Storage::TwoD { edges, .. } => edges.len(),
```

`edge_count`:
```rust
            Storage::TwoD { edge_count, .. } => *edge_count,
```

`degrees`:
```rust
            Storage::TwoD { edges, .. } => edges.iter().map(|v| v.len()).collect(),
```

`add_edge` (call site `(from, edge)`):
```rust
            Storage::TwoD { edges, edge_count } => {
                let to = edge.to();
                assert!(to < edges.len());
                let direct_pos = edges[from].len();
                let mut edge = edge;
                edge.set_id(*edge_count);
                edges[from].push(edge);
                let mut direct_cursor = direct_pos as u32;
                if E::REVERSABLE {
                    let rev_pos = edges[to].len();
                    let mut rev_edge = edges[from][direct_pos].reverse_edge(from);
                    rev_edge.set_id(*edge_count);
                    edges[from][direct_pos].set_reverse_id(rev_pos);
                    rev_edge.set_reverse_id(direct_pos);
                    edges[to].push(rev_edge);
                }
                *edge_count += 1;
                direct_cursor as usize
                // Note: the return shape is `usize` for back-compat with the
                // existing public signature. Cursors fit in u32; widen at the
                // boundary.
            }
```

`add_vertices`:
```rust
            Storage::TwoD { edges, .. } => {
                for _ in 0..cnt { edges.push(Vec::new()); }
            }
```

`clear`:
```rust
            Storage::TwoD { edges, edge_count } => {
                for v in edges.iter_mut() { v.clear(); }
                *edge_count = 0;
            }
```

`adj(v)`:
```rust
            Storage::TwoD { edges, .. } => AdjView {
                storage: &self.storage,
                head: 0,
                len: edges[v].len() as u32,
            },
```

`head_edge(v)`:
```rust
            Storage::TwoD { edges, .. } => {
                if edges[v].is_empty() { u32::MAX } else { 0 }
            }
```

`step_edge(v, cursor)`:
```rust
            Storage::TwoD { edges, .. } => {
                let next = cursor + 1;
                if (next as usize) < edges[v].len() { next } else { u32::MAX }
            }
```

`edge_at(v, cursor)`:
```rust
            Storage::TwoD { edges, .. } => &edges[v][cursor as usize],
```

`edge_at_mut(v, cursor)`:
```rust
            Storage::TwoD { edges, .. } => &mut edges[v][cursor as usize],
```

- [ ] **Step 3: Update `AdjView` / `AdjIter` / `AdjViewMut` / `AdjIterMut` to dispatch on storage**

In each iterator's `next()` body, add the TwoD arm. For `AdjIter`:

```rust
impl<'a, E: EdgeTrait> Iterator for AdjIter<'a, E> {
    type Item = &'a E;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur == u32::MAX { return None; }
        let id = self.cur as usize;
        match self.storage {
            Storage::Linked { edges, next, .. } => {
                let edge = &edges[id];
                self.cur = next[id];
                Some(edge)
            }
            Storage::TwoD { edges, .. } => {
                // For TwoD the head was set to vertex's own head, but
                // AdjIter only carries `cur` — we need to know the vertex.
                // Carry the vertex in AdjIter for TwoD case.
                unreachable!("AdjIter for TwoD needs vertex; use the vertex-aware iter()")
            }
        }
    }
}
```

The current `AdjIter` shape doesn't carry the vertex; for TwoD, iteration over the vertex's slice needs `&edges[v]`. Two options:

a. **Carry the vertex in `AdjIter`:** add `vertex: u32` field. For Linked it's unused. For TwoD the iterator becomes `slice::Iter`-like.

b. **Snapshot the slice into `AdjView`:** for TwoD, `adj(v)` returns an `AdjView` whose internal state holds the slice directly. The enum variant of `AdjView` carries the slice for TwoD; `head + len` for Linked.

Option (b) is cleaner. Change `AdjView` to:

```rust
pub struct AdjView<'a, E: EdgeTrait> {
    inner: AdjViewInner<'a, E>,
}

enum AdjViewInner<'a, E: EdgeTrait> {
    Linked { storage: &'a Storage<E>, head: u32, len: u32 },
    Slice(&'a [E]),
}

impl<'a, E: EdgeTrait> AdjView<'a, E> {
    pub fn iter(&self) -> AdjIter<'a, E> {
        match &self.inner {
            AdjViewInner::Linked { storage, head, .. } =>
                AdjIter::Linked { storage: *storage, cur: *head },
            AdjViewInner::Slice(s) =>
                AdjIter::Slice(s.iter()),
        }
    }
    pub fn iter_with_id(&self) -> AdjIterWithId<'a, E> {
        match &self.inner {
            AdjViewInner::Linked { storage, head, .. } =>
                AdjIterWithId::Linked { storage: *storage, cur: *head },
            AdjViewInner::Slice(s) =>
                AdjIterWithId::Slice { iter: s.iter().enumerate() },
        }
    }
    pub fn len(&self) -> usize {
        match &self.inner {
            AdjViewInner::Linked { len, .. } => *len as usize,
            AdjViewInner::Slice(s) => s.len(),
        }
    }
    pub fn is_empty(&self) -> bool { self.len() == 0 }
    pub fn get(&self, i: usize) -> Option<&'a E> {
        match &self.inner {
            AdjViewInner::Linked { .. } => self.iter().nth(i),
            AdjViewInner::Slice(s) => s.get(i),
        }
    }
    pub fn head_id(&self) -> u32 {
        match &self.inner {
            AdjViewInner::Linked { head, .. } => *head,
            AdjViewInner::Slice(_) => 0,
        }
    }
}

pub enum AdjIter<'a, E: EdgeTrait> {
    Linked { storage: &'a Storage<E>, cur: u32 },
    Slice(std::slice::Iter<'a, E>),
}

impl<'a, E: EdgeTrait> Iterator for AdjIter<'a, E> {
    type Item = &'a E;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            AdjIter::Linked { storage, cur } => {
                if *cur == u32::MAX { return None; }
                match storage {
                    Storage::Linked { edges, next, .. } => {
                        let id = *cur as usize;
                        let e = &edges[id];
                        *cur = next[id];
                        Some(e)
                    }
                    Storage::TwoD { .. } => unreachable!(),
                }
            }
            AdjIter::Slice(it) => it.next(),
        }
    }
}

pub enum AdjIterWithId<'a, E: EdgeTrait> {
    Linked { storage: &'a Storage<E>, cur: u32 },
    Slice { iter: std::iter::Enumerate<std::slice::Iter<'a, E>> },
}

impl<'a, E: EdgeTrait> Iterator for AdjIterWithId<'a, E> {
    type Item = (usize, &'a E);
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            AdjIterWithId::Linked { storage, cur } => {
                if *cur == u32::MAX { return None; }
                match storage {
                    Storage::Linked { edges, next, .. } => {
                        let id = *cur as usize;
                        let e = &edges[id];
                        *cur = next[id];
                        Some((id, e))
                    }
                    Storage::TwoD { .. } => unreachable!(),
                }
            }
            AdjIterWithId::Slice { iter } => iter.next(),
        }
    }
}
```

`adj(v)`'s `Storage::Linked` arm now wraps in `AdjViewInner::Linked { storage: &self.storage, head, len }`, and the `Storage::TwoD` arm wraps in `AdjViewInner::Slice(&edges[v])`.

For `AdjViewMut` / `AdjIterMut`, do the same shape with `slice::IterMut` for the TwoD arm and the current `unsafe` walk for the Linked arm.

- [ ] **Step 4: Add new constructors**

After the existing `new(n)` method, add:

```rust
    pub fn new_linked(vertex_count: usize) -> Self {
        Self {
            storage: Storage::Linked {
                first: vec![u32::MAX; vertex_count],
                next: Vec::new(),
                edges: Vec::new(),
                degree: vec![0u32; vertex_count],
            },
        }
    }

    pub fn new_2d(vertex_count: usize) -> Self {
        Self {
            storage: Storage::TwoD {
                edges: (0..vertex_count).map(|_| Vec::new()).collect(),
                edge_count: 0,
            },
        }
    }

    /// Auto-selecting constructor. `expected_edge_count` is a hint: if
    /// `m >= K * n` the dense `TwoD` storage is chosen, else the sparse
    /// `Linked` storage. `K` is tuned empirically (see the bench sweep).
    pub fn new(vertex_count: usize, expected_edge_count: usize) -> Self {
        const K: usize = 8; // placeholder; tuned in Task 7
        if expected_edge_count >= K.saturating_mul(vertex_count) {
            Self::new_2d(vertex_count)
        } else {
            Self::new_linked(vertex_count)
        }
    }
```

Two notes:
- The old `Graph::new(n)` (single-arg) collides with the new `Graph::new(n, m)` (two-arg). Rust does NOT allow function overloading by arity. So rename the old `new(n)` to a temporary name first — see next step.

- [ ] **Step 5: Rename old `new(n)` → temporary, route old name to `new_linked`**

The old single-arg `new(n)` becomes `new_legacy(n)` (a temporary alias). All call sites in `algo_lib/src/` that called `Graph::new(n)` still compile because we add a `pub fn new_legacy(n: usize) -> Self { Self::new_linked(n) }` shim. Task 6 sweeps these call sites and removes the shim.

```rust
    /// Temporary alias for `new_linked`. Removed in the upcoming call-site
    /// sweep — see Task 6.
    #[deprecated(note = "Use Graph::new_linked, Graph::new_2d, or Graph::new(n, m).")]
    pub fn new_legacy(vertex_count: usize) -> Self {
        Self::new_linked(vertex_count)
    }
```

Then delete the bare `pub fn new(vertex_count: usize) -> Self` you wrote in Task 4 — the new two-arg `new(n, m)` from Step 4 replaces it.

In `algo_lib/src/`, sweep `Graph::new(` → `Graph::new_legacy(` so the crate still compiles. (This is mechanical — only `Graph::new(n)` single-arg sites need updating; the new `Graph::new(n, m)` shouldn't yet exist at any caller.)

```bash
# from /home/egor/proj/rust_algo
sed -i 's/Graph::new(/Graph::new_legacy(/g' \
    algo_lib/src/graph/*.rs algo_lib/src/graph/edges/*.rs
```

This is safe because `Graph::new(` only appears as the single-arg constructor at this point.

- [ ] **Step 6: Build and test**

Run: `cargo build -p algo_lib && cargo test -p algo_lib --tests 2>&1 | grep -E "^test result|FAILED" | tail -5`

Expected: only `d_maxflow` fails. (Watch for `deprecated` warnings from `new_legacy`; those are expected and will be cleaned in Task 6.)

- [ ] **Step 7: Add a smoke-test for the TwoD storage**

Add `algo_lib/tests/twod_storage_smoke.rs`:

```rust
// Smoke test: construct a small graph with Graph::new_2d and run a few
// algorithms to confirm TwoD storage works end-to-end.
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::strongly_connected_components::StronglyConnectedComponentsTrait;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::bridges::BridgeSearch;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::Graph;

#[test]
fn twod_dijkstra() {
    let mut g: Graph<BiWeightedEdge<u64, ()>> = Graph::new_2d(4);
    g.add_edge(BiWeightedEdge::new(0, 1, 1));
    g.add_edge(BiWeightedEdge::new(1, 2, 2));
    g.add_edge(BiWeightedEdge::new(2, 3, 3));
    let d = g.distances_from(0);
    assert_eq!(d[3].unwrap().0, 6);
}

#[test]
fn twod_scc() {
    let mut g: Graph<Edge<()>> = Graph::new_2d(3);
    g.add_edge(Edge::new(0, 1));
    g.add_edge(Edge::new(1, 2));
    g.add_edge(Edge::new(2, 0));
    let scc = g.strongly_connected_components();
    // All three vertices in one SCC.
    assert_eq!(scc.color[0], scc.color[1]);
    assert_eq!(scc.color[1], scc.color[2]);
}

#[test]
fn twod_bridges() {
    // Path graph: every edge is a bridge.
    let mut g: Graph<BiEdge<()>> = Graph::new_2d(4);
    g.add_edge(BiEdge::new(0, 1));
    g.add_edge(BiEdge::new(1, 2));
    g.add_edge(BiEdge::new(2, 3));
    let mut bridges = g.bridges();
    bridges.sort();
    let mut expected = vec![(0usize, 1usize), (1, 2), (2, 3)];
    for (u, v) in bridges.iter_mut() {
        if u > v { std::mem::swap(u, v); }
    }
    bridges.sort();
    expected.sort();
    assert_eq!(bridges, expected);
}
```

Run: `cargo test -p algo_lib --test twod_storage_smoke`
Expected: 3 passed, 0 failed.

- [ ] **Step 8: Commit**

```bash
git add algo_lib/src/graph/mod.rs algo_lib/tests/twod_storage_smoke.rs
git commit -m "feat(algo_lib/graph): add TwoD storage variant + new constructors

Storage<E> gains a TwoD { edges: Vec<Vec<E>>, edge_count } variant.
AdjView/AdjIter become enum-dispatched (Linked or Slice).
Constructors: new_linked(n), new_2d(n), new(n, m) auto-select (K=8 placeholder).
Old single-arg new(n) is renamed new_legacy and marked #[deprecated]; the
call-site sweep is Task 6. A new smoke test exercises TwoD across Dijkstra,
SCC, and bridges."
```

---

### Task 6: Sweep all `Graph::new(n)` / `Graph::new_legacy(n)` call sites

Replace every `Graph::new(n)` single-arg and `Graph::new_legacy(n)` call across the workspace with `Graph::new_linked(n)`. Then remove the temporary `new_legacy` shim.

**Files:**
- Modify (sweep): `algo_lib/src/**/*.rs`, `algo_lib/tests/**/*.rs`, `algo_lib/benches/**/*.rs`, `tasks/**/*.rs`, `archive/**/*.rs`
- Modify: `algo_lib/src/graph/mod.rs` (remove `new_legacy`)

- [ ] **Step 1: Find all single-arg `Graph::new(` call sites**

Run:
```bash
grep -rn --include='*.rs' 'Graph::new(' /home/egor/proj/rust_algo | head -50
grep -rn --include='*.rs' 'Graph::new_legacy(' /home/egor/proj/rust_algo | head -50
```

Inspect the results. Single-arg `Graph::new(n)` should appear in some places (the user-facing tests/benches we didn't sweep in Task 5); `Graph::new_legacy(n)` should appear in `algo_lib/src/graph/`.

- [ ] **Step 2: Mass-replace with sed**

```bash
cd /home/egor/proj/rust_algo
# Replace Graph::new_legacy( with Graph::new_linked( everywhere
find algo_lib tasks archive -name '*.rs' -print0 \
  | xargs -0 sed -i 's/Graph::new_legacy(/Graph::new_linked(/g'

# Replace Graph::new( with Graph::new_linked( everywhere — except files
# that already use the two-arg new(n, m) signature.
# Two-arg new is only used in bench files (post-tuning), which don't exist
# yet at this stage. So a global sed is safe for now.
find algo_lib tasks archive -name '*.rs' -print0 \
  | xargs -0 sed -i 's/Graph::new(/Graph::new_linked(/g'
```

- [ ] **Step 3: Remove the `new_legacy` shim from `algo_lib/src/graph/mod.rs`**

Delete the `new_legacy` method block and its `#[deprecated]` attribute that you added in Task 5.

- [ ] **Step 4: Build and test the workspace**

Run: `cargo build && cargo test -p algo_lib --tests 2>&1 | grep -E "^test result|FAILED" | tail -10`

Expected: only `d_maxflow` fails. No build errors. No `deprecated` warnings.

- [ ] **Step 5: Spot-check archive/tasks compile**

Run: `cargo build --workspace 2>&1 | tail -10`
Expected: clean (or only pre-existing warnings unrelated to this change).

- [ ] **Step 6: Commit**

```bash
git add -u algo_lib tasks archive
git commit -m "refactor: sweep Graph::new(n) -> Graph::new_linked(n)

Mechanical sed across algo_lib/src, algo_lib/tests, algo_lib/benches,
tasks/, archive/. Removes the temporary new_legacy shim from
algo_lib/src/graph/mod.rs. The bare single-arg Graph::new(n) constructor
no longer exists; callers must pick new_linked, new_2d, or new(n, m).

The auto-select constructor new(n, m) is still using a placeholder K=8;
K is tuned in the next commit."
```

---

### Task 7: K-tuning bench sweep

Add a bench section that runs Dijkstra, SCC, and max-flow Dinic against both storages at varying `m/n` ratios, then set `K` in `Graph::new` to the empirically-justified crossover.

**Files:**
- Modify: `algo_lib/benches/graph.rs`
- Modify: `algo_lib/src/graph/mod.rs`

- [ ] **Step 1: Add a parameterized bench section**

In `algo_lib/benches/graph.rs`, after the existing benches, add:

```rust
// --- K-tuning sweep ---------------------------------------------------------

fn bench_storage_sweep_dijkstra(c: &mut Criterion) {
    let n = 100_000;
    for &ratio in &[1u32, 2, 4, 8, 16, 32, 64] {
        let m = (ratio as usize) * n;
        // build once per (storage, ratio); each storage gets its own bench
        let mut rng = ChaCha20Rng::seed_from_u64(0xC0DE);
        let mut linked: Graph<BiWeightedEdge<u64, ()>> = Graph::new_linked(n);
        let mut twod: Graph<BiWeightedEdge<u64, ()>> = Graph::new_2d(n);
        for _ in 0..m {
            let u = rng.gen_range(0..n);
            let mut v = rng.gen_range(0..n);
            while v == u { v = rng.gen_range(0..n); }
            let w = rng.gen_range(1u64..=1_000_000_000);
            linked.add_edge(BiWeightedEdge::new(u, v, w));
            twod.add_edge(BiWeightedEdge::new(u, v, w));
        }
        let label_l = format!("dijkstra/linked/m_per_n={}", ratio);
        let label_t = format!("dijkstra/twod/m_per_n={}", ratio);
        c.bench_function(&label_l, |b| b.iter(|| black_box(linked.distances_from(0))));
        c.bench_function(&label_t, |b| b.iter(|| black_box(twod.distances_from(0))));
    }
}

fn bench_storage_sweep_scc(c: &mut Criterion) {
    let n = 100_000;
    for &ratio in &[1u32, 2, 4, 8, 16, 32, 64] {
        let m = (ratio as usize) * n;
        let mut rng = ChaCha20Rng::seed_from_u64(0xBEEF);
        let mut linked: Graph<Edge<()>> = Graph::new_linked(n);
        let mut twod: Graph<Edge<()>> = Graph::new_2d(n);
        for _ in 0..m {
            let u = rng.gen_range(0..n);
            let mut v = rng.gen_range(0..n);
            while v == u { v = rng.gen_range(0..n); }
            linked.add_edge(Edge::new(u, v));
            twod.add_edge(Edge::new(u, v));
        }
        let label_l = format!("scc/linked/m_per_n={}", ratio);
        let label_t = format!("scc/twod/m_per_n={}", ratio);
        c.bench_function(&label_l, |b| b.iter(|| black_box(linked.strongly_connected_components())));
        c.bench_function(&label_t, |b| b.iter(|| black_box(twod.strongly_connected_components())));
    }
}

fn bench_storage_sweep_dinic(c: &mut Criterion) {
    // Dinic on a complete bipartite graph; vary right-side count to vary degree.
    let left = 500usize;
    for &right in &[10usize, 30, 100, 300, 500] {
        let (g_linked0, src_l, snk_l) = dense_bipartite_flow_storage(left, right, 0xABBA, /*twod=*/false);
        let (g_twod0,   src_t, snk_t) = dense_bipartite_flow_storage(left, right, 0xABBA, /*twod=*/true);
        let label_l = format!("dinic/linked/right={}", right);
        let label_t = format!("dinic/twod/right={}", right);
        c.bench_function(&label_l, |b| {
            b.iter_batched(|| g_linked0.clone(), |mut g| black_box(g.max_flow(src_l, snk_l)), BatchSize::SmallInput);
        });
        c.bench_function(&label_t, |b| {
            b.iter_batched(|| g_twod0.clone(), |mut g| black_box(g.max_flow(src_t, snk_t)), BatchSize::SmallInput);
        });
    }
}

fn dense_bipartite_flow_storage(
    left: usize,
    right: usize,
    seed: u64,
    twod: bool,
) -> (Graph<FlowEdge<u64, ()>>, usize, usize) {
    let n = left + right + 2;
    let source = left + right;
    let sink = left + right + 1;
    let mut g: Graph<FlowEdge<u64, ()>> = if twod { Graph::new_2d(n) } else { Graph::new_linked(n) };
    let mut rng = ChaCha20Rng::seed_from_u64(seed);
    for i in 0..left  { g.add_edge(FlowEdge::new(source, i, 1)); }
    for j in 0..right { g.add_edge(FlowEdge::new(left + j, sink, 1)); }
    for i in 0..left {
        for j in 0..right {
            let c = rng.gen_range(1u64..=100);
            g.add_edge(FlowEdge::new(i, left + j, c));
        }
    }
    (g, source, sink)
}
```

Register the three new bench fns in the existing `criterion_group!` invocation.

- [ ] **Step 2: Run the sweep**

Run: `cargo bench --bench graph --no-run` — verify it compiles.

Then: `cargo bench --bench graph -- --sample-size 10 storage_sweep` (or just run the three new bench fn names).

Expected: each `(algorithm, storage, ratio)` combination reports a time. The interesting observation is **for each algorithm, at what `m/n` ratio does TwoD start beating Linked**?

Record the per-bench numbers in a comment block in `algo_lib/benches/graph.rs` (above the sweep section).

- [ ] **Step 3: Set `K` in `Graph::new`**

Open `algo_lib/src/graph/mod.rs`. Find the `K` constant inside `new(n, m)` (currently a placeholder `8`). Replace it with the smallest crossover ratio observed across Dijkstra, SCC, and Dinic — i.e., the smallest `m/n` at which TwoD won for ANY of the three algorithms. (Conservative: prefer Linked except where 2D clearly wins.)

For example, if Dijkstra crossed over at ratio=16, SCC at 32, Dinic at 8, set `K = 8`.

Update the doc comment on `Graph::new` to say "tuned empirically; see comment in benches/graph.rs".

- [ ] **Step 4: Re-run the full bench to confirm parity vs. `pre` baseline**

Run: `cargo bench --bench graph -- --baseline pre 2>&1 | tail -40`

Expected:
- Sparse benches (Dijkstra, SCC, bridges, etc. — n=10⁵, m=2·10⁵, ratio=2): roughly unchanged vs. baseline (auto-select picks Linked).
- Dense flow benches: roughly unchanged vs. baseline (still Linked because the bench harness builds the graph with `new_linked`, not `new(n, m)`). To see the auto-selection take effect on flow benches, the bench helper would need to use `new(n, m)` — optional follow-up beyond this task.

- [ ] **Step 5: Commit**

```bash
git add algo_lib/benches/graph.rs algo_lib/src/graph/mod.rs
git commit -m "bench(algo_lib/graph): add storage-sweep + set auto-select K

Adds Dijkstra/SCC/Dinic benches across both storage variants over
m_per_n ratios {1, 2, 4, 8, 16, 32, 64} (Dinic varies right-side count).
The observed crossover sets the auto-select threshold K in Graph::new(n, m)
to <chosen value>; see the comment block in benches/graph.rs for the
empirical numbers."
```
