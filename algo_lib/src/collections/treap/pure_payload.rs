use crate::collections::treap::payload::{OrdPayload, Payload, Pushable};

pub struct PurePayload<T>(pub T);

impl<T> Payload for PurePayload<T> {}

impl<T: Ord> OrdPayload for PurePayload<T> {
    type Key = T;

    #[inline]
    fn key(&self) -> &Self::Key {
        &self.0
    }

    fn union(a: Self, _b: Self) -> Self {
        a
    }
}

impl<T> Pushable<T> for PurePayload<T> {
    fn push(&mut self, delta: T) {
        self.0 = delta;
    }
}
