pub trait Size: Payload {
    const ONE: Self;
    fn size(&self) -> usize;
}

impl Payload for () {}

impl Size for () {
    const ONE: Self = ();
    #[inline]
    fn size(&self) -> usize {
        unimplemented!()
    }
}

impl Payload for u32 {
    const NEED_UPDATE: bool = true;

    #[inline]
    fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
        *self = 1 + left.unwrap_or(&0) + right.unwrap_or(&0);
    }
}

impl Size for u32 {
    const ONE: Self = 1;
    #[inline]
    fn size(&self) -> usize {
        *self as usize
    }
}

pub trait Reverse: Default {
    fn need_reverse(&self) -> bool;
    fn reset_reverse(&mut self);
    fn flip(&mut self);
}

impl Reverse for () {
    #[inline]
    fn need_reverse(&self) -> bool {
        false
    }

    #[inline]
    fn flip(&mut self) {
        unimplemented!()
    }

    #[inline]
    fn reset_reverse(&mut self) {
        unimplemented!()
    }
}

impl Reverse for bool {
    #[inline]
    fn need_reverse(&self) -> bool {
        *self
    }

    #[inline]
    fn reset_reverse(&mut self) {
        *self = false;
    }

    #[inline]
    fn flip(&mut self) {
        *self ^= true;
    }
}

#[allow(unused_variables)]
pub trait Payload: Sized {
    const NEED_UPDATE: bool = false;
    const NEED_PUSH_DOWN: bool = false;
    fn reset_delta(&mut self) {
        unimplemented!()
    }
    fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
        unimplemented!()
    }
    fn push_delta(&mut self, delta: &Self) {
        unimplemented!()
    }
    fn need_push_down(&self) -> bool {
        unimplemented!()
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

pub trait Pushable<Delta>: Payload {
    fn push(&mut self, delta: Delta);
}

impl<P: Payload> Pushable<&P> for P {
    #[inline]
    fn push(&mut self, delta: &P) {
        self.push_delta(delta);
    }
}

impl<P: Payload> Pushable<P> for P {
    #[inline]
    fn push(&mut self, delta: P) {
        *self = delta;
    }
}
