pub trait TransposePairVec<U, V> {
    fn transpose_pair_vec(self) -> Vec<(V, U)>;
}

impl<U, V> TransposePairVec<U, V> for Vec<(U, V)> {
    fn transpose_pair_vec(self) -> Vec<(V, U)> {
        self.into_iter().map(|(u, v)| (v, u)).collect()
    }
}
