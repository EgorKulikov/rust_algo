use crate::graph::edges::edge_id::{EdgeId, NoId, WithId};
use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use crate::graph::graph::Graph;
use crate::io::input::{Input, Readable};
use crate::numbers::num_traits::add_sub::Addable;
use crate::numbers::num_traits::zero_one::ZeroOne;

#[derive(Clone)]
pub struct WeightedEdgeRaw<W: Addable + PartialOrd + Copy + ZeroOne, Id: EdgeId> {
    to: u32,
    weight: W,
    id: Id,
}

impl<W: Addable + PartialOrd + Copy + ZeroOne, Id: EdgeId> WeightedEdgeRaw<W, Id> {
    pub fn new(to: usize, w: W) -> Self {
        Self {
            to: to as u32,
            weight: w,
            id: Id::new(),
        }
    }
}

impl<W: Addable + PartialOrd + Copy + ZeroOne, Id: EdgeId> EdgeTrait for WeightedEdgeRaw<W, Id> {
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

impl<W: Addable + PartialOrd + Copy + ZeroOne, Id: EdgeId> WeightedEdgeTrait<W>
    for WeightedEdgeRaw<W, Id>
{
    fn weight(&self) -> W {
        self.weight
    }

    fn weight_mut(&mut self) -> &mut W {
        &mut self.weight
    }
}

pub type WeightedEdge<W> = WeightedEdgeRaw<W, NoId>;
pub type WeightedEdgeWithId<W> = WeightedEdgeRaw<W, WithId>;

pub trait ReadWeightedEdgeGraph {
    fn read_graph<W: Addable + PartialOrd + Copy + ZeroOne + Readable, Id: EdgeId>(
        &mut self,
        n: usize,
        m: usize,
    ) -> Graph<WeightedEdgeRaw<W, Id>>;
}

impl ReadWeightedEdgeGraph for Input<'_> {
    fn read_graph<W: Addable + PartialOrd + Copy + ZeroOne + Readable, Id: EdgeId>(
        &mut self,
        n: usize,
        m: usize,
    ) -> Graph<WeightedEdgeRaw<W, Id>> {
        let mut graph = Graph::new(n);
        for _ in 0..m {
            graph.add_edge(self.read(), WeightedEdgeRaw::new(self.read(), self.read()));
        }
        graph
    }
}

impl<W: Addable + PartialOrd + Copy + ZeroOne + Readable, Id: EdgeId> Readable
    for Graph<WeightedEdgeRaw<W, Id>>
{
    fn read(input: &mut Input) -> Self {
        let n = input.read();
        let m = input.read();
        <Input as ReadWeightedEdgeGraph>::read_graph(input, n, m)
    }
}
