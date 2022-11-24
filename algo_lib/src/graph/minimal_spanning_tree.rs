use crate::collections::dsu::DSU;
use crate::graph::edges::edge_trait::BidirectionalEdgeTrait;
use crate::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use crate::graph::graph::Graph;

pub trait MinimalSpanningTree<W: Ord + Copy, E: WeightedEdgeTrait<W>> {
    fn minimal_spanning_tree(&self) -> Graph<E>;
}

impl<W: Ord + Copy, E: WeightedEdgeTrait<W> + BidirectionalEdgeTrait> MinimalSpanningTree<W, E>
    for Graph<E>
{
    fn minimal_spanning_tree(&self) -> Graph<E> {
        let mut edges = Vec::with_capacity(self.edge_count());
        for i in 0..self.vertex_count() {
            for e in &self[i] {
                if e.to() > i {
                    edges.push((i, e.clone()));
                }
            }
        }
        edges.sort_by_key(|(_, e)| e.weight());
        let mut res = Graph::new(self.vertex_count());
        let mut dsu = DSU::new(self.vertex_count());
        for (i, e) in edges {
            if dsu.join(i, e.to()) {
                res.add_edge(i, e);
            }
        }
        res
    }
}
