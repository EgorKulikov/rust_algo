use crate::graph::bi_edge_trait::BiEdgeTrait;
use crate::graph::edge_id::{EdgeId, NoId, WithId};
use crate::graph::edge_trait::EdgeTrait;

#[derive(Clone)]
pub struct BiEdgeRaw<Id: EdgeId> {
    to: u32,
    id: Id,
}

impl<Id: EdgeId> BiEdgeRaw<Id> {
    pub fn new(to: usize) -> Self {
        Self {
            to: to as u32,
            id: Id::new(),
        }
    }
}

impl<Id: EdgeId> EdgeTrait for BiEdgeRaw<Id> {
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
        panic!("no reverse id")
    }

    fn set_reverse_id(&mut self, _: usize) {}

    fn reverse_edge(&self, from: usize) -> Self {
        Self::new(from)
    }
}

impl<Id: EdgeId> BiEdgeTrait for BiEdgeRaw<Id> {}

pub type BiEdge = BiEdgeRaw<NoId>;
pub type BiEdgeWithId = BiEdgeRaw<WithId>;
