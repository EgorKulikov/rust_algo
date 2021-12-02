use crate::graph::edge_id::{EdgeId, NoId, WithId};
use crate::graph::edge_trait::EdgeTrait;
use crate::graph::flow_edge_trait::FlowEdgeTrait;
use crate::graph::graph::Graph;
use crate::io::input::{Input, Readable};
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
}

pub type FlowEdge<C> = FlowEdgeRaw<C, NoId>;
pub type FlowEdgeWithId<C> = FlowEdgeRaw<C, WithId>;

pub trait ReadFlowEdgeGraph {
    fn read_graph<C: Integer + Readable, Id: EdgeId>(
        &mut self,
        n: usize,
        m: usize,
    ) -> Graph<FlowEdgeRaw<C, Id>>;
}

impl ReadFlowEdgeGraph for Input<'_> {
    fn read_graph<C: Integer + Readable, Id: EdgeId>(
        &mut self,
        n: usize,
        m: usize,
    ) -> Graph<FlowEdgeRaw<C, Id>> {
        let mut graph = Graph::new(n);
        for _ in 0..m {
            graph.add_edge(self.read(), FlowEdgeRaw::new(self.read(), self.read()));
        }
        graph
    }
}

impl<W: Integer + Readable, Id: EdgeId> Readable for Graph<FlowEdgeRaw<W, Id>> {
    fn read(input: &mut Input) -> Self {
        let n = input.read();
        let m = input.read();
        <Input as ReadFlowEdgeGraph>::read_graph(input, n, m)
    }
}
