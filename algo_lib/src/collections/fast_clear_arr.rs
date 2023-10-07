use std::ops::{Index, IndexMut};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct FastClearArr<T> {
    arr: Vec<(T, u32)>,
    epoch: u32,
    default: T,
}

impl<T: Default> FastClearArr<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            arr: Vec::with_capacity(cap),
            epoch: 0,
            default: T::default(),
        }
    }
}

impl<T: Clone> FastClearArr<T> {
    pub fn with_default(default: T) -> Self {
        Self {
            arr: Vec::new(),
            epoch: 0,
            default,
        }
    }

    pub fn with_capacity_and_default(cap: usize, default: T) -> Self {
        Self {
            arr: Vec::with_capacity(cap),
            epoch: 0,
            default,
        }
    }
}

impl<T> FastClearArr<T> {
    pub fn clear(&mut self) {
        self.epoch += 1;
    }
}

impl<T> Index<usize> for FastClearArr<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.arr.len() || self.arr[index].1 != self.epoch {
            &self.default
        } else {
            &self.arr[index].0
        }
    }
}

impl<T: Clone> IndexMut<usize> for FastClearArr<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.arr.len() || self.arr[index].1 != self.epoch {
            self.arr
                .resize_with(index + 1, || (self.default.clone(), self.epoch));
            self.arr[index].1 = self.epoch;
        }
        &mut self.arr[index].0
    }
}
