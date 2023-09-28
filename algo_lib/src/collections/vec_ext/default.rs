pub fn default_vec<T: Default>(len: usize) -> Vec<T> {
    let mut v = Vec::with_capacity(len);
    for _ in 0..len {
        v.push(T::default());
    }
    v
}
