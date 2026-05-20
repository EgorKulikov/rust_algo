# Graph storage variant comparison (`Linked` vs `TwoD`) and K trade-off

Median runtimes from `cargo bench --bench graph -- storage_sweep --sample-size
30 --measurement-time 5`. n = 100,000 for Dijkstra/SCC; bipartite flow with
left = 500 for Dinic. Reproducing: `cargo bench --bench graph -- storage_sweep`.

For each row, the **K=4** and **K=8** columns show which storage
`Graph::new(n, m)` would pick at that ratio, and the corresponding runtime
(equal to the Linked or TwoD column). For Dinic the effective `m/n` is
approximately `right` once `left = 500 ≫ right`.

## Dijkstra (`distances_from(0)`, n = 100,000)

| m/n | Linked    | TwoD      | TwoD/Linked | K=4 (≥4)  | K=8 (≥8)  |
|-----|-----------|-----------|-------------|-----------|-----------|
| 1   | 261 µs    | 256 µs    | 0.98×       | Linked    | Linked    |
| 2   | 249 µs    | 284 µs    | 1.14×       | Linked    | Linked    |
| 4   | 206 ms    | 103 ms    | 0.50×       | **TwoD**  | Linked    |
| 8   | 321 ms    | 110 ms    | 0.34×       | **TwoD**  | **TwoD**  |
| 16  | 679 ms    | 116 ms    | 0.17×       | **TwoD**  | **TwoD**  |
| 32  | 1.39 s    | 156 ms    | 0.11×       | **TwoD**  | **TwoD**  |
| 64  | 2.97 s    | 210 ms    | 0.07×       | **TwoD**  | **TwoD**  |

Dijkstra is *the* algorithm where TwoD shines: TwoD is 2× faster at m/n=4
and the gap widens dramatically as the graph densifies (14× at m/n=64).
m/n=1,2 are essentially tied — the graph is so sparse that most vertices
are unreachable from vertex 0 and Dijkstra terminates early.

## SCC (`strongly_connected_components`, n = 100,000)

| m/n | Linked    | TwoD      | TwoD/Linked | K=4 (≥4)  | K=8 (≥8)  |
|-----|-----------|-----------|-------------|-----------|-----------|
| 1   | 7.3 ms    | 11.5 ms   | 1.57×       | Linked    | Linked    |
| 2   | 15.3 ms   | 19.9 ms   | 1.30×       | Linked    | Linked    |
| 4   | 27.6 ms   | 33.0 ms   | 1.20×       | **TwoD**  | Linked    |
| 8   | 72.1 ms   | 57.3 ms   | 0.79×       | **TwoD**  | **TwoD**  |
| 16  | 549 ms    | 181 ms    | 0.33×       | **TwoD**  | **TwoD**  |
| 32  | 1.67 s    | 591 ms    | 0.35×       | **TwoD**  | **TwoD**  |
| 64  | 3.90 s    | 1.41 s    | 0.36×       | **TwoD**  | **TwoD**  |

SCC pulls in the opposite direction: Linked is faster through m/n=4 (where
K=4 already switches to TwoD and pays a 20% penalty). TwoD only starts
winning at m/n ≥ 8.

## Dinic max-flow (bipartite, left = 500)

| right | Effective m/n | Linked   | TwoD     | TwoD/Linked | K=4    | K=8    |
|-------|---------------|----------|----------|-------------|--------|--------|
| 10    | ≈ 10.7        | 92.9 µs  | 89.0 µs  | 0.96×       | TwoD   | TwoD   |
| 30    | ≈ 28.3        | 264 µs   | 222 µs   | 0.84×       | TwoD   | TwoD   |
| 100   | ≈ 83          | 1.05 ms  | 0.70 ms  | 0.66×       | TwoD   | TwoD   |
| 300   | ≈ 187         | 5.21 ms  | 2.34 ms  | 0.45×       | TwoD   | TwoD   |
| 500   | ≈ 250         | 10.8 ms  | 3.13 ms  | 0.29×       | TwoD   | TwoD   |

All Dinic configurations sit at m/n ≫ 8, so both K=4 and K=8 always pick TwoD.
Dinic doesn't constrain K.

## The K=4 vs K=8 trade-off

The disagreement between the algorithms concentrates at m/n=4:

- **Dijkstra at m/n=4:** TwoD is 2× faster (103 ms vs 206 ms). K=4 saves 103 ms.
- **SCC at m/n=4:** Linked is 20% faster (27.6 ms vs 33.0 ms). K=4 loses 5.4 ms.

At m/n=8 both already prefer TwoD; they agree.

If your typical workload is Dijkstra-like (shortest paths, MST-via-Prim, 0-1
BFS, etc.) K=4 is clearly right. If your typical workload is SCC-like
(directed reachability with Tarjan's) K=8 avoids the small regression at the
m/n=4 boundary.

In competitive programming, Dijkstra-on-medium-dense graphs shows up more
often than SCC-on-m/n=4 graphs, and even when both apply, the absolute time
saved by picking TwoD for Dijkstra (~100 ms) dwarfs the time lost by picking
TwoD for SCC (~5 ms). **K=4 is the better default.**

A future option: expose K as a runtime override via
`Graph::new_with_threshold(n, m, K)`, defaulting to 4. The current code uses
a `const K: usize = 4` inside `Graph::new`.
