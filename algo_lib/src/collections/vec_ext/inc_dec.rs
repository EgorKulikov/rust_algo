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

/// Implements [`IncDec`] for tuples treated as records: the **first two fields
/// are coordinates** and are shifted, while any remaining fields are an
/// untouched payload (edge capacity, cost, char, weight, …).
///
/// Only `.0` and `.1` are inc/dec'd; fields from `.2` onward are left as-is and
/// carry **no** `IncDec`/numeric bound — so `(usize, usize, char)` and
/// `(usize, usize, i64)` both work, and payloads are never corrupted.
///
/// Do not extend this to recurse into every field: decrementing a flow capacity
/// or cost is a silent bug, and it would reject valid non-numeric payloads.
macro_rules! tuple_inc_dec_impl {
    ($U: tt $V: tt $($tail: tt)*) => {
        impl<$U: IncDec, $V: IncDec, $($tail,)*> IncDec for ($U, $V, $($tail,)*) {
            fn inc_mut(&mut self) {
                self.0.inc_mut();
                self.1.inc_mut();
            }

            fn dec_mut(&mut self) {
                self.0.dec_mut();
                self.1.dec_mut();
            }
        }
    };
}

tuple_inc_dec_impl!(U V);
tuple_inc_dec_impl!(T U V);
tuple_inc_dec_impl!(T U V W);
tuple_inc_dec_impl!(T U V W X);
