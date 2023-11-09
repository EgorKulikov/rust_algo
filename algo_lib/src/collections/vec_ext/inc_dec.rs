use crate::numbers::num_traits::add_sub::AddSub;
use crate::numbers::num_traits::zero_one::ZeroOne;

pub trait IncDec {
    #[must_use]
    fn inc(self) -> Self;
    #[must_use]
    fn dec(self) -> Self;
}

impl<T: AddSub + ZeroOne> IncDec for Vec<T> {
    fn inc(mut self) -> Self {
        self.iter_mut().for_each(|i| *i += T::one());
        self
    }

    fn dec(mut self) -> Self {
        self.iter_mut().for_each(|i| *i -= T::one());
        self
    }
}

impl<T: AddSub + ZeroOne, U: AddSub + ZeroOne> IncDec for Vec<(T, U)> {
    fn inc(mut self) -> Self {
        self.iter_mut().for_each(|(i, j)| {
            *i += T::one();
            *j += U::one();
        });
        self
    }

    fn dec(mut self) -> Self {
        self.iter_mut().for_each(|(i, j)| {
            *i -= T::one();
            *j -= U::one();
        });
        self
    }
}

impl<T: AddSub + ZeroOne, U: AddSub + ZeroOne, V> IncDec for Vec<(T, U, V)> {
    fn inc(mut self) -> Self {
        self.iter_mut().for_each(|(i, j, k)| {
            *i += T::one();
            *j += U::one();
        });
        self
    }

    fn dec(mut self) -> Self {
        self.iter_mut().for_each(|(i, j, k)| {
            *i -= T::one();
            *j -= U::one();
        });
        self
    }
}

impl<T: AddSub + ZeroOne, U: AddSub + ZeroOne> IncDec for (T, U) {
    fn inc(mut self) -> Self {
        self.0 += T::one();
        self.1 += U::one();
        self
    }

    fn dec(mut self) -> Self {
        self.0 -= T::one();
        self.1 -= U::one();
        self
    }
}
