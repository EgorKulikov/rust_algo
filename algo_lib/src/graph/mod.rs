use crate::collections::dsu::DSU;
use crate::graph::edges::bi_edge::BiEdge;
use crate::graph::edges::edge::Edge;
use crate::graph::edges::edge_trait::{BidirectionalEdgeTrait, EdgeTrait};
use std::marker::PhantomData;

pub mod all_distances;
pub mod block_cut_tree;
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
pub mod min_cost_flow_slow;
pub mod minimal_spanning_tree;
pub mod negative_distances;
pub mod strongly_connected_components;
pub mod topological_sort;
pub mod two_sat;

/// Linked-list graph storage.
///
/// Each vertex `v` owns a singly-linked list of edges, threaded through the
/// global `next` array (indexed by edge id). `first[v]` is the head, `last[v]`
/// is the tail (for O(1) FIFO append), `u32::MAX` denotes "no edge".
///
/// `edge_count` counts logical (user-visible) edges; for an undirected edge
/// type (`E::REVERSABLE == true`), `add_edge` stores two entries in `edges`
/// but only increments `edge_count` by 1.
#[derive(Clone)]
pub struct Graph<E: EdgeTrait> {
    first: Vec<u32>,
    last: Vec<u32>,
    next: Vec<u32>,
    edges: Vec<E>,
    degree: Vec<u32>,
    edge_count: usize,
}

impl<E: EdgeTrait> Graph<E> {
    pub fn new(vertex_count: usize) -> Self {
        Self {
            first: vec![u32::MAX; vertex_count],
            last: vec![u32::MAX; vertex_count],
            next: Vec::new(),
            edges: Vec::new(),
            degree: vec![0u32; vertex_count],
            edge_count: 0,
        }
    }

    fn push_one(&mut self, from: usize, mut edge: E) -> usize {
        let id = self.edges.len();
        edge.set_id(self.edge_count);
        self.edges.push(edge);
        self.next.push(u32::MAX);
        let last = self.last[from];
        if last == u32::MAX {
            self.first[from] = id as u32;
        } else {
            self.next[last as usize] = id as u32;
        }
        self.last[from] = id as u32;
        self.degree[from] += 1;
        id
    }

    pub fn add_edge(&mut self, (from, edge): (usize, E)) -> usize {
        let to = edge.to();
        assert!(to < self.vertex_count());
        let direct_id = self.push_one(from, edge);
        if E::REVERSABLE {
            let mut rev_edge = self.edges[direct_id].reverse_edge(from);
            let rev_id = self.edges.len();
            // Cross-link forward <-> reverse via global edge ids.
            self.edges[direct_id].set_reverse_id(rev_id);
            rev_edge.set_reverse_id(direct_id);
            self.push_one(to, rev_edge);
        }
        self.edge_count += 1;
        direct_id
    }

    pub fn add_vertices(&mut self, cnt: usize) {
        self.first.resize(self.first.len() + cnt, u32::MAX);
        self.last.resize(self.last.len() + cnt, u32::MAX);
        self.degree.resize(self.degree.len() + cnt, 0);
    }

    pub fn clear(&mut self) {
        self.edge_count = 0;
        self.edges.clear();
        self.next.clear();
        for f in self.first.iter_mut() {
            *f = u32::MAX;
        }
        for l in self.last.iter_mut() {
            *l = u32::MAX;
        }
        for d in self.degree.iter_mut() {
            *d = 0;
        }
    }

    pub fn vertex_count(&self) -> usize {
        self.first.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edge_count
    }

    pub fn degrees(&self) -> Vec<usize> {
        self.degree.iter().map(|&d| d as usize).collect()
    }

    /// Iterator over `(from, &edge)` pairs covering every stored edge entry
    /// (i.e. both directions of an undirected edge are yielded).
    pub fn edges(&self) -> impl Iterator<Item = (usize, &E)> + '_ {
        (0..self.vertex_count()).flat_map(move |v| self.adj(v).iter().map(move |e| (v, e)))
    }

    /// Adjacency view for vertex `v`.
    pub fn adj(&self, v: usize) -> AdjView<'_, E> {
        AdjView {
            graph: self,
            head: self.first[v],
            len: self.degree[v],
        }
    }

    /// Mutable adjacency view for vertex `v`.
    pub fn adj_mut(&mut self, v: usize) -> AdjViewMut<'_, E> {
        let head = self.first[v];
        let len = self.degree[v];
        AdjViewMut {
            graph: self,
            head,
            len,
        }
    }

    /// Returns a reference to the edge stored at the given global edge id.
    pub fn edge(&self, id: usize) -> &E {
        &self.edges[id]
    }

    /// Returns a mutable reference to the edge stored at the given global edge id.
    pub fn edge_mut(&mut self, id: usize) -> &mut E {
        &mut self.edges[id]
    }

    /// Head edge id for vertex `v`'s adjacency list, or `u32::MAX` if empty.
    pub fn head_edge(&self, v: usize) -> u32 {
        self.first[v]
    }

    /// Next edge id in the adjacency list, after `id`. Returns `u32::MAX` at end.
    pub fn next_edge(&self, id: usize) -> u32 {
        self.next[id]
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
            for e in self.adj(i).iter() {
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
            for e in self.adj(i).iter() {
                dsu.union(i, e.to());
            }
        }
        dsu.set_count() == 1
    }
}

/// Immutable view over a vertex's adjacency list.
pub struct AdjView<'a, E: EdgeTrait> {
    graph: &'a Graph<E>,
    head: u32,
    len: u32,
}

impl<'a, E: EdgeTrait> AdjView<'a, E> {
    pub fn iter(&self) -> AdjIter<'a, E> {
        AdjIter {
            graph: self.graph,
            cur: self.head,
        }
    }

    /// Iterator yielding `(edge_id, &edge)`.
    pub fn iter_with_id(&self) -> AdjIterWithId<'a, E> {
        AdjIterWithId {
            graph: self.graph,
            cur: self.head,
        }
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// O(i) walk; only intended for code that previously did `g[v][i]`.
    pub fn get(&self, i: usize) -> Option<&'a E> {
        self.iter().nth(i)
    }

    pub fn head_id(&self) -> u32 {
        self.head
    }
}

impl<'a, E: EdgeTrait> IntoIterator for &AdjView<'a, E> {
    type Item = &'a E;
    type IntoIter = AdjIter<'a, E>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, E: EdgeTrait> IntoIterator for AdjView<'a, E> {
    type Item = &'a E;
    type IntoIter = AdjIter<'a, E>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct AdjIter<'a, E: EdgeTrait> {
    graph: &'a Graph<E>,
    cur: u32,
}

impl<'a, E: EdgeTrait> Iterator for AdjIter<'a, E> {
    type Item = &'a E;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur == u32::MAX {
            return None;
        }
        let id = self.cur as usize;
        let edge = &self.graph.edges[id];
        self.cur = self.graph.next[id];
        Some(edge)
    }
}

pub struct AdjIterWithId<'a, E: EdgeTrait> {
    graph: &'a Graph<E>,
    cur: u32,
}

impl<'a, E: EdgeTrait> Iterator for AdjIterWithId<'a, E> {
    type Item = (usize, &'a E);
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur == u32::MAX {
            return None;
        }
        let id = self.cur as usize;
        let edge = &self.graph.edges[id];
        self.cur = self.graph.next[id];
        Some((id, edge))
    }
}

/// Mutable adjacency view over a vertex's edges.
pub struct AdjViewMut<'a, E: EdgeTrait> {
    graph: &'a mut Graph<E>,
    head: u32,
    #[allow(dead_code)]
    len: u32,
}

impl<'a, E: EdgeTrait> AdjViewMut<'a, E> {
    pub fn iter_mut(&mut self) -> AdjIterMut<'_, E> {
        AdjIterMut {
            graph: self.graph,
            cur: self.head,
            _marker: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

pub struct AdjIterMut<'a, E: EdgeTrait> {
    graph: *mut Graph<E>,
    cur: u32,
    _marker: PhantomData<&'a mut E>,
}

impl<'a, E: EdgeTrait> Iterator for AdjIterMut<'a, E> {
    type Item = &'a mut E;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur == u32::MAX {
            return None;
        }
        let id = self.cur as usize;
        // SAFETY: we hold an exclusive borrow of the graph for `'a`, and each
        // call advances `cur` along the linked list — every edge index is
        // visited at most once, so the yielded `&mut E` references do not
        // alias.
        unsafe {
            let edges_ptr = (*self.graph).edges.as_mut_ptr();
            let next_ptr = (*self.graph).next.as_ptr();
            self.cur = *next_ptr.add(id);
            Some(&mut *edges_ptr.add(id))
        }
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

pub struct CostAndFlow<C> {
    pub cost: C,
    pub flow: C,
}
