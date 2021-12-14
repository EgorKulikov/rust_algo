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
    W: Addable + Neg<Output = W> + PartialOrd + Copy + ZeroOne,
    C: AddSub + PartialOrd + Copy + ZeroOne,
    Id: EdgeId,
> {
    to: u32,
    weight: W,
    capacity: C,
    reverse_id: u32,
    id: Id,
}

impl<
        W: Addable + Neg<Output = W> + PartialOrd + Copy + ZeroOne,
        C: AddSub + PartialOrd + Copy + ZeroOne,
        Id: EdgeId,
    > WeightedFlowEdgeRaw<W, C, Id>
{
    pub fn new(to: usize, w: W, c: C) -> Self {
        Self {
            to: to as u32,
            weight: w,
            capacity: c,
            reverse_id: 0,
            id: Id::new(),
        }
    }
}

impl<
        W: Addable + Neg<Output = W> + PartialOrd + Copy + ZeroOne,
        C: AddSub + PartialOrd + Copy + ZeroOne,
        Id: EdgeId,
    > EdgeTrait for WeightedFlowEdgeRaw<W, C, Id>
{
    const REVERSABLE: bool = true;
    const BIDIRECTIONAL: bool = false;

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
        Self::new(from, -self.weight, C::zero())
    }
}

impl<
        W: Addable + Neg<Output = W> + PartialOrd + Copy + ZeroOne,
        C: AddSub + PartialOrd + Copy + ZeroOne,
        Id: EdgeId,
    > WeightedEdgeTrait<W> for WeightedFlowEdgeRaw<W, C, Id>
{
    fn weight(&self) -> W {
        self.weight
    }

    fn weight_mut(&mut self) -> &mut W {
        &mut self.weight
    }
}

impl<
        W: Addable + Neg<Output = W> + PartialOrd + Copy + ZeroOne,
        C: AddSub + PartialOrd + Copy + ZeroOne,
        Id: EdgeId,
    > FlowEdgeTrait<C> for WeightedFlowEdgeRaw<W, C, Id>
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

pub type WeightedFlowEdge<W, C> = WeightedFlowEdgeRaw<W, C, NoId>;
pub type WeightedFlowEdgeWithId<W, C> = WeightedFlowEdgeRaw<W, C, WithId>;
