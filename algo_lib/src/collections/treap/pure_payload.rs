use crate::collections::treap::payload::{OrdPayload, Payload, Pushable};

pub struct PurePayload<T>(pub T);

impl<T: Unpin> Payload for PurePayload<T> {}

impl<T: Ord + Unpin> OrdPayload for PurePayload<T> {
    type Key = T;

    #[inline]
    fn key(&self) -> &Self::Key {
        &self.0
    }
}

impl<T: Ord + Unpin> Pushable<T> for PurePayload<T> {
    fn push(&mut self, delta: T) {
        self.0 = delta;
    }
}
