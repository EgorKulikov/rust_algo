use crate::graph::edge_id::{EdgeId, NoId, WithId};
use crate::graph::edge_trait::EdgeTrait;
use crate::graph::flow_edge_trait::FlowEdgeTrait;
use crate::graph::graph::Graph;
use crate::numbers::integer::Integer;

#[derive(Clone)]
pub struct FlowEdgeRaw<C: Integer, Id: EdgeId> {
    to: u32,
    capacity: C,
    reverse_id: u32,
    id: Id,
}

impl<C: Integer, Id: EdgeId> FlowEdgeRaw<C, Id> {
    pub fn new(to: usize, c: C) -> Self {
        Self {
            to: to as u32,
            capacity: c,
            reverse_id: 0,
            id: Id::new(),
        }
    }
}

impl<C: Integer, Id: EdgeId> EdgeTrait for FlowEdgeRaw<C, Id> {
    const REVERSABLE: bool = false;
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
        Self::new(from, C::zero())
    }
}

impl<C: Integer, Id: EdgeId> FlowEdgeTrait<C> for FlowEdgeRaw<C, Id> {
    fn capacity(&self) -> C {
        self.capacity
    }

    fn capacity_mut(&mut self) -> &mut C {
        &mut self.capacity
    }

    fn flow(&self, graph: &Graph<Self>) -> C {
        graph[self.to as usize][self.reverse_id as usize].capacity
    }

    fn push_flow(&mut self, flow: C, graph: &mut Graph<Self>) {
        debug_assert!(flow >= C::zero() && flow <= self.capacity);
        self.capacity -= flow;
        graph[self.to as usize][self.reverse_id as usize].capacity += flow;
    }
}

pub type FlowEdge<C> = FlowEdgeRaw<C, NoId>;
pub type FlowEdgeWithId<C> = FlowEdgeRaw<C, WithId>;
