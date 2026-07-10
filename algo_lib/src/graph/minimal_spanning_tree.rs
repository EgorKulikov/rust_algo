use crate::collections::dsu::DSU;
use crate::graph::edges::edge_trait::BidirectionalEdgeTrait;
use crate::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use crate::graph::Graph;
use crate::numbers::num_traits::algebra::AdditionMonoid;

pub(crate) trait MstTarget<W: Copy, E: WeightedEdgeTrait<W>> {
    fn new(vertex_count: usize) -> Self;
    fn add_edge(&mut self, from: usize, edge: E);
}

impl<W: Copy, E: WeightedEdgeTrait<W>> MstTarget<W, E> for Graph<E> {
    fn new(vertex_count: usize) -> Self {
        Graph::new_linked(vertex_count)
    }

    fn add_edge(&mut self, from: usize, edge: E) {
        Graph::add_edge(self, (from, edge));
    }
}

pub(crate) struct WeightSum<W>(pub(crate) W);

impl<W: AdditionMonoid + Copy, E: WeightedEdgeTrait<W>> MstTarget<W, E> for WeightSum<W> {
    fn new(_vertex_count: usize) -> Self {
        Self(W::zero())
    }

    fn add_edge(&mut self, _from: usize, edge: E) {
        self.0 += edge.weight();
    }
}

fn mst<W: Ord + Copy, E: WeightedEdgeTrait<W> + BidirectionalEdgeTrait, T: MstTarget<W, E>>(
    graph: &Graph<E>,
) -> T {
    let mut edges = Vec::with_capacity(graph.edge_count());
    for i in 0..graph.vertex_count() {
        for e in graph.adj(i).iter() {
            if e.to() > i {
                edges.push((i, e.clone()));
            }
        }
    }
    edges.sort_by_key(|(_, e)| e.weight());
    let mut res = T::new(graph.vertex_count());
    let mut dsu = DSU::new(graph.vertex_count());
    for (i, e) in edges {
        if dsu.union(i, e.to()) {
            res.add_edge(i, e);
        }
    }
    res
}

pub trait MinimalSpanningTree<W: Ord + AdditionMonoid + Copy, E: WeightedEdgeTrait<W>> {
    fn minimal_spanning_tree(&self) -> Graph<E>;
    fn minimal_spanning_tree_weight(&self) -> W;
}

impl<W: Ord + AdditionMonoid + Copy, E: WeightedEdgeTrait<W> + BidirectionalEdgeTrait>
    MinimalSpanningTree<W, E> for Graph<E>
{
    fn minimal_spanning_tree(&self) -> Graph<E> {
        mst(self)
    }

    fn minimal_spanning_tree_weight(&self) -> W {
        mst::<W, E, WeightSum<W>>(self).0
    }
}
