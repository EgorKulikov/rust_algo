pub trait ValueDeltaTrait {
    type V;
    type D;
    fn join(v1: Self::V, v2: Self::V) -> Self::V;
    fn accumulate(d1: Self::D, d2: Self::D) -> Self::D;
    fn apply(v: Self::V, d: Self::D) -> Self::V;
}

pub trait ValueTrait {
    type V;
    fn join(v1: Self::V, v2: Self::V) -> Self::V;
}

impl<T: ValueTrait> ValueDeltaTrait for T {
    type V = T::V;
    type D = ();

    fn join(v1: T::V, v2: T::V) -> T::V {
        T::join(v1, v2)
    }

    fn accumulate(_d1: (), _d2: ()) {}

    fn apply(v: T::V, _d: ()) -> T::V {
        v
    }
}
