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
pub mod path_segment_tree;
pub mod strongly_connected_components;
pub mod topological_sort;
pub mod two_sat;

/// Storage layout for `Graph<E>`.
///
/// `Linked` is the classical sparse-graph representation: a singly-linked list
/// per vertex, threaded through the global `next` array (indexed by edge id).
/// `first[v]` is the head; iteration is LIFO (most-recently-added first).
/// `u32::MAX` denotes "no edge". Logical (user-visible) edge count is derived
/// from `edges.len()` -- divided by 2 for undirected edge types (which store
/// two entries per logical edge).
///
/// `TwoD` is the dense `Vec<Vec<E>>` representation: each vertex owns its
/// adjacency directly as a contiguous slice. The cursor used by
/// `edge_at(v, cursor)` is a position within `edges[v]`. `edge_count` tracks
/// the logical edge count (not divided -- each `add_edge` increments by one
/// regardless of `REVERSABLE`).
#[derive(Clone)]
enum Storage<E: EdgeTrait> {
    Linked {
        first: Vec<u32>,
        next: Vec<u32>,
        edges: Vec<E>,
        degree: Vec<u32>,
    },
    TwoD {
        edges: Vec<Vec<E>>,
        edge_count: usize,
    },
}

#[derive(Clone)]
pub struct Graph<E: EdgeTrait> {
    storage: Storage<E>,
}

impl<E: EdgeTrait> Graph<E> {
    /// Construct a sparse linked-list-backed graph with `vertex_count` vertices.
    pub fn new_linked(vertex_count: usize) -> Self {
        Self {
            storage: Storage::Linked {
                first: vec![u32::MAX; vertex_count],
                next: Vec::new(),
                edges: Vec::new(),
                degree: vec![0u32; vertex_count],
            },
        }
    }

    /// Construct a dense `Vec<Vec<E>>`-backed graph with `vertex_count` vertices.
    pub fn new_2d(vertex_count: usize) -> Self {
        Self {
            storage: Storage::TwoD {
                edges: (0..vertex_count).map(|_| Vec::new()).collect(),
                edge_count: 0,
            },
        }
    }

    /// Auto-selecting constructor. `expected_edge_count` is a hint: if
    /// `m >= K * n` the dense `TwoD` storage is chosen, else the sparse
    /// `Linked` storage. `K` is tuned empirically (see Task 7's bench sweep).
    pub fn new(vertex_count: usize, expected_edge_count: usize) -> Self {
        const K: usize = 4; // tuned empirically; see storage_sweep benches.
        if expected_edge_count >= K.saturating_mul(vertex_count) {
            Self::new_2d(vertex_count)
        } else {
            Self::new_linked(vertex_count)
        }
    }

    /// Helper used by `add_edge`'s Linked arm. Pushes a single edge entry,
    /// returning the global edge id of the new entry.
    fn push_one_linked(&mut self, from: usize, mut edge: E) -> usize {
        let logical_count = self.edge_count();
        match &mut self.storage {
            Storage::Linked {
                first,
                next,
                edges,
                degree,
            } => {
                let id = edges.len();
                edge.set_id(logical_count);
                edges.push(edge);
                // LIFO prepend: new edge becomes head, points at the previous head.
                next.push(first[from]);
                first[from] = id as u32;
                degree[from] += 1;
                id
            }
            Storage::TwoD { .. } => unreachable!("push_one_linked called on TwoD storage"),
        }
    }

    pub fn add_edge(&mut self, (from, edge): (usize, E)) -> usize {
        let to = edge.to();
        assert!(to < self.vertex_count());
        match &self.storage {
            Storage::Linked { .. } => {
                let direct_id = self.push_one_linked(from, edge);
                if E::REVERSABLE {
                    let mut rev_edge = match &mut self.storage {
                        Storage::Linked { edges, .. } => {
                            let rev_id = edges.len();
                            let rev_edge = edges[direct_id].reverse_edge(from);
                            // Cross-link forward <-> reverse via global edge ids.
                            edges[direct_id].set_reverse_id(rev_id);
                            rev_edge
                        }
                        Storage::TwoD { .. } => unreachable!(),
                    };
                    rev_edge.set_reverse_id(direct_id);
                    self.push_one_linked(to, rev_edge);
                }
                direct_id
            }
            Storage::TwoD { .. } => match &mut self.storage {
                Storage::TwoD { edges, edge_count } => {
                    let mut edge = edge;
                    let direct_pos = edges[from].len();
                    let logical_id = *edge_count;
                    edge.set_id(logical_id);
                    edges[from].push(edge);
                    if E::REVERSABLE {
                        // `rev_pos` is computed AFTER the direct push so a
                        // self-loop (`from == to`) accounts for the direct
                        // entry already in `edges[to]`.
                        let rev_pos = edges[to].len();
                        let mut rev_edge = edges[from][direct_pos].reverse_edge(from);
                        rev_edge.set_id(logical_id);
                        edges[from][direct_pos].set_reverse_id(rev_pos);
                        rev_edge.set_reverse_id(direct_pos);
                        edges[to].push(rev_edge);
                    }
                    *edge_count += 1;
                    direct_pos
                }
                Storage::Linked { .. } => unreachable!(),
            },
        }
    }

    pub fn add_vertices(&mut self, cnt: usize) {
        match &mut self.storage {
            Storage::Linked { first, degree, .. } => {
                let n = first.len();
                first.resize(n + cnt, u32::MAX);
                degree.resize(degree.len() + cnt, 0);
            }
            Storage::TwoD { edges, .. } => {
                for _ in 0..cnt {
                    edges.push(Vec::new());
                }
            }
        }
    }

    pub fn clear(&mut self) {
        match &mut self.storage {
            Storage::Linked {
                first,
                next,
                edges,
                degree,
            } => {
                edges.clear();
                next.clear();
                for f in first.iter_mut() {
                    *f = u32::MAX;
                }
                for d in degree.iter_mut() {
                    *d = 0;
                }
            }
            Storage::TwoD { edges, edge_count } => {
                for v in edges.iter_mut() {
                    v.clear();
                }
                *edge_count = 0;
            }
        }
    }

    pub fn vertex_count(&self) -> usize {
        match &self.storage {
            Storage::Linked { first, .. } => first.len(),
            Storage::TwoD { edges, .. } => edges.len(),
        }
    }

    pub fn edge_count(&self) -> usize {
        match &self.storage {
            Storage::Linked { edges, .. } => {
                if E::REVERSABLE {
                    edges.len() / 2
                } else {
                    edges.len()
                }
            }
            Storage::TwoD { edge_count, .. } => *edge_count,
        }
    }

    pub fn degree(&self, v: usize) -> usize {
        match &self.storage {
            Storage::Linked { degree, .. } => degree[v] as usize,
            Storage::TwoD { edges, .. } => edges[v].len(),
        }
    }

    pub fn degrees(&self) -> Vec<usize> {
        match &self.storage {
            Storage::Linked { degree, .. } => degree.iter().map(|&d| d as usize).collect(),
            Storage::TwoD { edges, .. } => edges.iter().map(|v| v.len()).collect(),
        }
    }

    /// Iterator over `(from, &edge)` pairs covering every stored edge entry
    /// (i.e. both directions of an undirected edge are yielded).
    pub fn edges(&self) -> impl Iterator<Item = (usize, &E)> + '_ {
        (0..self.vertex_count()).flat_map(move |v| self.adj(v).iter().map(move |e| (v, e)))
    }

    /// Adjacency view for vertex `v`.
    pub fn adj(&self, v: usize) -> AdjView<'_, E> {
        match &self.storage {
            Storage::Linked { first, degree, .. } => AdjView {
                inner: AdjViewInner::Linked {
                    storage: &self.storage,
                    head: first[v],
                    len: degree[v],
                },
            },
            Storage::TwoD { edges, .. } => AdjView {
                inner: AdjViewInner::Slice(&edges[v]),
            },
        }
    }

    /// Mutable adjacency view for vertex `v`.
    pub fn adj_mut(&mut self, v: usize) -> AdjViewMut<'_, E> {
        // For Linked we need both a Copy `head` AND `self as *mut _`. Borrow
        // shapes prevent doing this inside a single `match &mut self.storage`
        // (the &mut borrow of `self.storage` blocks re-borrowing self as a
        // raw pointer), so peek the discriminant first and branch.
        if matches!(self.storage, Storage::Linked { .. }) {
            let head = self.head_edge(v);
            AdjViewMut {
                inner: AdjViewMutInner::Linked {
                    graph: self as *mut _,
                    head,
                },
            }
        } else {
            match &mut self.storage {
                Storage::TwoD { edges, .. } => AdjViewMut {
                    inner: AdjViewMutInner::Slice(&mut edges[v]),
                },
                Storage::Linked { .. } => unreachable!(),
            }
        }
    }

    /// Head edge id for vertex `v`'s adjacency list, or `u32::MAX` if empty.
    pub fn head_edge(&self, v: usize) -> u32 {
        match &self.storage {
            Storage::Linked { first, .. } => first[v],
            Storage::TwoD { edges, .. } => {
                if edges[v].is_empty() {
                    u32::MAX
                } else {
                    0
                }
            }
        }
    }

    /// Edge access by (vertex, cursor). For Linked storage the cursor is a
    /// global edge id and the vertex is ignored. For TwoD storage the cursor
    /// is a position within `edges[v]`.
    pub fn edge_at(&self, _v: usize, cursor: u32) -> &E {
        match &self.storage {
            Storage::Linked { edges, .. } => &edges[cursor as usize],
            Storage::TwoD { edges, .. } => &edges[_v][cursor as usize],
        }
    }

    /// Mutable counterpart of `edge_at`.
    pub fn edge_at_mut(&mut self, _v: usize, cursor: u32) -> &mut E {
        match &mut self.storage {
            Storage::Linked { edges, .. } => &mut edges[cursor as usize],
            Storage::TwoD { edges, .. } => &mut edges[_v][cursor as usize],
        }
    }

    /// Advance a cursor in `v`'s adjacency list. Returns the next cursor or
    /// `u32::MAX` at the end.
    pub fn step_edge(&self, _v: usize, cursor: u32) -> u32 {
        match &self.storage {
            Storage::Linked { next, .. } => next[cursor as usize],
            Storage::TwoD { edges, .. } => {
                let nxt = cursor + 1;
                if (nxt as usize) < edges[_v].len() {
                    nxt
                } else {
                    u32::MAX
                }
            }
        }
    }
}

impl<E: BidirectionalEdgeTrait> Graph<E> {
    pub fn is_tree(&self) -> bool {
        if self.edge_count() + 1 != self.vertex_count() {
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
    inner: AdjViewInner<'a, E>,
}

enum AdjViewInner<'a, E: EdgeTrait> {
    Linked {
        storage: &'a Storage<E>,
        head: u32,
        len: u32,
    },
    Slice(&'a [E]),
}

impl<'a, E: EdgeTrait> AdjView<'a, E> {
    pub fn iter(&self) -> AdjIter<'a, E> {
        let inner = match &self.inner {
            AdjViewInner::Linked { storage, head, .. } => AdjIterInner::Linked {
                storage: *storage,
                cur: *head,
            },
            AdjViewInner::Slice(s) => AdjIterInner::Slice(s.iter()),
        };
        AdjIter { inner }
    }

    /// Iterator yielding `(cursor, &edge)`.
    pub fn iter_with_id(&self) -> AdjIterWithId<'a, E> {
        let inner = match &self.inner {
            AdjViewInner::Linked { storage, head, .. } => AdjIterWithIdInner::Linked {
                storage: *storage,
                cur: *head,
            },
            AdjViewInner::Slice(s) => AdjIterWithIdInner::Slice {
                iter: s.iter().enumerate(),
            },
        };
        AdjIterWithId { inner }
    }

    pub fn len(&self) -> usize {
        match &self.inner {
            AdjViewInner::Linked { len, .. } => *len as usize,
            AdjViewInner::Slice(s) => s.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// O(i) walk for Linked; O(1) for TwoD.
    pub fn get(&self, i: usize) -> Option<&'a E> {
        match &self.inner {
            AdjViewInner::Linked { .. } => self.iter().nth(i),
            AdjViewInner::Slice(s) => s.get(i),
        }
    }

    pub fn head_id(&self) -> u32 {
        match &self.inner {
            AdjViewInner::Linked { head, .. } => *head,
            AdjViewInner::Slice(s) => {
                if s.is_empty() {
                    u32::MAX
                } else {
                    0
                }
            }
        }
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
    inner: AdjIterInner<'a, E>,
}

enum AdjIterInner<'a, E: EdgeTrait> {
    Linked { storage: &'a Storage<E>, cur: u32 },
    Slice(std::slice::Iter<'a, E>),
}

impl<'a, E: EdgeTrait> Iterator for AdjIter<'a, E> {
    type Item = &'a E;
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.inner {
            AdjIterInner::Linked { storage, cur } => {
                if *cur == u32::MAX {
                    return None;
                }
                match storage {
                    Storage::Linked { edges, next, .. } => {
                        let id = *cur as usize;
                        let e = &edges[id];
                        *cur = next[id];
                        Some(e)
                    }
                    Storage::TwoD { .. } => {
                        unreachable!("AdjIter::Linked never holds TwoD storage")
                    }
                }
            }
            AdjIterInner::Slice(it) => it.next(),
        }
    }
}

pub struct AdjIterWithId<'a, E: EdgeTrait> {
    inner: AdjIterWithIdInner<'a, E>,
}

enum AdjIterWithIdInner<'a, E: EdgeTrait> {
    Linked {
        storage: &'a Storage<E>,
        cur: u32,
    },
    Slice {
        iter: std::iter::Enumerate<std::slice::Iter<'a, E>>,
    },
}

impl<'a, E: EdgeTrait> Iterator for AdjIterWithId<'a, E> {
    type Item = (usize, &'a E);
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.inner {
            AdjIterWithIdInner::Linked { storage, cur } => {
                if *cur == u32::MAX {
                    return None;
                }
                match storage {
                    Storage::Linked { edges, next, .. } => {
                        let id = *cur as usize;
                        let e = &edges[id];
                        *cur = next[id];
                        Some((id, e))
                    }
                    Storage::TwoD { .. } => {
                        unreachable!("AdjIterWithId::Linked never holds TwoD storage")
                    }
                }
            }
            AdjIterWithIdInner::Slice { iter } => iter.next(),
        }
    }
}

/// Mutable adjacency view over a vertex's edges. Only `iter_mut` is offered;
/// for `.len()` / `.is_empty()` use the immutable `adj(v)` view instead.
pub struct AdjViewMut<'a, E: EdgeTrait> {
    inner: AdjViewMutInner<'a, E>,
}

enum AdjViewMutInner<'a, E: EdgeTrait> {
    Linked { graph: *mut Graph<E>, head: u32 },
    Slice(&'a mut [E]),
}

// PhantomData isn't strictly necessary here because `Slice` carries the
// lifetime, but the Linked variant needs to tie its raw pointer to `'a`.
impl<'a, E: EdgeTrait> AdjViewMut<'a, E> {
    pub fn iter_mut(&mut self) -> AdjIterMut<'_, E> {
        let inner = match &mut self.inner {
            AdjViewMutInner::Linked { graph, head } => AdjIterMutInner::Linked {
                graph: *graph,
                cur: *head,
                _marker: PhantomData,
            },
            AdjViewMutInner::Slice(s) => AdjIterMutInner::Slice(s.iter_mut()),
        };
        AdjIterMut { inner }
    }
}

pub struct AdjIterMut<'a, E: EdgeTrait> {
    inner: AdjIterMutInner<'a, E>,
}

enum AdjIterMutInner<'a, E: EdgeTrait> {
    Linked {
        graph: *mut Graph<E>,
        cur: u32,
        _marker: PhantomData<&'a mut E>,
    },
    Slice(std::slice::IterMut<'a, E>),
}

impl<'a, E: EdgeTrait> Iterator for AdjIterMut<'a, E> {
    type Item = &'a mut E;
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.inner {
            AdjIterMutInner::Linked { graph, cur, .. } => {
                if *cur == u32::MAX {
                    return None;
                }
                let id = *cur as usize;
                // SAFETY: we hold an exclusive borrow of the graph for `'a`,
                // and each call advances `cur` along the linked list -- every
                // edge index is visited at most once, so the yielded `&mut E`
                // references do not alias.
                unsafe {
                    match &mut (**graph).storage {
                        Storage::Linked { edges, next, .. } => {
                            let edges_ptr = edges.as_mut_ptr();
                            let next_ptr = next.as_ptr();
                            *cur = *next_ptr.add(id);
                            Some(&mut *edges_ptr.add(id))
                        }
                        Storage::TwoD { .. } => {
                            unreachable!("AdjIterMut::Linked never holds TwoD storage")
                        }
                    }
                }
            }
            AdjIterMutInner::Slice(it) => it.next(),
        }
    }
}

macro_rules! with_edges_impl {
    ($edge: ident, $name: ident) => {
        impl Graph<$edge<()>> {
            pub fn $name(n: usize, edges: &[(usize, usize)]) -> Self {
                let mut graph = Self::new_linked(n);
                for &(from, to) in edges {
                    graph.add_edge($edge::new(from, to));
                }
                graph
            }
        }
    };
    ($edge: ident, $name: ident, payload) => {
        impl<P: Clone> Graph<$edge<P>> {
            pub fn $name(n: usize, edges: &[(usize, usize, P)]) -> Self {
                let mut graph = Self::new_linked(n);
                for (from, to, p) in edges.iter() {
                    graph.add_edge($edge::with_payload(*from, *to, p.clone()));
                }
                graph
            }
        }
    };
}

with_edges_impl!(Edge, with_edges);
with_edges_impl!(Edge, with_edges_with_payload, payload);
with_edges_impl!(BiEdge, with_biedges);
with_edges_impl!(BiEdge, with_biedges_with_payload, payload);

pub struct CostAndFlow<C> {
    pub cost: C,
    pub flow: C,
}
