#![allow(clippy::needless_range_loop)]

use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::flow_edge::FlowEdgeWithId;
use algo_lib::graph::edges::flow_edge_trait::FlowEdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::edges::weighted_flow_edge::WeightedFlowEdgeWithId;
use algo_lib::graph::fast_max_flow::FastMaxFlow;
use algo_lib::graph::flow_with_demand::FlowWithDemand;
use algo_lib::graph::max_flow::MaxFlow;
use algo_lib::graph::min_cost_flow::MinCostFlow;
use algo_lib::graph::Graph;
use algo_lib::misc::random::{Random, RandomTrait};

fn mk<E: EdgeTrait>(n: usize, two_d: bool) -> Graph<E> {
    if two_d {
        Graph::new_2d(n)
    } else {
        Graph::new_linked(n)
    }
}

// ---------------------------------------------------------------- max flow

fn min_cut_brute(n: usize, edges: &[(usize, usize, i64)], s: usize, t: usize) -> i64 {
    let mut best = i64::MAX;
    for mask in 0usize..1 << n {
        if mask >> s & 1 == 0 || mask >> t & 1 == 1 {
            continue;
        }
        let mut cut = 0;
        for &(a, b, c) in edges {
            if mask >> a & 1 == 1 && mask >> b & 1 == 0 {
                cut += c;
            }
        }
        best = best.min(cut);
    }
    best
}

/// Checks the flow stored in the graph; returns net flow out of `s`.
fn check_flow(
    g: &Graph<FlowEdgeWithId<i64, ()>>,
    edges: &[(usize, usize, i64)],
    s: usize,
    t: usize,
) -> i64 {
    let n = g.vertex_count();
    let mut balance = vec![0i64; n];
    let mut seen = vec![0; edges.len()];
    for v in 0..n {
        for e in g.adj(v).iter() {
            let (a, b, c) = edges[e.id()];
            if a == b {
                continue;
            }
            if v == a && e.to() == b {
                seen[e.id()] += 1;
                let f = e.flow(g);
                assert!(0 <= f && f <= c, "flow {f} cap {c}");
                assert_eq!(e.capacity() + f, c);
                balance[a] -= f;
                balance[b] += f;
            }
        }
    }
    for (i, &(a, b, _)) in edges.iter().enumerate() {
        if a != b {
            assert_eq!(seen[i], 1);
        }
    }
    for v in 0..n {
        if v != s && v != t {
            assert_eq!(balance[v], 0, "conservation at {v}");
        }
    }
    assert_eq!(balance[s], -balance[t]);
    -balance[s]
}

#[test]
fn max_flow_vs_min_cut() {
    let mut rng = Random::new_with_seed(1);
    for it in 0..6000 {
        let n = rng.gen_range(2..=7usize);
        let m = rng.gen_range(0..=3 * n);
        let big = it % 5 == 0;
        let edges: Vec<(usize, usize, i64)> = (0..m)
            .map(|_| {
                (
                    rng.gen_range(0..n),
                    rng.gen_range(0..n),
                    if big {
                        rng.gen_range(0..=1_000_000_000_000i64)
                    } else {
                        rng.gen_range(0..=4i64)
                    },
                )
            })
            .collect();
        let s = rng.gen_range(0..n);
        let mut t = rng.gen_range(0..n);
        while t == s {
            t = rng.gen_range(0..n);
        }
        let expected = min_cut_brute(n, &edges, s, t);
        for two_d in [false, true] {
            for fast in [false, true] {
                let mut g: Graph<FlowEdgeWithId<i64, ()>> = mk(n, two_d);
                for &(a, b, c) in &edges {
                    g.add_edge(FlowEdgeWithId::new(a, b, c));
                }
                let got = if fast {
                    g.fast_max_flow(s, t)
                } else {
                    g.max_flow(s, t)
                };
                assert_eq!(got, expected, "fast={fast} n={n} s={s} t={t} {edges:?}");
                assert_eq!(check_flow(&g, &edges, s, t), expected);
                // second call on the same graph: nothing left
                let again = if fast {
                    g.max_flow(s, t)
                } else {
                    g.fast_max_flow(s, t)
                };
                assert_eq!(again, 0);
                assert_eq!(check_flow(&g, &edges, s, t), expected);
            }
        }
    }
}

#[test]
fn max_flow_incremental_edges() {
    // add edges after a flow has been computed, rerun, totals must add up
    let mut rng = Random::new_with_seed(2);
    for _ in 0..2000 {
        let n = rng.gen_range(2..=6usize);
        let (s, t) = (0, n - 1);
        for fast in [false, true] {
            let mut g: Graph<FlowEdgeWithId<i64, ()>> = mk(n, rng.gen_bool());
            let mut edges = Vec::new();
            let mut total = 0;
            for _ in 0..3 {
                for _ in 0..rng.gen_range(0..=2 * n) {
                    let e = (
                        rng.gen_range(0..n),
                        rng.gen_range(0..n),
                        rng.gen_range(0..=5i64),
                    );
                    edges.push(e);
                    g.add_edge(FlowEdgeWithId::new(e.0, e.1, e.2));
                }
                total += if fast {
                    g.fast_max_flow(s, t)
                } else {
                    g.max_flow(s, t)
                };
                assert_eq!(total, min_cut_brute(n, &edges, s, t), "fast={fast} {edges:?}");
                assert_eq!(check_flow(&g, &edges, s, t), total);
            }
        }
    }
}

#[test]
fn max_flow_unsigned_and_i32() {
    let mut rng = Random::new_with_seed(3);
    for _ in 0..2000 {
        let n = rng.gen_range(2..=7usize);
        let m = rng.gen_range(0..=3 * n);
        let edges: Vec<(usize, usize, i64)> = (0..m)
            .map(|_| {
                (
                    rng.gen_range(0..n),
                    rng.gen_range(0..n),
                    rng.gen_range(0..=6i64),
                )
            })
            .collect();
        let expected = min_cut_brute(n, &edges, 0, n - 1);
        let mut gu: Graph<FlowEdgeWithId<u32, ()>> = mk(n, rng.gen_bool());
        let mut gi: Graph<FlowEdgeWithId<i32, ()>> = mk(n, rng.gen_bool());
        let mut gu2 = gu.clone();
        for &(a, b, c) in &edges {
            gu.add_edge(FlowEdgeWithId::new(a, b, c as u32));
            gu2.add_edge(FlowEdgeWithId::new(a, b, c as u32));
            gi.add_edge(FlowEdgeWithId::new(a, b, c as i32));
        }
        assert_eq!(gu.max_flow(0, n - 1) as i64, expected);
        assert_eq!(gu2.fast_max_flow(0, n - 1) as i64, expected);
        assert_eq!(gi.fast_max_flow(0, n - 1) as i64, expected);
    }
}

#[test]
fn fast_max_flow_long_paths() {
    // long layered graphs: the regime fast_max_flow is meant for
    let mut rng = Random::new_with_seed(4);
    for _ in 0..30 {
        let layers = rng.gen_range(50..=300usize);
        let width = rng.gen_range(1..=4usize);
        let n = layers * width + 2;
        let (s, t) = (n - 2, n - 1);
        let mut a: Graph<FlowEdgeWithId<i64, ()>> = Graph::new_linked(n);
        let mut b = a.clone();
        let mut add = |x: usize, y: usize, c: i64| {
            a.add_edge(FlowEdgeWithId::new(x, y, c));
            b.add_edge(FlowEdgeWithId::new(x, y, c));
        };
        for w in 0..width {
            add(s, w, rng.gen_range(1..=100));
            add((layers - 1) * width + w, t, rng.gen_range(1..=100));
        }
        for l in 0..layers - 1 {
            for w in 0..width {
                for w2 in 0..width {
                    if rng.gen_range(0..3) != 0 {
                        add(l * width + w, (l + 1) * width + w2, rng.gen_range(0..=50));
                    }
                }
                if rng.gen_bool() {
                    add(l * width + w, l * width + (w + 1) % width, rng.gen_range(0..=50));
                }
                if l > 0 && rng.gen_range(0..4) == 0 {
                    add(l * width + w, (l - 1) * width + w, rng.gen_range(0..=50));
                }
            }
        }
        assert_eq!(a.max_flow(s, t), b.fast_max_flow(s, t));
    }
}

// ----------------------------------------------------------- min cost flow

// Copy of min_cost_flow_slow_impl with the already-known stop-condition fix applied
// (the worktree still has the unfixed version).
use algo_lib::collections::indexed_heap::IndexedHeap;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::flow_graph::FlowGraph;
use algo_lib::graph::negative_distances::{Distance, NegativeDistances};
use algo_lib::graph::CostAndFlow;
use algo_lib::numbers::num_traits::algebra::{AdditionMonoidWithSub, MultiplicationMonoid};

fn slow_fixed_impl<
    C: Copy + AdditionMonoidWithSub + MultiplicationMonoid + Ord,
    E: WeightedEdgeTrait<C> + FlowEdgeTrait<C>,
>(
    graph: &mut Graph<E>,
    source: usize,
    sink: usize,
    take_positive: bool,
) -> CostAndFlow<C> {
    let dist = graph.negative_distances_from(source);
    let mut adj = vec![C::zero(); graph.vertex_count()];
    for i in 0..graph.vertex_count() {
        if let Distance::Finite { distance, .. } = dist[i] {
            adj[i] = distance;
        }
    }
    let mut heap = IndexedHeap::new(graph.vertex_count());
    let mut d = vec![C::zero(); graph.vertex_count()];
    // prev[v] = Some((from, edge_id)) where edge_id is the global edge id.
    let mut prev: Vec<Option<(usize, usize)>> = vec![None; graph.vertex_count()];
    let mut flow = C::zero();
    let mut cost = C::zero();
    loop {
        prev.fill(None);
        d[source] = C::zero();
        heap.add_or_adjust(source, C::zero());
        while let Some((cur, dist)) = heap.pop() {
            for (i, e) in graph.adj(cur).iter_with_id() {
                if e.capacity() != C::zero() {
                    let next = e.to();
                    let cost = e.weight() + adj[cur] - adj[next];
                    let total = dist + cost;
                    if prev[next].is_none() || total < d[next] {
                        d[next] = total;
                        prev[next] = Some((cur, i));
                        heap.add_or_relax(next, total);
                    }
                }
            }
        }
        if prev[sink].is_none() || !take_positive && d[sink] + adj[sink] >= adj[source] {
            break;
        }
        let mut cur_flow = None;
        let mut v = sink;
        while v != source {
            let (from, edge) = prev[v].unwrap();
            let e = graph.edge_at(from, edge as u32);
            cur_flow.minim(e.capacity());
            v = from;
        }
        let cur_flow = cur_flow.unwrap();
        flow += cur_flow;
        cost += (d[sink] + adj[sink] - adj[source]) * cur_flow;
        let mut v = sink;
        while v != source {
            let (from, edge) = prev[v].unwrap();
            let push_data = graph.edge_at(from, edge as u32).push_flow(cur_flow);
            graph.push_flow(push_data);
            v = from;
        }
        for i in 0..graph.vertex_count() {
            if prev[i].is_some() {
                adj[i] += d[i];
            }
        }
    }
    CostAndFlow { cost, flow }
}


#[derive(Clone, Debug)]
struct CE {
    from: usize,
    to: usize,
    cap: i64,
    cost: i64,
}

/// Successive shortest paths with Bellman-Ford. Returns (flow, cost).
fn ssp_brute(n: usize, edges: &[CE], s: usize, t: usize, max_flow: bool) -> (i64, i64) {
    let mut flow = vec![0i64; edges.len()];
    let (mut total_flow, mut total_cost) = (0i64, 0i64);
    loop {
        const INF: i64 = i64::MAX / 4;
        let mut d = vec![INF; n];
        // parent arc: (edge index, forward?)
        let mut par: Vec<Option<(usize, bool)>> = vec![None; n];
        d[s] = 0;
        for _ in 0..n {
            for (i, e) in edges.iter().enumerate() {
                if flow[i] < e.cap && d[e.from] < INF && d[e.from] + e.cost < d[e.to] {
                    d[e.to] = d[e.from] + e.cost;
                    par[e.to] = Some((i, true));
                }
                if flow[i] > 0 && d[e.to] < INF && d[e.to] - e.cost < d[e.from] {
                    d[e.from] = d[e.to] - e.cost;
                    par[e.from] = Some((i, false));
                }
            }
        }
        if d[t] >= INF || (!max_flow && d[t] >= 0) {
            break;
        }
        let mut bottleneck = i64::MAX;
        let mut v = t;
        while v != s {
            let (i, fwd) = par[v].unwrap();
            if fwd {
                bottleneck = bottleneck.min(edges[i].cap - flow[i]);
                v = edges[i].from;
            } else {
                bottleneck = bottleneck.min(flow[i]);
                v = edges[i].to;
            }
        }
        let mut v = t;
        while v != s {
            let (i, fwd) = par[v].unwrap();
            if fwd {
                flow[i] += bottleneck;
                v = edges[i].from;
            } else {
                flow[i] -= bottleneck;
                v = edges[i].to;
            }
        }
        total_flow += bottleneck;
        total_cost += bottleneck * d[t];
    }
    (total_flow, total_cost)
}

/// Exhaustive min cost circulation (with a free t -> s return edge).
fn circulation_brute(n: usize, edges: &[CE], s: usize, t: usize, max_flow: bool) -> (i64, i64) {
    let m = edges.len();
    let mut f = vec![0i64; m];
    let mut best: Option<(i64, i64)> = None;
    loop {
        let mut balance = vec![0i64; n];
        let mut cost = 0;
        for (i, e) in edges.iter().enumerate() {
            balance[e.from] -= f[i];
            balance[e.to] += f[i];
            cost += f[i] * e.cost;
        }
        let ok = (0..n).all(|v| v == s || v == t || balance[v] == 0) && balance[t] >= 0;
        if ok {
            let fl = balance[t];
            let better = match best {
                None => true,
                Some((bf, bc)) => {
                    if max_flow {
                        fl > bf || (fl == bf && cost < bc)
                    } else {
                        cost < bc
                    }
                }
            };
            if better {
                best = Some((fl, cost));
            }
        }
        let mut i = 0;
        while i < m {
            if f[i] < edges[i].cap {
                f[i] += 1;
                break;
            }
            f[i] = 0;
            i += 1;
        }
        if i == m {
            break;
        }
    }
    best.unwrap()
}

type WG = Graph<WeightedFlowEdgeWithId<i64, i64, ()>>;

fn build_wg(n: usize, edges: &[CE], two_d: bool) -> WG {
    let mut g: WG = mk(n, two_d);
    for e in edges {
        g.add_edge(WeightedFlowEdgeWithId::new(e.from, e.to, e.cost, e.cap));
    }
    g
}

/// Validates the flow stored in the graph, returns (flow, cost).
fn check_cost_flow(g: &WG, edges: &[CE], s: usize, t: usize) -> (i64, i64) {
    let n = g.vertex_count();
    let mut balance = vec![0i64; n];
    let mut cost = 0;
    for v in 0..n {
        for e in g.adj(v).iter() {
            let ce = &edges[e.id()];
            if ce.from == ce.to {
                // forward entry of a self-loop has the original weight and the
                // reverse entry the negated one; with cost 0 they are symmetric
                continue;
            }
            if v == ce.from && e.to() == ce.to && e.weight() == ce.cost {
                // may match the reverse entry of an antiparallel... no: ids differ
                let f = e.flow(g);
                assert!(0 <= f && f <= ce.cap, "flow {f} cap {}", ce.cap);
                assert_eq!(e.capacity() + f, ce.cap);
                balance[ce.from] -= f;
                balance[ce.to] += f;
                cost += f * ce.cost;
            }
        }
    }
    for v in 0..n {
        if v != s && v != t {
            assert_eq!(balance[v], 0, "conservation at {v}");
        }
    }
    (balance[t], cost)
}

fn gen_cost_graph(rng: &mut Random, kind: usize, n: usize, m: usize, max_cap: i64) -> Vec<CE> {
    let pi: Vec<i64> = (0..n).map(|_| rng.gen_range(-6..=6i64)).collect();
    let mut edges = Vec::new();
    for _ in 0..m {
        let (mut a, mut b) = (rng.gen_range(0..n), rng.gen_range(0..n));
        let cap = rng.gen_range(0..=max_cap);
        let cost = match kind {
            0 => rng.gen_range(0..=6i64) + pi[a] - pi[b],
            1 => {
                if a == b {
                    continue;
                }
                if a > b {
                    std::mem::swap(&mut a, &mut b);
                }
                rng.gen_range(-6..=6i64)
            }
            _ => rng.gen_range(0..=6i64),
        };
        edges.push(CE {
            from: a,
            to: b,
            cap,
            cost,
        });
    }
    edges
}

#[test]
fn min_cost_flow_vs_ssp() {
    let mut rng = Random::new_with_seed(5);
    for it in 0..6000 {
        let n = rng.gen_range(2..=7usize);
        let m = rng.gen_range(0..=3 * n);
        let kind = it % 3;
        let max_cap = if it % 7 == 0 { 1000 } else { 3 };
        let edges = gen_cost_graph(&mut rng, kind, n, m, max_cap);
        let s = rng.gen_range(0..n);
        let mut t = rng.gen_range(0..n);
        while t == s {
            t = rng.gen_range(0..n);
        }
        if kind == 1 && s > t {
            continue;
        }
        for max_flow in [false, true] {
            let expected = ssp_brute(n, &edges, s, t, max_flow);
            for two_d in [false, true] {
                // slow
                let mut g = build_wg(n, &edges, two_d);
                let r = if max_flow {
                    slow_fixed_impl(&mut g, s, t, true)
                } else {
                    slow_fixed_impl(&mut g, s, t, false)
                };
                assert_eq!(
                    (r.flow, r.cost),
                    expected,
                    "slow max={max_flow} n={n} s={s} t={t} {edges:?}"
                );
                assert_eq!(check_cost_flow(&g, &edges, s, t), expected);
                let r2 = if max_flow {
                    slow_fixed_impl(&mut g, s, t, true)
                } else {
                    slow_fixed_impl(&mut g, s, t, false)
                };
                assert_eq!((r2.flow, r2.cost), (0, 0), "slow second call {edges:?}");
                // fast
                if edges.iter().all(|e| e.cap == 0) {
                    continue; // reported separately: panics
                }
                let mut g = build_wg(n, &edges, two_d);
                let r = if max_flow {
                    g.min_cost_max_flow(s, t)
                } else {
                    g.min_cost_flow(s, t)
                };
                let stored = check_cost_flow(&g, &edges, s, t);
                assert_eq!(stored, (r.flow, r.cost), "fast reported vs stored {edges:?}");
                if max_flow {
                    assert_eq!(
                        (r.flow, r.cost),
                        expected,
                        "fast max n={n} s={s} t={t} {edges:?}"
                    );
                } else {
                    assert_eq!(r.cost, expected.1, "fast n={n} s={s} t={t} {edges:?}");
                }
                if g.edges().all(|(_, e)| e.capacity() == 0) {
                    continue;
                }
                let r2 = if max_flow {
                    g.min_cost_max_flow(s, t)
                } else {
                    g.min_cost_flow(s, t)
                };
                assert_eq!(r2.cost, 0, "fast second call {edges:?}");
                if max_flow {
                    assert_eq!(r2.flow, 0);
                }
            }
        }
    }
}

#[test]
fn min_cost_flow_fast_with_negative_cycles() {
    let mut rng = Random::new_with_seed(6);
    for _ in 0..4000 {
        let n = rng.gen_range(2..=4usize);
        let m = rng.gen_range(1..=7usize);
        let edges: Vec<CE> = (0..m)
            .map(|_| CE {
                from: rng.gen_range(0..n),
                to: rng.gen_range(0..n),
                cap: rng.gen_range(0..=2i64),
                cost: rng.gen_range(-4..=4i64),
            })
            .filter(|e| e.from != e.to)
            .collect();
        if edges.iter().all(|e| e.cap == 0) {
            continue;
        }
        let (s, t) = (0, n - 1);
        for max_flow in [false, true] {
            let expected = circulation_brute(n, &edges, s, t, max_flow);
            let mut g = build_wg(n, &edges, rng.gen_bool());
            let r = if max_flow {
                g.min_cost_max_flow(s, t)
            } else {
                g.min_cost_flow(s, t)
            };
            let stored = check_cost_flow(&g, &edges, s, t);
            assert_eq!(stored, (r.flow, r.cost), "reported vs stored {edges:?}");
            if max_flow {
                assert_eq!((r.flow, r.cost), expected, "max n={n} {edges:?}");
            } else {
                assert_eq!(r.cost, expected.1, "n={n} {edges:?}");
            }
        }
    }
}

#[test]
fn min_cost_flow_medium_cross_check() {
    // slow vs fast on bigger graphs with negative costs (DAG and potentials)
    let mut rng = Random::new_with_seed(7);
    for it in 0..300 {
        let n = rng.gen_range(8..=40usize);
        let m = rng.gen_range(n..=6 * n);
        let max_cap = 1i64 << rng.gen_range(0..20);
        let edges = gen_cost_graph(&mut rng, it % 3, n, m, max_cap);
        if edges.iter().all(|e| e.cap == 0) {
            continue;
        }
        let (s, t) = (0, n - 1);
        for max_flow in [false, true] {
            let expected = ssp_brute(n, &edges, s, t, max_flow);
            let mut a = build_wg(n, &edges, false);
            let mut b = build_wg(n, &edges, true);
            let (ra, rb) = if max_flow {
                (slow_fixed_impl(&mut a, s, t, true), b.min_cost_max_flow(s, t))
            } else {
                (slow_fixed_impl(&mut a, s, t, false), b.min_cost_flow(s, t))
            };
            assert_eq!((ra.flow, ra.cost), expected, "slow");
            assert_eq!(rb.cost, expected.1, "fast");
            if max_flow {
                assert_eq!(rb.flow, expected.0);
            }
            assert_eq!(check_cost_flow(&b, &edges, s, t), (rb.flow, rb.cost));
        }
    }
}

// ------------------------------------------------------- flow with demand

#[test]
fn flow_with_demand_vs_exhaustive() {
    let mut rng = Random::new_with_seed(8);
    for _ in 0..6000 {
        let n = rng.gen_range(2..=4usize);
        let m = rng.gen_range(0..=6usize);
        // (from, to, cap, low)
        let edges: Vec<(usize, usize, i64, i64)> = (0..m)
            .map(|_| {
                let cap = rng.gen_range(1..=2i64);
                let low = if rng.gen_range(0..10) == 0 {
                    rng.gen_range(0..=3i64)
                } else {
                    rng.gen_range(0..=cap)
                };
                (rng.gen_range(0..n), rng.gen_range(0..n), cap, low)
            })
            .filter(|e| e.0 != e.1)
            .collect();
        let (s, t) = (0, n - 1);
        // exhaustive
        let mut feasible = false;
        let m = edges.len();
        let mut f = vec![0i64; m];
        'outer: loop {
            let mut balance = vec![0i64; n];
            let mut ok = true;
            for (i, e) in edges.iter().enumerate() {
                ok &= f[i] >= e.3 && f[i] <= e.2;
                balance[e.0] -= f[i];
                balance[e.1] += f[i];
            }
            ok &= (0..n).all(|v| v == s || v == t || balance[v] == 0) && balance[t] >= 0;
            if ok {
                feasible = true;
                break;
            }
            let mut i = 0;
            while i < m {
                if f[i] < 3 {
                    f[i] += 1;
                    continue 'outer;
                }
                f[i] = 0;
                i += 1;
            }
            break;
        }
        let mut g: Graph<FlowEdgeWithId<i64, i64>> = mk(n, rng.gen_bool());
        for &(a, b, cap, low) in &edges {
            g.add_edge(FlowEdgeWithId::with_payload(a, b, cap, low));
        }
        let got = g.flow_with_demand(s, t);
        assert_eq!(got, feasible, "n={n} {edges:?}");
        if got {
            let mut balance = vec![0i64; n];
            for v in 0..n {
                for e in g.adj(v).iter() {
                    let (a, b, cap, low) = edges[e.id()];
                    if v == a && e.to() == b && e.capacity() + e.flow(&g) == cap {
                        // forward entry (reverse entries of antiparallel edges
                        // have a different id, so this is unambiguous)
                        let fl = e.flow(&g);
                        assert!(low <= fl && fl <= cap, "{low} <= {fl} <= {cap} {edges:?}");
                        balance[a] -= fl;
                        balance[b] += fl;
                    }
                }
            }
            for v in 0..n {
                if v != s && v != t {
                    assert_eq!(balance[v], 0, "{edges:?}");
                }
            }
            assert!(balance[t] >= 0);
        }
    }
}

// ------------------------------------------------------- targeted suspects

#[test]
fn suspect_mcf_no_positive_capacity() {
    let mut g: WG = Graph::new_linked(2);
    g.add_edge(WeightedFlowEdgeWithId::new(0, 1, 5, 0));
    let r = g.min_cost_flow(0, 1);
    assert_eq!((r.flow, r.cost), (0, 0));
}

#[test]
fn suspect_mcf_empty_graph() {
    let mut g: WG = Graph::new_linked(2);
    let r = g.min_cost_max_flow(0, 1);
    assert_eq!((r.flow, r.cost), (0, 0));
}

#[test]
fn suspect_mcf_big_capacity_i32() {
    use algo_lib::graph::edges::weighted_flow_edge::WeightedFlowEdge;
    let mut g: Graph<WeightedFlowEdge<i32, i32, ()>> = Graph::new_linked(3);
    g.add_edge(WeightedFlowEdge::new(0, 1, 1, i32::MAX)); // "infinite" capacity
    g.add_edge(WeightedFlowEdge::new(1, 2, 1, 3));
    let r = g.min_cost_max_flow(0, 2);
    assert_eq!((r.flow, r.cost), (3, 6));
}

#[test]
fn suspect_mcf_big_capacity_i64() {
    let mut g: WG = Graph::new_linked(3);
    g.add_edge(WeightedFlowEdgeWithId::new(0, 1, 1, i64::MAX));
    g.add_edge(WeightedFlowEdgeWithId::new(1, 2, 1, 3));
    let r = g.min_cost_max_flow(0, 2);
    assert_eq!((r.flow, r.cost), (3, 6));
}

#[test]
fn suspect_demand_zero_capacity_edge() {
    let mut g: Graph<FlowEdgeWithId<i64, i64>> = Graph::new_linked(2);
    g.add_edge(FlowEdgeWithId::with_payload(0, 1, 0, 5)); // cap 0, lower bound 5
    assert!(!g.flow_with_demand(0, 1));
}

#[test]
fn suspect_mcf_big_capacity_i32_b() {
    use algo_lib::graph::edges::weighted_flow_edge::WeightedFlowEdge;
    for big in [1 << 29, 1 << 30, i32::MAX] {
        let mut g: Graph<WeightedFlowEdge<i32, i32, ()>> = Graph::new_linked(3);
        g.add_edge(WeightedFlowEdge::new(0, 1, 1, big)); // "infinite" capacity
        g.add_edge(WeightedFlowEdge::new(1, 2, 1, 1));
        let r = g.min_cost_max_flow(0, 2);
        assert_eq!((r.flow, r.cost), (1, 2), "big = {big}");
    }
}

#[test]
fn suspect_mcf_big_capacity_i64_b() {
    for big in [1 << 61, 1 << 62, i64::MAX] {
        let mut g: WG = Graph::new_linked(3);
        g.add_edge(WeightedFlowEdgeWithId::new(0, 1, 1, big));
        g.add_edge(WeightedFlowEdgeWithId::new(1, 2, 1, 1));
        let r = g.min_cost_max_flow(0, 2);
        assert_eq!((r.flow, r.cost), (1, 2), "big = {big}");
        let mut g: WG = Graph::new_linked(3);
        g.add_edge(WeightedFlowEdgeWithId::new(0, 1, -1, big));
        g.add_edge(WeightedFlowEdgeWithId::new(1, 2, -1, 1));
        let r = g.min_cost_flow(0, 2);
        assert_eq!((r.flow, r.cost), (1, -2), "big = {big}");
    }
}

// ---- copy of min_cost_flow.rs impl (i64) with the two suggested fixes applied
mod mcf_fixed {
use algo_lib::collections::indexed_heap::IndexedHeap;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::edges::edge_id::EdgeId;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::flow_edge_trait::FlowEdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::edges::weighted_flow_edge::WeightedFlowEdgeRaw;
use algo_lib::graph::flow_graph::FlowGraph;
use algo_lib::graph::{CostAndFlow, Graph};
use algo_lib::numbers::num_traits::bit_ops::BitOps;

pub fn mcf_fixed_impl<Id, P: Clone + Default>(
                or_graph: &mut Graph<WeightedFlowEdgeRaw<i64, i64, Id, P>>,
                source: usize,
                sink: usize,
                take_positive_cycles: bool,
            ) -> CostAndFlow<i64>
            where
                Id: EdgeId,
            {
                type C = i64;
                let inf = C::MAX >> 1;

                let n = or_graph.vertex_count();
                let mut p = vec![0; n + 1];
                p[n] = inf;

                let mut graph = Graph::new_linked(n + 1);
                // (from, or_graph_edge_id, graph_edge_id)
                let mut corresponding: Vec<(usize, usize, usize)> = Vec::new();
                let mut max_capacity = 0;
                let mut sum_weight = 0;
                for i in 0..n {
                    for (j, e) in or_graph.adj(i).iter_with_id() {
                        if e.capacity() > 0 {
                            max_capacity.maxim(e.capacity());
                            sum_weight += e.weight().max(-e.weight());
                            corresponding.push((
                                i,
                                j,
                                graph.add_edge(WeightedFlowEdgeRaw::new(i, e.to(), e.weight(), 0)),
                            ));
                        }
                    }
                }
                if max_capacity == 0 {
                    return CostAndFlow { cost: 0, flow: 0 };
                }
                let bits = max_capacity.highest_bit() + 1;
                let back = graph.add_edge(WeightedFlowEdgeRaw::new(
                    sink,
                    source,
                    if take_positive_cycles {
                        -sum_weight - 1
                    } else {
                        0
                    },
                    0,
                ));
                for i in 0..n {
                    graph.add_edge(WeightedFlowEdgeRaw::new(n, i, 0, 1));
                }

                let mut dis = vec![0; n + 1];
                // pre[v] = (from, global_edge_id_in_graph)
                let mut pre = vec![(0usize, 0usize); n + 1];
                let mut heap = IndexedHeap::new(n + 1);

                let c = |from: usize, e: &WeightedFlowEdgeRaw<C, C, Id, ()>, p: &Vec<C>| -> C {
                    p[from] + e.weight() - p[e.to()]
                };

                let dijkstra = |graph: &mut Graph<WeightedFlowEdgeRaw<C, C, Id, ()>>,
                                dis: &mut Vec<C>,
                                pre: &mut Vec<(usize, usize)>,
                                heap: &mut IndexedHeap<C>,
                                p: &Vec<C>,
                                s: usize| {
                    dis.fill(inf);
                    dis[s] = 0;
                    assert!(heap.is_empty());
                    heap.add_or_adjust(s, 0);

                    while let Some((u, w)) = heap.pop() {
                        assert!(w == dis[u]);
                        for (i, e) in graph.adj(u).iter_with_id() {
                            let v = e.to();

                            debug_assert!(e.capacity() <= 0 || c(u, e, p) >= 0);

                            if e.capacity() > 0 && dis[v] > w + c(u, e, p) {
                                dis[v] = w + c(u, e, p);
                                pre[v] = (u, i);
                                heap.add_or_adjust(v, dis[v]);
                            }
                        }
                    }
                };

                let mut add_one =
                    |graph: &mut Graph<WeightedFlowEdgeRaw<C, C, Id, ()>>, from: usize, id: usize| {
                        if graph.edge_at(from, id as u32).capacity() > 0 {
                            *graph.edge_at_mut(from, id as u32).capacity_mut() += 1;
                            return;
                        }
                        let mut u = from;
                        let v = graph.edge_at(from, id as u32).to();
                        let cur_len = c(u, graph.edge_at(from, id as u32), &p);
                        dijkstra(graph, &mut dis, &mut pre, &mut heap, &p, v);
                        let e = graph.edge_at(from, id as u32);
                        if dis[u] < inf && dis[u] + c(u, e, &p) < 0 {
                            let rev_id = e.reverse_id();
                            *graph.edge_at_mut(v, rev_id as u32).capacity_mut() += 1;
                            while u != v {
                                let push_data = graph.edge_at(pre[u].0, pre[u].1 as u32).push_flow(1);
                                graph.push_flow(push_data);
                                u = pre[u].0;
                            }
                        } else {
                            *graph.edge_at_mut(from, id as u32).capacity_mut() += 1;
                        }
                        let mut max_dis = 0;
                        for i in dis.iter().take(n) {
                            if *i != inf {
                                max_dis.maxim(*i);
                            }
                        }
                        for i in 0..n {
                            p[i] += if dis[i] < inf {
                                dis[i]
                            } else {
                                max_dis + cur_len.abs()
                            };
                        }
                        dijkstra(graph, &mut dis, &mut pre, &mut heap, &p, n);
                        for i in 0..n {
                            let npi = dis[i] - p[n];
                            p[i] += npi;
                        }
                    };

                add_one(&mut graph, sink, back);
                for i in (0..bits).rev() {
                    for j in 0..=n {
                        for e in graph.adj_mut(j).iter_mut() {
                            *e.capacity_mut() <<= 1;
                        }
                    }
                    *graph.edge_at_mut(sink, back as u32).capacity_mut() = 2;
                    for (from, self_edge_id, graph_edge_id) in corresponding.iter() {
                        if or_graph.edge_at(*from, *self_edge_id as u32).capacity().is_set(i) {
                            add_one(&mut graph, *from, *graph_edge_id);
                            if graph.edge_at(sink, back as u32).capacity() == 1 {
                                *graph.edge_at_mut(sink, back as u32).capacity_mut() += 1;
                            }
                        }
                    }
                }

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
                CostAndFlow {
                    cost: min_cost,
                    flow: max_flow,
                }
            }

}

#[test]
fn fixed_copy_big_capacity_and_empty() {
    for big in [1 << 61, 1 << 62, i64::MAX] {
        let mut g: WG = Graph::new_linked(3);
        g.add_edge(WeightedFlowEdgeWithId::new(0, 1, 1, big));
        g.add_edge(WeightedFlowEdgeWithId::new(1, 2, 1, 1));
        g.compact();
        let r = mcf_fixed::mcf_fixed_impl(&mut g, 0, 2, true);
        assert_eq!((r.flow, r.cost), (1, 2), "big = {big}");
    }
    let mut g: WG = Graph::new_linked(2);
    g.add_edge(WeightedFlowEdgeWithId::new(0, 1, 5, 0));
    let r = mcf_fixed::mcf_fixed_impl(&mut g, 0, 1, false);
    assert_eq!((r.flow, r.cost), (0, 0));
}

#[test]
fn fixed_copy_random() {
    let mut rng = Random::new_with_seed(9);
    for it in 0..6000 {
        let n = rng.gen_range(2..=7usize);
        let m = rng.gen_range(0..=3 * n);
        let max_cap = if it % 7 == 0 { 1000 } else { 3 };
        let edges = gen_cost_graph(&mut rng, it % 3, n, m, max_cap);
        let (s, t) = (0, n - 1);
        for max_flow in [false, true] {
            let expected = ssp_brute(n, &edges, s, t, max_flow);
            let mut g = build_wg(n, &edges, rng.gen_bool());
            g.compact();
            let r = mcf_fixed::mcf_fixed_impl(&mut g, s, t, max_flow);
            assert_eq!(check_cost_flow(&g, &edges, s, t), (r.flow, r.cost));
            assert_eq!(r.cost, expected.1);
            if max_flow {
                assert_eq!(r.flow, expected.0);
            }
        }
    }
}

fn johnson_graph(k: usize) -> (WG, usize, usize, i64) {
    // chain 0..=k; between i and i+1 a direct edge of cost 0 and a detour
    // i -> u_i -> i+1 of total cost -delta_i whose first leg is expensive
    let mut g: WG = Graph::new_linked(2 * k + 1);
    let mut expected = 0;
    for i in 0..k {
        let delta = 1i64 << (k - 1 - i);
        let h = delta * 4;
        let u = k + 1 + i;
        g.add_edge(WeightedFlowEdgeWithId::new(i, i + 1, 0, 1));
        g.add_edge(WeightedFlowEdgeWithId::new(i, u, h, 1));
        g.add_edge(WeightedFlowEdgeWithId::new(u, i + 1, -h - delta, 1));
        expected -= delta;
    }
    (g, 0, k, expected)
}

#[test]
fn suspect_slow_mcf_exponential_timing() {
    use algo_lib::graph::min_cost_flow_slow::MinCostFlowSlow;
    for k in [14, 16, 18, 20, 22] {
        let (mut g, s, t, expected) = johnson_graph(k);
        let start = std::time::Instant::now();
        let r = g.min_cost_max_flow_slow(s, t);
        eprintln!("k={k} n={} time={:?}", 2 * k + 1, start.elapsed());
        assert_eq!((r.flow, r.cost), (2, expected));
    }
}

// slow MCF copy: stop-condition fix + capacity-aware initial potentials
fn slow_fixed2_impl<
    C: Copy + AdditionMonoidWithSub + MultiplicationMonoid + Ord,
    E: WeightedEdgeTrait<C> + FlowEdgeTrait<C>,
>(
    graph: &mut Graph<E>,
    source: usize,
    sink: usize,
    take_positive: bool,
) -> CostAndFlow<C> {
    // capacity-aware Bellman-Ford for the initial potentials
    let n = graph.vertex_count();
    let mut adj = vec![C::zero(); n];
    let mut reach = vec![false; n];
    reach[source] = true;
    for _ in 0..n {
        let mut updated = false;
        for i in 0..n {
            if !reach[i] {
                continue;
            }
            for e in graph.adj(i).iter() {
                if e.capacity() != C::zero() {
                    let cand = adj[i] + e.weight();
                    let to = e.to();
                    if !reach[to] || cand < adj[to] {
                        reach[to] = true;
                        adj[to] = cand;
                        updated = true;
                    }
                }
            }
        }
        if !updated {
            break;
        }
    }
    let mut heap = IndexedHeap::new(graph.vertex_count());
    let mut d = vec![C::zero(); graph.vertex_count()];
    // prev[v] = Some((from, edge_id)) where edge_id is the global edge id.
    let mut prev: Vec<Option<(usize, usize)>> = vec![None; graph.vertex_count()];
    let mut flow = C::zero();
    let mut cost = C::zero();
    loop {
        prev.fill(None);
        d[source] = C::zero();
        heap.add_or_adjust(source, C::zero());
        while let Some((cur, dist)) = heap.pop() {
            for (i, e) in graph.adj(cur).iter_with_id() {
                if e.capacity() != C::zero() {
                    let next = e.to();
                    let cost = e.weight() + adj[cur] - adj[next];
                    let total = dist + cost;
                    if prev[next].is_none() || total < d[next] {
                        d[next] = total;
                        prev[next] = Some((cur, i));
                        heap.add_or_relax(next, total);
                    }
                }
            }
        }
        if prev[sink].is_none() || !take_positive && d[sink] + adj[sink] >= adj[source] {
            break;
        }
        let mut cur_flow = None;
        let mut v = sink;
        while v != source {
            let (from, edge) = prev[v].unwrap();
            let e = graph.edge_at(from, edge as u32);
            cur_flow.minim(e.capacity());
            v = from;
        }
        let cur_flow = cur_flow.unwrap();
        flow += cur_flow;
        cost += (d[sink] + adj[sink] - adj[source]) * cur_flow;
        let mut v = sink;
        while v != source {
            let (from, edge) = prev[v].unwrap();
            let push_data = graph.edge_at(from, edge as u32).push_flow(cur_flow);
            graph.push_flow(push_data);
            v = from;
        }
        for i in 0..graph.vertex_count() {
            if prev[i].is_some() {
                adj[i] += d[i];
            }
        }
    }
    CostAndFlow { cost, flow }
}

#[test]
fn fixed2_slow_random_and_johnson() {
    let mut rng = Random::new_with_seed(10);
    for it in 0..6000 {
        let n = rng.gen_range(2..=7usize);
        let m = rng.gen_range(0..=3 * n);
        let edges = gen_cost_graph(&mut rng, it % 3, n, m, 3);
        let s = rng.gen_range(0..n);
        let t = (s + rng.gen_range(1..n)) % n;
        if it % 3 == 1 && s > t {
            continue;
        }
        for max_flow in [false, true] {
            let expected = ssp_brute(n, &edges, s, t, max_flow);
            let mut g = build_wg(n, &edges, rng.gen_bool());
            let r = slow_fixed2_impl(&mut g, s, t, max_flow);
            assert_eq!((r.flow, r.cost), expected);
            assert_eq!(check_cost_flow(&g, &edges, s, t), expected);
            let r = slow_fixed2_impl(&mut g, s, t, max_flow);
            assert_eq!((r.flow, r.cost), (0, 0));
        }
    }
    let (mut g, s, t, expected) = johnson_graph(40);
    let r = slow_fixed2_impl(&mut g, s, t, true);
    assert_eq!((r.flow, r.cost), (2, expected));
}

#[test]
#[ignore]
fn note_slow_mcf_negative_cycle_hangs() {
    use algo_lib::graph::min_cost_flow_slow::MinCostFlowSlow;
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let mut g: WG = Graph::new_linked(4);
        g.add_edge(WeightedFlowEdgeWithId::new(0, 1, 0, 1));
        g.add_edge(WeightedFlowEdgeWithId::new(1, 2, -1, 1));
        g.add_edge(WeightedFlowEdgeWithId::new(2, 1, -1, 1));
        g.add_edge(WeightedFlowEdgeWithId::new(2, 3, 0, 1));
        let r = g.min_cost_max_flow_slow(0, 3);
        tx.send((r.flow, r.cost)).ok();
    });
    let r = rx.recv_timeout(std::time::Duration::from_secs(3));
    assert!(r.is_ok(), "did not terminate in 3s");
}

// ------------------------------------------------------------- distances
mod dist_tests {
    use super::mk;
    use algo_lib::graph::all_distances::{AllDistances, Distance as AD};
    use algo_lib::graph::distances::Distances;
    use algo_lib::graph::edge_distances::{BiEdgeAlgos, EdgeAlgos};
    use algo_lib::graph::edges::bi_edge::BiEdge;
    use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
    use algo_lib::graph::edges::edge::Edge;
    use algo_lib::graph::edges::edge_trait::EdgeTrait;
    use algo_lib::graph::edges::weighted_edge::WeightedEdge;
    use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
    use algo_lib::graph::negative_distances::{Distance as ND, NegativeDistances};
    use algo_lib::graph::Graph;
    use algo_lib::misc::random::{Random, RandomTrait};

    const INF: i64 = i64::MAX / 4;
    // all_distances is a confirmed bug (see bug_all_distances_*), checked separately
    const CHECK_ALL: bool = false;

    /// (dist matrix from plain FW, reach, on_negative_cycle)
    fn oracle(n: usize, edges: &[(usize, usize, i64)]) -> (Vec<Vec<i64>>, Vec<Vec<bool>>, Vec<bool>) {
        let mut d = vec![vec![INF; n]; n];
        let mut reach = vec![vec![false; n]; n];
        for i in 0..n {
            d[i][i] = 0;
            reach[i][i] = true;
        }
        for &(a, b, w) in edges {
            d[a][b] = d[a][b].min(w);
            reach[a][b] = true;
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if d[i][k] < INF && d[k][j] < INF {
                        d[i][j] = d[i][j].min(d[i][k] + d[k][j]).max(-INF);
                    }
                    reach[i][j] |= reach[i][k] && reach[k][j];
                }
            }
        }
        let neg = (0..n).map(|c| d[c][c] < 0).collect();
        (d, reach, neg)
    }

    #[test]
    fn negative_and_all_distances_vs_floyd() {
        let mut rng = Random::new_with_seed(21);
        for it in 0..20000 {
            let n = rng.gen_range(1..=7usize);
            let m = rng.gen_range(0..=3 * n);
            let lo = if it % 2 == 0 { -2 } else { -6 };
            let edges: Vec<(usize, usize, i64)> = (0..m)
                .map(|_| {
                    (
                        rng.gen_range(0..n),
                        rng.gen_range(0..n),
                        rng.gen_range(lo..=8i64),
                    )
                })
                .collect();
            let (d, reach, neg) = oracle(n, &edges);
            let mut g: Graph<WeightedEdge<i64, ()>> = mk(n, rng.gen_bool());
            for &(a, b, w) in &edges {
                g.add_edge(WeightedEdge::new(a, b, w));
            }
            let expect = |i: usize, j: usize| -> Option<Option<i64>> {
                // None = unreachable, Some(None) = -inf, Some(Some(d))
                if !reach[i][j] {
                    return None;
                }
                if (0..n).any(|c| neg[c] && reach[i][c] && reach[c][j]) {
                    return Some(None);
                }
                Some(Some(d[i][j]))
            };
            let all = g.all_distances();
            for i in 0..n {
                let nd = g.negative_distances_from(i);
                for j in 0..n {
                    let e = expect(i, j);
                    let got_all = match all[(i, j)] {
                        AD::None => None,
                        AD::Infinite => Some(None),
                        AD::Finite(w) => Some(Some(w)),
                    };
                    if CHECK_ALL {
                        assert_eq!(got_all, e, "all_distances ({i},{j}) n={n} {edges:?}");
                    }
                    let got_nd = match nd[j] {
                        ND::None => None,
                        ND::Infinite => Some(None),
                        ND::Finite { distance, .. } => Some(Some(distance)),
                    };
                    assert_eq!(got_nd, e, "negative_distances {i}->{j} n={n} {edges:?}");
                    if let ND::Finite { distance, from, edge } = nd[j] {
                        if j != i {
                            let ed = g.edge_at(from, edge as u32);
                            assert_eq!(ed.to(), j);
                            match nd[from] {
                                ND::Finite { distance: df, .. } => {
                                    assert_eq!(df + ed.weight(), distance)
                                }
                                _ => panic!("parent not finite"),
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn dijkstra_zero_one_bfs_vs_floyd() {
        let mut rng = Random::new_with_seed(22);
        for it in 0..10000 {
            let n = rng.gen_range(1..=8usize);
            let m = rng.gen_range(0..=3 * n);
            let hi = match it % 3 {
                0 => 1i64,
                1 => 3,
                _ => 1_000_000_000_000_000,
            };
            let edges: Vec<(usize, usize, i64)> = (0..m)
                .map(|_| {
                    (
                        rng.gen_range(0..n),
                        rng.gen_range(0..n),
                        rng.gen_range(0..=hi),
                    )
                })
                .collect();
            let (d, reach, _) = oracle(n, &edges);
            let two_d = rng.gen_bool();
            let bi = rng.gen_bool();
            // directed / undirected
            let (d, reach) = if bi {
                let mut sym = edges.clone();
                sym.extend(edges.iter().map(|&(a, b, w)| (b, a, w)));
                let (d, r, _) = oracle(n, &sym);
                (d, r)
            } else {
                (d, reach)
            };
            let mut gd: Graph<WeightedEdge<i64, ()>> = mk(n, two_d);
            let mut gb: Graph<BiWeightedEdge<i64, ()>> = mk(n, two_d);
            let mut ge: Graph<Edge<()>> = mk(n, two_d);
            let mut gbe: Graph<BiEdge<()>> = mk(n, two_d);
            for &(a, b, w) in &edges {
                gd.add_edge(WeightedEdge::new(a, b, w));
                gb.add_edge(BiWeightedEdge::new(a, b, w));
                ge.add_edge(Edge::new(a, b));
                gbe.add_edge(BiEdge::new(a, b));
            }
            // unit distances oracle
            let unit: Vec<(usize, usize, i64)> = edges
                .iter()
                .flat_map(|&(a, b, _)| {
                    if bi {
                        vec![(a, b, 1), (b, a, 1)]
                    } else {
                        vec![(a, b, 1)]
                    }
                })
                .collect();
            let (du, _, _) = oracle(n, &unit);
            for s in 0..n {
                macro_rules! check {
                    ($g: expr, $res: expr, $what: expr) => {
                        for v in 0..n {
                            match $res[v] {
                                None => assert!(!reach[s][v], "{} {s}->{v} {edges:?}", $what),
                                Some((dist, from, edge)) => {
                                    assert!(reach[s][v]);
                                    assert_eq!(dist, d[s][v], "{} {s}->{v} bi={bi} {edges:?}", $what);
                                    if v != s {
                                        let e = $g.edge_at(from, edge as u32);
                                        assert_eq!(e.to(), v);
                                        assert_eq!($res[from].unwrap().0 + e.weight(), dist);
                                    }
                                }
                            }
                        }
                    };
                }
                if bi {
                    let r = gb.distances_from(s);
                    check!(gb, r, "dijkstra-bi");
                    if hi == 1 {
                        let r = gb.zero_one_distances_from(s);
                        check!(gb, r, "01-bi");
                    }
                    let ed = gbe.edge_distances(s);
                    for v in 0..n {
                        let e = if du[s][v] >= INF { u32::MAX } else { du[s][v] as u32 };
                        assert_eq!(ed[v], e);
                    }
                } else {
                    let r = gd.distances_from(s);
                    check!(gd, r, "dijkstra");
                    if hi == 1 {
                        let r = gd.zero_one_distances_from(s);
                        check!(gd, r, "01");
                    }
                    let ed = ge.edge_distances(s);
                    for v in 0..n {
                        let e = if du[s][v] >= INF { u32::MAX } else { du[s][v] as u32 };
                        assert_eq!(ed[v], e);
                    }
                }
                let t = rng.gen_range(0..n);
                if !bi {
                    match gd.distance(s, t) {
                        None => assert!(!reach[s][t]),
                        Some((w, path)) => {
                            assert_eq!(w, d[s][t]);
                            let mut at = s;
                            let mut sum = 0;
                            for (from, edge) in path {
                                assert_eq!(from, at);
                                let e = gd.edge_at(from, edge as u32);
                                sum += e.weight();
                                at = e.to();
                            }
                            assert_eq!((at, sum), (t, w));
                        }
                    }
                    if hi == 1 {
                        match gd.zero_one_distance(s, t) {
                            None => assert!(!reach[s][t]),
                            Some((w, path)) => {
                                assert_eq!(w, d[s][t]);
                                let mut at = s;
                                let mut sum = 0;
                                for (from, edge) in path {
                                    assert_eq!(from, at);
                                    let e = gd.edge_at(from, edge as u32);
                                    sum += e.weight();
                                    at = e.to();
                                }
                                assert_eq!((at, sum), (t, w));
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn centers_and_diameter() {
        let mut rng = Random::new_with_seed(23);
        for it in 0..5000 {
            let n = rng.gen_range(1..=9usize);
            let mut edges = Vec::new();
            for v in 1..n {
                let p = if it % 3 == 0 { v - 1 } else { rng.gen_range(0..v) };
                edges.push(if rng.gen_bool() { (p, v, 1i64) } else { (v, p, 1) });
            }
            let mut sym = edges.clone();
            sym.extend(edges.iter().map(|&(a, b, w)| (b, a, w)));
            let (d, _, _) = oracle(n, &sym);
            let ecc: Vec<i64> = (0..n).map(|v| *d[v].iter().max().unwrap()).collect();
            let radius = *ecc.iter().min().unwrap();
            let expected_centers: Vec<usize> = (0..n).filter(|&v| ecc[v] == radius).collect();
            let mut g: Graph<BiEdge<()>> = mk(n, rng.gen_bool());
            for &(a, b, _) in &edges {
                g.add_edge(BiEdge::new(a, b));
            }
            assert_eq!(g.diameter() as i64, *ecc.iter().max().unwrap());
            assert_eq!(g.centers(), expected_centers, "{edges:?}");
        }
    }
}

// -------------------------------------------------------------- matching
mod matching_tests {
    use super::mk;
    use algo_lib::graph::bipartite_matching::BipartiteMatching;
    use algo_lib::graph::edges::bi_edge::BiEdge;
    use algo_lib::graph::general_matching::GeneralMatching;
    use algo_lib::graph::Graph;
    use algo_lib::misc::random::{Random, RandomTrait};

    fn general_brute(n: usize, adjacent: &[Vec<bool>]) -> usize {
        let mut best = vec![0usize; 1 << n];
        for mask in 1usize..1 << n {
            let v = mask.trailing_zeros() as usize;
            let rest = mask & (mask - 1);
            let mut res = best[rest];
            for u in 0..n {
                if rest >> u & 1 == 1 && adjacent[v][u] {
                    res = res.max(1 + best[rest & !(1 << u)]);
                }
            }
            best[mask] = res;
        }
        best[(1 << n) - 1]
    }

    #[test]
    fn general_matching_with_loops_and_multiedges() {
        let mut rng = Random::new_with_seed(31);
        for it in 0..20000 {
            let n = rng.gen_range(0..=10usize);
            let m = if n == 0 { 0 } else { rng.gen_range(0..=(if it % 2 == 0 { n } else { 3 * n })) };
            let edges: Vec<(usize, usize)> = (0..m)
                .map(|_| (rng.gen_range(0..n), rng.gen_range(0..n)))
                .collect();
            let mut adjacent = vec![vec![false; n]; n];
            let mut g: Graph<BiEdge<()>> = mk(n, rng.gen_bool());
            for &(a, b) in &edges {
                g.add_edge(BiEdge::new(a, b));
                if a != b {
                    adjacent[a][b] = true;
                    adjacent[b][a] = true;
                }
            }
            let mate = g.general_matching();
            assert_eq!(mate.len(), n);
            let mut size = 0;
            for v in 0..n {
                if let Some(u) = mate[v] {
                    assert!(adjacent[v][u], "{v}-{u} is not an edge {edges:?}");
                    assert_eq!(mate[u], Some(v));
                    size += 1;
                }
            }
            assert_eq!(size / 2, general_brute(n, &adjacent), "n={n} {edges:?}");
        }
    }

    #[test]
    fn bipartite_matching_vs_brute() {
        let mut rng = Random::new_with_seed(32);
        for _ in 0..20000 {
            let l = rng.gen_range(0..=6usize);
            let r = rng.gen_range(0..=6usize);
            let n = l + r;
            let mut bm = BipartiteMatching::new(l, r);
            let mut adjacent = vec![vec![false; n]; n];
            let mut edges = Vec::new();
            for round in 0..2 {
                let m = if l == 0 || r == 0 { 0 } else { rng.gen_range(0..=l * r / (round + 1)) };
                for _ in 0..m {
                    let (u, v) = (rng.gen_range(0..l), rng.gen_range(0..r));
                    bm.add_edge(u, v);
                    edges.push((u, v));
                    adjacent[u][l + v] = true;
                    adjacent[l + v][u] = true;
                }
                let size = bm.run();
                assert_eq!(size, general_brute(n, &adjacent), "l={l} r={r} {edges:?}");
                assert_eq!(bm.run(), size);
                let mut cnt = 0;
                for u in 0..l {
                    if let Some(v) = bm.left_mate()[u] {
                        assert!(adjacent[u][l + v]);
                        assert_eq!(bm.right_mate()[v], Some(u));
                        cnt += 1;
                    }
                }
                for v in 0..r {
                    if let Some(u) = bm.right_mate()[v] {
                        assert_eq!(bm.left_mate()[u], Some(v));
                    }
                }
                assert_eq!(cnt, size);
            }
        }
    }
}

// ----------------------------------------------------------------- 2-SAT
mod two_sat_tests {
    use algo_lib::graph::two_sat::TwoSat;
    use algo_lib::misc::random::{Random, RandomTrait};

    #[derive(Debug, Clone)]
    enum Cl {
        Or(usize, bool, usize, bool),
        Imp(usize, bool, usize, bool),
        True(usize),
        False(usize),
        MaxOne(Vec<(usize, bool)>),
    }

    fn holds(c: &Cl, x: &[bool]) -> bool {
        match c {
            Cl::Or(a, av, b, bv) => x[*a] == *av || x[*b] == *bv,
            Cl::Imp(a, av, b, bv) => x[*a] != *av || x[*b] == *bv,
            Cl::True(a) => x[*a],
            Cl::False(a) => !x[*a],
            Cl::MaxOne(r) => r.iter().filter(|&&(a, v)| x[a] == v).count() <= 1,
        }
    }

    #[test]
    fn two_sat_vs_brute() {
        let mut rng = Random::new_with_seed(41);
        let mut sat_count = 0;
        for it in 0..20000 {
            let n = rng.gen_range(1..=6usize);
            let k = rng.gen_range(0..=2 * n + 2);
            let mut ts = TwoSat::new(n);
            if it % 5 == 0 {
                // stale state: fill, then clear
                ts.add_or(0, true, n - 1, false);
                ts.max_one(&(0..n).map(|i| (i, true)).collect::<Vec<_>>());
                ts.set_true(0);
                ts.set_false(0);
                ts.clear();
            }
            let mut clauses = Vec::new();
            for _ in 0..k {
                let (a, b) = (rng.gen_range(0..n), rng.gen_range(0..n));
                let (av, bv) = (rng.gen_bool(), rng.gen_bool());
                let c = match rng.gen_range(0..8) {
                    0..=2 => Cl::Or(a, av, b, bv),
                    3..=4 => Cl::Imp(a, av, b, bv),
                    5 => Cl::True(a),
                    6 => Cl::False(a),
                    _ => {
                        // distinct variables, random polarity
                        let mut vars: Vec<usize> = (0..n).collect();
                        for i in 0..n {
                            let j = rng.gen_range(i..n);
                            vars.swap(i, j);
                        }
                        let len = rng.gen_range(0..=n);
                        Cl::MaxOne(vars[..len].iter().map(|&v| (v, rng.gen_bool())).collect())
                    }
                };
                match &c {
                    Cl::Or(a, av, b, bv) => ts.add_or(*a, *av, *b, *bv),
                    Cl::Imp(a, av, b, bv) => ts.add_implication(*a, *av, *b, *bv),
                    Cl::True(a) => ts.set_true(*a),
                    Cl::False(a) => ts.set_false(*a),
                    Cl::MaxOne(r) => ts.max_one(r),
                }
                clauses.push(c);
            }
            let sat = (0u32..1 << n).any(|mask| {
                let x: Vec<bool> = (0..n).map(|i| mask >> i & 1 == 1).collect();
                clauses.iter().all(|c| holds(c, &x))
            });
            let res = ts.solve();
            assert_eq!(res.is_some(), sat, "n={n} {clauses:?}");
            if let Some(x) = res {
                sat_count += 1;
                assert_eq!(x.len(), n);
                for c in &clauses {
                    assert!(holds(c, &x), "violated {c:?} by {x:?}; {clauses:?}");
                }
            }
        }
        assert!(sat_count > 2000 && sat_count < 18000, "{sat_count}");
    }
}

// ------------------------------------------------------------- hungarian
mod hungarian_tests {
    use algo_lib::collections::md_arr::arr2d::Arr2d;
    use algo_lib::misc::hungarian_algorithm::{assignment, hungarian_algorithm};
    use algo_lib::misc::random::{Random, RandomTrait};

    fn brute(a: &Arr2d<i64>, row: usize, used: &mut Vec<bool>) -> i64 {
        if row == a.d1() {
            return 0;
        }
        let mut best = i64::MAX;
        for j in 0..a.d2() {
            if !used[j] {
                used[j] = true;
                best = best.min(a[(row, j)] + brute(a, row + 1, used));
                used[j] = false;
            }
        }
        best
    }

    #[test]
    fn assignment_vs_permutations() {
        let mut rng = Random::new_with_seed(51);
        for it in 0..20000 {
            let n = rng.gen_range(0..=5usize);
            let m = rng.gen_range(n..=6usize);
            let (lo, hi) = match it % 4 {
                0 => (0i64, 1i64),
                1 => (-3, 3),
                2 => (-1_000_000_000_000_000, 1_000_000_000_000_000),
                _ => (5, 5),
            };
            let a = Arr2d::with_gen(n, m, |_, _| rng.gen_range(lo..=hi));
            let expected = brute(&a, 0, &mut vec![false; m]);
            let (cost, cols) = assignment(&a);
            assert_eq!(cost, expected);
            assert_eq!(hungarian_algorithm(&a), expected);
            assert_eq!(cols.len(), n);
            let mut seen = vec![false; m];
            let mut sum = 0;
            for (i, &j) in cols.iter().enumerate() {
                assert!(!seen[j]);
                seen[j] = true;
                sum += a[(i, j)];
            }
            assert_eq!(sum, expected);
        }
    }
}

// --------------------------------------------------------------- compact
mod compact_tests {
    use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdgeWithId;
    use algo_lib::graph::edges::edge_trait::EdgeTrait;
    use algo_lib::graph::edges::flow_edge_trait::FlowEdgeTrait;
    use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
    use algo_lib::graph::edges::weighted_flow_edge::WeightedFlowEdgeWithId;
    use algo_lib::graph::flow_graph::FlowGraph;
    use algo_lib::graph::Graph;
    use algo_lib::misc::random::{Random, RandomTrait};

    #[test]
    fn compact_interleaved_with_add_edge_and_push_flow() {
        let mut rng = Random::new_with_seed(61);
        for _ in 0..3000 {
            let n = rng.gen_range(1..=6usize);
            let mut a: Graph<WeightedFlowEdgeWithId<i64, i64, ()>> = Graph::new_linked(n);
            let mut b: Graph<WeightedFlowEdgeWithId<i64, i64, ()>> = Graph::new_2d(n);
            let mut bw: Graph<BiWeightedEdgeWithId<i64, ()>> = Graph::new_linked(n);
            let mut bw2: Graph<BiWeightedEdgeWithId<i64, ()>> = Graph::new_2d(n);
            let mut cnt = 0;
            for _ in 0..rng.gen_range(1..=4) {
                for _ in 0..rng.gen_range(0..=6) {
                    let (x, y, w, c) = (
                        rng.gen_range(0..n),
                        rng.gen_range(0..n),
                        rng.gen_range(-5..=5i64),
                        rng.gen_range(1..=5i64),
                    );
                    a.add_edge(WeightedFlowEdgeWithId::new(x, y, w, c));
                    b.add_edge(WeightedFlowEdgeWithId::new(x, y, w, c));
                    bw.add_edge(BiWeightedEdgeWithId::new(x, y, w));
                    bw2.add_edge(BiWeightedEdgeWithId::new(x, y, w));
                    cnt += 1;
                }
                if rng.gen_bool() {
                    a.compact();
                    bw.compact();
                }
                assert_eq!(a.edge_count(), cnt);
                assert_eq!(bw.edge_count(), cnt);
                assert_eq!(bw2.edge_count(), cnt);
                // NOTE: Linked iterates LIFO, TwoD FIFO; compare as multisets
                for v in 0..n {
                    let key = |g: &Graph<WeightedFlowEdgeWithId<i64, i64, ()>>| {
                        let mut r: Vec<_> = g
                            .adj(v)
                            .iter()
                            .map(|e| (e.to(), e.id(), e.weight(), e.capacity(), e.flow(g)))
                            .collect();
                        r.sort();
                        r
                    };
                    assert_eq!(key(&a), key(&b));
                    assert_eq!(a.degree(v), b.degree(v));
                    let key2 = |g: &Graph<BiWeightedEdgeWithId<i64, ()>>| {
                        let mut r: Vec<_> =
                            g.adj(v).iter().map(|e| (e.to(), e.id(), e.weight())).collect();
                        r.sort();
                        r
                    };
                    assert_eq!(key2(&bw), key2(&bw2));
                    // reverse links
                    for (cursor, e) in a.adj(v).iter_with_id() {
                        let rev = a.edge_at(e.to(), e.reverse_id() as u32);
                        assert_eq!(rev.to(), v);
                        assert_eq!(rev.reverse_id(), cursor);
                        assert_eq!(rev.id(), e.id());
                        assert_eq!(rev.weight(), -e.weight());
                    }
                }
                // push one unit along every edge with capacity in both
                for v in 0..n {
                    let ids: Vec<usize> = a
                        .adj(v)
                        .iter_with_id()
                        .filter(|(_, e)| e.capacity() > 0 && e.weight() > 0)
                        .map(|(_, e)| e.id())
                        .collect();
                    for id in ids {
                        for g in [&mut a, &mut b] {
                            let (cursor, _) = g
                                .adj(v)
                                .iter_with_id()
                                .find(|(_, e)| e.id() == id && e.weight() > 0 && e.capacity() > 0)
                                .unwrap();
                            let pd = g.edge_at(v, cursor as u32).push_flow(1);
                            g.push_flow(pd);
                        }
                    }
                }
            }
        }
    }
}

mod all_dist_shrink {
    use algo_lib::graph::all_distances::{AllDistances, Distance as AD};
    use algo_lib::graph::edges::weighted_edge::WeightedEdge;
    use algo_lib::graph::Graph;
    use algo_lib::misc::random::{Random, RandomTrait};

    pub fn expected(n: usize, edges: &[(usize, usize, i64)]) -> Vec<Vec<Option<Option<i64>>>> {
        const INF: i64 = i64::MAX / 4;
        let mut d = vec![vec![INF; n]; n];
        let mut reach = vec![vec![false; n]; n];
        for i in 0..n {
            d[i][i] = 0;
            reach[i][i] = true;
        }
        for &(a, b, w) in edges {
            d[a][b] = d[a][b].min(w);
            reach[a][b] = true;
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if d[i][k] < INF && d[k][j] < INF {
                        d[i][j] = d[i][j].min(d[i][k] + d[k][j]).max(-INF);
                    }
                    reach[i][j] |= reach[i][k] && reach[k][j];
                }
            }
        }
        (0..n)
            .map(|i| {
                (0..n)
                    .map(|j| {
                        if !reach[i][j] {
                            None
                        } else if (0..n).any(|c| d[c][c] < 0 && reach[i][c] && reach[c][j]) {
                            Some(None)
                        } else {
                            Some(Some(d[i][j]))
                        }
                    })
                    .collect()
            })
            .collect()
    }

    pub fn got(n: usize, edges: &[(usize, usize, i64)]) -> Vec<Vec<Option<Option<i64>>>> {
        let mut g: Graph<WeightedEdge<i64, ()>> = Graph::new_linked(n);
        for &(a, b, w) in edges {
            g.add_edge(WeightedEdge::new(a, b, w));
        }
        let all = g.all_distances();
        (0..n)
            .map(|i| {
                (0..n)
                    .map(|j| match all[(i, j)] {
                        AD::None => None,
                        AD::Infinite => Some(None),
                        AD::Finite(w) => Some(Some(w)),
                    })
                    .collect()
            })
            .collect()
    }

    #[test]
    #[ignore]
    fn shrink() {
        let mut rng = Random::new_with_seed(71);
        let mut best: Option<(usize, Vec<(usize, usize, i64)>)> = None;
        let mut fails = 0;
        for _ in 0..300000 {
            let n = rng.gen_range(2..=4usize);
            let m = rng.gen_range(1..=5usize);
            let edges: Vec<(usize, usize, i64)> = (0..m)
                .map(|_| (rng.gen_range(0..n), rng.gen_range(0..n), rng.gen_range(-1..=1i64)))
                .collect();
            if expected(n, &edges) != got(n, &edges) {
                fails += 1;
                let better = match &best {
                    None => true,
                    Some((bn, be)) => (edges.len(), n) < (be.len(), *bn),
                };
                if better {
                    best = Some((n, edges));
                }
            }
        }
        panic!("fails={fails} best={best:?}");
    }
}

mod all_dist_fix {
    use super::all_dist_shrink::{expected, got};
    use algo_lib::collections::md_arr::arr2d::Arr2d;
    use algo_lib::collections::min_max::MinimMaxim;
    use algo_lib::graph::all_distances::Distance;
    use algo_lib::misc::random::{Random, RandomTrait};

    // copy of all_distances with a final closure pass added
    fn all_distances_fixed(n: usize, edges: &[(usize, usize, i64)]) -> Arr2d<Distance<i64>> {
        let mut res = Arr2d::new(n, n, Distance::None);
        let mut has_negative = false;
        for i in 0..n {
            res[(i, i)] = Distance::Finite(0);
        }
        for &(a, b, w) in edges {
            res[(a, b)].minim(Distance::Finite(w));
            has_negative |= w < 0;
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    let r1 = res[(i, k)];
                    let r2 = res[(k, j)];
                    res[(i, j)].minim(r1 + r2);
                }
            }
            if has_negative {
                for k in 0..n {
                    if let Distance::Finite(w) = res[(k, k)] {
                        if w < 0 {
                            res[(k, k)] = Distance::Infinite;
                            for i in 0..n {
                                if res[(i, k)] == Distance::None {
                                    continue;
                                }
                                for j in 0..n {
                                    if res[(k, j)] == Distance::None {
                                        continue;
                                    }
                                    res[(i, j)] = Distance::Infinite;
                                }
                            }
                        }
                    }
                }
            }
        }
        // FIX: reachability is complete only now, redo the closure
        if has_negative {
            for k in 0..n {
                if res[(k, k)] == Distance::Infinite {
                    for i in 0..n {
                        if res[(i, k)] == Distance::None {
                            continue;
                        }
                        for j in 0..n {
                            if res[(k, j)] != Distance::None {
                                res[(i, j)] = Distance::Infinite;
                            }
                        }
                    }
                }
            }
        }
        res
    }

    #[test]
    fn fixed_copy_matches_oracle() {
        let mut rng = Random::new_with_seed(72);
        for it in 0..200000 {
            let n = rng.gen_range(1..=7usize);
            let m = rng.gen_range(0..=3 * n);
            let lo = [-1, -3, -8][it % 3];
            let edges: Vec<(usize, usize, i64)> = (0..m)
                .map(|_| (rng.gen_range(0..n), rng.gen_range(0..n), rng.gen_range(lo..=6i64)))
                .collect();
            let all = all_distances_fixed(n, &edges);
            let e = expected(n, &edges);
            for i in 0..n {
                for j in 0..n {
                    let g = match all[(i, j)] {
                        Distance::None => None,
                        Distance::Infinite => Some(None),
                        Distance::Finite(w) => Some(Some(w)),
                    };
                    assert_eq!(g, e[i][j], "{edges:?}");
                }
            }
        }
    }

    #[test]
    #[ignore]
    fn shrink_no_self_loops() {
        let mut rng = Random::new_with_seed(73);
        let mut best: Option<(usize, Vec<(usize, usize, i64)>)> = None;
        let mut fails = 0;
        for _ in 0..1000000 {
            let n = rng.gen_range(2..=5usize);
            let m = rng.gen_range(1..=7usize);
            let edges: Vec<(usize, usize, i64)> = (0..m)
                .map(|_| (rng.gen_range(0..n), rng.gen_range(0..n), rng.gen_range(-2..=2i64)))
                .filter(|e| e.0 != e.1)
                .collect();
            if expected(n, &edges) != got(n, &edges) {
                fails += 1;
                let better = match &best {
                    None => true,
                    Some((bn, be)) => (edges.len(), n) < (be.len(), *bn),
                };
                if better {
                    best = Some((n, edges));
                }
            }
        }
        panic!("fails={fails} best={best:?}");
    }
}

#[test]
fn bug_all_distances_negative_cycle() {
    use algo_lib::graph::all_distances::{AllDistances, Distance};
    use algo_lib::graph::edges::weighted_edge::WeightedEdge;
    // negative cycle 1 -> 2 -> 1 (weight -1); 0 -> 3 -> 1 reaches it and 1 -> 3 -> 0 returns
    let mut g: Graph<WeightedEdge<i64, ()>> = Graph::new_linked(4);
    for (a, b, w) in [(3, 0, 1), (3, 1, 0), (1, 3, -1), (0, 3, 1), (1, 2, -1), (2, 1, 0)] {
        g.add_edge(WeightedEdge::new(a, b, w));
    }
    let d = g.all_distances();
    for i in 0..4 {
        for j in 0..4 {
            // everything is strongly connected and contains a negative cycle
            assert!(
                d[(i, j)] == Distance::Infinite,
                "({i}, {j}) should be -inf, got {}",
                match d[(i, j)] {
                    Distance::Finite(w) => format!("Finite({w})"),
                    Distance::None => "None".to_string(),
                    Distance::Infinite => unreachable!(),
                }
            );
        }
    }
}

// ------------------------------------------------------------ extra rounds

#[test]
#[ignore]
fn sanity_harness_detects_known_slow_bug() {
    // the unfixed library version must FAIL here (proves the harness has teeth)
    use algo_lib::graph::min_cost_flow_slow::MinCostFlowSlow;
    let mut rng = Random::new_with_seed(5);
    for it in 0..6000 {
        let n = rng.gen_range(2..=7usize);
        let m = rng.gen_range(0..=3 * n);
        let edges = gen_cost_graph(&mut rng, it % 3, n, m, 3);
        let (s, t) = (0, n - 1);
        let expected = ssp_brute(n, &edges, s, t, false);
        let mut g = build_wg(n, &edges, false);
        let r = g.min_cost_flow_slow(s, t);
        assert_eq!((r.flow, r.cost), expected, "{edges:?}");
    }
}

#[test]
fn mcf_large_costs_and_caps() {
    let mut rng = Random::new_with_seed(81);
    for it in 0..3000 {
        let n = rng.gen_range(2..=8usize);
        let m = rng.gen_range(1..=3 * n);
        let mut edges = gen_cost_graph(&mut rng, it % 3, n, m, 1);
        let scale = 1_000i64 * rng.gen_range(1..=1000i64);
        for e in edges.iter_mut() {
            e.cost *= scale;
            e.cap = if rng.gen_range(0..4) == 0 {
                0
            } else {
                let bits = rng.gen_range(1..31);
                rng.gen_range(1..=(1i64 << bits))
            };
        }
        if edges.iter().all(|e| e.cap == 0) {
            continue;
        }
        let (s, t) = (0, n - 1);
        for max_flow in [false, true] {
            let mut a = build_wg(n, &edges, rng.gen_bool());
            let mut b = build_wg(n, &edges, rng.gen_bool());
            let ra = slow_fixed_impl(&mut a, s, t, max_flow);
            let rb = if max_flow {
                b.min_cost_max_flow(s, t)
            } else {
                b.min_cost_flow(s, t)
            };
            // costs can exceed i64 only if flow*cost does; keep them in range by construction
            assert_eq!(ra.cost, rb.cost, "{edges:?}");
            if max_flow {
                assert_eq!(ra.flow, rb.flow);
            }
            assert_eq!(check_cost_flow(&b, &edges, s, t), (rb.flow, rb.cost));
            assert_eq!(check_cost_flow(&a, &edges, s, t), (ra.flow, ra.cost));
        }
    }
}

#[test]
fn fast_max_flow_medium_random() {
    let mut rng = Random::new_with_seed(82);
    for it in 0..3000 {
        let n = rng.gen_range(2..=40usize);
        let m = rng.gen_range(0..=5 * n);
        let mut a: Graph<FlowEdgeWithId<i64, ()>> = mk(n, rng.gen_bool());
        let mut b = a.clone();
        let mut edges = Vec::new();
        for _ in 0..m {
            let (x, y) = (rng.gen_range(0..n), rng.gen_range(0..n));
            let c = if it % 2 == 0 {
                rng.gen_range(0..=3i64)
            } else {
                rng.gen_range(0..=1_000_000_000i64)
            };
            a.add_edge(FlowEdgeWithId::new(x, y, c));
            b.add_edge(FlowEdgeWithId::new(x, y, c));
            edges.push((x, y, c));
        }
        let s = rng.gen_range(0..n);
        let t = (s + rng.gen_range(1..n)) % n;
        let fa = a.max_flow(s, t);
        let fb = b.fast_max_flow(s, t);
        assert_eq!(fa, fb);
        assert_eq!(check_flow(&a, &edges, s, t), fa);
        assert_eq!(check_flow(&b, &edges, s, t), fb);
    }
}

mod extra {
    use super::*;
    use algo_lib::graph::edges::bi_edge::BiEdge;
    use algo_lib::graph::edges::weighted_flow_edge::WeightedFlowEdge;
    use algo_lib::graph::general_matching::GeneralMatching;
    use algo_lib::graph::two_sat::TwoSat;

    #[test]
    fn mcf_i32_and_i128_noid() {
        let mut rng = Random::new_with_seed(91);
        for it in 0..3000 {
            let n = rng.gen_range(2..=7usize);
            let m = rng.gen_range(1..=3 * n);
            let edges = gen_cost_graph(&mut rng, it % 3, n, m, 5);
            if edges.iter().all(|e| e.cap == 0) {
                continue;
            }
            let (s, t) = (0, n - 1);
            for max_flow in [false, true] {
                let expected = ssp_brute(n, &edges, s, t, max_flow);
                let mut a: Graph<WeightedFlowEdge<i32, i32, ()>> = mk(n, rng.gen_bool());
                let mut b: Graph<WeightedFlowEdge<i128, i128, u8>> = mk(n, rng.gen_bool());
                for e in &edges {
                    a.add_edge(WeightedFlowEdge::new(e.from, e.to, e.cost as i32, e.cap as i32));
                    b.add_edge(WeightedFlowEdge::with_payload(
                        e.from,
                        e.to,
                        e.cost as i128,
                        e.cap as i128,
                        7,
                    ));
                }
                let (ra, rb) = if max_flow {
                    (a.min_cost_max_flow(s, t), b.min_cost_max_flow(s, t))
                } else {
                    (a.min_cost_flow(s, t), b.min_cost_flow(s, t))
                };
                assert_eq!(ra.cost as i64, expected.1);
                assert_eq!(rb.cost as i64, expected.1);
                if max_flow {
                    assert_eq!(ra.flow as i64, expected.0);
                    assert_eq!(rb.flow as i64, expected.0);
                }
            }
        }
    }

    #[test]
    fn general_matching_bigger() {
        let mut rng = Random::new_with_seed(92);
        for it in 0..1500 {
            let n = rng.gen_range(10..=15usize);
            let m = match it % 3 {
                0 => rng.gen_range(n / 2..=n),
                1 => rng.gen_range(n..=2 * n),
                _ => rng.gen_range(n..=4 * n),
            };
            let mut adjacent = vec![vec![false; n]; n];
            let mut g: Graph<BiEdge<()>> = mk(n, rng.gen_bool());
            for _ in 0..m {
                let (a, b) = (rng.gen_range(0..n), rng.gen_range(0..n));
                g.add_edge(BiEdge::new(a, b));
                if a != b {
                    adjacent[a][b] = true;
                    adjacent[b][a] = true;
                }
            }
            let mut best = vec![0usize; 1 << n];
            for mask in 1usize..1 << n {
                let v = mask.trailing_zeros() as usize;
                let rest = mask & (mask - 1);
                let mut res = best[rest];
                for u in 0..n {
                    if rest >> u & 1 == 1 && adjacent[v][u] {
                        res = res.max(1 + best[rest & !(1 << u)]);
                    }
                }
                best[mask] = res;
            }
            let mate = g.general_matching();
            let mut size = 0;
            for v in 0..n {
                if let Some(u) = mate[v] {
                    assert!(adjacent[v][u]);
                    assert_eq!(mate[u], Some(v));
                    size += 1;
                }
            }
            assert_eq!(size / 2, best[(1 << n) - 1]);
        }
    }

    #[test]
    fn two_sat_max_one_with_repeated_variables() {
        // "at most one of the listed literals (counted by position) is true"
        let mut rng = Random::new_with_seed(93);
        for _ in 0..20000 {
            let n = rng.gen_range(1..=4usize);
            let len = rng.gen_range(0..=5usize);
            let lits: Vec<(usize, bool)> =
                (0..len).map(|_| (rng.gen_range(0..n), rng.gen_bool())).collect();
            let mut ts = TwoSat::new(n);
            ts.max_one(&lits);
            let mut forced = Vec::new();
            for v in 0..n {
                match rng.gen_range(0..3) {
                    0 => {
                        ts.set_true(v);
                        forced.push((v, true));
                    }
                    1 => {
                        ts.set_false(v);
                        forced.push((v, false));
                    }
                    _ => {}
                }
            }
            let ok = |x: &[bool]| {
                lits.iter().filter(|&&(a, v)| x[a] == v).count() <= 1
                    && forced.iter().all(|&(a, v)| x[a] == v)
            };
            let sat = (0u32..1 << n).any(|mask| {
                let x: Vec<bool> = (0..n).map(|i| mask >> i & 1 == 1).collect();
                ok(&x)
            });
            let res = ts.solve();
            assert_eq!(res.is_some(), sat, "n={n} lits={lits:?} forced={forced:?}");
            if let Some(x) = res {
                assert!(ok(&x), "n={n} lits={lits:?} forced={forced:?} x={x:?}");
            }
        }
    }

    #[test]
    #[ignore]
    fn note_max_flow_source_equals_sink() {
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let mut g: Graph<FlowEdgeWithId<i64, ()>> = Graph::new_linked(2);
            g.add_edge(FlowEdgeWithId::new(0, 1, 1));
            tx.send(g.max_flow(0, 0)).ok();
        });
        let r = rx.recv_timeout(std::time::Duration::from_secs(3));
        assert_eq!(r, Ok(0));
    }

    #[test]
    #[ignore]
    fn note_diameter_of_empty_graph() {
        use algo_lib::graph::edge_distances::BiEdgeAlgos;
        let g: Graph<BiEdge<()>> = Graph::new_linked(0);
        assert_eq!(g.centers(), Vec::<usize>::new());
        assert_eq!(g.diameter(), 0);
    }
}

#[test]
#[ignore] // burns a core until the test binary exits
fn bug_slow_mcf_exponential_hang() {
    use algo_lib::graph::min_cost_flow_slow::MinCostFlowSlow;
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let (mut g, s, t, _) = johnson_graph(40); // 81 vertices, 120 edges, no negative cycle
        let r = g.min_cost_max_flow_slow(s, t);
        tx.send((r.flow, r.cost)).ok();
    });
    let r = rx.recv_timeout(std::time::Duration::from_secs(10));
    assert_eq!(r, Ok((2, -((1i64 << 40) - 1))));
}
