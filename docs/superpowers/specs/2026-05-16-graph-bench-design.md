# Graph Algorithm Benchmark Suite

## Goal

Establish a reproducible performance baseline for every algorithm in
`algo_lib/src/graph/` so an upcoming refactor of the underlying graph data
structure can be evaluated with `cargo bench --baseline pre`. Numbers must be
stable enough that a few-percent regression is detectable above noise, and the
suite must be quick enough to re-run after every iteration of the refactor.

## Non-Goals

- Verifying algorithm correctness — existing tests in `algo_lib/tests/`
  already do that. The bench inputs are chosen so the algorithms produce
  non-trivial output (no empty graphs / degenerate cases), but we don't
  cross-check results against reference implementations.
- Benchmarking the building primitives themselves (`flow_graph`,
  `graph/edges/*`). They get coverage indirectly through the algorithms that
  use them.
- A full input-size sweep. We bench each algorithm at one representative
  size — the size at which it's typically used in competitive programming.
  Scaling curves are out of scope.

## Framework

`criterion = "0.5"` with the `html_reports` feature, as a `[dev-dependencies]`
entry on `algo_lib`. Workflow:

```
cargo bench --bench graph -- --save-baseline pre   # before refactor
# ... refactor ...
cargo bench --bench graph -- --baseline pre        # compare
```

Criterion's `--baseline` mode shows per-bench % change with statistical
significance markers, which is exactly the signal we want.

## File Layout

Single bench file: `algo_lib/benches/graph.rs`. `algo_lib/Cargo.toml` gains:

```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }
rand_chacha = "0.3"
rand = "0.8"

[[bench]]
name = "graph"
harness = false
```

The file is organised into three sections in this order: generators, bench
functions (one per algorithm), and the `criterion_group!` / `criterion_main!`
registration.

## Generators

All generators take a `u64` seed and use a single `ChaCha20Rng` internally so
runs are bit-for-bit reproducible. They return either an edge list
(`Vec<(u32, u32, EdgeWeight)>`) or a tree-as-parent-array, whichever the
consuming algorithm expects.

- `er_sparse(n, seed)` — `m = 2n` random undirected edges, no self-loops, no
  duplicate edge filter (so multi-edges happen — matches CP reality).
- `er_medium(n, seed)` — `m = n · ceil(log2 n)`.
- `er_dense(n, seed)` — `m ≈ n²/4`.
- `random_tree(n, seed)` — uniform random labelled tree via Prüfer sequence.
- `path(n)` — vertices `0—1—2—…—n-1`.
- `dense_bipartite(left, right, seed)` — every left-right pair gets an edge
  with random capacity (and random cost when needed).
- `disjoint_small(k, n_each, seed)` — `k` independent paths of length
  `n_each`.

Weights and capacities come from `rng.gen_range(1..=W)` with `W` chosen per
algorithm (`W = 10⁹` for shortest-path-style, `W = 100` for flow-style).

## Bench Coverage

One bench function per algorithm. Representative size per algorithm:

| Algorithm | File | Input | Size |
|---|---|---|---|
| `distances` (Dijkstra) | `distances.rs` | sparse ER | n=10⁵, m=2·10⁵ |
| `edge_distances` (0-1 BFS) | `edge_distances.rs` | sparse ER, 0/1 weights | n=10⁵, m=2·10⁵ |
| `negative_distances` (Bellman-Ford) | `negative_distances.rs` | sparse ER | n=2000, m=5000 |
| `all_distances` (Floyd-Warshall) | `all_distances.rs` | dense ER | n=500 |
| `strongly_connected_components` | `strongly_connected_components.rs` | directed sparse ER | n=10⁵, m=2·10⁵ |
| `topological_sort` | `topological_sort.rs` | DAG (sorted endpoints) | n=10⁵, m=2·10⁵ |
| `dfs_order` | `dfs_order.rs` | sparse ER | n=10⁵, m=2·10⁵ |
| `bridges` | `bridges.rs` | undirected sparse ER | n=10⁵, m=2·10⁵ |
| `cut_points` | `cut_points.rs` | undirected sparse ER | n=10⁵, m=2·10⁵ |
| `block_cut_tree` | `block_cut_tree.rs` | undirected sparse ER | n=10⁵, m=2·10⁵ |
| `two_sat` | `two_sat.rs` | random 2-CNF (~2n clauses, mostly satisfiable) | n=10⁵, m=2·10⁵ |
| `euler_path` | `euler_path.rs` | union of two random Hamiltonian cycles (every vertex has degree 4 → Eulerian circuit exists) | n=10⁵, m=2·10⁵ |
| `minimal_spanning_tree` | `minimal_spanning_tree.rs` | undirected weighted sparse ER | n=10⁵, m=2·10⁵ |
| `central_decomposition` | `central_decomposition.rs` | random tree | n=10⁵ |
| `lca` build | `lca.rs` | random tree | n=10⁵ |
| `lca` query | `lca.rs` | 10⁵ random pair queries | n=10⁵ |
| `hl_decomposition` build | `hl_decomposition.rs` | random tree | n=10⁵ |
| `hl_decomposition` path-iter | `hl_decomposition.rs` | 10⁵ random pair paths | n=10⁵ |
| `max_flow` (Dinic) | `max_flow.rs` | dense bipartite | n=500 × 500 |
| `fast_max_flow` (LCT Dinic) | `fast_max_flow.rs` | dense bipartite | n=500 × 500 |
| `flow_with_demand` | `flow_with_demand.rs` | dense bipartite + demands | n=500 × 500 |
| `min_cost_flow` | `min_cost_flow.rs` | dense bipartite with costs | n=200 × 200 |
| `min_cost_flow_slow` | `min_cost_flow_slow.rs` | dense bipartite with costs | n=200 × 200 |

Files explicitly skipped: `flow_graph.rs`, `graph/edges/*` (primitive
containers used through other algos). A comment block at the top of
`benches/graph.rs` lists every `pub` algorithm entry-point and either points
to its bench or to this skip list, so future reviewers can confirm coverage
in one glance.

## Bench Shape Patterns

Three patterns cover all cases:

**(P1) Pure function over an immutable graph** — Dijkstra, BFS, SCC, bridges,
cut points, BCT, DFS order, topo sort, MST, Bellman-Ford, Floyd-Warshall,
0-1 BFS, central decomposition, LCA-build, HL-build, two-SAT, Euler path.

```rust
let g = er_sparse(100_000, 42);
c.bench_function("dijkstra/er_sparse/n=1e5", |b| {
    b.iter(|| distances(&g, 0));
});
```

**(P2) Mutating algorithm** — max-flow variants, MCMF, flow-with-demand. Use
`iter_batched` so graph reconstruction isn't timed.

```rust
c.bench_function("max_flow/bipartite/500", |b| {
    b.iter_batched(
        || build_flow_graph(/* deterministic */),
        |mut g| max_flow(&mut g, source, sink),
        BatchSize::SmallInput,
    );
});
```

**(P3) Query throughput on a precomputed structure** — LCA and HL after
construction. Build the structure once outside `iter`, materialise the query
batch once, time only the query loop.

```rust
let tree = random_tree(n, 7);
let lca = LCA::new(&tree, 0);
let queries: Vec<(u32, u32)> = generate_queries(n, 100_000, 13);
c.bench_function("lca/query/n=1e5", |b| {
    b.iter(|| {
        let mut acc = 0u32;
        for &(u, v) in &queries {
            acc = acc.wrapping_add(lca.lca(u, v) as u32);
        }
        acc  // returned so the loop body isn't dead-code-eliminated
    });
});
```

The query loop accumulates results to prevent the optimiser from elide-ing
the calls; `black_box` is applied where needed.

## Determinism

- Every random input takes an explicit seed; all seeds are constants in the
  bench file. Re-running the same bench against unchanged code produces the
  same input bytes.
- `unstable_sort` / `HashMap` iteration order inside the algorithms can still
  jitter timing slightly; criterion's statistical analysis absorbs this.

## Reporting Workflow

1. Pre-refactor: `cargo bench --bench graph -- --save-baseline pre`. Stores
   results under `target/criterion/<name>/pre`.
2. After each round of refactor: `cargo bench --bench graph -- --baseline pre`.
   Output shows per-bench mean and `change: -X.Y% [significant/no change]`.
3. To compare two refactor variants: `--save-baseline v1`, then
   `--save-baseline v2`, then `--load-baseline v1 --baseline v2`.

HTML reports under `target/criterion/report/index.html` for spot checks.

## Risk / Out-of-Scope Hazards

- **Bench wall time.** Criterion's default 100 samples × 5s warm-up per bench
  → ~20 benches × ~5s each ≈ 2 minutes. Acceptable for an interactive refactor
  loop. If it becomes painful we can lower `sample_size` per bench.
- **Cargo workspace conflict.** `algo_lib` is a workspace member; `cargo
  bench --bench graph -p algo_lib` is the disambiguating form if needed.
- **Bench inputs may shift behaviour over time** if the underlying generators
  change. Generators must not be edited carelessly — the seed + generator
  together are the contract that makes baselines comparable.
- The bench file will be ~500–800 LoC. Acceptable for a single, well-scoped
  benchmark target; if it grows much larger we can revisit splitting by
  category later.

## Done When

- `cargo bench --bench graph` compiles and runs every bench listed above.
- The coverage comment block at the top of the bench file accounts for every
  `pub fn` and `pub fn new` in `algo_lib/src/graph/*.rs` (other than the
  skipped primitives).
- One full bench run completes within ~3 minutes on the dev machine.
- Output for at least one representative bench is verified visually to show
  reasonable numbers (not zero, not seconds-per-iteration).
