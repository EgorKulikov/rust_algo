pub trait Order {
    fn order(&self) -> Vec<usize>;
}

impl<T: Ord> Order for [T] {
    fn order(&self) -> Vec<usize> {
        let mut order = (0..self.len()).collect::<Vec<usize>>();
        order.sort_by_key(|&i| &self[i]);
        order
    }
}
