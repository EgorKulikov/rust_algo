use crate::graph::edges::bi_edge_trait::BiEdgeTrait;
use crate::graph::edges::edge_id::{EdgeId, NoId, WithId};
use crate::graph::edges::edge_trait::{BidirectionalEdgeTrait, EdgeTrait};
use crate::graph::edges::weighted_edge_trait::WeightedEdgeTrait;

#[derive(Clone)]
pub struct BiWeightedEdgeRaw<W: Copy, Id: EdgeId, P> {
    to: u32,
    weight: W,
    id: Id,
    payload: P,
}

impl<W: Copy, Id: EdgeId> BiWeightedEdgeRaw<W, Id, ()> {
    pub fn new(to: usize, w: W) -> Self {
        Self {
            to: to as u32,
            weight: w,
            id: Id::new(),
            payload: (),
        }
    }
}

impl<W: Copy, Id: EdgeId, P> BiWeightedEdgeRaw<W, Id, P> {
    pub fn with_payload(to: usize, w: W, payload: P) -> Self {
        Self {
            to: to as u32,
            weight: w,
            id: Id::new(),
            payload,
        }
    }
}

impl<W: Copy, Id: EdgeId, P: Clone> BidirectionalEdgeTrait for BiWeightedEdgeRaw<W, Id, P> {}

impl<W: Copy, Id: EdgeId, P: Clone> EdgeTrait for BiWeightedEdgeRaw<W, Id, P> {
    type Payload = P;
    const REVERSABLE: bool = true;

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
        Self::with_payload(from, self.weight, self.payload.clone())
    }

    fn payload(&self) -> &P {
        &self.payload
    }
}

impl<W: Copy, Id: EdgeId, P: Clone> BiEdgeTrait for BiWeightedEdgeRaw<W, Id, P> {}

impl<W: Copy, Id: EdgeId, P: Clone> WeightedEdgeTrait<W> for BiWeightedEdgeRaw<W, Id, P> {
    fn weight(&self) -> W {
        self.weight
    }

    fn weight_mut(&mut self) -> &mut W {
        &mut self.weight
    }
}

pub type BiWeightedEdge<W, P> = BiWeightedEdgeRaw<W, NoId, P>;
pub type BiWeightedEdgeWithId<W, P> = BiWeightedEdgeRaw<W, WithId, P>;
