pub trait EdgeTrait: Clone {
    type Payload;

    const REVERSABLE: bool;
    /// Whether `reverse_id` is meaningful (flow edges); other reversible
    /// edges are stored in both directions without cross links.
    const TRACKS_REVERSE: bool = false;

    fn to(&self) -> usize;
    fn id(&self) -> usize;
    fn set_id(&mut self, id: usize);
    #[doc(hidden)]
    fn reverse_id(&self) -> usize;
    #[doc(hidden)]
    fn set_reverse_id(&mut self, reverse_id: usize);
    #[must_use]
    fn reverse_edge(&self, from: usize) -> Self;
    fn payload(&self) -> &Self::Payload;
}

pub trait BidirectionalEdgeTrait: EdgeTrait {}
