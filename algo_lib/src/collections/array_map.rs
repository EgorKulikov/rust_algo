use crate::collections::vec_ext::gen_vec::VecGen;
use crate::numbers::num_traits::algebra::{AdditionMonoidWithSub, One};
use std::fmt::Debug;
use std::ops::{Index, IndexMut, RangeBounds};

pub struct ArrayMap<I, T> {
    data: Vec<T>,
    base: I,
}

impl<T: Default, I: Copy + AdditionMonoidWithSub + One + TryFrom<usize> + Debug> ArrayMap<I, T>
where
    usize: TryFrom<I>,
{
    pub fn new(range: impl RangeBounds<I>) -> Self {
        Self::with_gen(range, |_| T::default())
    }
}

impl<T, I: Copy + AdditionMonoidWithSub + One + TryFrom<usize>> ArrayMap<I, T>
where
    usize: TryFrom<I>,
{
    pub fn with_gen(range: impl RangeBounds<I>, mut f: impl FnMut(I) -> T) -> Self {
        let from = match range.start_bound() {
            std::ops::Bound::Included(&x) => x,
            std::ops::Bound::Excluded(&x) => x + I::one(),
            std::ops::Bound::Unbounded => unreachable!(),
        };
        let (to, add) = match range.end_bound() {
            std::ops::Bound::Included(&x) => (x, 1),
            std::ops::Bound::Excluded(&x) => (x, 0),
            std::ops::Bound::Unbounded => unreachable!(),
        };
        let size = usize::try_from(to - from).ok().expect("Range too large") + add;
        Self {
            data: Vec::with_gen(size, |i| f(from + I::try_from(i).ok().unwrap())),
            base: from,
        }
    }
}

impl<T, I: Copy + AdditionMonoidWithSub + One + TryFrom<usize>> Index<I> for ArrayMap<I, T>
where
    usize: TryFrom<I>,
{
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        let i = usize::try_from(index - self.base)
            .ok()
            .expect("Index out of bounds");
        &self.data[i]
    }
}

impl<T, I: Copy + AdditionMonoidWithSub + One + TryFrom<usize>> IndexMut<I> for ArrayMap<I, T>
where
    usize: TryFrom<I>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        let i = usize::try_from(index - self.base)
            .ok()
            .expect("Index out of bounds");
        &mut self.data[i]
    }
}
