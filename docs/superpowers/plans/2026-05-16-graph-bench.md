# Graph Algorithm Benchmark Suite Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Establish a reproducible criterion-based perf baseline for every algorithm in `algo_lib/src/graph/` so an upcoming graph-data-structure refactor can be measured with `cargo bench --baseline pre`.

**Architecture:** Single bench file `algo_lib/benches/graph.rs` with three sections (generators, per-algo bench fns, `criterion_group!`/`criterion_main!`). Each generator is seeded for bit-for-bit reproducibility. Each algorithm gets one representative-size bench using the trait method as called in the algo's `tests/`. Mutating algorithms use `criterion::BatchSize::SmallInput` via `iter_batched` so graph reconstruction is excluded from the measurement.

**Tech Stack:** criterion 0.5 (html_reports), rand 0.8, rand_chacha 0.3. Existing `algo_lib` `Graph<E>` + per-trait method API.

**Reference:** `docs/superpowers/specs/2026-05-16-graph-bench-design.md`.

---

### Task 1: Cargo wiring + scaffold + first bench (Dijkstra smoke test)

Get the bench target compiling end-to-end with one working bench, so subsequent tasks just add more functions.

**Files:**
- Modify: `algo_lib/Cargo.toml`
- Create: `algo_lib/benches/graph.rs`

- [ ] **Step 1: Add criterion + rand deps and bench target to `algo_lib/Cargo.toml`**

Append under `[dev-dependencies]` (existing `tester = ...` line stays):

```toml
criterion = { version = "0.5", features = ["html_reports"] }
rand = "0.8"
rand_chacha = "0.3"
```

Append a new bench target at the end of the file (after the existing `[[bench]] name = "optimizations"` block):

```toml
[[bench]]
name = "graph"
harness = false
```

- [ ] **Step 2: Create `algo_lib/benches/graph.rs` with scaffold + Dijkstra bench**

```rust
//! Criterion benches for `algo_lib::graph::*`.
//!
//! Run all:        cargo bench --bench graph
//! Save baseline:  cargo bench --bench graph -- --save-baseline pre
//! Compare:        cargo bench --bench graph -- --baseline pre

use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::Graph;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

// ---------------------------------------------------------------------------
// Generators (seeded for bit-for-bit reproducibility).
// ---------------------------------------------------------------------------

/// Sparse Erdős–Rényi undirected weighted graph: `m = 2 * n` random edges,
/// weights in `1..=10^9`. May contain multi-edges (matches CP reality).
fn er_sparse_weighted(n: usize, seed: u64) -> Graph<BiWeightedEdge<u64, ()>> {
    let mut rng = ChaCha20Rng::seed_from_u64(seed);
    let m = 2 * n;
    let mut g: Graph<BiWeightedEdge<u64, ()>> = Graph::new(n);
    for _ in 0..m {
        let u = rng.gen_range(0..n);
        let mut v = rng.gen_range(0..n);
        while v == u {
            v = rng.gen_range(0..n);
        }
        let w = rng.gen_range(1u64..=1_000_000_000);
        g.add_edge(BiWeightedEdge::new(u, v, w));
    }
    g
}

// ---------------------------------------------------------------------------
// Benches.
// ---------------------------------------------------------------------------

fn bench_dijkstra(c: &mut Criterion) {
    let g = er_sparse_weighted(100_000, 1);
    c.bench_function("dijkstra/er_sparse/n=1e5", |b| {
        b.iter(|| {
            let d = g.distances_from(black_box(0));
            black_box(d);
        });
    });
}

criterion_group!(graph_benches, bench_dijkstra);
criterion_main!(graph_benches);
```

- [ ] **Step 3: Verify it builds**

Run: `cargo bench --bench graph --no-run`

Expected: no errors. Warnings about unused imports are OK at this stage — they go away as more benches are added.

- [ ] **Step 4: Verify it runs (short sample)**

Run: `cargo bench --bench graph -- --sample-size 10 dijkstra`

Expected: one bench reports `dijkstra/er_sparse/n=1e5  time: [X ms Y ms Z ms]`. The number should be in the milliseconds range (Dijkstra on 10⁵ vertices, sparse).

- [ ] **Step 5: Commit**

```bash
git add algo_lib/Cargo.toml algo_lib/benches/graph.rs
git commit -m "feat(algo_lib): scaffold criterion graph bench (Dijkstra smoke)"
```

---

### Task 2: All remaining generators

Add the rest of the input generators in one batch. No new benches yet — just generators ready for the algorithm benches.

**Files:**
- Modify: `algo_lib/benches/graph.rs`

- [ ] **Step 1: Add directed/0-1/dense ER + tree + path + bipartite + disjoint generators**

Replace the `// Generators` section's body (between the comment and the `// Benches` divider) with:

```rust
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::flow_edge::FlowEdge;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::edges::weighted_flow_edge::WeightedFlowEdge;

/// Undirected ER, unweighted (`BiEdge`). `m = 2n` edges, no self-loops.
fn er_sparse_bi(n: usize, seed: u64) -> Graph<BiEdge<()>> {
    let mut rng = ChaCha20Rng::seed_from_u64(seed);
    let m = 2 * n;
    let mut g: Graph<BiEdge<()>> = Graph::new(n);
    for _ in 0..m {
        let u = rng.gen_range(0..n);
        let mut v = rng.gen_range(0..n);
        while v == u {
            v = rng.gen_range(0..n);
        }
        g.add_edge(BiEdge::new(u, v));
    }
    g
}

/// Directed ER, unweighted. `m = 2n` edges.
fn er_sparse_directed(n: usize, seed: u64) -> Graph<Edge<()>> {
    let mut rng = ChaCha20Rng::seed_from_u64(seed);
    let m = 2 * n;
    let mut g: Graph<Edge<()>> = Graph::new(n);
    for _ in 0..m {
        let u = rng.gen_range(0..n);
        let mut v = rng.gen_range(0..n);
        while v == u {
            v = rng.gen_range(0..n);
        }
        g.add_edge(Edge::new(u, v));
    }
    g
}

/// DAG: `m = 2n` directed edges from smaller to larger index.
fn dag_sparse(n: usize, seed: u64) -> Graph<Edge<()>> {
    let mut rng = ChaCha20Rng::seed_from_u64(seed);
    let m = 2 * n;
    let mut g: Graph<Edge<()>> = Graph::new(n);
    for _ in 0..m {
        let a = rng.gen_range(0..n);
        let b = rng.gen_range(0..n);
        if a == b {
            continue;
        }
        let (u, v) = if a < b { (a, b) } else { (b, a) };
        g.add_edge(Edge::new(u, v));
    }
    g
}

/// 0-1 weighted undirected ER for `edge_distances`.
fn er_sparse_01(n: usize, seed: u64) -> Graph<BiWeightedEdge<u32, ()>> {
    let mut rng = ChaCha20Rng::seed_from_u64(seed);
    let m = 2 * n;
    let mut g: Graph<BiWeightedEdge<u32, ()>> = Graph::new(n);
    for _ in 0..m {
        let u = rng.gen_range(0..n);
        let mut v = rng.gen_range(0..n);
        while v == u {
            v = rng.gen_range(0..n);
        }
        let w = if rng.gen_bool(0.5) { 0u32 } else { 1u32 };
        g.add_edge(BiWeightedEdge::new(u, v, w));
    }
    g
}

/// Directed weighted ER for Bellman-Ford.
fn er_sparse_weighted_directed(n: usize, seed: u64) -> Graph<WeightedEdge<i64, ()>> {
    let mut rng = ChaCha20Rng::seed_from_u64(seed);
    let m = 2 * n;
    let mut g: Graph<WeightedEdge<i64, ()>> = Graph::new(n);
    for _ in 0..m {
        let u = rng.gen_range(0..n);
        let mut v = rng.gen_range(0..n);
        while v == u {
            v = rng.gen_range(0..n);
        }
        // mostly positive weights so the graph has finite shortest paths
        let w = rng.gen_range(-100i64..=1000i64);
        g.add_edge(WeightedEdge::new(u, v, w));
    }
    g
}

/// Dense ER weighted (directed) for Floyd-Warshall. `m ≈ n²/4`.
fn er_dense_weighted_directed(n: usize, seed: u64) -> Graph<WeightedEdge<u64, ()>> {
    let mut rng = ChaCha20Rng::seed_from_u64(seed);
    let m = (n * n) / 4;
    let mut g: Graph<WeightedEdge<u64, ()>> = Graph::new(n);
    for _ in 0..m {
        let u = rng.gen_range(0..n);
        let mut v = rng.gen_range(0..n);
        while v == u {
            v = rng.gen_range(0..n);
        }
        let w = rng.gen_range(1u64..=1_000_000);
        g.add_edge(WeightedEdge::new(u, v, w));
    }
    g
}

/// Uniform-random labelled tree via Prüfer sequence. Vertex 0 is the root.
fn random_tree(n: usize, seed: u64) -> Graph<BiEdge<()>> {
    let mut rng = ChaCha20Rng::seed_from_u64(seed);
    let mut g: Graph<BiEdge<()>> = Graph::new(n);
    if n <= 1 {
        return g;
    }
    if n == 2 {
        g.add_edge(BiEdge::new(0, 1));
        return g;
    }
    // Prüfer sequence of length n-2
    let prufer: Vec<usize> = (0..n - 2).map(|_| rng.gen_range(0..n)).collect();
    let mut degree = vec![1usize; n];
    for &x in &prufer {
        degree[x] += 1;
    }
    let mut ptr = 0usize;
    while degree[ptr] != 1 {
        ptr += 1;
    }
    let mut leaf = ptr;
    for &v in &prufer {
        g.add_edge(BiEdge::new(leaf, v));
        degree[v] -= 1;
        if degree[v] == 1 && v < ptr {
            leaf = v;
        } else {
            ptr += 1;
            while degree[ptr] != 1 {
                ptr += 1;
            }
            leaf = ptr;
        }
    }
    g.add_edge(BiEdge::new(leaf, n - 1));
    g
}

/// Long path 0—1—2—…—n-1 as BiEdge graph.
#[allow(dead_code)]
fn path(n: usize) -> Graph<BiEdge<()>> {
    let mut g: Graph<BiEdge<()>> = Graph::new(n);
    for i in 0..n.saturating_sub(1) {
        g.add_edge(BiEdge::new(i, i + 1));
    }
    g
}

/// Dense complete bipartite flow graph with random capacities in `1..=100`.
/// Source = `left + right`, sink = `left + right + 1`. Each left node has a
/// source-edge (cap 1); each right node has a sink-edge (cap 1); every
/// left-right pair gets a forward edge with a random small capacity.
fn dense_bipartite_flow(
    left: usize,
    right: usize,
    seed: u64,
) -> (Graph<FlowEdge<u64, ()>>, usize, usize) {
    let mut rng = ChaCha20Rng::seed_from_u64(seed);
    let n = left + right + 2;
    let source = left + right;
    let sink = left + right + 1;
    let mut g: Graph<FlowEdge<u64, ()>> = Graph::new(n);
    for i in 0..left {
        g.add_edge(FlowEdge::new(source, i, 1));
    }
    for j in 0..right {
        g.add_edge(FlowEdge::new(left + j, sink, 1));
    }
    for i in 0..left {
        for j in 0..right {
            let c = rng.gen_range(1u64..=100);
            g.add_edge(FlowEdge::new(i, left + j, c));
        }
    }
    (g, source, sink)
}

/// Like `dense_bipartite_flow` but with random per-edge costs in `0..=100`.
fn dense_bipartite_mcmf(
    left: usize,
    right: usize,
    seed: u64,
) -> (Graph<WeightedFlowEdge<i64, i64, ()>>, usize, usize) {
    let mut rng = ChaCha20Rng::seed_from_u64(seed);
    let n = left + right + 2;
    let source = left + right;
    let sink = left + right + 1;
    let mut g: Graph<WeightedFlowEdge<i64, i64, ()>> = Graph::new(n);
    for i in 0..left {
        g.add_edge(WeightedFlowEdge::new(source, i, 0, 1));
    }
    for j in 0..right {
        g.add_edge(WeightedFlowEdge::new(left + j, sink, 0, 1));
    }
    for i in 0..left {
        for j in 0..right {
            let cost = rng.gen_range(0i64..=100);
            let cap = rng.gen_range(1i64..=100);
            g.add_edge(WeightedFlowEdge::new(i, left + j, cost, cap));
        }
    }
    (g, source, sink)
}

/// Two random Hamiltonian cycles superimposed → every vertex has degree 4 →
/// Eulerian circuit exists. Used for the `euler_path` bench.
fn two_hamiltonians(n: usize, seed: u64) -> Graph<BiEdge<()>> {
    use rand::seq::SliceRandom;
    let mut rng = ChaCha20Rng::seed_from_u64(seed);
    let mut g: Graph<BiEdge<()>> = Graph::new(n);
    for _ in 0..2 {
        let mut perm: Vec<usize> = (0..n).collect();
        perm.shuffle(&mut rng);
        for i in 0..n {
            g.add_edge(BiEdge::new(perm[i], perm[(i + 1) % n]));
        }
    }
    g
}
```

Keep the existing `er_sparse_weighted` (used by Dijkstra). Move the original line `use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;` so all `use algo_lib::graph::edges::*` lines sit together at the top.

- [ ] **Step 2: Verify it builds**

Run: `cargo bench --bench graph --no-run`

Expected: no errors. Warnings about unused functions are expected (the generators are used in later tasks).

- [ ] **Step 3: Commit**

```bash
git add algo_lib/benches/graph.rs
git commit -m "feat(algo_lib/bench): add full graph-input generator set"
```

---

### Task 3: Shortest-path benches

Add Dijkstra (already there, just leave it), 0-1 BFS, Bellman-Ford, Floyd-Warshall.

**Files:**
- Modify: `algo_lib/benches/graph.rs`

- [ ] **Step 1: Add imports for shortest-path traits**

At the top imports (after the existing `use algo_lib::graph::distances::Distances;` line), insert:

```rust
use algo_lib::graph::all_distances::AllDistances;
use algo_lib::graph::edge_distances::EdgeDistances;
use algo_lib::graph::negative_distances::NegativeDistances;
```

- [ ] **Step 2: Add bench functions**

In the `// Benches.` section, after `bench_dijkstra`, append:

```rust
fn bench_edge_distances(c: &mut Criterion) {
    let g = er_sparse_01(100_000, 2);
    c.bench_function("edge_distances/01_BFS/n=1e5", |b| {
        b.iter(|| {
            let d = g.edge_distances(black_box(0));
            black_box(d);
        });
    });
}

fn bench_negative_distances(c: &mut Criterion) {
    let g = er_sparse_weighted_directed(2_000, 3);
    c.bench_function("bellman_ford/er_sparse/n=2000", |b| {
        b.iter(|| {
            let d = g.negative_distances_from(black_box(0));
            black_box(d);
        });
    });
}

fn bench_all_distances(c: &mut Criterion) {
    let g = er_dense_weighted_directed(500, 4);
    c.bench_function("floyd_warshall/er_dense/n=500", |b| {
        b.iter(|| {
            let d = g.all_distances();
            black_box(d);
        });
    });
}
```

- [ ] **Step 3: Register in `criterion_group!`**

Replace the existing `criterion_group!(graph_benches, bench_dijkstra);` line with:

```rust
criterion_group!(
    graph_benches,
    bench_dijkstra,
    bench_edge_distances,
    bench_negative_distances,
    bench_all_distances,
);
```

- [ ] **Step 4: Verify build and quick run**

Run: `cargo bench --bench graph --no-run`
Expected: no errors.

Run: `cargo bench --bench graph -- --sample-size 10 floyd_warshall`
Expected: one bench reports a time; Floyd-Warshall on n=500 should be in the tens-to-hundreds of ms range.

- [ ] **Step 5: Commit**

```bash
git add algo_lib/benches/graph.rs
git commit -m "feat(algo_lib/bench): add shortest-path benches (Dijkstra/0-1 BFS/Bellman-Ford/Floyd-Warshall)"
```

---

### Task 4: Connectivity benches

SCC, bridges, cut points, block-cut tree, 2-SAT.

**Files:**
- Modify: `algo_lib/benches/graph.rs`

- [ ] **Step 1: Add imports**

After the existing trait imports (top of file), add:

```rust
use algo_lib::graph::block_cut_tree::BlockCutTreeBuild;
use algo_lib::graph::bridges::BridgeSearch;
use algo_lib::graph::cut_points::CutPointSearch;
use algo_lib::graph::strongly_connected_components::StronglyConnectedComponentsTrait;
use algo_lib::graph::two_sat::TwoSat;
```

- [ ] **Step 2: Add bench functions**

Append after `bench_all_distances` in the `// Benches.` section:

```rust
fn bench_scc(c: &mut Criterion) {
    let g = er_sparse_directed(100_000, 5);
    c.bench_function("scc/er_sparse_directed/n=1e5", |b| {
        b.iter(|| {
            let r = g.strongly_connected_components();
            black_box(r);
        });
    });
}

fn bench_bridges(c: &mut Criterion) {
    let g = er_sparse_bi(100_000, 6);
    c.bench_function("bridges/er_sparse_bi/n=1e5", |b| {
        b.iter(|| {
            let r = g.bridges();
            black_box(r);
        });
    });
}

fn bench_cut_points(c: &mut Criterion) {
    let g = er_sparse_bi(100_000, 7);
    c.bench_function("cut_points/er_sparse_bi/n=1e5", |b| {
        b.iter(|| {
            let r = g.cut_points();
            black_box(r);
        });
    });
}

fn bench_block_cut_tree(c: &mut Criterion) {
    let g = er_sparse_bi(100_000, 8);
    c.bench_function("block_cut_tree/er_sparse_bi/n=1e5", |b| {
        b.iter(|| {
            let r = g.block_cut_tree();
            black_box(r);
        });
    });
}

fn bench_two_sat(c: &mut Criterion) {
    // 10⁵ variables, ~2·10⁵ random clauses. The clause list is built ONCE
    // (deterministic from the seed), then each iter rebuilds a fresh TwoSat
    // by replaying the same clauses. This keeps the input identical across
    // batches so timing is stable.
    let n = 100_000;
    let m = 200_000;
    let mut rng = ChaCha20Rng::seed_from_u64(9);
    let clauses: Vec<(usize, bool, usize, bool)> = (0..m)
        .map(|_| {
            (
                rng.gen_range(0..n),
                rng.gen_bool(0.5),
                rng.gen_range(0..n),
                rng.gen_bool(0.5),
            )
        })
        .collect();
    c.bench_function("two_sat/n=1e5/m=2e5", |b| {
        b.iter_batched(
            || {
                let mut ts = TwoSat::new(n);
                for &(a, av, bx, bv) in &clauses {
                    ts.add_or(a, av, bx, bv);
                }
                ts
            },
            |ts| {
                let r = ts.solve();
                black_box(r);
            },
            criterion::BatchSize::SmallInput,
        );
    });
}
```

- [ ] **Step 3: Register in `criterion_group!`**

Append to the existing macro invocation:

```rust
criterion_group!(
    graph_benches,
    bench_dijkstra,
    bench_edge_distances,
    bench_negative_distances,
    bench_all_distances,
    bench_scc,
    bench_bridges,
    bench_cut_points,
    bench_block_cut_tree,
    bench_two_sat,
);
```

- [ ] **Step 4: Verify build**

Run: `cargo bench --bench graph --no-run`
Expected: no errors.

- [ ] **Step 5: Commit**

```bash
git add algo_lib/benches/graph.rs
git commit -m "feat(algo_lib/bench): add connectivity benches (SCC/bridges/cut points/BCT/2-SAT)"
```

---

### Task 5: Other linear-time benches

DFS order, topological sort, Euler path, MST.

**Files:**
- Modify: `algo_lib/benches/graph.rs`

- [ ] **Step 1: Add imports**

```rust
use algo_lib::graph::dfs_order::DFSOrderTrait;
use algo_lib::graph::euler_path::EulerPath;
use algo_lib::graph::minimal_spanning_tree::MinimalSpanningTree;
use algo_lib::graph::topological_sort::TopologicalSort;
```

- [ ] **Step 2: Add bench functions**

```rust
fn bench_dfs_order(c: &mut Criterion) {
    let g = er_sparse_bi(100_000, 10);
    c.bench_function("dfs_order/er_sparse_bi/n=1e5", |b| {
        b.iter(|| {
            let r = g.dfs_order();
            black_box(r);
        });
    });
}

fn bench_topological_sort(c: &mut Criterion) {
    let g = dag_sparse(100_000, 11);
    c.bench_function("topological_sort/dag_sparse/n=1e5", |b| {
        b.iter(|| {
            let r = g.topological_sort();
            black_box(r);
        });
    });
}

fn bench_euler_path(c: &mut Criterion) {
    let g = two_hamiltonians(100_000, 12);
    c.bench_function("euler_path/two_hams/n=1e5", |b| {
        b.iter(|| {
            let r = g.euler_path();
            black_box(r);
        });
    });
}

fn bench_mst(c: &mut Criterion) {
    let g = er_sparse_weighted(100_000, 13);
    c.bench_function("mst/er_sparse_weighted/n=1e5", |b| {
        b.iter(|| {
            let r = g.minimal_spanning_tree();
            black_box(r);
        });
    });
}
```

- [ ] **Step 3: Register**

Update `criterion_group!` (append the four new functions in the order above).

- [ ] **Step 4: Verify build**

Run: `cargo bench --bench graph --no-run`
Expected: no errors.

- [ ] **Step 5: Commit**

```bash
git add algo_lib/benches/graph.rs
git commit -m "feat(algo_lib/bench): add DFS-order/topo-sort/Euler-path/MST benches"
```

---

### Task 6: Tree benches (build phase)

Central decomposition, LCA build, HL build.

**Files:**
- Modify: `algo_lib/benches/graph.rs`

- [ ] **Step 1: Add imports**

```rust
use algo_lib::graph::central_decomposition::Decompose;
use algo_lib::graph::hl_decomposition::HLDecompositionTrait;
use algo_lib::graph::lca::LCATrait;
```

- [ ] **Step 2: Add bench functions**

```rust
fn bench_central_decomposition(c: &mut Criterion) {
    let g = random_tree(100_000, 20);
    c.bench_function("central_decomposition/tree/n=1e5", |b| {
        b.iter(|| {
            let mut count = 0usize;
            (&g).decompose(|center, _mask| {
                count = count.wrapping_add(center);
            });
            black_box(count);
        });
    });
}

fn bench_lca_build(c: &mut Criterion) {
    let g = random_tree(100_000, 21);
    c.bench_function("lca/build/tree/n=1e5", |b| {
        b.iter(|| {
            let l = g.lca();
            black_box(l);
        });
    });
}

fn bench_hl_build(c: &mut Criterion) {
    let g = random_tree(100_000, 22);
    c.bench_function("hl_decomposition/build/tree/n=1e5", |b| {
        b.iter(|| {
            let h = g.hl_decomposition();
            black_box(h);
        });
    });
}
```

Note: `Decompose<F>` has a free `F` parameter — call it as `(&g).decompose(|c, _| ...)`. The trait is `impl<F: FnMut(usize, &BitSet)> Decompose<F> for Graph<...>`; the closure's body must be cheap (we use `wrapping_add` for an opaque side-effect).

- [ ] **Step 3: Register**

Append to `criterion_group!`.

- [ ] **Step 4: Verify build**

Run: `cargo bench --bench graph --no-run`
Expected: no errors.

- [ ] **Step 5: Commit**

```bash
git add algo_lib/benches/graph.rs
git commit -m "feat(algo_lib/bench): add tree-build benches (central decomp / LCA / HL)"
```

---

### Task 7: Tree query benches

LCA: 10⁵ random pair queries on a precomputed structure. HL path-iter: 10⁵ queries `iter(u..=root)` walking each query's HL parts.

**Files:**
- Modify: `algo_lib/benches/graph.rs`

- [ ] **Step 1: Add LCA query bench**

```rust
fn bench_lca_query(c: &mut Criterion) {
    let g = random_tree(100_000, 21);
    let lca = g.lca();
    let n = 100_000;
    let q = 100_000;
    let mut rng = ChaCha20Rng::seed_from_u64(30);
    let queries: Vec<(usize, usize)> = (0..q)
        .map(|_| (rng.gen_range(0..n), rng.gen_range(0..n)))
        .collect();
    c.bench_function("lca/query/n=1e5/q=1e5", |b| {
        b.iter(|| {
            let mut acc = 0usize;
            for &(u, v) in &queries {
                acc = acc.wrapping_add(lca.lca(u, v));
            }
            black_box(acc);
        });
    });
}
```

- [ ] **Step 2: Add HL path-iter bench**

```rust
fn bench_hl_query(c: &mut Criterion) {
    let g = random_tree(100_000, 22);
    let hld = g.hl_decomposition();
    let root = hld.root;
    let n = 100_000;
    let q = 100_000;
    let mut rng = ChaCha20Rng::seed_from_u64(31);
    let starts: Vec<usize> = (0..q).map(|_| rng.gen_range(0..n)).collect();
    c.bench_function("hl_decomposition/path_iter/n=1e5/q=1e5", |b| {
        b.iter(|| {
            let mut acc = 0usize;
            for &u in &starts {
                for part in hld.iter(u..=root) {
                    acc = acc
                        .wrapping_add(part.id)
                        .wrapping_add(part.pos_from)
                        .wrapping_add(part.pos_to);
                }
            }
            black_box(acc);
        });
    });
}
```

- [ ] **Step 3: Register**

Append to `criterion_group!`.

- [ ] **Step 4: Verify build**

Run: `cargo bench --bench graph --no-run`
Expected: no errors.

- [ ] **Step 5: Commit**

```bash
git add algo_lib/benches/graph.rs
git commit -m "feat(algo_lib/bench): add LCA + HL query benches"
```

---

### Task 8: Flow benches

`max_flow` (Dinic), `fast_max_flow` (LCT Dinic), `flow_with_demand`. All mutate the graph; use `iter_batched`.

**Files:**
- Modify: `algo_lib/benches/graph.rs`

- [ ] **Step 1: Add imports**

```rust
use algo_lib::graph::fast_max_flow::FastMaxFlow;
use algo_lib::graph::flow_with_demand::FlowWithDemand;
use algo_lib::graph::max_flow::MaxFlow;
```

Also bring `BatchSize` into scope if not already done; replace the existing `use criterion::{black_box, criterion_group, criterion_main, Criterion};` with:

```rust
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
```

- [ ] **Step 2: Add bench functions**

```rust
fn bench_max_flow(c: &mut Criterion) {
    let (g0, source, sink) = dense_bipartite_flow(500, 500, 40);
    c.bench_function("max_flow/dinic/bipartite_500x500", |b| {
        b.iter_batched(
            || g0.clone(),
            |mut g| {
                let f = g.max_flow(source, sink);
                black_box(f);
            },
            BatchSize::SmallInput,
        );
    });
}

fn bench_fast_max_flow(c: &mut Criterion) {
    let (g0, source, sink) = dense_bipartite_flow(500, 500, 41);
    c.bench_function("max_flow/lct_dinic/bipartite_500x500", |b| {
        b.iter_batched(
            || g0.clone(),
            |mut g| {
                let f = g.fast_max_flow(source, sink);
                black_box(f);
            },
            BatchSize::SmallInput,
        );
    });
}

fn bench_flow_with_demand(c: &mut Criterion) {
    // Reuse the same bipartite graph shape; with non-zero demands the result
    // might be infeasible, but `flow_with_demand` still does the work we want
    // to measure. Inputs are deterministic so timing is stable.
    let (g0, source, sink) = dense_bipartite_flow(500, 500, 42);
    c.bench_function("flow_with_demand/bipartite_500x500", |b| {
        b.iter_batched(
            || g0.clone(),
            |mut g| {
                let ok = g.flow_with_demand(source, sink);
                black_box(ok);
            },
            BatchSize::SmallInput,
        );
    });
}
```

Note: `Graph<E>` derives `Clone` because `E` does (verify by `grep -n "derive.*Clone" algo_lib/src/graph/mod.rs`). If it doesn't derive Clone, build the graph inside the setup closure instead by adding a helper that takes the seed and rebuilds.

- [ ] **Step 3: Register**

Append the three functions to `criterion_group!`.

- [ ] **Step 4: Verify build**

Run: `cargo bench --bench graph --no-run`
Expected: no errors.

- [ ] **Step 5: Commit**

```bash
git add algo_lib/benches/graph.rs
git commit -m "feat(algo_lib/bench): add flow benches (Dinic / LCT-Dinic / flow-with-demand)"
```

---

### Task 9: Min-cost flow benches

`min_cost_flow`, `min_cost_max_flow`, `min_cost_flow_slow`, `min_cost_max_flow_slow` — all four, on a smaller graph (`200 × 200`) because they're slower.

**Files:**
- Modify: `algo_lib/benches/graph.rs`

- [ ] **Step 1: Add imports**

```rust
use algo_lib::graph::min_cost_flow::MinCostFlow;
use algo_lib::graph::min_cost_flow_slow::MinCostFlowSlow;
```

- [ ] **Step 2: Add bench functions**

```rust
fn bench_min_cost_flow(c: &mut Criterion) {
    let (g0, source, sink) = dense_bipartite_mcmf(200, 200, 50);
    c.bench_function("min_cost_flow/bipartite_200x200", |b| {
        b.iter_batched(
            || g0.clone(),
            |mut g| {
                let r = g.min_cost_flow(source, sink);
                black_box(r);
            },
            BatchSize::SmallInput,
        );
    });
}

fn bench_min_cost_max_flow(c: &mut Criterion) {
    let (g0, source, sink) = dense_bipartite_mcmf(200, 200, 51);
    c.bench_function("min_cost_max_flow/bipartite_200x200", |b| {
        b.iter_batched(
            || g0.clone(),
            |mut g| {
                let r = g.min_cost_max_flow(source, sink);
                black_box(r);
            },
            BatchSize::SmallInput,
        );
    });
}

fn bench_min_cost_flow_slow(c: &mut Criterion) {
    let (g0, source, sink) = dense_bipartite_mcmf(200, 200, 52);
    c.bench_function("min_cost_flow_slow/bipartite_200x200", |b| {
        b.iter_batched(
            || g0.clone(),
            |mut g| {
                let r = g.min_cost_flow_slow(source, sink);
                black_box(r);
            },
            BatchSize::SmallInput,
        );
    });
}

fn bench_min_cost_max_flow_slow(c: &mut Criterion) {
    let (g0, source, sink) = dense_bipartite_mcmf(200, 200, 53);
    c.bench_function("min_cost_max_flow_slow/bipartite_200x200", |b| {
        b.iter_batched(
            || g0.clone(),
            |mut g| {
                let r = g.min_cost_max_flow_slow(source, sink);
                black_box(r);
            },
            BatchSize::SmallInput,
        );
    });
}
```

- [ ] **Step 3: Register**

Append the four functions to `criterion_group!`.

- [ ] **Step 4: Verify build**

Run: `cargo bench --bench graph --no-run`
Expected: no errors.

- [ ] **Step 5: Commit**

```bash
git add algo_lib/benches/graph.rs
git commit -m "feat(algo_lib/bench): add MCMF benches (fast + slow variants)"
```

---

### Task 10: Coverage comment + full bench run

Add a coverage-checklist comment at the top of `algo_lib/benches/graph.rs`, then run the full suite end-to-end to verify nothing regresses on compile or runtime.

**Files:**
- Modify: `algo_lib/benches/graph.rs`

- [ ] **Step 1: Add coverage comment block**

Insert just after the existing top-of-file doc comment (after the `//! Compare:` line, before any `use` statements):

```rust
//
// Coverage of algo_lib/src/graph (see also docs/superpowers/specs/2026-05-16-...).
//
//   all_distances              -> bench_all_distances
//   block_cut_tree             -> bench_block_cut_tree
//   bridges                    -> bench_bridges
//   central_decomposition      -> bench_central_decomposition
//   cut_points                 -> bench_cut_points
//   dfs_order                  -> bench_dfs_order
//   distances                  -> bench_dijkstra
//   edge_distances             -> bench_edge_distances
//   euler_path                 -> bench_euler_path
//   fast_max_flow              -> bench_fast_max_flow
//   flow_graph                 -> SKIPPED (primitive, covered via flow algos)
//   flow_with_demand           -> bench_flow_with_demand
//   hl_decomposition           -> bench_hl_build + bench_hl_query
//   lca                        -> bench_lca_build + bench_lca_query
//   max_flow                   -> bench_max_flow
//   min_cost_flow              -> bench_min_cost_flow + bench_min_cost_max_flow
//   min_cost_flow_slow         -> bench_min_cost_flow_slow + bench_min_cost_max_flow_slow
//   minimal_spanning_tree      -> bench_mst
//   negative_distances         -> bench_negative_distances
//   strongly_connected_*       -> bench_scc
//   topological_sort           -> bench_topological_sort
//   two_sat                    -> bench_two_sat
//   edges/*                    -> SKIPPED (primitive containers, exercised via all algos)
```

- [ ] **Step 2: Run the full suite end-to-end (short sample size for speed)**

Run: `cargo bench --bench graph -- --sample-size 10`

Expected:
- All 21 benches complete (count names emitted; the registered function count in `criterion_group!` should be 21).
- No panics, no compile errors.
- Numbers look sane: 10⁵-scale benches in the milliseconds, Floyd-Warshall in tens of ms, MCMF in tens-to-hundreds of ms, max-flow in tens of ms.

If a bench panics: most likely an algorithm doesn't support the chosen input shape (e.g., `euler_path` returning `None`). Inspect the failing bench and adjust input shape, then re-commit as part of this task.

- [ ] **Step 3: Save the baseline**

Run: `cargo bench --bench graph -- --save-baseline pre`

This is the "before refactor" reference. Subsequent runs with `--baseline pre` will compare against this. Criterion stores baselines under `target/criterion/` — they're untracked by git (expected; baselines are local).

- [ ] **Step 4: Commit**

```bash
git add algo_lib/benches/graph.rs
git commit -m "docs(algo_lib/bench): coverage checklist mapping graph algos to benches"
```
