use std::iter::{Chain, Copied, Enumerate, Filter, Map, Rev, Skip, StepBy, Sum, Take, Zip};

pub trait ItersCopied<'a, T: 'a + Copy>: Sized + 'a
where
    &'a Self: IntoIterator<Item = &'a T>,
{
    fn copy_iter(&'a self) -> Copied<<&'a Self as IntoIterator>::IntoIter> {
        self.into_iter().copied()
    }
    fn copy_enumerate(&'a self) -> Enumerate<Copied<<&'a Self as IntoIterator>::IntoIter>> {
        self.copy_iter().enumerate()
    }
    fn copy_rev(&'a self) -> Rev<Copied<<&'a Self as IntoIterator>::IntoIter>>
    where
        Copied<<&'a Self as IntoIterator>::IntoIter>: DoubleEndedIterator,
    {
        self.copy_iter().rev()
    }
    fn copy_skip(&'a self, n: usize) -> Skip<Copied<<&'a Self as IntoIterator>::IntoIter>> {
        self.copy_iter().skip(n)
    }
    fn copy_take(&'a self, n: usize) -> Take<Copied<<&'a Self as IntoIterator>::IntoIter>> {
        self.copy_iter().take(n)
    }
    fn copy_chain<V>(
        &'a self,
        chained: &'a V,
    ) -> Chain<
        Copied<<&'a Self as IntoIterator>::IntoIter>,
        Copied<<&'a V as IntoIterator>::IntoIter>,
    >
    where
        &'a V: IntoIterator<Item = &'a T>,
    {
        self.copy_iter().chain(chained.into_iter().copied())
    }
    fn copy_zip<V>(
        &'a self,
        other: &'a V,
    ) -> Zip<Copied<<&'a Self as IntoIterator>::IntoIter>, Copied<<&'a V as IntoIterator>::IntoIter>>
    where
        &'a V: IntoIterator<Item = &'a T>,
    {
        self.copy_iter().zip(other.into_iter().copied())
    }
    fn copy_max(&'a self) -> T
    where
        T: Ord,
    {
        self.copy_iter().max().unwrap()
    }
    fn copy_max_by_key<B, F>(&'a self, f: F) -> T
    where
        F: FnMut(&T) -> B,
        B: Ord,
    {
        self.copy_iter().max_by_key(f).unwrap()
    }
    fn copy_min(&'a self) -> T
    where
        T: Ord,
    {
        self.copy_iter().min().unwrap()
    }
    fn copy_min_by_key<B, F>(&'a self, f: F) -> T
    where
        F: FnMut(&T) -> B,
        B: Ord,
    {
        self.copy_iter().min_by_key(f).unwrap()
    }
    fn copy_sum(&'a self) -> T
    where
        T: Sum<T>,
    {
        self.copy_iter().sum()
    }
    fn copy_map<F, U>(&'a self, f: F) -> Map<Copied<<&'a Self as IntoIterator>::IntoIter>, F>
    where
        F: FnMut(T) -> U,
    {
        self.copy_iter().map(f)
    }
    fn copy_all(&'a self, f: impl FnMut(T) -> bool) -> bool {
        self.copy_iter().all(f)
    }
    fn copy_any(&'a self, f: impl FnMut(T) -> bool) -> bool {
        self.copy_iter().any(f)
    }
    fn copy_step_by(&'a self, step: usize) -> StepBy<Copied<<&'a Self as IntoIterator>::IntoIter>> {
        self.copy_iter().step_by(step)
    }
    fn copy_filter<F: FnMut(&T) -> bool>(
        &'a self,
        f: F,
    ) -> Filter<Copied<<&'a Self as IntoIterator>::IntoIter>, F> {
        self.copy_iter().filter(f)
    }
    fn copy_fold<Acc, F>(&'a self, init: Acc, f: F) -> Acc
    where
        F: FnMut(Acc, T) -> Acc,
    {
        self.copy_iter().fold(init, f)
    }
    fn copy_reduce<F>(&'a self, f: F) -> Option<T>
    where
        F: FnMut(T, T) -> T,
    {
        self.copy_iter().reduce(f)
    }
    fn copy_position<P>(&'a self, predicate: P) -> Option<usize>
    where
        P: FnMut(T) -> bool,
    {
        self.copy_iter().position(predicate)
    }
}

impl<'a, U: 'a, T: 'a + Copy> ItersCopied<'a, T> for U where &'a U: IntoIterator<Item = &'a T> {}
