use crate::collections::dsu::DSU;
use crate::graph::edges::bi_edge::BiEdge;
use crate::graph::edges::edge::Edge;
use crate::graph::edges::edge_trait::{BidirectionalEdgeTrait, EdgeTrait};
use std::ops::{Index, IndexMut};

pub mod all_distances;
pub mod bridges;
pub mod central_decomposition;
pub mod cut_points;
pub mod dfs_order;
pub mod distances;
pub mod edge_distances;
pub mod edges;
pub mod euler_path;
pub mod fast_max_flow;
pub mod flow_graph;
pub mod flow_with_demand;
pub mod hl_decomposition;
pub mod lca;
pub mod max_flow;
pub mod min_cost_flow;
pub mod minimal_spanning_tree;
pub mod negative_distances;
pub mod strongly_connected_components;
pub mod topological_sort;
pub mod two_sat;

#[derive(Clone)]
pub struct Graph<E: EdgeTrait> {
    edges: Vec<Vec<E>>,
    edge_count: usize,
}

impl<E: EdgeTrait> Graph<E> {
    pub fn new(vertex_count: usize) -> Self {
        Self {
            edges: vec![Vec::new(); vertex_count],
            edge_count: 0,
        }
    }

    pub fn add_edge(&mut self, (from, mut edge): (usize, E)) -> usize {
        let to = edge.to();
        assert!(to < self.vertex_count());
        let direct_id = self.edges[from].len();
        edge.set_id(self.edge_count);
        self.edges[from].push(edge);
        if E::REVERSABLE {
            let rev_id = self.edges[to].len();
            self.edges[from][direct_id].set_reverse_id(rev_id);
            let mut rev_edge = self.edges[from][direct_id].reverse_edge(from);
            rev_edge.set_id(self.edge_count);
            rev_edge.set_reverse_id(direct_id);
            self.edges[to].push(rev_edge);
        }
        self.edge_count += 1;
        direct_id
    }

    pub fn add_vertices(&mut self, cnt: usize) {
        self.edges.resize(self.edges.len() + cnt, Vec::new());
    }

    pub fn clear(&mut self) {
        self.edge_count = 0;
        for ve in self.edges.iter_mut() {
            ve.clear();
        }
    }

    pub fn vertex_count(&self) -> usize {
        self.edges.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edge_count
    }

    pub fn degrees(&self) -> Vec<usize> {
        self.edges.iter().map(|v| v.len()).collect()
    }
}

impl<E: BidirectionalEdgeTrait> Graph<E> {
    pub fn is_tree(&self) -> bool {
        if self.edge_count + 1 != self.vertex_count() {
            false
        } else {
            self.is_connected()
        }
    }

    pub fn is_forest(&self) -> bool {
        let mut dsu = DSU::new(self.vertex_count());
        for i in 0..self.vertex_count() {
            for e in self[i].iter() {
                if i <= e.to() && !dsu.union(i, e.to()) {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_connected(&self) -> bool {
        let mut dsu = DSU::new(self.vertex_count());
        for i in 0..self.vertex_count() {
            for e in self[i].iter() {
                dsu.union(i, e.to());
            }
        }
        dsu.set_count() == 1
    }
}

impl<E: EdgeTrait> Index<usize> for Graph<E> {
    type Output = [E];

    fn index(&self, index: usize) -> &Self::Output {
        &self.edges[index]
    }
}

impl<E: EdgeTrait> IndexMut<usize> for Graph<E> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.edges[index]
    }
}

impl Graph<Edge<()>> {
    pub fn with_edges(n: usize, edges: &[(usize, usize)]) -> Self {
        let mut graph = Self::new(n);
        for &(from, to) in edges {
            graph.add_edge(Edge::new(from, to));
        }
        graph
    }
}

impl<P: Clone> Graph<Edge<P>> {
    pub fn with_edges_with_payload(n: usize, edges: &[(usize, usize, P)]) -> Self {
        let mut graph = Self::new(n);
        for (from, to, p) in edges.iter() {
            graph.add_edge(Edge::with_payload(*from, *to, p.clone()));
        }
        graph
    }
}

impl Graph<BiEdge<()>> {
    pub fn with_biedges(n: usize, edges: &[(usize, usize)]) -> Self {
        let mut graph = Self::new(n);
        for &(from, to) in edges {
            graph.add_edge(BiEdge::new(from, to));
        }
        graph
    }
}

impl<P: Clone> Graph<BiEdge<P>> {
    pub fn with_biedges_with_payload(n: usize, edges: &[(usize, usize, P)]) -> Self {
        let mut graph = Self::new(n);
        for (from, to, p) in edges.iter() {
            graph.add_edge(BiEdge::with_payload(*from, *to, p.clone()));
        }
        graph
    }
}
