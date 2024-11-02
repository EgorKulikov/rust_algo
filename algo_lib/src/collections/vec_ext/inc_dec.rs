use crate::numbers::num_traits::algebra::{AdditionMonoidWithSub, One};

pub trait IncDec {
    #[must_use]
    fn inc(self) -> Self;
    #[must_use]
    fn dec(self) -> Self;
}

impl<T: AdditionMonoidWithSub + One> IncDec for T {
    fn inc(self) -> Self {
        self + T::one()
    }

    fn dec(self) -> Self {
        self - T::one()
    }
}

impl<T: AdditionMonoidWithSub + One> IncDec for Vec<T> {
    fn inc(mut self) -> Self {
        self.iter_mut().for_each(|i| *i += T::one());
        self
    }

    fn dec(mut self) -> Self {
        self.iter_mut().for_each(|i| *i -= T::one());
        self
    }
}

impl<T: AdditionMonoidWithSub + One> IncDec for Vec<Vec<T>> {
    fn inc(mut self) -> Self {
        self.iter_mut()
            .for_each(|v| v.iter_mut().for_each(|i| *i += T::one()));
        self
    }

    fn dec(mut self) -> Self {
        self.iter_mut()
            .for_each(|v| v.iter_mut().for_each(|i| *i -= T::one()));
        self
    }
}

impl<T: AdditionMonoidWithSub + One, U: AdditionMonoidWithSub + One> IncDec for Vec<(T, U)> {
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

impl<T: AdditionMonoidWithSub + One, U: AdditionMonoidWithSub + One, V> IncDec for Vec<(T, U, V)> {
    fn inc(mut self) -> Self {
        self.iter_mut().for_each(|(i, j, _)| {
            *i += T::one();
            *j += U::one();
        });
        self
    }

    fn dec(mut self) -> Self {
        self.iter_mut().for_each(|(i, j, _)| {
            *i -= T::one();
            *j -= U::one();
        });
        self
    }
}

impl<T: AdditionMonoidWithSub + One, U: AdditionMonoidWithSub + One, V, W> IncDec
    for Vec<(T, U, V, W)>
{
    fn inc(mut self) -> Self {
        self.iter_mut().for_each(|(i, j, ..)| {
            *i += T::one();
            *j += U::one();
        });
        self
    }

    fn dec(mut self) -> Self {
        self.iter_mut().for_each(|(i, j, ..)| {
            *i -= T::one();
            *j -= U::one();
        });
        self
    }
}

impl<T: AdditionMonoidWithSub + One, U: AdditionMonoidWithSub + One, V, W, X> IncDec
    for Vec<(T, U, V, W, X)>
{
    fn inc(mut self) -> Self {
        self.iter_mut().for_each(|(i, j, ..)| {
            *i += T::one();
            *j += U::one();
        });
        self
    }

    fn dec(mut self) -> Self {
        self.iter_mut().for_each(|(i, j, ..)| {
            *i -= T::one();
            *j -= U::one();
        });
        self
    }
}

impl<T: AdditionMonoidWithSub + One, U: AdditionMonoidWithSub + One> IncDec for (T, U) {
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
