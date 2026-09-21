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
        if index >= self.arr.len() {
            // Only ever grow: `resize_with` to a smaller length would drop
            // live elements above `index`.
            self.arr
                .resize_with(index + 1, || (self.default.clone(), self.epoch));
        } else if self.arr[index].1 != self.epoch {
            // A slot from before the last `clear` still holds its old value.
            self.arr[index] = (self.default.clone(), self.epoch);
        }
        &mut self.arr[index].0
    }
}

#[cfg(test)]
mod tests {
    use super::FastClearArr;

    #[test]
    fn write_below_live_elements_after_clear() {
        let mut a: FastClearArr<i32> = FastClearArr::with_default(0);
        a[0] = 1;
        a[1] = 2;
        a.clear();
        a[1] = 3;
        a[0] = 4;
        assert_eq!((a[0], a[1], a[2]), (4, 3, 0));
    }

    #[test]
    fn compound_assignment_starts_from_default_after_clear() {
        let mut a: FastClearArr<i32> = FastClearArr::with_default(7);
        a[3] += 5;
        assert_eq!(a[3], 12);
        a.clear();
        assert_eq!(a[3], 7);
        a[3] += 1;
        assert_eq!(a[3], 8);
    }
}
