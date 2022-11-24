use crate::graph::edges::bi_edge_trait::BiEdgeTrait;
use crate::graph::edges::edge_id::{EdgeId, NoId, WithId};
use crate::graph::edges::edge_trait::{BidirectionalEdgeTrait, EdgeTrait};
use crate::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use crate::graph::graph::Graph;
use crate::io::input::{Input, Readable};
use crate::numbers::num_traits::add_sub::Addable;
use crate::numbers::num_traits::zero_one::ZeroOne;

#[derive(Clone)]
pub struct BiWeightedEdgeRaw<W: Copy, Id: EdgeId> {
    to: u32,
    weight: W,
    id: Id,
}

impl<W: Copy, Id: EdgeId> BiWeightedEdgeRaw<W, Id> {
    pub fn new(to: usize, w: W) -> Self {
        Self {
            to: to as u32,
            weight: w,
            id: Id::new(),
        }
    }
}

impl<W: Copy, Id: EdgeId> BidirectionalEdgeTrait for BiWeightedEdgeRaw<W, Id> {}

impl<W: Copy, Id: EdgeId> EdgeTrait for BiWeightedEdgeRaw<W, Id> {
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
        panic!("no reverse")
    }

    fn set_reverse_id(&mut self, _: usize) {}

    fn reverse_edge(&self, from: usize) -> Self {
        Self::new(from, self.weight)
    }
}

impl<W: Copy, Id: EdgeId> BiEdgeTrait for BiWeightedEdgeRaw<W, Id> {}

impl<W: Copy, Id: EdgeId> WeightedEdgeTrait<W> for BiWeightedEdgeRaw<W, Id> {
    fn weight(&self) -> W {
        self.weight
    }

    fn weight_mut(&mut self) -> &mut W {
        &mut self.weight
    }
}

pub type BiWeightedEdge<W> = BiWeightedEdgeRaw<W, NoId>;
pub type BiWeightedEdgeWithId<W> = BiWeightedEdgeRaw<W, WithId>;

pub trait ReadBiWeightedEdgeGraph {
    fn read_graph<W: Addable + Copy + ZeroOne + Readable, Id: EdgeId>(
        &mut self,
        n: usize,
        m: usize,
    ) -> Graph<BiWeightedEdgeRaw<W, Id>>;

    fn read_tree<W: Addable + Copy + ZeroOne + Readable, Id: EdgeId>(
        &mut self,
        n: usize,
    ) -> Graph<BiWeightedEdgeRaw<W, Id>> {
        self.read_graph(n, n - 1)
    }
}

impl ReadBiWeightedEdgeGraph for Input<'_> {
    fn read_graph<W: Addable + Copy + ZeroOne + Readable, Id: EdgeId>(
        &mut self,
        n: usize,
        m: usize,
    ) -> Graph<BiWeightedEdgeRaw<W, Id>> {
        let mut graph = Graph::new(n);
        for _ in 0..m {
            graph.add_edge(
                self.read(),
                BiWeightedEdgeRaw::new(self.read(), self.read()),
            );
        }
        graph
    }
}

impl<W: Addable + Copy + ZeroOne + Readable, Id: EdgeId> Readable
    for Graph<BiWeightedEdgeRaw<W, Id>>
{
    fn read(input: &mut Input) -> Self {
        let n = input.read();
        let m = input.read();
        <Input as ReadBiWeightedEdgeGraph>::read_graph(input, n, m)
    }
}
