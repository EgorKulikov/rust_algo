use crate::graph::bi_edge_trait::BiEdgeTrait;
use crate::graph::edge_id::{EdgeId, NoId, WithId};
use crate::graph::edge_trait::EdgeTrait;
use crate::graph::weighted_edge_trait::WeightedEdgeTrait;
use crate::numbers::integer::Integer;

#[derive(Clone)]
pub struct BiWeightedEdgeRaw<W: Integer, Id: EdgeId> {
    to: u32,
    weight: W,
    id: Id,
}

impl<W: Integer, Id: EdgeId> BiWeightedEdgeRaw<W, Id> {
    pub fn new(to: usize, w: W) -> Self {
        Self {
            to: to as u32,
            weight: w,
            id: Id::new(),
        }
    }
}

impl<W: Integer, Id: EdgeId> EdgeTrait for BiWeightedEdgeRaw<W, Id> {
    const REVERSABLE: bool = true;
    const BIDIRECTIONAL: bool = true;

    fn to(&self) -> usize {
        self.to as usize
    }

    fn id(&self) -> usize {
        self.id.id()
    }

    fn set_id(&mut self, id: usize) {
        self.id.set_id(id);
    }

    fn reverse_id(&self) -> usize {
        panic!("no reverse")
    }

    fn set_reverse_id(&mut self, _: usize) {}

    fn reverse_edge(&self, from: usize) -> Self {
        Self::new(from, self.weight)
    }
}

impl<W: Integer, Id: EdgeId> BiEdgeTrait for BiWeightedEdgeRaw<W, Id> {}

impl<W: Integer, Id: EdgeId> WeightedEdgeTrait<W> for BiWeightedEdgeRaw<W, Id> {
    fn weight(&self) -> W {
        self.weight
    }

    fn weight_mut(&mut self) -> &mut W {
        &mut self.weight
    }
}

pub type BiWeightedEdge<W> = BiWeightedEdgeRaw<W, NoId>;
pub type BiWeightedEdgeWithId<W> = BiWeightedEdgeRaw<W, WithId>;
