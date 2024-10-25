use std::ops::{Index, IndexMut};

pub struct Back(pub usize);

impl<T> Index<Back> for [T] {
    type Output = T;

    fn index(&self, index: Back) -> &Self::Output {
        &self[self.len() - index.0 - 1]
    }
}

impl<T> IndexMut<Back> for [T] {
    fn index_mut(&mut self, index: Back) -> &mut Self::Output {
        &mut self[self.len() - index.0 - 1]
    }
}

impl<T> Index<Back> for Vec<T> {
    type Output = T;

    fn index(&self, index: Back) -> &Self::Output {
        self.as_slice().index(index)
    }
}

impl<T> IndexMut<Back> for Vec<T> {
    fn index_mut(&mut self, index: Back) -> &mut Self::Output {
        self.as_mut_slice().index_mut(index)
    }
}
