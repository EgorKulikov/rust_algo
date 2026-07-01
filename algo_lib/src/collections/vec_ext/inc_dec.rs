use crate::numbers::num_traits::algebra::{AdditionMonoidWithSub, One};

pub trait IncDec: Sized {
    fn inc_mut(&mut self);
    fn dec_mut(&mut self);

    #[must_use]
    fn inc(mut self) -> Self {
        self.inc_mut();
        self
    }

    #[must_use]
    fn dec(mut self) -> Self {
        self.dec_mut();
        self
    }
}

impl<T: AdditionMonoidWithSub + One> IncDec for T {
    fn inc_mut(&mut self) {
        *self += T::one()
    }

    fn dec_mut(&mut self) {
        *self -= T::one()
    }
}

impl<T: IncDec> IncDec for Vec<T> {
    fn inc_mut(&mut self) {
        self.iter_mut().for_each(T::inc_mut);
    }

    fn dec_mut(&mut self) {
        self.iter_mut().for_each(T::dec_mut);
    }
}

impl<U: IncDec, V: IncDec> IncDec for (U, V) {
    fn inc_mut(&mut self) {
        self.0.inc_mut();
        self.1.inc_mut();
    }

    fn dec_mut(&mut self) {
        self.0.dec_mut();
        self.1.dec_mut();
    }
}

impl<T: IncDec, U: IncDec, V: IncDec> IncDec for (T, U, V) {
    fn inc_mut(&mut self) {
        self.0.inc_mut();
        self.1.inc_mut();
        self.2.inc_mut();
    }

    fn dec_mut(&mut self) {
        self.0.dec_mut();
        self.1.dec_mut();
        self.2.dec_mut();
    }
}
