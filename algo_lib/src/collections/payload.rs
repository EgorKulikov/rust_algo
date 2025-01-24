#[allow(unused_variables)]
pub trait Payload: Sized {
    const NEED_UPDATE: bool = false;
    const NEED_ACCUMULATE: bool = false;
    fn reset_delta(&mut self) {
        unimplemented!()
    }
    fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
        unimplemented!()
    }
    fn accumulate(&mut self, delta: &Self) {
        unimplemented!()
    }
    fn need_push_down(&self) -> bool {
        true
    }
}

#[allow(unused_variables)]
pub trait OrdPayload: Payload {
    type Key: Ord;
    fn key(&self) -> &Self::Key;
    fn union(a: Self, b: Self) -> Self {
        unimplemented!()
    }
}
#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
