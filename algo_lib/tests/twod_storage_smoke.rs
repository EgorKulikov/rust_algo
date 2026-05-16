// Smoke test: construct a small graph with Graph::new_2d and run a few
// algorithms to confirm TwoD storage works end-to-end.
use algo_lib::graph::bridges::BridgeSearch;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait; // for .to() on adj edges
use algo_lib::graph::strongly_connected_components::StronglyConnectedComponentsTrait;
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
    assert_eq!(scc.color[0], scc.color[1]);
    assert_eq!(scc.color[1], scc.color[2]);
    assert_eq!(scc.condensed.vertex_count(), 1);
}

#[test]
fn twod_bridges() {
    // Path graph: every edge is a bridge.
    let mut g: Graph<BiEdge<()>> = Graph::new_2d(4);
    g.add_edge(BiEdge::new(0, 1));
    g.add_edge(BiEdge::new(1, 2));
    g.add_edge(BiEdge::new(2, 3));
    let mut bridges = g.bridges();
    for (u, v) in bridges.iter_mut() {
        if *u > *v {
            std::mem::swap(u, v);
        }
    }
    bridges.sort();
    let mut expected = vec![(0usize, 1usize), (1, 2), (2, 3)];
    expected.sort();
    assert_eq!(bridges, expected);
}

#[test]
fn twod_adj_basic() {
    // Sanity: add edges, walk adjacency, verify counts/targets.
    let mut g: Graph<Edge<()>> = Graph::new_2d(3);
    g.add_edge(Edge::new(0, 1));
    g.add_edge(Edge::new(0, 2));
    g.add_edge(Edge::new(1, 2));
    assert_eq!(g.vertex_count(), 3);
    assert_eq!(g.edge_count(), 3);
    let mut adj0: Vec<usize> = g.adj(0).iter().map(|e| e.to()).collect();
    adj0.sort();
    assert_eq!(adj0, vec![1, 2]);
    assert_eq!(g.adj(0).len(), 2);
    assert_eq!(g.adj(2).len(), 0);
    let degrees = g.degrees();
    assert_eq!(degrees, vec![2, 1, 0]);
}
