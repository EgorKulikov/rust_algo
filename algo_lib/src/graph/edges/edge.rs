use crate::graph::edges::edge_id::{EdgeId, NoId, WithId};
use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::graph::Graph;
use crate::io::input::{Input, Readable};

#[derive(Clone)]
pub struct EdgeRaw<Id: EdgeId> {
    to: u32,
    id: Id,
}

impl<Id: EdgeId> EdgeRaw<Id> {
    pub fn new(to: usize) -> Self {
        Self {
            to: to as u32,
            id: Id::new(),
        }
    }
}

impl<Id: EdgeId> EdgeTrait for EdgeRaw<Id> {
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
        panic!("no reverse")
    }

    fn set_reverse_id(&mut self, _: usize) {
        panic!("no reverse")
    }

    fn reverse_edge(&self, _: usize) -> Self {
        panic!("no reverse")
    }
}

pub type Edge = EdgeRaw<NoId>;
pub type EdgeWithId = EdgeRaw<WithId>;

pub trait ReadEdgeGraph {
    fn read_graph<Id: EdgeId>(&mut self, n: usize, m: usize) -> Graph<EdgeRaw<Id>>;
}

impl ReadEdgeGraph for Input<'_> {
    fn read_graph<Id: EdgeId>(&mut self, n: usize, m: usize) -> Graph<EdgeRaw<Id>> {
        let mut graph = Graph::new(n);
        for _ in 0..m {
            graph.add_edge(self.read(), EdgeRaw::new(self.read()));
        }
        graph
    }
}

impl<Id: EdgeId> Readable for Graph<EdgeRaw<Id>> {
    fn read(input: &mut Input) -> Self {
        let n = input.read();
        let m = input.read();
        <Input as ReadEdgeGraph>::read_graph(input, n, m)
    }
}
