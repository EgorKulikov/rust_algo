//! Criterion benches for `algo_lib::graph::*`.
//!
//! Run all:        cargo bench --bench graph
//! Save baseline:  cargo bench --bench graph -- --save-baseline pre
//! Compare:        cargo bench --bench graph -- --baseline pre

use algo_lib::graph::all_distances::AllDistances;
use algo_lib::graph::block_cut_tree::BlockCutTreeBuild;
use algo_lib::graph::bridges::BridgeSearch;
use algo_lib::graph::central_decomposition::Decompose;
use algo_lib::graph::cut_points::CutPointSearch;
use algo_lib::graph::dfs_order::DFSOrderTrait;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edge_distances::EdgeAlgos;
use algo_lib::graph::euler_path::EulerPath;
use algo_lib::graph::hl_decomposition::HLDecompositionTrait;
use algo_lib::graph::lca::LCATrait;
use algo_lib::graph::minimal_spanning_tree::MinimalSpanningTree;
use algo_lib::graph::negative_distances::NegativeDistances;
use algo_lib::graph::strongly_connected_components::StronglyConnectedComponentsTrait;
use algo_lib::graph::topological_sort::TopologicalSort;
use algo_lib::graph::two_sat::TwoSat;
use algo_lib::graph::fast_max_flow::FastMaxFlow;
use algo_lib::graph::flow_with_demand::FlowWithDemand;
use algo_lib::graph::max_flow::MaxFlow;
use algo_lib::graph::min_cost_flow::MinCostFlow;
use algo_lib::graph::min_cost_flow_slow::MinCostFlowSlow;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::bi_edge::BiEdgeWithId;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::flow_edge::FlowEdge;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::edges::weighted_flow_edge::WeightedFlowEdge;
use algo_lib::graph::Graph;
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
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

/// Uniform-random labelled tree via Prüfer sequence.
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

/// Like `dense_bipartite_flow` but with non-zero lower bounds (demands) on
/// each edge so that `flow_with_demand` has something non-trivial to do.
/// Returns `Graph<FlowEdge<u64, u64>>` (payload = lower bound = demand).
fn dense_bipartite_flow_with_demand(
    left: usize,
    right: usize,
    seed: u64,
) -> (Graph<FlowEdge<u64, u64>>, usize, usize) {
    let mut rng = ChaCha20Rng::seed_from_u64(seed);
    let n = left + right + 2;
    let source = left + right;
    let sink = left + right + 1;
    let mut g: Graph<FlowEdge<u64, u64>> = Graph::new(n);
    for i in 0..left {
        // demand = 0, cap = 1
        g.add_edge(FlowEdge::with_payload(source, i, 1, 0));
    }
    for j in 0..right {
        g.add_edge(FlowEdge::with_payload(left + j, sink, 1, 0));
    }
    for i in 0..left {
        for j in 0..right {
            let cap: u64 = rng.gen_range(2u64..=100);
            let demand: u64 = rng.gen_range(0u64..=1); // small demand so feasibility likely
            g.add_edge(FlowEdge::with_payload(i, left + j, cap, demand));
        }
    }
    (g, source, sink)
}

/// Like `dense_bipartite_flow` but with random per-edge costs.
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

/// Two random Hamiltonian cycles superimposed (degree 4 everywhere → Eulerian).
fn two_hamiltonians(n: usize, seed: u64) -> Graph<BiEdgeWithId<()>> {
    use rand::seq::SliceRandom;
    let mut rng = ChaCha20Rng::seed_from_u64(seed);
    let mut g: Graph<BiEdgeWithId<()>> = Graph::new(n);
    for _ in 0..2 {
        let mut perm: Vec<usize> = (0..n).collect();
        perm.shuffle(&mut rng);
        for i in 0..n {
            g.add_edge(BiEdgeWithId::new(perm[i], perm[(i + 1) % n]));
        }
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
    let (g0, source, sink) = dense_bipartite_flow_with_demand(500, 500, 42);
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
    bench_dfs_order,
    bench_topological_sort,
    bench_euler_path,
    bench_mst,
    bench_central_decomposition,
    bench_lca_build,
    bench_hl_build,
    bench_lca_query,
    bench_hl_query,
    bench_max_flow,
    bench_fast_max_flow,
    bench_flow_with_demand,
    bench_min_cost_flow,
    bench_min_cost_max_flow,
    bench_min_cost_flow_slow,
    bench_min_cost_max_flow_slow,
);
criterion_main!(graph_benches);
