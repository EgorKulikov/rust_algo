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
