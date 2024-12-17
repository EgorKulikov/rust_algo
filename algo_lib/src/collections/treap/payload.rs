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
