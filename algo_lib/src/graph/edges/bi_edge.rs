use crate::graph::edges::bi_edge_trait::BiEdgeTrait;
use crate::graph::edges::edge_id::{EdgeId, NoId, WithId};
use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::graph::Graph;
use crate::io::input::{Input, Readable};

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

pub trait ReadBiEdgeGraph {
    fn read_graph<Id: EdgeId>(&mut self, n: usize, m: usize) -> Graph<BiEdgeRaw<Id>>;

    fn read_tree<Id: EdgeId>(&mut self, n: usize) -> Graph<BiEdgeRaw<Id>> {
        self.read_graph(n, n - 1)
    }
}

impl ReadBiEdgeGraph for Input<'_> {
    fn read_graph<Id: EdgeId>(&mut self, n: usize, m: usize) -> Graph<BiEdgeRaw<Id>> {
        let mut graph = Graph::new(n);
        for _ in 0..m {
            graph.add_edge(self.read(), BiEdgeRaw::new(self.read()));
        }
        graph
    }
}

impl<Id: EdgeId> Readable for Graph<BiEdgeRaw<Id>> {
    fn read(input: &mut Input) -> Self {
        let n = input.read();
        let m = input.read();
        <Input as ReadBiEdgeGraph>::read_graph(input, n, m)
    }
}
