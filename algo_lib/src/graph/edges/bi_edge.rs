use crate::graph::edges::bi_edge_trait::BiEdgeTrait;
use crate::graph::edges::edge_id::{EdgeId, NoId, WithId};
use crate::graph::edges::edge_trait::{BidirectionalEdgeTrait, EdgeTrait};

#[derive(Clone)]
pub struct BiEdgeRaw<Id: EdgeId, P> {
    to: u32,
    id: Id,
    payload: P,
}

impl<Id: EdgeId> BiEdgeRaw<Id, ()> {
    pub fn new(from: usize, to: usize) -> (usize, Self) {
        (
            from,
            Self {
                to: to as u32,
                id: Id::new(),
                payload: (),
            },
        )
    }
}

impl<Id: EdgeId, P> BiEdgeRaw<Id, P> {
    pub fn with_payload(from: usize, to: usize, payload: P) -> (usize, Self) {
        (from, Self::with_payload_impl(to, payload))
    }

    fn with_payload_impl(to: usize, payload: P) -> BiEdgeRaw<Id, P> {
        Self {
            to: to as u32,
            id: Id::new(),
            payload,
        }
    }
}

impl<Id: EdgeId, P: Clone> BidirectionalEdgeTrait for BiEdgeRaw<Id, P> {}

impl<Id: EdgeId, P: Clone> EdgeTrait for BiEdgeRaw<Id, P> {
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
        panic!("no reverse id")
    }

    fn set_reverse_id(&mut self, _: usize) {}

    fn reverse_edge(&self, from: usize) -> Self {
        Self::with_payload_impl(from, self.payload.clone())
    }

    fn payload(&self) -> &P {
        &self.payload
    }
}

impl<Id: EdgeId, P: Clone> BiEdgeTrait for BiEdgeRaw<Id, P> {}

pub type BiEdge<P> = BiEdgeRaw<NoId, P>;
pub type BiEdgeWithId<P> = BiEdgeRaw<WithId, P>;
