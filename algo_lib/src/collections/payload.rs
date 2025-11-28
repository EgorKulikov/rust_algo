use crate::misc::value_delta::ValueDeltaTrait;
use std::marker::PhantomData;

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

#[derive(Copy, Clone)]
pub struct ValueDeltaPayload<VDT: ValueDeltaTrait> {
    pub self_v: VDT::V,
    pub v: VDT::V,
    pub d: VDT::D,
    phantom: PhantomData<VDT>,
}

impl<VDT: ValueDeltaTrait> ValueDeltaPayload<VDT>
where
    VDT::V: Copy,
    VDT::D: Default,
{
    pub fn new(value: VDT::V) -> Self {
        Self {
            self_v: value,
            v: value,
            d: VDT::D::default(),
            phantom: PhantomData,
        }
    }
}

impl<VDT: ValueDeltaTrait> Payload for ValueDeltaPayload<VDT>
where
    VDT::V: Copy,
    VDT::D: Default + Copy,
{
    const NEED_UPDATE: bool = true;
    const NEED_ACCUMULATE: bool = true;

    fn reset_delta(&mut self) {
        self.d = VDT::D::default();
    }

    fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
        self.v = self.self_v;
        if let Some(left) = left {
            self.v = VDT::join(left.v, self.v);
        }
        if let Some(right) = right {
            self.v = VDT::join(self.v, right.v);
        }
    }

    fn accumulate(&mut self, delta: &Self) {
        self.d = VDT::accumulate(self.d, delta.d);
        self.v = VDT::apply(self.v, delta.d);
    }
}

impl<VDT: ValueDeltaTrait> OrdPayload for ValueDeltaPayload<VDT>
where
    VDT::V: Copy + Ord,
    VDT::D: Default + Copy,
{
    type Key = VDT::V;

    fn key(&self) -> &Self::Key {
        &self.self_v
    }

    fn union(a: Self, b: Self) -> Self {
        Self::new(VDT::join(a.self_v, b.self_v))
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

    fn key(&self) -> &Self::Key {
        &self.0
    }

    fn union(a: Self, _b: Self) -> Self {
        a
    }
}
