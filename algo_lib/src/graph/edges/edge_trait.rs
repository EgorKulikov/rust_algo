pub trait EdgeTrait: Clone {
    const REVERSABLE: bool;

    fn to(&self) -> usize;
    fn id(&self) -> usize;
    fn set_id(&mut self, id: usize);
    fn reverse_id(&self) -> usize;
    fn set_reverse_id(&mut self, reverse_id: usize);
    #[must_use]
    fn reverse_edge(&self, from: usize) -> Self;
}

pub trait BidirectionalEdgeTrait: EdgeTrait {}
