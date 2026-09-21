#![allow(clippy::all)]
#![allow(dead_code)]
use algo_lib::collections::dsu::DSU;
use algo_lib::graph::block_cut_tree::BlockCutTreeBuild;
use algo_lib::graph::bridges::BridgeSearch;
use algo_lib::graph::complement_components::ComplementComponents;
use algo_lib::graph::cut_points::CutPointSearch;
use algo_lib::graph::dominator_tree::DominatorTree;
use algo_lib::graph::edges::bi_edge::{BiEdge, BiEdgeWithId};
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::edge::{Edge, EdgeWithId};
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::strongly_connected_components::StronglyConnectedComponentsTrait;
use algo_lib::graph::topological_sort::TopologicalSort;
use algo_lib::graph::Graph;

pub struct Rng(pub u64);
impl Rng {
    pub fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    pub fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
    pub fn range(&mut self, lo: i64, hi: i64) -> i64 {
        lo + (self.next() % (hi - lo + 1) as u64) as i64
    }
}

fn mk<E: EdgeTrait>(n: usize, mode: usize, edges: Vec<(usize, E)>) -> Graph<E> {
    let mut g = if mode == 1 {
        Graph::new_2d(n)
    } else {
        Graph::new_linked(n)
    };
    let half = edges.len() / 2;
    for (i, e) in edges.into_iter().enumerate() {
        if mode == 3 && i == half {
            g.compact();
        }
        g.add_edge(e);
    }
    if mode == 2 {
        g.compact();
    }
    g
}

fn rand_edges(rng: &mut Rng, n: usize, m: usize, loops: bool) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    for _ in 0..m {
        let a = rng.below(n);
        let b = rng.below(n);
        if a != b || loops {
            res.push((a, b));
        }
    }
    res
}

// ---------- Graph storage ----------
#[test]
fn graph_storage_consistency() {
    let mut rng = Rng(12345);
    for it in 0..3000 {
        let n = 1 + rng.below(7);
        let m = rng.below(15);
        let edges = rand_edges(&mut rng, n, m, true);
        for mode in 0..4 {
            let g: Graph<EdgeWithId<()>> = mk(
                n,
                mode,
                edges.iter().map(|&(a, b)| EdgeWithId::new(a, b)).collect(),
            );
            assert_eq!(g.edge_count(), edges.len());
            let mut expect: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
            for (i, &(a, b)) in edges.iter().enumerate() {
                expect[a].push((b, i));
            }
            for v in 0..n {
                let mut got: Vec<(usize, usize)> =
                    g.adj(v).iter().map(|e| (e.to(), e.id())).collect();
                got.sort();
                let mut ex = expect[v].clone();
                ex.sort();
                assert_eq!(got, ex, "it={it} mode={mode}");
                assert_eq!(g.degree(v), ex.len());
                assert_eq!(g.adj(v).len(), ex.len());
                assert_eq!(g.degrees()[v], ex.len());
                let via_iter: Vec<(usize, usize)> =
                    g.adj(v).iter().map(|e| (e.to(), e.id())).collect();
                let mut via_cursor = Vec::new();
                let mut c = g.head_edge(v);
                while c != u32::MAX {
                    let e = g.edge_at(v, c);
                    via_cursor.push((e.to(), e.id()));
                    c = g.step_edge(v, c);
                }
                assert_eq!(via_iter, via_cursor);
                let via_id: Vec<(usize, usize)> = g
                    .adj(v)
                    .iter_with_id()
                    .map(|(c, _)| {
                        let e = g.edge_at(v, c as u32);
                        (e.to(), e.id())
                    })
                    .collect();
                assert_eq!(via_iter, via_id);
                for i in 0..=ex.len() {
                    assert_eq!(
                        g.adj(v).get(i).map(|e| (e.to(), e.id())),
                        via_iter.get(i).copied()
                    );
                }
                assert_eq!(g.adj(v).head_id(), g.head_edge(v));
            }
            assert_eq!(g.edges().count(), edges.len());
            let g: Graph<BiEdgeWithId<()>> = mk(
                n,
                mode,
                edges.iter().map(|&(a, b)| BiEdgeWithId::new(a, b)).collect(),
            );
            assert_eq!(g.edge_count(), edges.len());
            let mut expect: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
            for (i, &(a, b)) in edges.iter().enumerate() {
                expect[a].push((b, i));
                expect[b].push((a, i));
            }
            let mut dsu = DSU::new(n);
            let mut acyclic = true;
            for &(a, b) in &edges {
                if !dsu.union(a, b) {
                    acyclic = false;
                }
            }
            assert_eq!(g.is_connected(), dsu.set_count() == 1);
            assert_eq!(
                g.is_tree(),
                dsu.set_count() == 1 && edges.len() + 1 == n,
                "{edges:?}"
            );
            assert_eq!(g.is_forest(), acyclic, "is_forest n={n} {edges:?} mode={mode}");
            for v in 0..n {
                let mut got: Vec<(usize, usize)> =
                    g.adj(v).iter().map(|e| (e.to(), e.id())).collect();
                got.sort();
                let mut ex = expect[v].clone();
                ex.sort();
                assert_eq!(got, ex, "bi it={it} mode={mode}");
                assert_eq!(g.degree(v), ex.len());
                let via_iter: Vec<(usize, usize)> =
                    g.adj(v).iter().map(|e| (e.to(), e.id())).collect();
                let mut via_cursor = Vec::new();
                let mut c = g.head_edge(v);
                while c != u32::MAX {
                    let e = g.edge_at(v, c);
                    via_cursor.push((e.to(), e.id()));
                    c = g.step_edge(v, c);
                }
                assert_eq!(via_iter, via_cursor);
            }
            let mut g: Graph<BiWeightedEdge<i64, ()>> = mk(
                n,
                mode,
                edges
                    .iter()
                    .map(|&(a, b)| BiWeightedEdge::new(a, b, 1))
                    .collect(),
            );
            for v in 0..n {
                for e in g.adj_mut(v).iter_mut() {
                    *e.weight_mut() += 10;
                }
            }
            for v in 0..n {
                for e in g.adj(v).iter() {
                    assert_eq!(e.weight(), 11);
                }
            }
            g.clear();
            assert_eq!(g.edge_count(), 0);
            g.add_vertices(2);
            assert_eq!(g.vertex_count(), n + 2);
            g.add_edge(BiWeightedEdge::new(n + 1, 0, 5));
            assert_eq!(g.edge_count(), 1);
            assert_eq!(g.degree(n + 1), 1);
            assert_eq!(g.degree(0), 1);
            assert_eq!(g.adj(0).iter().next().unwrap().to(), n + 1);
        }
    }
}

// ---------- SCC ----------
#[test]
fn scc_vs_reachability() {
    let mut rng = Rng(777);
    for it in 0..6000 {
        let n = 1 + rng.below(9);
        let m = rng.below(3 * n + 1);
        let edges = rand_edges(&mut rng, n, m, true);
        let mut reach = vec![vec![false; n]; n];
        for i in 0..n {
            reach[i][i] = true;
        }
        for &(a, b) in &edges {
            reach[a][b] = true;
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if reach[i][k] && reach[k][j] {
                        reach[i][j] = true;
                    }
                }
            }
        }
        let mode = it % 4;
        let g: Graph<Edge<()>> = mk(
            n,
            mode,
            edges.iter().map(|&(a, b)| Edge::new(a, b)).collect(),
        );
        let scc = g.strongly_connected_components();
        assert_eq!(scc.color, g.strongly_connected_component_colors());
        let comps = scc.color.iter().max().unwrap() + 1;
        assert_eq!(scc.condensed.vertex_count(), comps);
        let mut used = vec![false; comps];
        for i in 0..n {
            used[scc.color[i]] = true;
            for j in 0..n {
                assert_eq!(scc.color[i] == scc.color[j], reach[i][j] && reach[j][i]);
                if reach[i][j] {
                    assert!(scc.color[i] <= scc.color[j]);
                }
            }
        }
        assert!(used.iter().all(|&u| u));
        let mut expected = std::collections::BTreeSet::new();
        for &(a, b) in &edges {
            if scc.color[a] != scc.color[b] {
                expected.insert((scc.color[a], scc.color[b]));
            }
        }
        let mut got = Vec::new();
        for c in 0..comps {
            for e in scc.condensed.adj(c).iter() {
                got.push((c, e.to()));
            }
        }
        got.sort();
        assert_eq!(got, expected.into_iter().collect::<Vec<_>>(), "{edges:?}");
    }
    let g: Graph<Edge<()>> = Graph::new_linked(0);
    let scc = g.strongly_connected_components();
    assert!(scc.color.is_empty());
    assert_eq!(scc.condensed.vertex_count(), 0);
}

fn comps_without(
    n: usize,
    edges: &[(usize, usize)],
    skip_edge: Option<usize>,
    skip_v: Option<usize>,
) -> usize {
    let mut dsu = DSU::new(n);
    for (i, &(a, b)) in edges.iter().enumerate() {
        if Some(i) == skip_edge || Some(a) == skip_v || Some(b) == skip_v {
            continue;
        }
        dsu.union(a, b);
    }
    dsu.set_count() - if skip_v.is_some() { 1 } else { 0 }
}

// ---------- bridges / cut points / block-cut ----------
#[test]
fn bridges_cutpoints_bct_vs_brute() {
    let mut rng = Rng(4242);
    for it in 0..8000 {
        let n = 1 + rng.below(8);
        let m = rng.below(12);
        let edges = rand_edges(&mut rng, n, m, true);
        let mode = it % 4;
        let g: Graph<BiEdge<()>> = mk(
            n,
            mode,
            edges.iter().map(|&(a, b)| BiEdge::new(a, b)).collect(),
        );
        let base = comps_without(n, &edges, None, None);
        let mut exp_b: Vec<(usize, usize)> = (0..edges.len())
            .filter(|&i| comps_without(n, &edges, Some(i), None) > base)
            .map(|i| (edges[i].0.min(edges[i].1), edges[i].0.max(edges[i].1)))
            .collect();
        exp_b.sort();
        let mut got_b: Vec<(usize, usize)> = g
            .bridges()
            .into_iter()
            .map(|(a, b)| (a.min(b), a.max(b)))
            .collect();
        got_b.sort();
        assert_eq!(got_b, exp_b, "bridges n={n} {edges:?} mode={mode}");

        let exp_c: Vec<usize> = (0..n)
            .filter(|&v| comps_without(n, &edges, None, Some(v)) > base)
            .collect();
        let mut got_c = g.cut_points();
        got_c.sort();
        assert_eq!(got_c, exp_c, "cut points n={n} {edges:?} mode={mode}");

        let bct = g.block_cut_tree();
        let from_tree: Vec<usize> = (0..n)
            .filter(|&v| bct.is_cut[bct.vertex_node[v]])
            .collect();
        assert_eq!(from_tree, exp_c, "bct cuts n={n} {edges:?}");
        assert_eq!(bct.node_vertices.len(), bct.node_count);
        assert_eq!(bct.tree.vertex_count(), bct.node_count);
        assert!(bct.tree.is_forest(), "bct forest n={n} {edges:?}");
        let blocks: Vec<&Vec<usize>> = (0..bct.node_count)
            .filter(|&k| !bct.is_cut[k])
            .map(|k| &bct.node_vertices[k])
            .collect();
        for &(a, b) in &edges {
            if a == b {
                continue;
            }
            let holding = blocks
                .iter()
                .filter(|bl| bl.contains(&a) && bl.contains(&b))
                .count();
            assert_eq!(holding, 1, "edge {a}-{b} n={n} {edges:?}");
        }
        for i in 0..blocks.len() {
            for j in 0..i {
                let shared = blocks[i].iter().filter(|v| blocks[j].contains(v)).count();
                assert!(shared <= 1, "blocks share >1 n={n} {edges:?}");
            }
            let bl = blocks[i];
            if bl.len() >= 2 {
                let idx = |v: usize| bl.iter().position(|&x| x == v);
                let sub: Vec<(usize, usize)> = edges
                    .iter()
                    .filter_map(|&(a, b)| Some((idx(a)?, idx(b)?)))
                    .collect();
                assert_eq!(comps_without(bl.len(), &sub, None, None), 1);
                if bl.len() >= 3 {
                    for v in 0..bl.len() {
                        assert_eq!(
                            comps_without(bl.len(), &sub, None, Some(v)),
                            1,
                            "block has cut vertex n={n} {edges:?}"
                        );
                    }
                }
            }
        }
        for v in 0..n {
            let cnt = blocks.iter().filter(|bl| bl.contains(&v)).count();
            if exp_c.contains(&v) {
                assert!(cnt >= 2);
                assert_eq!(bct.tree.degree(bct.vertex_node[v]), cnt);
            } else {
                assert_eq!(cnt, 1, "v={v} n={n} {edges:?}");
                assert!(bct.node_vertices[bct.vertex_node[v]].contains(&v));
            }
        }
    }
}

// ---------- dominators ----------
fn reachable(n: usize, edges: &[(usize, usize)], root: usize, removed: Option<usize>) -> Vec<bool> {
    let mut seen = vec![false; n];
    if Some(root) == removed {
        return seen;
    }
    seen[root] = true;
    let mut stack = vec![root];
    while let Some(v) = stack.pop() {
        for &(a, b) in edges {
            if a == v && Some(b) != removed && !seen[b] {
                seen[b] = true;
                stack.push(b);
            }
        }
    }
    seen
}

#[test]
fn dominators_vs_brute() {
    let mut rng = Rng(99);
    for it in 0..5000 {
        let n = 1 + rng.below(if it % 10 == 0 { 25 } else { 9 });
        let m = rng.below(if it % 3 == 0 { 2 * n + 1 } else { 4 * n + 1 });
        let edges = rand_edges(&mut rng, n, m, true);
        let root = rng.below(n);
        let base = reachable(n, &edges, root, None);
        let rem: Vec<Vec<bool>> = (0..n)
            .map(|u| reachable(n, &edges, root, Some(u)))
            .collect();
        let g: Graph<Edge<()>> = mk(
            n,
            it % 4,
            edges.iter().map(|&(a, b)| Edge::new(a, b)).collect(),
        );
        let got = g.dominator_tree(root);
        for v in 0..n {
            if !base[v] || v == root {
                assert_eq!(got[v], None);
                continue;
            }
            let doms: Vec<usize> = (0..n).filter(|&u| u != v && !rem[u][v]).collect();
            let idom = doms
                .iter()
                .copied()
                .find(|&d| doms.iter().all(|&o| o == d || !rem[o][d]))
                .unwrap();
            assert_eq!(got[v], Some(idom), "v={v} root={root} {edges:?}");
        }
    }
}

// ---------- topological sort ----------
#[test]
fn topo_sort() {
    let mut rng = Rng(5);
    for it in 0..3000 {
        let n = 1 + rng.below(8);
        let m = rng.below(2 * n + 1);
        let mut edges = rand_edges(&mut rng, n, m, it % 2 == 0);
        if it % 3 == 0 {
            let mut perm: Vec<usize> = (0..n).collect();
            for i in 1..n {
                perm.swap(i, rng.below(i + 1));
            }
            edges = edges
                .into_iter()
                .filter(|&(a, b)| a != b)
                .map(|(a, b)| (perm[a.min(b)], perm[a.max(b)]))
                .collect();
        }
        let g: Graph<Edge<()>> = mk(
            n,
            it % 4,
            edges.iter().map(|&(a, b)| Edge::new(a, b)).collect(),
        );
        let scc = g.strongly_connected_component_colors();
        let comps = scc.iter().max().unwrap() + 1;
        let cyclic = comps < n || edges.iter().any(|&(a, b)| a == b);
        match g.topological_sort() {
            None => assert!(cyclic, "{edges:?}"),
            Some(order) => {
                assert!(!cyclic);
                let mut pos = vec![usize::MAX; n];
                for (i, &v) in order.iter().enumerate() {
                    assert_eq!(pos[v], usize::MAX);
                    pos[v] = i;
                }
                assert_eq!(order.len(), n);
                for &(a, b) in &edges {
                    assert!(pos[a] < pos[b]);
                }
            }
        }
    }
}

// ---------- complement components ----------
#[test]
fn complement_components() {
    let mut rng = Rng(8);
    for it in 0..3000 {
        let n = 1 + rng.below(9);
        let m = rng.below(n * n + 1);
        let edges = rand_edges(&mut rng, n, m, true);
        let mut adjm = vec![vec![false; n]; n];
        for &(a, b) in &edges {
            adjm[a][b] = true;
            adjm[b][a] = true;
        }
        let mut dsu = DSU::new(n);
        for a in 0..n {
            for b in 0..a {
                if !adjm[a][b] {
                    dsu.union(a, b);
                }
            }
        }
        let g: Graph<BiEdge<()>> = mk(
            n,
            it % 4,
            edges.iter().map(|&(a, b)| BiEdge::new(a, b)).collect(),
        );
        let color = g.complement_components();
        let cnt = color.iter().max().unwrap() + 1;
        assert_eq!(cnt, dsu.set_count());
        for a in 0..n {
            for b in 0..n {
                assert_eq!(
                    color[a] == color[b],
                    dsu.find(a) == dsu.find(b),
                    "{edges:?}"
                );
            }
        }
    }
}

// =====================================================================
// PART 2: euler path, MST, rooted MST, trees
// =====================================================================
mod part2 {
    use super::{mk, rand_edges, Rng};
    use algo_lib::collections::dsu::DSU;
    use algo_lib::collections::segment_tree::SegmentTreeNode;
    use algo_lib::graph::central_decomposition::Decompose;
    use algo_lib::graph::dfs_order::DFSOrderTrait;
    use algo_lib::graph::edges::bi_edge::{BiEdge, BiEdgeWithId};
    use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
    use algo_lib::graph::edges::edge_trait::EdgeTrait;
    use algo_lib::graph::edges::weighted_edge::WeightedEdge;
    use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
    use algo_lib::graph::euler_path::EulerPath;
    use algo_lib::graph::hl_decomposition::HLDecompositionTrait;
    use algo_lib::graph::lca::LCATrait;
    use algo_lib::graph::minimal_spanning_rooted_tree::MinimalSpanningRootedTree;
    use algo_lib::graph::minimal_spanning_tree::MinimalSpanningTree;
    use algo_lib::graph::mo_on_tree::{mo_on_tree, MoOnTreeWorker};
    use algo_lib::graph::path_segment_tree::{PathDirection, PathSegmentTreeTrait};
    use algo_lib::graph::Graph;
    use std::ops::Bound;

    fn euler_exists(n: usize, edges: &[(usize, usize)]) -> bool {
        let mut deg = vec![0; n];
        let mut dsu = DSU::new(n);
        for &(a, b) in edges {
            deg[a] += 1;
            deg[b] += 1;
            dsu.union(a, b);
        }
        let odd = deg.iter().filter(|&&d| d % 2 == 1).count();
        if odd > 2 {
            return false;
        }
        let mut roots: Vec<usize> = (0..n).filter(|&v| deg[v] > 0).map(|v| dsu.find(v)).collect();
        roots.sort();
        roots.dedup();
        roots.len() <= 1
    }

    fn check_euler(n: usize, edges: &[(usize, usize)], mode: usize) -> Result<(), String> {
        let g: Graph<BiEdgeWithId<()>> = mk(
            n,
            mode,
            edges.iter().map(|&(a, b)| BiEdgeWithId::new(a, b)).collect(),
        );
        let exists = euler_exists(n, edges);
        match g.euler_path() {
            None => {
                if exists {
                    return Err(format!("path exists but None: n={n} {edges:?}"));
                }
            }
            Some(p) => {
                if !exists {
                    return Err(format!("no path but Some: n={n} {edges:?} {p:?}"));
                }
                if p.len() != edges.len() + 1 {
                    return Err("bad len".to_string());
                }
                let mut a: Vec<(usize, usize)> =
                    p.windows(2).map(|w| (w[0].min(w[1]), w[0].max(w[1]))).collect();
                let mut b: Vec<(usize, usize)> =
                    edges.iter().map(|&(x, y)| (x.min(y), x.max(y))).collect();
                a.sort();
                b.sort();
                if a != b {
                    return Err(format!("not an euler path n={n} {edges:?} {p:?}"));
                }
            }
        }
        Ok(())
    }

    #[test]
    fn euler_path_connected_from_zero() {
        // restrict to cases where vertex 0 is not isolated (or no edges), all modes
        let mut rng = Rng(31337);
        for it in 0..20000 {
            let n = 1 + rng.below(6);
            let m = rng.below(9);
            let edges = rand_edges(&mut rng, n, m, true);
            if !edges.is_empty() && !edges.iter().any(|&(a, b)| a == 0 || b == 0) {
                continue;
            }
            check_euler(n, &edges, it % 4).unwrap();
        }
    }

    #[test]
    fn euler_path_isolated_start() {
        let mut rng = Rng(31338);
        let mut failures = Vec::new();
        for it in 0..20000 {
            let n = 1 + rng.below(6);
            let m = rng.below(9);
            let edges = rand_edges(&mut rng, n, m, true);
            if let Err(e) = check_euler(n, &edges, it % 4) {
                failures.push(e);
            }
        }
        assert!(failures.is_empty(), "{} failures, first: {}", failures.len(), failures[0]);
    }

    #[test]
    fn mst_vs_brute() {
        let mut rng = Rng(271828);
        for it in 0..4000 {
            let n = 1 + rng.below(6);
            let m = rng.below(10);
            let edges = rand_edges(&mut rng, n, m, true);
            let ws: Vec<i64> = edges.iter().map(|_| rng.range(-5, 5)).collect();
            let mut dsu = DSU::new(n);
            for &(a, b) in &edges {
                dsu.union(a, b);
            }
            let need = n - dsu.set_count();
            let mut best = i64::MAX;
            for mask in 0u32..(1 << edges.len()) {
                if mask.count_ones() as usize != need {
                    continue;
                }
                let mut d = DSU::new(n);
                let mut ok = true;
                let mut w = 0;
                for i in 0..edges.len() {
                    if mask >> i & 1 == 1 {
                        if !d.union(edges[i].0, edges[i].1) {
                            ok = false;
                            break;
                        }
                        w += ws[i];
                    }
                }
                if ok {
                    best = best.min(w);
                }
            }
            let g: Graph<BiWeightedEdge<i64, ()>> = mk(
                n,
                it % 4,
                edges
                    .iter()
                    .zip(ws.iter())
                    .map(|(&(a, b), &w)| BiWeightedEdge::new(a, b, w))
                    .collect(),
            );
            assert_eq!(g.minimal_spanning_tree_weight(), best, "n={n} {edges:?} {ws:?}");
            let t = g.minimal_spanning_tree();
            assert_eq!(t.edge_count(), need);
            assert!(t.is_forest());
            let mut total = 0;
            let mut d = DSU::new(n);
            for v in 0..n {
                for e in t.adj(v).iter() {
                    if e.to() > v {
                        total += e.weight();
                    }
                    d.union(v, e.to());
                    assert!(edges.iter().zip(ws.iter()).any(|(&(a, b), &w)| w == e.weight()
                        && ((a, b) == (v, e.to()) || (b, a) == (v, e.to()))));
                }
            }
            assert_eq!(total, best);
            assert_eq!(d.set_count(), dsu.set_count());
        }
    }

    #[test]
    fn rooted_mst_vs_brute() {
        let mut rng = Rng(161803);
        for it in 0..30000 {
            let n = 1 + rng.below(if it % 5 == 0 { 7 } else { 5 });
            let m = rng.below(if it % 2 == 0 { 14 } else { 7 });
            let edges = rand_edges(&mut rng, n, m, true);
            let wr = if it % 3 == 0 { 2 } else { 20 };
            let ws: Vec<i64> = edges.iter().map(|_| rng.range(-wr, wr)).collect();
            let root = rng.below(n);
            // brute force: choose an incoming edge for every non-root vertex
            let mut inc: Vec<Vec<usize>> = vec![Vec::new(); n];
            for (i, &(a, b)) in edges.iter().enumerate() {
                if a != b && b != root {
                    inc[b].push(i);
                }
            }
            let mut best: Option<i64> = None;
            let others: Vec<usize> = (0..n).filter(|&v| v != root).collect();
            if others.iter().all(|&v| !inc[v].is_empty()) {
                let mut choice = vec![0usize; others.len()];
                'outer: loop {
                    let mut par = vec![usize::MAX; n];
                    let mut w = 0;
                    for (k, &v) in others.iter().enumerate() {
                        let e = inc[v][choice[k]];
                        par[v] = edges[e].0;
                        w += ws[e];
                    }
                    let mut ok = true;
                    for &v in &others {
                        let mut cur = v;
                        let mut steps = 0;
                        while cur != root && steps <= n {
                            cur = par[cur];
                            steps += 1;
                        }
                        if cur != root {
                            ok = false;
                            break;
                        }
                    }
                    if ok {
                        best = Some(best.map_or(w, |b: i64| b.min(w)));
                    }
                    let mut k = 0;
                    loop {
                        if k == others.len() {
                            break 'outer;
                        }
                        choice[k] += 1;
                        if choice[k] < inc[others[k]].len() {
                            break;
                        }
                        choice[k] = 0;
                        k += 1;
                    }
                }
            }
            let g: Graph<WeightedEdge<i64, ()>> = mk(
                n,
                it % 4,
                edges
                    .iter()
                    .zip(ws.iter())
                    .map(|(&(a, b), &w)| WeightedEdge::new(a, b, w))
                    .collect(),
            );
            let got_w = g.minimal_spanning_rooted_tree_weight(root);
            assert_eq!(got_w, best, "n={n} root={root} {edges:?} {ws:?}");
            let t = g.minimal_spanning_rooted_tree(root);
            assert_eq!(t.is_some(), best.is_some());
            if let Some(t) = t {
                assert_eq!(t.vertex_count(), n);
                assert_eq!(t.edge_count(), n - 1);
                let mut indeg = vec![0; n];
                let mut total = 0;
                for v in 0..n {
                    for e in t.adj(v).iter() {
                        indeg[e.to()] += 1;
                        total += e.weight();
                        assert!(edges
                            .iter()
                            .zip(ws.iter())
                            .any(|(&(a, b), &w)| (a, b, w) == (v, e.to(), e.weight())));
                    }
                }
                assert_eq!(total, best.unwrap(), "tree weight n={n} root={root} {edges:?} {ws:?}");
                for v in 0..n {
                    assert_eq!(indeg[v], if v == root { 0 } else { 1 }, "indeg n={n} root={root} {edges:?} {ws:?}");
                }
                // reachability from root
                let mut seen = vec![false; n];
                seen[root] = true;
                let mut st = vec![root];
                while let Some(v) = st.pop() {
                    for e in t.adj(v).iter() {
                        assert!(!seen[e.to()]);
                        seen[e.to()] = true;
                        st.push(e.to());
                    }
                }
                assert!(seen.iter().all(|&s| s));
            }
        }
    }

    // ------------ trees ------------
    pub struct NaiveTree {
        pub n: usize,
        pub edges: Vec<(usize, usize)>,
        pub parent: Vec<usize>,
        pub level: Vec<usize>,
    }

    impl NaiveTree {
        pub fn random(rng: &mut Rng, n: usize, root: usize, shape: usize) -> Self {
            let mut perm: Vec<usize> = (0..n).collect();
            for i in 1..n {
                perm.swap(i, rng.below(i + 1));
            }
            let mut edges = Vec::new();
            for v in 1..n {
                let p = match shape {
                    0 => rng.below(v),
                    1 => v - 1,
                    2 => 0,
                    3 => v - 1 - rng.below(v.min(3)),
                    _ => (v - 1) / 2,
                };
                if rng.below(2) == 0 {
                    edges.push((perm[p], perm[v]));
                } else {
                    edges.push((perm[v], perm[p]));
                }
            }
            for i in 1..edges.len() {
                let j = rng.below(i + 1);
                edges.swap(i, j);
            }
            let mut adj = vec![Vec::new(); n];
            for &(a, b) in &edges {
                adj[a].push(b);
                adj[b].push(a);
            }
            let mut parent = vec![usize::MAX; n];
            let mut level = vec![0; n];
            parent[root] = root;
            let mut st = vec![root];
            while let Some(v) = st.pop() {
                for &u in &adj[v] {
                    if parent[u] == usize::MAX {
                        parent[u] = v;
                        level[u] = level[v] + 1;
                        st.push(u);
                    }
                }
            }
            Self {
                n,
                edges,
                parent,
                level,
            }
        }

        pub fn lca(&self, mut a: usize, mut b: usize) -> usize {
            while a != b {
                if self.level[a] >= self.level[b] {
                    a = self.parent[a];
                } else {
                    b = self.parent[b];
                }
            }
            a
        }

        pub fn path(&self, a: usize, b: usize) -> Vec<usize> {
            let l = self.lca(a, b);
            let mut left = vec![];
            let mut x = a;
            while x != l {
                left.push(x);
                x = self.parent[x];
            }
            left.push(l);
            let mut right = vec![];
            let mut y = b;
            while y != l {
                right.push(y);
                y = self.parent[y];
            }
            right.reverse();
            left.extend(right);
            left
        }

        pub fn graph(&self, mode: usize) -> Graph<BiEdge<()>> {
            mk(
                self.n,
                mode,
                self.edges.iter().map(|&(a, b)| BiEdge::new(a, b)).collect(),
            )
        }
    }

    #[test]
    fn lca_vs_naive() {
        let mut rng = Rng(1618);
        for it in 0..1500 {
            let n = 1 + rng.below(if it % 20 == 0 { 70 } else { 12 });
            let root = rng.below(n);
            let t = NaiveTree::random(&mut rng, n, root, it % 5);
            let g = t.graph(it % 4);
            let lca = g.lca_with_root(root);
            let mut pos: Vec<usize> = (0..n).map(|v| lca.position(v)).collect();
            pos.sort();
            assert_eq!(pos, (0..n).collect::<Vec<_>>());
            for a in 0..n {
                assert_eq!(lca.level(a), t.level[a]);
                assert_eq!(lca.parent(a), if a == root { None } else { Some(t.parent[a]) });
                for k in 0..n + 2 {
                    let mut x = Some(a);
                    for _ in 0..k {
                        x = x.and_then(|x| if x == root { None } else { Some(t.parent[x]) });
                    }
                    assert_eq!(lca.nth_ancestor(a, k), x, "nth_ancestor n={n} a={a} k={k}");
                }
                assert_eq!(lca.nth_ancestor(a, usize::MAX), None);
                for b in 0..n {
                    let p = t.path(a, b);
                    assert_eq!(lca.lca(a, b), t.lca(a, b));
                    assert_eq!(lca.path_length(a, b), p.len() - 1);
                    assert_eq!(lca.path(a, b), p);
                    for (i, &v) in p.iter().enumerate() {
                        assert_eq!(lca.nth_vert_on_path(a, b, i), v);
                    }
                    for c in 0..n {
                        assert_eq!(lca.on_path(a, b, c), p.contains(&c), "on_path {a} {b} {c}");
                    }
                }
            }
            assert!(lca.num_levels() >= 1);
        }
    }

    #[test]
    fn dfs_order_vs_naive() {
        let mut rng = Rng(2718);
        for it in 0..3000 {
            let n = 1 + rng.below(15);
            let root = rng.below(n);
            let t = NaiveTree::random(&mut rng, n, root, it % 5);
            let g = t.graph(it % 4);
            let d = g.dfs_order_with_root(root);
            let mut pos = d.position.clone();
            pos.sort();
            assert_eq!(pos, (0..n).collect::<Vec<_>>());
            for v in 0..n {
                let sub: Vec<usize> = (0..n).filter(|&u| t.lca(u, v) == v).collect();
                assert_eq!(d.len(v), sub.len());
                for u in 0..n {
                    assert_eq!(d.subtree(v).contains(&d.position[u]), sub.contains(&u));
                }
            }
        }
    }

    #[test]
    fn hld_vs_naive() {
        let mut rng = Rng(314159);
        for it in 0..3000 {
            let n = 1 + rng.below(if it % 20 == 0 { 60 } else { 12 });
            let root = rng.below(n);
            let t = NaiveTree::random(&mut rng, n, root, it % 5);
            let g = t.graph(it % 4);
            let h = g.hl_decomposition_with_root(root);
            assert_eq!(h.root, root);
            // paths partition the vertices; consecutive path vertices are parent/child
            let mut cnt = vec![0; n];
            for (pid, p) in h.paths.iter().enumerate() {
                for (i, &v) in p.iter().enumerate() {
                    cnt[v] += 1;
                    assert_eq!(h.id[v], pid);
                    assert_eq!(h.pos[v], i);
                    if i > 0 {
                        assert_eq!(t.parent[v], p[i - 1]);
                    }
                }
            }
            assert!(cnt.iter().all(|&c| c == 1));
            for v in 0..n {
                assert_eq!(h.parent[v], t.parent[v]);
                // number of light edges to the root is O(log n)
                let mut x = v;
                let mut jumps = 0;
                while h.id[x] != h.id[root] {
                    x = h.parent[h.paths[h.id[x]][0]];
                    jumps += 1;
                }
                assert!(1usize << jumps <= n, "too many light edges");
            }
            for a in 0..n {
                for b in 0..n {
                    assert_eq!(h.lca(a, b), t.lca(a, b));
                }
                // iterate to all ancestors
                let mut anc = a;
                loop {
                    let p = t.path(a, anc);
                    let collect = |it: algo_lib::graph::hl_decomposition::HLIter| -> Vec<usize> {
                        let mut res = Vec::new();
                        for part in it {
                            assert!(part.pos_from <= part.pos_to);
                            for i in (part.pos_from..=part.pos_to).rev() {
                                res.push(h.paths[part.id][i]);
                            }
                        }
                        res
                    };
                    assert_eq!(collect(h.iter(a..=anc)), p, "incl n={n} root={root} a={a} anc={anc} {:?}", t.edges);
                    assert_eq!(collect(h.iter(a..anc)), p[..p.len() - 1].to_vec(), "excl");
                    if a != anc {
                        assert_eq!(
                            collect(h.iter((Bound::Excluded(a), Bound::Included(anc)))),
                            p[1..].to_vec()
                        );
                        assert_eq!(
                            collect(h.iter((Bound::Excluded(a), Bound::Excluded(anc)))),
                            p[1..p.len() - 1].to_vec()
                        );
                    }
                    if anc == root {
                        assert_eq!(collect(h.iter(a..)), p);
                        break;
                    }
                    anc = t.parent[anc];
                }
            }
        }
    }

    #[derive(Clone, Default, Debug)]
    struct SeqNode {
        seq: Vec<usize>,
    }

    impl SegmentTreeNode for SeqNode {
        fn update(&mut self, l: &Self, r: &Self) {
            self.seq.clear();
            self.seq.extend_from_slice(&l.seq);
            self.seq.extend_from_slice(&r.seq);
        }
        fn swap(&mut self) {
            self.seq.reverse();
        }
    }

    const MD: i64 = 1_000_003;

    #[derive(Clone, Debug)]
    struct Affine {
        sum: i64,
        size: i64,
        mul: i64,
        add: i64,
    }

    impl Default for Affine {
        fn default() -> Self {
            Affine {
                sum: 0,
                size: 0,
                mul: 1,
                add: 0,
            }
        }
    }

    impl SegmentTreeNode for Affine {
        fn update(&mut self, l: &Self, r: &Self) {
            self.sum = (l.sum + r.sum) % MD;
            self.size = l.size + r.size;
        }
        fn accumulate(&mut self, v: &Self) {
            self.sum = (self.sum * v.mul + self.size * v.add) % MD;
            self.mul = self.mul * v.mul % MD;
            self.add = (self.add * v.mul + v.add) % MD;
        }
        fn reset_delta(&mut self) {
            self.mul = 1;
            self.add = 0;
        }
    }

    #[test]
    fn path_segment_tree_orientation() {
        let mut rng = Rng(55555);
        for it in 0..1500 {
            let n = 1 + rng.below(if it % 20 == 0 { 50 } else { 11 });
            let root = rng.below(n);
            let t = NaiveTree::random(&mut rng, n, root, it % 5);
            let g = t.graph(it % 4);
            for include_lca in [true, false] {
                let mut pst = g.path_segment_tree_with_gen_with_root(root, include_lca, |v| SeqNode {
                    seq: vec![v],
                });
                for a in 0..n {
                    for b in 0..n {
                        let l = t.lca(a, b);
                        assert_eq!(pst.lca(a, b), l);
                        let p: Vec<usize> = t
                            .path(a, b)
                            .into_iter()
                            .filter(|&v| include_lca || v != l)
                            .collect();
                        assert_eq!(pst.query(a..=b).seq, p, "query n={n} root={root} {a}->{b} incl={include_lca} {:?}", t.edges);
                        // for_each: collect sets for up/down
                        let (up, down) = pst.for_each(a..=b, |mut acc: (Vec<usize>, Vec<usize>), node, dir| {
                            match dir {
                                PathDirection::Up => acc.0.extend_from_slice(&node.seq),
                                PathDirection::Down => acc.1.extend_from_slice(&node.seq),
                            }
                            acc
                        });
                        let up_len = t.level[a] - t.level[l] + if include_lca { 1 } else { 0 };
                        let mut exp_up = p[..up_len].to_vec();
                        let mut up_sorted = up.clone();
                        up_sorted.sort();
                        exp_up.sort();
                        assert_eq!(up_sorted, exp_up);
                        assert_eq!(down, p[up_len..].to_vec(), "down leg order");
                        let (up2, down2) = pst.for_each_mut(a..=b, |mut acc: (Vec<usize>, Vec<usize>), node, dir| {
                            match dir {
                                PathDirection::Up => acc.0.extend_from_slice(&node.seq),
                                PathDirection::Down => acc.1.extend_from_slice(&node.seq),
                            }
                            acc
                        });
                        assert_eq!((up2, down2), (up, down));
                        // k-th vertex on the path via binary search
                        for k in 1..=p.len() + 1 {
                            let mut cnt = 0;
                            let res = pst.binary_search(
                                a..=b,
                                |node| {
                                    if cnt + node.seq.len() >= k {
                                        true
                                    } else {
                                        cnt += node.seq.len();
                                        false
                                    }
                                },
                                |node, v| {
                                    assert_eq!(node.seq, vec![v]);
                                    v
                                },
                            );
                            assert_eq!(res, p.get(k - 1).copied(), "bs n={n} {a}->{b} k={k}");
                            let mut cnt = 0;
                            let res = pst.binary_search_mut(
                                a..=b,
                                |node| {
                                    if cnt + node.seq.len() >= k {
                                        true
                                    } else {
                                        cnt += node.seq.len();
                                        false
                                    }
                                },
                                |_, v| v,
                            );
                            assert_eq!(res, p.get(k - 1).copied(), "bsm n={n} {a}->{b} k={k}");
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn path_segment_tree_lazy() {
        let mut rng = Rng(66666);
        for it in 0..1500 {
            let n = 1 + rng.below(if it % 10 == 0 { 60 } else { 10 });
            let root = rng.below(n);
            let t = NaiveTree::random(&mut rng, n, root, it % 5);
            let g = t.graph(it % 4);
            let include_lca = it % 2 == 0;
            let mut vals: Vec<i64> = (0..n).map(|_| rng.range(0, 100)).collect();
            let init = vals.clone();
            let mut pst = g.path_segment_tree_with_gen_with_root(root, include_lca, |v| Affine {
                sum: init[v],
                size: 1,
                mul: 1,
                add: 0,
            });
            for _ in 0..60 {
                let a = rng.below(n);
                let b = rng.below(n);
                let l = t.lca(a, b);
                let p: Vec<usize> = t
                    .path(a, b)
                    .into_iter()
                    .filter(|&v| include_lca || v != l)
                    .collect();
                match rng.below(4) {
                    0 => {
                        let mul = rng.range(0, 5);
                        let add = rng.range(0, 100);
                        pst.update(
                            a..=b,
                            Affine {
                                sum: 0,
                                size: 0,
                                mul,
                                add,
                            },
                        );
                        for &v in &p {
                            vals[v] = (vals[v] * mul + add) % MD;
                        }
                    }
                    1 => {
                        let exp = p.iter().map(|&v| vals[v]).sum::<i64>() % MD;
                        let q = pst.query(a..=b);
                        assert_eq!(q.sum, exp, "n={n} {a}->{b}");
                        assert_eq!(q.size as usize, p.len());
                    }
                    2 => {
                        if include_lca || a != root {
                            assert_eq!(pst.point_query(a).sum, vals[a]);
                        }
                    }
                    _ => {
                        let x = rng.range(0, 100);
                        if rng.below(2) == 0 {
                            pst.point_update(
                                a,
                                Affine {
                                    sum: x,
                                    size: 1,
                                    mul: 1,
                                    add: 0,
                                },
                            );
                        } else {
                            pst.point_through_update(a, |node| node.sum = x);
                        }
                        vals[a] = x;
                    }
                }
            }
        }
    }

    #[test]
    fn central_decomposition_valid() {
        let mut rng = Rng(424242);
        for it in 0..3000 {
            let n = 1 + rng.below(if it % 20 == 0 { 60 } else { 12 });
            let t = NaiveTree::random(&mut rng, n, 0, it % 5);
            let g = t.graph(it % 4);
            let mut adj = vec![Vec::new(); n];
            for &(a, b) in &t.edges {
                adj[a].push(b);
                adj[b].push(a);
            }
            let mut visited = vec![false; n];
            let mut my_closed = vec![false; n];
            g.decompose(|c, closed| {
                assert!(!visited[c]);
                visited[c] = true;
                for v in 0..n {
                    assert_eq!(closed[v], my_closed[v]);
                }
                // component size and max piece
                let size_from = |start: usize, banned: usize| -> usize {
                    let mut seen = vec![false; n];
                    seen[start] = true;
                    seen[banned] = true;
                    let mut st = vec![start];
                    let mut s = 0;
                    while let Some(v) = st.pop() {
                        s += 1;
                        for &u in &adj[v] {
                            if !seen[u] && !my_closed[u] {
                                seen[u] = true;
                                st.push(u);
                            }
                        }
                    }
                    s
                };
                let mut total = 1;
                let mut mx = 0;
                for &u in &adj[c] {
                    if !my_closed[u] {
                        let s = size_from(u, c);
                        total += s;
                        mx = mx.max(s);
                    }
                }
                assert!(2 * mx <= total, "not a centroid: n={n} c={c} mx={mx} total={total} {:?}", t.edges);
                my_closed[c] = true;
            });
            assert!(visited.iter().all(|&v| v));
        }
    }

    struct SeqWorker {
        cnt: Vec<i64>,
        sum: i64,
        distinct: i64,
    }

    impl MoOnTreeWorker for SeqWorker {
        type T = usize;
        type R = (i64, i64);
        fn empty() -> Self {
            Self {
                cnt: vec![0; 8],
                sum: 0,
                distinct: 0,
            }
        }
        fn add(&mut self, v: &usize) {
            self.cnt[*v] += 1;
            if self.cnt[*v] == 1 {
                self.distinct += 1;
            }
            self.sum += *v as i64;
        }
        fn remove(&mut self, v: &usize) {
            self.cnt[*v] -= 1;
            assert!(self.cnt[*v] >= 0, "removed value not present");
            if self.cnt[*v] == 0 {
                self.distinct -= 1;
            }
            self.sum -= *v as i64;
        }
        fn result(&self) -> (i64, i64) {
            (self.distinct, self.sum)
        }
    }

    #[test]
    fn mo_on_tree_vs_naive() {
        let mut rng = Rng(8675309);
        for it in 0..3000 {
            let n = 1 + rng.below(if it % 10 == 0 { 50 } else { 9 });
            let t = NaiveTree::random(&mut rng, n, 0, it % 5);
            let g = t.graph(it % 4);
            let values: Vec<usize> = (0..n).map(|_| rng.below(8)).collect();
            let q = rng.below(if it % 7 == 0 { 300 } else { 12 });
            let queries: Vec<(usize, usize)> = (0..q).map(|_| (rng.below(n), rng.below(n))).collect();
            let ans = mo_on_tree::<SeqWorker, _>(&g, &values, &queries);
            assert_eq!(ans.len(), q);
            for (i, &(a, b)) in queries.iter().enumerate() {
                let mut w = SeqWorker::empty();
                for v in t.path(a, b) {
                    w.add(&values[v]);
                }
                assert_eq!(ans[i], w.result(), "n={n} {a}-{b}");
            }
        }
    }
}

// =====================================================================
// PART 3: DSU family, link-cut, euler tour tree, deep paths
// =====================================================================
mod part3 {
    use super::Rng;
    use algo_lib::collections::dsu::DSU;
    use algo_lib::collections::dsu2d::DSU2d;
    use algo_lib::collections::dsu_rollback::DSURollback;
    use algo_lib::collections::dsu_weighted::WeightedDsu;
    use algo_lib::collections::euler_tour_tree::EulerTourForest;
    use algo_lib::collections::link_cut::LinkCutNode;
    use algo_lib::collections::payload::Payload;

    fn relabel(comp: &mut Vec<usize>, a: usize, b: usize) {
        let (ca, cb) = (comp[a], comp[b]);
        for c in comp.iter_mut() {
            if *c == cb {
                *c = ca;
            }
        }
    }

    #[test]
    fn dsu_vs_naive() {
        let mut rng = Rng(1001);
        for _ in 0..2000 {
            let n = 1 + rng.below(10);
            let mut dsu = DSU::new(n);
            let mut comp: Vec<usize> = (0..n).collect();
            assert_eq!(dsu.len(), n);
            for _ in 0..40 {
                let a = rng.below(n);
                let b = rng.below(n);
                match rng.below(8) {
                    0..=2 => {
                        assert_eq!(dsu.union(a, b), comp[a] != comp[b]);
                        relabel(&mut comp, a, b);
                    }
                    3 => assert_eq!(dsu.find(a) == dsu.find(b), comp[a] == comp[b]),
                    4 => assert_eq!(dsu.size(a), comp.iter().filter(|&&c| c == comp[a]).count()),
                    5 => {
                        let mut d: Vec<usize> = comp.clone();
                        d.sort();
                        d.dedup();
                        assert_eq!(dsu.set_count(), d.len());
                        assert_eq!(dsu.iter().count(), d.len());
                        for r in dsu.iter() {
                            assert_eq!(dsu.find(r), r);
                        }
                    }
                    6 => {
                        let parts = dsu.parts();
                        let mut seen = vec![false; n];
                        for p in &parts {
                            assert!(!p.is_empty());
                            for &v in p {
                                assert!(!seen[v]);
                                seen[v] = true;
                                assert_eq!(comp[v], comp[p[0]]);
                            }
                            assert_eq!(p.len(), dsu.size(p[0]));
                        }
                        assert!(seen.iter().all(|&s| s));
                        assert_eq!(parts.len(), dsu.set_count());
                    }
                    _ => {
                        if rng.below(10) == 0 {
                            dsu.clear();
                            comp = (0..n).collect();
                            assert_eq!(dsu.set_count(), n);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn dsu2d_vs_naive() {
        let mut rng = Rng(1002);
        for _ in 0..1000 {
            let rows = 1 + rng.below(4);
            let cols = 1 + rng.below(4);
            let mut dsu = DSU2d::new(rows, cols);
            let mut comp: Vec<usize> = (0..rows * cols).collect();
            for _ in 0..30 {
                let (r1, c1, r2, c2) = (rng.below(rows), rng.below(cols), rng.below(rows), rng.below(cols));
                let (a, b) = (r1 * cols + c1, r2 * cols + c2);
                match rng.below(4) {
                    0 | 1 => {
                        assert_eq!(dsu.union(r1, c1, r2, c2), comp[a] != comp[b]);
                        relabel(&mut comp, a, b);
                    }
                    2 => {
                        assert_eq!(dsu.find(r1, c1) == dsu.find(r2, c2), comp[a] == comp[b]);
                        let (fr, fc) = dsu.find(r1, c1);
                        assert!(fr < rows && fc < cols);
                        assert_eq!(comp[fr * cols + fc], comp[a]);
                        assert_eq!(dsu.size(r1, c1), comp.iter().filter(|&&c| c == comp[a]).count());
                    }
                    _ => {
                        let parts = dsu.parts();
                        let mut total = 0;
                        for p in &parts {
                            total += p.len();
                            for &(r, c) in p {
                                assert_eq!(comp[r * cols + c], comp[p[0].0 * cols + p[0].1]);
                            }
                        }
                        assert_eq!(total, rows * cols);
                        assert_eq!(parts.len(), dsu.set_count());
                        assert_eq!(dsu.iter().count(), parts.len());
                    }
                }
            }
        }
    }

    #[test]
    fn dsu_rollback_vs_naive() {
        let mut rng = Rng(1003);
        for _ in 0..2000 {
            let n = 1 + rng.below(9);
            let mut dsu = DSURollback::new(n);
            let mut comp: Vec<usize> = (0..n).collect();
            let mut saves: Vec<(usize, Vec<usize>)> = Vec::new();
            for _ in 0..60 {
                let a = rng.below(n);
                let b = rng.below(n);
                match rng.below(7) {
                    0..=2 => {
                        assert_eq!(dsu.union(a, b), comp[a] != comp[b]);
                        relabel(&mut comp, a, b);
                    }
                    3 => saves.push((dsu.save(), comp.clone())),
                    4 => {
                        if !saves.is_empty() {
                            let k = rng.below(saves.len());
                            saves.truncate(k + 1);
                            let (cp, c) = saves.pop().unwrap();
                            dsu.rollback(cp);
                            comp = c;
                        }
                    }
                    _ => {
                        assert_eq!(dsu.find(a) == dsu.find(b), comp[a] == comp[b]);
                        assert_eq!(dsu.size(a), comp.iter().filter(|&&c| c == comp[a]).count());
                        let mut d = comp.clone();
                        d.sort();
                        d.dedup();
                        assert_eq!(dsu.set_count(), d.len());
                    }
                }
            }
        }
    }

    #[test]
    fn dsu_weighted_vs_naive() {
        let mut rng = Rng(1004);
        for _ in 0..2000 {
            let n = 1 + rng.below(9);
            let mut dsu = WeightedDsu::<i64>::new(n);
            let truth: Vec<i64> = (0..n).map(|_| rng.range(-100, 100)).collect();
            let mut comp: Vec<usize> = (0..n).collect();
            for _ in 0..50 {
                let a = rng.below(n);
                let b = rng.below(n);
                match rng.below(4) {
                    0 => {
                        assert_eq!(dsu.union(a, b, truth[b] - truth[a]), Ok(comp[a] != comp[b]));
                        relabel(&mut comp, a, b);
                    }
                    1 => {
                        if comp[a] == comp[b] {
                            assert_eq!(dsu.union(a, b, truth[b] - truth[a] + 3), Err(truth[b] - truth[a]));
                        }
                    }
                    2 => {
                        assert_eq!(dsu.diff(a, b), (comp[a] == comp[b]).then(|| truth[b] - truth[a]));
                        let (r, p) = dsu.find(a);
                        assert_eq!(comp[r], comp[a]);
                        assert_eq!(p, truth[a] - truth[r]);
                    }
                    _ => {
                        assert_eq!(dsu.same(a, b), comp[a] == comp[b]);
                        assert_eq!(dsu.size(a), comp.iter().filter(|&&c| c == comp[a]).count());
                        let mut d = comp.clone();
                        d.sort();
                        d.dedup();
                        assert_eq!(dsu.set_count(), d.len());
                    }
                }
            }
        }
    }

    // ---------------- link-cut ----------------
    #[derive(Clone, Default)]
    struct LcPayload {
        id: usize,
        val: i64,
        sum: i64,
        cnt: i64,
        delta: i64,
        seq: Vec<usize>,
    }

    impl Payload for LcPayload {
        const NEED_UPDATE: bool = true;
        const NEED_ACCUMULATE: bool = true;
        fn reset_delta(&mut self) {
            self.delta = 0;
        }
        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            self.sum = self.val;
            self.cnt = 1;
            self.seq.clear();
            if let Some(l) = left {
                self.sum += l.sum;
                self.cnt += l.cnt;
                self.seq.extend_from_slice(&l.seq);
            }
            self.seq.push(self.id);
            if let Some(r) = right {
                self.sum += r.sum;
                self.cnt += r.cnt;
                self.seq.extend_from_slice(&r.seq);
            }
        }
        fn accumulate(&mut self, d: &Self) {
            self.val += d.delta;
            self.sum += d.delta * self.cnt;
            self.delta += d.delta;
        }
        fn need_push_down(&self) -> bool {
            self.delta != 0
        }
    }

    #[test]
    fn link_cut_vs_naive() {
        let mut rng = Rng(2001);
        for it in 0..1500 {
            let big = it % 100 == 0;
            let n = 1 + rng.below(if big { 300 } else if it % 10 == 0 { 40 } else { 8 });
            let mut vals: Vec<i64> = (0..n).map(|_| rng.range(-50, 50)).collect();
            let nodes: Vec<LinkCutNode<LcPayload>> = (0..n)
                .map(|i| {
                    LinkCutNode::new(LcPayload {
                        id: i,
                        val: vals[i],
                        sum: vals[i],
                        cnt: 1,
                        delta: 0,
                        seq: vec![i],
                    })
                })
                .collect();
            let mut par: Vec<Option<usize>> = vec![None; n];
            let root_path = |par: &Vec<Option<usize>>, mut v: usize| -> Vec<usize> {
                let mut p = vec![v];
                while let Some(u) = par[v] {
                    p.push(u);
                    v = u;
                }
                p.reverse();
                p
            };
            for _ in 0..(if big { 4000 } else { 120 }) {
                let a = rng.below(n);
                let b = rng.below(n);
                let pa = root_path(&par, a);
                let pb = root_path(&par, b);
                match rng.below(9) {
                    0 | 1 => {
                        // link a (must be a root) under b (different tree)
                        if par[a].is_none() && pb[0] != a {
                            nodes[a].link(nodes[b]);
                            par[a] = Some(b);
                        }
                    }
                    2 => {
                        nodes[a].cut();
                        par[a] = None;
                    }
                    3 => {
                        let r = nodes[a].find_root();
                        assert!(r == nodes[pa[0]], "find_root");
                    }
                    4 => {
                        let l = LinkCutNode::lca(nodes[a], nodes[b]);
                        if pa[0] != pb[0] {
                            assert!(l.is_none(), "lca in different trees");
                        } else {
                            let mut k = 0;
                            while k + 1 < pa.len() && k + 1 < pb.len() && pa[k + 1] == pb[k + 1] {
                                k += 1;
                            }
                            assert!(l == Some(nodes[pa[k]]), "lca n={n} a={a} b={b}");
                        }
                    }
                    5 => {
                        assert_eq!(nodes[a].dist_to_root(), pa.len() - 1);
                        assert!(nodes[a].parent() == par[a].map(|p| nodes[p]));
                    }
                    6 => {
                        let (sum, cnt, seq, val) =
                            nodes[a].with_payload(|p| (p.sum, p.cnt, p.seq.clone(), p.val));
                        assert_eq!(seq, pa, "path seq");
                        assert_eq!(cnt as usize, pa.len());
                        assert_eq!(sum, pa.iter().map(|&v| vals[v]).sum::<i64>());
                        assert_eq!(val, vals[a]);
                    }
                    7 => {
                        let d = rng.range(-10, 10);
                        nodes[a].with_payload_mut(|p| {
                            let delta = LcPayload {
                                delta: d,
                                ..Default::default()
                            };
                            p.accumulate(&delta);
                        });
                        for &v in &pa {
                            vals[v] += d;
                        }
                    }
                    _ => {
                        let x = rng.range(-50, 50);
                        nodes[a].with_payload_mut(|p| {
                            p.sum += x - p.val;
                            p.val = x;
                        });
                        vals[a] = x;
                    }
                }
            }
            for a in 0..n {
                let pa = root_path(&par, a);
                let (sum, seq) = nodes[a].with_payload(|p| (p.sum, p.seq.clone()));
                assert_eq!(seq, pa);
                assert_eq!(sum, pa.iter().map(|&v| vals[v]).sum::<i64>());
            }
        }
    }

    // ---------------- euler tour forest ----------------
    #[derive(Clone, Default, Debug)]
    struct EtPayload {
        self_cnt: i64,
        val: i64,
        cnt: i64,
        sum: i64,
        delta: i64,
        tag: (usize, usize),
    }

    impl EtPayload {
        fn vertex(v: usize, val: i64) -> Self {
            Self {
                self_cnt: 1,
                val,
                cnt: 1,
                sum: val,
                delta: 0,
                tag: (v, v),
            }
        }
        fn edge(u: usize, v: usize) -> Self {
            Self {
                tag: (u, v),
                ..Default::default()
            }
        }
    }

    impl Payload for EtPayload {
        const NEED_UPDATE: bool = true;
        const NEED_ACCUMULATE: bool = true;
        fn reset_delta(&mut self) {
            self.delta = 0;
        }
        fn update(&mut self, l: Option<&Self>, r: Option<&Self>) {
            assert_eq!(self.delta, 0, "update with pending delta");
            self.cnt = self.self_cnt + l.map_or(0, |x| x.cnt) + r.map_or(0, |x| x.cnt);
            self.sum = self.val + l.map_or(0, |x| x.sum) + r.map_or(0, |x| x.sum);
        }
        fn accumulate(&mut self, d: &Self) {
            self.val += d.delta * self.self_cnt;
            self.sum += d.delta * self.cnt;
            self.delta += d.delta;
        }
        fn need_push_down(&self) -> bool {
            self.delta != 0
        }
    }

    #[test]
    fn euler_tour_forest_vs_naive() {
        let mut rng = Rng(3001);
        for it in 0..1500 {
            let big = it % 100 == 0;
            let n = 1 + rng.below(if big { 150 } else if it % 10 == 0 { 30 } else { 7 });
            let mut f = EulerTourForest::new();
            let mut vals: Vec<i64> = (0..n).map(|_| rng.range(-50, 50)).collect();
            for v in 0..n {
                assert_eq!(f.add_node(EtPayload::vertex(v, vals[v])), v);
            }
            assert_eq!(f.node_count(), n);
            let mut edges: Vec<(usize, usize)> = Vec::new();
            let comp_of = |edges: &Vec<(usize, usize)>, start: usize, banned: Option<(usize, usize)>| -> Vec<usize> {
                let mut seen = vec![false; n];
                seen[start] = true;
                let mut st = vec![start];
                let mut res = vec![];
                while let Some(v) = st.pop() {
                    res.push(v);
                    for &(a, b) in edges.iter() {
                        if Some((a, b)) == banned || Some((b, a)) == banned {
                            continue;
                        }
                        for (x, y) in [(a, b), (b, a)] {
                            if x == v && !seen[y] {
                                seen[y] = true;
                                st.push(y);
                            }
                        }
                    }
                }
                res.sort();
                res
            };
            for _ in 0..(if big { 3000 } else { 100 }) {
                let a = rng.below(n);
                let b = rng.below(n);
                let ca = comp_of(&edges, a, None);
                match rng.below(11) {
                    0..=2 => {
                        if !ca.contains(&b) {
                            f.add_edge(a, b, EtPayload::edge(a, b), EtPayload::edge(b, a));
                            edges.push((a, b));
                        }
                    }
                    3 => {
                        if !edges.is_empty() {
                            let k = rng.below(edges.len());
                            let (u, v) = edges.swap_remove(k);
                            if rng.below(2) == 0 {
                                f.remove_edge(u, v);
                            } else {
                                f.remove_edge(v, u);
                            }
                        }
                    }
                    4 => {
                        assert_eq!(f.is_connected(a, b), ca.contains(&b));
                        assert_eq!(f.component_size(a), ca.len());
                    }
                    5 => {
                        let (sum, cnt) = f.with_component(a, |p| (p.sum, p.cnt));
                        assert_eq!(cnt as usize, ca.len());
                        assert_eq!(sum, ca.iter().map(|&v| vals[v]).sum::<i64>(), "component sum");
                    }
                    6 => {
                        let d = rng.range(-10, 10);
                        f.with_component_mut(a, |p| {
                            p.accumulate(&EtPayload {
                                delta: d,
                                ..Default::default()
                            })
                        });
                        for &v in &ca {
                            vals[v] += d;
                        }
                    }
                    7 => {
                        if !edges.is_empty() {
                            let k = rng.below(edges.len());
                            let (mut u, mut v) = edges[k];
                            if rng.below(2) == 0 {
                                std::mem::swap(&mut u, &mut v);
                            }
                            // subtree of v when u is its parent
                            let sub = comp_of(&edges, v, Some((u, v)));
                            assert_eq!(f.subtree_size(v, u), sub.len(), "subtree size");
                            let (sum, cnt) = f.with_subtree(v, u, |p| (p.sum, p.cnt));
                            assert_eq!(cnt as usize, sub.len());
                            assert_eq!(sum, sub.iter().map(|&x| vals[x]).sum::<i64>(), "subtree sum");
                            if rng.below(2) == 0 {
                                let d = rng.range(-10, 10);
                                f.with_subtree_mut(v, u, |p| {
                                    p.accumulate(&EtPayload {
                                        delta: d,
                                        ..Default::default()
                                    })
                                });
                                for &x in &sub {
                                    vals[x] += d;
                                }
                            }
                            assert_eq!(f.with_edge(u, v, |p| p.tag), (u, v));
                            assert_eq!(f.with_edge(v, u, |p| p.tag), (v, u));
                        }
                    }
                    8 => {
                        let (val, tag) = f.with_node(a, |p| (p.val, p.tag));
                        assert_eq!(tag, (a, a));
                        assert_eq!(val, vals[a], "node val");
                    }
                    _ => {
                        let x = rng.range(-50, 50);
                        f.with_node_mut(a, |p| {
                            p.val = x;
                            p.sum = x;
                        });
                        vals[a] = x;
                    }
                }
            }
            for a in 0..n {
                assert_eq!(f.with_node(a, |p| p.val), vals[a]);
                let ca = comp_of(&edges, a, None);
                assert_eq!(f.with_component(a, |p| p.sum), ca.iter().map(|&v| vals[v]).sum::<i64>());
            }
        }
    }
}

// =====================================================================
// PART 4: deep paths and larger cross-checks
// =====================================================================
mod part4 {
    use super::part2::NaiveTree;
    use super::{mk, Rng};
    use algo_lib::collections::segment_tree::SegmentTreeNode;
    use algo_lib::graph::block_cut_tree::BlockCutTreeBuild;
    use algo_lib::graph::bridges::BridgeSearch;
    use algo_lib::graph::central_decomposition::Decompose;
    use algo_lib::graph::cut_points::CutPointSearch;
    use algo_lib::graph::dfs_order::DFSOrderTrait;
    use algo_lib::graph::dominator_tree::DominatorTree;
    use algo_lib::graph::edges::bi_edge::{BiEdge, BiEdgeWithId};
    use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
    use algo_lib::graph::edges::edge::Edge;
    use algo_lib::graph::edges::edge_trait::EdgeTrait;
    use algo_lib::graph::edges::weighted_edge::WeightedEdge;
    use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
    use algo_lib::graph::euler_path::EulerPath;
    use algo_lib::graph::hl_decomposition::HLDecompositionTrait;
    use algo_lib::graph::lca::LCATrait;
    use algo_lib::graph::minimal_spanning_rooted_tree::MinimalSpanningRootedTree;
    use algo_lib::graph::minimal_spanning_tree::MinimalSpanningTree;
    use algo_lib::graph::mo_on_tree::{mo_on_tree, MoOnTreeWorker};
    use algo_lib::graph::path_segment_tree::PathSegmentTreeTrait;
    use algo_lib::graph::strongly_connected_components::StronglyConnectedComponentsTrait;
    use algo_lib::graph::topological_sort::TopologicalSort;
    use algo_lib::graph::Graph;

    const N: usize = 200_000;

    fn small_stack<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> T {
        std::thread::Builder::new()
            .stack_size(256 * 1024)
            .spawn(f)
            .unwrap()
            .join()
            .unwrap()
    }

    fn line() -> Vec<(usize, usize)> {
        (1..N).map(|v| (v - 1, v)).collect()
    }

    #[test]
    fn deep_scc_topo_dominator() {
        small_stack(|| {
            for mode in [0, 1] {
                let g: Graph<Edge<()>> =
                    mk(N, mode, line().into_iter().map(|(a, b)| Edge::new(a, b)).collect());
                let scc = g.strongly_connected_components();
                assert!((0..N).all(|v| scc.color[v] == v));
                assert_eq!(g.topological_sort().unwrap(), (0..N).collect::<Vec<_>>());
                let d = g.dominator_tree(0);
                assert!((1..N).all(|v| d[v] == Some(v - 1)));
                // one big cycle
                let mut e = line();
                e.push((N - 1, 0));
                let g: Graph<Edge<()>> =
                    mk(N, mode, e.into_iter().map(|(a, b)| Edge::new(a, b)).collect());
                let scc = g.strongly_connected_components();
                assert!(scc.color.iter().all(|&c| c == 0));
                assert!(g.topological_sort().is_none());
            }
        });
    }

    #[test]
    fn deep_bridges_cut_bct() {
        small_stack(|| {
            for mode in [0, 1] {
                let g: Graph<BiEdge<()>> =
                    mk(N, mode, line().into_iter().map(|(a, b)| BiEdge::new(a, b)).collect());
                assert_eq!(g.bridges().len(), N - 1);
                assert_eq!(g.cut_points().len(), N - 2);
                assert_eq!(g.block_cut_tree().node_count, 2 * N - 3);
            }
        });
    }

    #[test]
    fn deep_lca_dfs_order_mo() {
        small_stack(|| {
            for mode in [0, 1] {
                let g: Graph<BiEdge<()>> =
                    mk(N, mode, line().into_iter().map(|(a, b)| BiEdge::new(a, b)).collect());
                let lca = g.lca_with_root(N / 2);
                assert_eq!(lca.lca(0, N - 1), N / 2);
                assert_eq!(lca.lca(0, 5), 5);
                assert_eq!(lca.path_length(0, N - 1), N - 1);
                assert_eq!(lca.nth_ancestor(0, N / 2), Some(N / 2));
                assert_eq!(lca.nth_ancestor(0, N / 2 + 1), None);
                let d = g.dfs_order_with_root(0);
                assert!((0..N).all(|v| d.position[v] == v && d.end[v] == N));
                struct W(i64);
                impl MoOnTreeWorker for W {
                    type T = i64;
                    type R = i64;
                    fn empty() -> Self {
                        W(0)
                    }
                    fn add(&mut self, v: &i64) {
                        self.0 += v;
                    }
                    fn remove(&mut self, v: &i64) {
                        self.0 -= v;
                    }
                    fn result(&self) -> i64 {
                        self.0
                    }
                }
                let values: Vec<i64> = (0..N as i64).collect();
                let res = mo_on_tree::<W, _>(&g, &values, &[(3, 10), (N - 1, 0), (7, 7)]);
                assert_eq!(res, vec![(3..=10).sum::<i64>(), (0..N as i64).sum::<i64>(), 7]);
            }
        });
    }

    #[test]
    fn deep_euler_mst() {
        small_stack(|| {
            for mode in [0, 1] {
                let g: Graph<BiEdgeWithId<()>> = mk(
                    N,
                    mode,
                    line().into_iter().map(|(a, b)| BiEdgeWithId::new(a, b)).collect(),
                );
                let p = g.euler_path().unwrap();
                assert_eq!(p.len(), N);
                let g: Graph<BiWeightedEdge<i64, ()>> = mk(
                    N,
                    mode,
                    line().into_iter().map(|(a, b)| BiWeightedEdge::new(a, b, 1)).collect(),
                );
                assert_eq!(g.minimal_spanning_tree_weight(), N as i64 - 1);
                let g: Graph<WeightedEdge<i64, ()>> = mk(
                    N,
                    mode,
                    line().into_iter().map(|(a, b)| WeightedEdge::new(a, b, 1)).collect(),
                );
                assert_eq!(g.minimal_spanning_rooted_tree_weight(0), Some(N as i64 - 1));
                assert_eq!(g.minimal_spanning_rooted_tree_weight(1), None);
                // big cycle, root in the middle, plus reverse line: many contractions
                let mut e: Vec<(usize, usize, i64)> = (1..N).map(|v| (v, v - 1, 1)).collect();
                e.extend((1..N).map(|v| (v - 1, v, 2)));
                let g: Graph<WeightedEdge<i64, ()>> = mk(
                    N,
                    mode,
                    e.into_iter().map(|(a, b, w)| WeightedEdge::new(a, b, w)).collect(),
                );
                assert_eq!(
                    g.minimal_spanning_rooted_tree_weight(0),
                    Some(2 * (N as i64 - 1))
                );
                let t = g.minimal_spanning_rooted_tree(0).unwrap();
                assert!((0..N - 1).all(|v| t.adj(v).iter().map(|e| e.to()).collect::<Vec<_>>() == vec![v + 1]));
            }
        });
    }

    // Not claimed iterative: run with the default test-thread stack (2 MB). Run individually.
    #[test]
    #[ignore = "recursive: needs a big stack, run with RUST_MIN_STACK=67108864"]
    fn deep_hld_default_stack() {
        let g: Graph<BiEdge<()>> =
            mk(N, 0, line().into_iter().map(|(a, b)| BiEdge::new(a, b)).collect());
        let h = g.hl_decomposition();
        assert_eq!(h.paths.len(), 1);
    }

    #[test]
    #[ignore = "recursive: needs a big stack, run with RUST_MIN_STACK=67108864"]
    fn deep_centroid_default_stack() {
        let g: Graph<BiEdge<()>> =
            mk(N, 0, line().into_iter().map(|(a, b)| BiEdge::new(a, b)).collect());
        let mut cnt = 0;
        g.decompose(|_, _| cnt += 1);
        assert_eq!(cnt, N);
    }

    #[test]
    #[ignore = "recursive: needs a big stack, run with RUST_MIN_STACK=67108864"]
    fn deep_link_cut_default_stack() {
        use algo_lib::collections::link_cut::LinkCutNode;
        use algo_lib::collections::payload::PurePayload;
        let nodes: Vec<LinkCutNode<PurePayload<usize>>> =
            (0..N).map(|i| LinkCutNode::new(PurePayload(i))).collect();
        // child i+1 under parent i, built top-down
        for i in 1..N {
            nodes[i].link(nodes[i - 1]);
        }
        assert_eq!(nodes[N - 1].dist_to_root(), N - 1);
        assert!(nodes[N - 1].find_root() == nodes[0]);
        // build bottom-up: every link accesses a fresh single node parent
        let nodes: Vec<LinkCutNode<PurePayload<usize>>> =
            (0..N).map(|i| LinkCutNode::new(PurePayload(i))).collect();
        for i in (1..N).rev() {
            nodes[i].link(nodes[i - 1]);
        }
        assert_eq!(nodes[N - 1].dist_to_root(), N - 1);
        assert_eq!(nodes[0].dist_to_root(), 0);
        assert!(nodes[N / 2].find_root() == nodes[0]);
    }

    // simple O(VE) Chu-Liu/Edmonds, weight only
    fn edmonds_simple(n: usize, root: usize, edges: &[(usize, usize, i64)]) -> Option<i64> {
        let mut n = n;
        let mut root = root;
        let mut edges: Vec<(usize, usize, i64)> = edges.to_vec();
        let mut res = 0;
        loop {
            let mut inw = vec![i64::MAX; n];
            let mut pre = vec![usize::MAX; n];
            for &(a, b, w) in &edges {
                if a != b && w < inw[b] {
                    inw[b] = w;
                    pre[b] = a;
                }
            }
            for v in 0..n {
                if v != root && inw[v] == i64::MAX {
                    return None;
                }
            }
            let mut id = vec![usize::MAX; n];
            let mut vis = vec![usize::MAX; n];
            let mut cnt = 0;
            inw[root] = 0;
            for v in 0..n {
                res += inw[v];
                let mut u = v;
                while vis[u] != v && id[u] == usize::MAX && u != root {
                    vis[u] = v;
                    u = pre[u];
                }
                if u != root && id[u] == usize::MAX {
                    let mut x = pre[u];
                    while x != u {
                        id[x] = cnt;
                        x = pre[x];
                    }
                    id[u] = cnt;
                    cnt += 1;
                }
            }
            if cnt == 0 {
                return Some(res);
            }
            for v in 0..n {
                if id[v] == usize::MAX {
                    id[v] = cnt;
                    cnt += 1;
                }
            }
            edges = edges
                .iter()
                .filter(|&&(a, b, _)| id[a] != id[b])
                .map(|&(a, b, w)| (id[a], id[b], w - inw[b]))
                .collect();
            root = id[root];
            n = cnt;
        }
    }

    #[test]
    fn rooted_mst_medium_vs_simple() {
        let mut rng = Rng(9090);
        for it in 0..6000 {
            let n = 2 + rng.below(if it % 10 == 0 { 60 } else { 14 });
            let m = rng.below(if it % 2 == 0 { 4 * n } else { 2 * n }) + 1;
            let wr = [1, 3, 100, 1_000_000_000][it % 4];
            let mut edges: Vec<(usize, usize, i64)> = (0..m)
                .map(|_| (rng.below(n), rng.below(n), rng.range(-wr, wr)))
                .collect();
            if it % 3 == 0 {
                // cycle-rich: ring with cheap edges
                for v in 0..n {
                    edges.push((v, (v + 1) % n, rng.range(-1, 1)));
                }
            }
            let root = rng.below(n);
            let exp = edmonds_simple(n, root, &edges);
            let g: Graph<WeightedEdge<i64, ()>> = mk(
                n,
                it % 4,
                edges.iter().map(|&(a, b, w)| WeightedEdge::new(a, b, w)).collect(),
            );
            assert_eq!(g.minimal_spanning_rooted_tree_weight(root), exp, "n={n} root={root} {edges:?}");
            if let Some(t) = g.minimal_spanning_rooted_tree(root) {
                let mut indeg = vec![0; n];
                let mut total = 0;
                for v in 0..n {
                    for e in t.adj(v).iter() {
                        indeg[e.to()] += 1;
                        total += e.weight();
                        assert!(edges.contains(&(v, e.to(), e.weight())));
                    }
                }
                assert_eq!(Some(total), exp, "tree weight n={n} root={root} {edges:?}");
                assert!((0..n).all(|v| indeg[v] == if v == root { 0 } else { 1 }));
                let mut seen = vec![false; n];
                seen[root] = true;
                let mut st = vec![root];
                let mut c = 1;
                while let Some(v) = st.pop() {
                    for e in t.adj(v).iter() {
                        assert!(!seen[e.to()]);
                        seen[e.to()] = true;
                        c += 1;
                        st.push(e.to());
                    }
                }
                assert_eq!(c, n, "not spanning n={n} root={root} {edges:?}");
            } else {
                assert!(exp.is_none());
            }
        }
    }

    #[derive(Clone, Default)]
    struct Sum(i64);
    impl SegmentTreeNode for Sum {
        fn update(&mut self, l: &Self, r: &Self) {
            self.0 = l.0 + r.0;
        }
    }

    #[test]
    fn big_tree_cross_checks() {
        let mut rng = Rng(777777);
        for it in 0..40 {
            let n = 1500 + rng.below(1000);
            let root = rng.below(n);
            let t = NaiveTree::random(&mut rng, n, root, it % 5);
            let g = t.graph(it % 4);
            let lca = g.lca_with_root(root);
            let hld = g.hl_decomposition_with_root(root);
            let mut pst = g.path_segment_tree_with_gen_with_root(root, true, |v| Sum(v as i64));
            let mut pst_e = g.path_segment_tree_with_gen_with_root(root, false, |v| Sum(v as i64));
            for _ in 0..3000 {
                let a = rng.below(n);
                let b = rng.below(n);
                let l = t.lca(a, b);
                assert_eq!(lca.lca(a, b), l);
                assert_eq!(hld.lca(a, b), l);
                let p = t.path(a, b);
                assert_eq!(lca.path_length(a, b), p.len() - 1);
                let s: i64 = p.iter().map(|&v| v as i64).sum();
                assert_eq!(pst.query(a..=b).0, s);
                assert_eq!(pst_e.query(a..=b).0, s - l as i64);
                let k = rng.below(p.len());
                assert_eq!(lca.nth_vert_on_path(a, b, k), p[k]);
            }
        }
    }
}

// =====================================================================
// PART 5: minimal reproductions
// =====================================================================
mod part5 {
    use algo_lib::collections::euler_tour_tree::EulerTourForest;
    use algo_lib::collections::payload::PurePayload;
    use algo_lib::graph::edges::bi_edge::BiEdgeWithId;
    use algo_lib::graph::euler_path::EulerPath;
    use algo_lib::graph::Graph;

    // BUG 1: Euler cycle exists but vertex 0 is isolated -> None
    #[test]
    fn repro_euler_path_isolated_vertex_zero() {
        let mut g: Graph<BiEdgeWithId<()>> = Graph::new_linked(3);
        g.add_edge(BiEdgeWithId::new(1, 2));
        g.add_edge(BiEdgeWithId::new(2, 1));
        // 1 -> 2 -> 1 is an Euler cycle; vertex 0 is isolated
        let path = g.euler_path();
        assert!(path.is_some(), "euler cycle 1-2-1 exists, got None");
    }

    // control: same graph with the isolated vertex placed last works
    #[test]
    fn control_euler_path_isolated_vertex_last() {
        let mut g: Graph<BiEdgeWithId<()>> = Graph::new_linked(3);
        g.add_edge(BiEdgeWithId::new(0, 1));
        g.add_edge(BiEdgeWithId::new(1, 0));
        assert_eq!(g.euler_path().map(|p| p.len()), Some(3));
    }

    #[test]
    fn repro_euler_path_empty_graph() {
        let g: Graph<BiEdgeWithId<()>> = Graph::new_linked(0);
        let res = std::panic::catch_unwind(|| g.euler_path());
        assert!(res.is_ok(), "euler_path panics on a graph with 0 vertices");
    }

    // how reliably does add_edge detect an edge inside one component?
    #[test]
    fn ett_loop_detection_rate() {
        let mut detected = 0;
        let mut total = 0;
        for n in 3..40usize {
            for a in 0..n {
                let b = (a + n / 2) % n;
                if a == b {
                    continue;
                }
                total += 1;
                let r = std::panic::catch_unwind(|| {
                    let mut f: EulerTourForest<PurePayload<usize>> = EulerTourForest::new();
                    for v in 0..n {
                        f.add_node(PurePayload(v));
                    }
                    for v in 1..n {
                        f.add_edge(v - 1, v, PurePayload(0), PurePayload(0));
                    }
                    f.add_edge(a, b, PurePayload(0), PurePayload(0));
                });
                if r.is_err() {
                    detected += 1;
                }
            }
        }
        eprintln!("ETT loop detection: {detected}/{total}");
        assert_eq!(detected, total, "add_edge inside one component not detected: {detected}/{total}");
    }

    // SUSPICIOUS: (Excluded(root), ..) should be an empty path, yields the root
    #[test]
    fn hld_iter_excluded_root() {
        use algo_lib::graph::hl_decomposition::HLDecompositionTrait;
        use std::ops::Bound;
        let g = Graph::with_biedges(3, &[(0, 1), (1, 2)]);
        let h = g.hl_decomposition();
        let parts: Vec<(usize, usize, usize)> = h
            .iter((Bound::Excluded(0), Bound::Included(0)))
            .map(|p| (p.id, p.pos_from, p.pos_to))
            .collect();
        assert!(parts.is_empty(), "expected empty, got {parts:?}");
    }
}

// =====================================================================
// PART 6: verification of the suggested euler_path fix on a copy
// =====================================================================
mod part6 {
    use super::{mk, rand_edges, Rng};
    use algo_lib::collections::bit_set::BitSet;
    use algo_lib::collections::dsu::DSU;
    use algo_lib::graph::edges::bi_edge::BiEdgeWithId;
    use algo_lib::graph::edges::edge_trait::EdgeTrait;
    use algo_lib::graph::Graph;

    // copy of graph/euler_path.rs with the start vertex fixed
    fn euler_path_fixed(g: &Graph<BiEdgeWithId<()>>) -> Option<Vec<usize>> {
        if g.vertex_count() == 0 {
            return None;
        }
        let mut start = usize::MAX;
        let mut odd_count = 0;
        for i in 0..g.vertex_count() {
            if g.adj(i).len() % 2 == 1 {
                odd_count += 1;
                start = i;
            }
        }
        if odd_count > 2 {
            return None;
        }
        if start == usize::MAX {
            // FIX: no odd vertex - start from any vertex that has edges
            start = (0..g.vertex_count()).find(|&v| g.degree(v) > 0).unwrap_or(0);
        }
        let mut removed = BitSet::new(g.edge_count());
        let mut id: Vec<u32> = (0..g.vertex_count()).map(|v| g.head_edge(v)).collect();
        let mut st = vec![start];
        let mut ans = Vec::with_capacity(g.edge_count() + 1);
        while let Some(&v) = st.last() {
            while id[v] != u32::MAX && removed[g.edge_at(v, id[v]).id()] {
                id[v] = g.step_edge(v, id[v]);
            }
            if id[v] == u32::MAX {
                st.pop();
                ans.push(v);
            } else {
                let edge = g.edge_at(v, id[v]);
                removed.set(edge.id());
                st.push(edge.to());
            }
        }
        if ans.len() == g.edge_count() + 1 {
            Some(ans)
        } else {
            None
        }
    }

    #[test]
    fn euler_path_fixed_copy_vs_brute() {
        let mut rng = Rng(31338);
        for it in 0..20000 {
            let n = 1 + rng.below(6);
            let m = rng.below(9);
            let edges = rand_edges(&mut rng, n, m, true);
            let g: Graph<BiEdgeWithId<()>> = mk(
                n,
                it % 4,
                edges.iter().map(|&(a, b)| BiEdgeWithId::new(a, b)).collect(),
            );
            let mut deg = vec![0; n];
            let mut dsu = DSU::new(n);
            for &(a, b) in &edges {
                deg[a] += 1;
                deg[b] += 1;
                dsu.union(a, b);
            }
            let odd = deg.iter().filter(|&&d| d % 2 == 1).count();
            let mut roots: Vec<usize> = (0..n).filter(|&v| deg[v] > 0).map(|v| dsu.find(v)).collect();
            roots.sort();
            roots.dedup();
            let exists = odd <= 2 && roots.len() <= 1;
            let got = euler_path_fixed(&g);
            assert_eq!(got.is_some(), exists, "n={n} {edges:?}");
            if let Some(p) = got {
                let mut a: Vec<(usize, usize)> =
                    p.windows(2).map(|w| (w[0].min(w[1]), w[0].max(w[1]))).collect();
                let mut b: Vec<(usize, usize)> =
                    edges.iter().map(|&(x, y)| (x.min(y), x.max(y))).collect();
                a.sort();
                b.sort();
                assert_eq!(a, b);
            }
        }
    }
}
