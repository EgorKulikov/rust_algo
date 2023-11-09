use crate::graph::edges::edge_id::{EdgeId, NoId, WithId};
use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::edges::flow_edge_trait::FlowEdgeTrait;
use crate::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use crate::graph::graph::Graph;
use crate::numbers::num_traits::add_sub::{AddSub, Addable};
use crate::numbers::num_traits::zero_one::ZeroOne;
use std::ops::Neg;

#[derive(Clone)]
pub struct WeightedFlowEdgeRaw<
    W: Addable + Neg<Output = W> + Copy + ZeroOne,
    C: AddSub + PartialOrd + Copy + ZeroOne,
    Id: EdgeId,
    P,
> {
    to: u32,
    weight: W,
    capacity: C,
    reverse_id: u32,
    id: Id,
    payload: P,
}

impl<
        W: Addable + Neg<Output = W> + Copy + ZeroOne,
        C: AddSub + PartialOrd + Copy + ZeroOne,
        Id: EdgeId,
    > WeightedFlowEdgeRaw<W, C, Id, ()>
{
    pub fn new(to: usize, w: W, c: C) -> Self {
        Self {
            to: to as u32,
            weight: w,
            capacity: c,
            reverse_id: 0,
            id: Id::new(),
            payload: (),
        }
    }
}

impl<
        W: Addable + Neg<Output = W> + Copy + ZeroOne,
        C: AddSub + PartialOrd + Copy + ZeroOne,
        Id: EdgeId,
        P,
    > WeightedFlowEdgeRaw<W, C, Id, P>
{
    pub fn with_payload(to: usize, w: W, c: C, payload: P) -> Self {
        Self {
            to: to as u32,
            weight: w,
            capacity: c,
            reverse_id: 0,
            id: Id::new(),
            payload,
        }
    }
}

impl<
        W: Addable + Neg<Output = W> + Copy + ZeroOne,
        C: AddSub + PartialOrd + Copy + ZeroOne,
        Id: EdgeId,
        P: Clone,
    > EdgeTrait for WeightedFlowEdgeRaw<W, C, Id, P>
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
        Self::with_payload(from, -self.weight, C::zero(), self.payload.clone())
    }

    fn payload(&self) -> &P {
        &self.payload
    }
}

impl<
        W: Addable + Neg<Output = W> + Copy + ZeroOne,
        C: AddSub + PartialOrd + Copy + ZeroOne,
        Id: EdgeId,
        P: Clone,
    > WeightedEdgeTrait<W> for WeightedFlowEdgeRaw<W, C, Id, P>
{
    fn weight(&self) -> W {
        self.weight
    }

    fn weight_mut(&mut self) -> &mut W {
        &mut self.weight
    }
}

impl<
        W: Addable + Neg<Output = W> + Copy + ZeroOne,
        C: AddSub + PartialOrd + Copy + ZeroOne,
        Id: EdgeId,
        P: Clone,
    > FlowEdgeTrait<C> for WeightedFlowEdgeRaw<W, C, Id, P>
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

pub type WeightedFlowEdge<W, C, P> = WeightedFlowEdgeRaw<W, C, NoId, P>;
pub type WeightedFlowEdgeWithId<W, C, P> = WeightedFlowEdgeRaw<W, C, WithId, P>;
