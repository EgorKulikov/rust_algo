use crate::graph::edges::edge_id::{EdgeId, NoId, WithId};
use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::edges::flow_edge_trait::FlowEdgeTrait;
use crate::graph::Graph;
use crate::numbers::num_traits::algebra::AdditionMonoidWithSub;

#[derive(Clone)]
pub struct FlowEdgeRaw<C: AdditionMonoidWithSub + PartialOrd + Copy, Id: EdgeId, P> {
    to: u32,
    capacity: C,
    reverse_id: u32,
    id: Id,
    payload: P,
}

impl<C: AdditionMonoidWithSub + PartialOrd + Copy, Id: EdgeId> FlowEdgeRaw<C, Id, ()> {
    pub fn new(from: usize, to: usize, c: C) -> (usize, Self) {
        (
            from,
            Self {
                to: to as u32,
                capacity: c,
                reverse_id: 0,
                id: Id::new(),
                payload: (),
            },
        )
    }
}

impl<C: AdditionMonoidWithSub + PartialOrd + Copy, Id: EdgeId, P> FlowEdgeRaw<C, Id, P> {
    pub fn with_payload(from: usize, to: usize, c: C, payload: P) -> (usize, Self) {
        (from, Self::with_payload_impl(to, c, payload))
    }

    fn with_payload_impl(to: usize, c: C, payload: P) -> Self {
        Self {
            to: to as u32,
            capacity: c,
            reverse_id: 0,
            id: Id::new(),
            payload,
        }
    }
}

impl<C: AdditionMonoidWithSub + PartialOrd + Copy, Id: EdgeId, P: Default + Clone> EdgeTrait
    for FlowEdgeRaw<C, Id, P>
{
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
        self.reverse_id as usize
    }

    fn set_reverse_id(&mut self, reverse_id: usize) {
        self.reverse_id = reverse_id as u32;
    }

    fn reverse_edge(&self, from: usize) -> Self {
        Self::with_payload_impl(from, C::zero(), P::default())
    }

    fn payload(&self) -> &P {
        &self.payload
    }
}

impl<C: AdditionMonoidWithSub + PartialOrd + Copy, Id: EdgeId, P: Default + Clone> FlowEdgeTrait<C>
    for FlowEdgeRaw<C, Id, P>
{
    fn capacity(&self) -> C {
        self.capacity
    }

    fn capacity_mut(&mut self) -> &mut C {
        &mut self.capacity
    }

    fn flow(&self, graph: &Graph<Self>) -> C {
        graph[self.to as usize][self.reverse_id as usize].capacity
    }
}

pub type FlowEdge<C, P> = FlowEdgeRaw<C, NoId, P>;
pub type FlowEdgeWithId<C, P> = FlowEdgeRaw<C, WithId, P>;
