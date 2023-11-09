use crate::graph::edges::edge_id::{EdgeId, NoId, WithId};
use crate::graph::edges::edge_trait::EdgeTrait;

#[derive(Clone)]
pub struct EdgeRaw<Id: EdgeId, P> {
    to: u32,
    id: Id,
    payload: P,
}

impl<Id: EdgeId> EdgeRaw<Id, ()> {
    pub fn new(to: usize) -> Self {
        Self {
            to: to as u32,
            id: Id::new(),
            payload: (),
        }
    }
}

impl<Id: EdgeId, P> EdgeRaw<Id, P> {
    pub fn with_payload(to: usize, payload: P) -> Self {
        Self {
            to: to as u32,
            id: Id::new(),
            payload,
        }
    }
}

impl<Id: EdgeId, P: Clone> EdgeTrait for EdgeRaw<Id, P> {
    type Payload = P;

    const REVERSABLE: bool = false;

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

    fn set_reverse_id(&mut self, _: usize) {
        panic!("no reverse")
    }

    fn reverse_edge(&self, _: usize) -> Self {
        panic!("no reverse")
    }

    fn payload(&self) -> &P {
        &self.payload
    }
}

pub type Edge<P> = EdgeRaw<NoId, P>;
pub type EdgeWithId<P> = EdgeRaw<WithId, P>;
