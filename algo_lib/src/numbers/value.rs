pub trait Value<T>: Copy + Eq {
    const VAL: T;
}
