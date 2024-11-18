use std::iter::{Chain, Enumerate, Filter, Map, Rev, Skip, StepBy, Sum, Take, Zip};

pub trait ItersRef<'a>: Sized + 'a
where
    &'a Self: IntoIterator,
{
    fn ref_enumerate(&'a self) -> Enumerate<<&'a Self as IntoIterator>::IntoIter> {
        self.into_iter().enumerate()
    }
    fn ref_rev(&'a self) -> Rev<<&'a Self as IntoIterator>::IntoIter>
    where
        <&'a Self as IntoIterator>::IntoIter: DoubleEndedIterator,
    {
        self.into_iter().rev()
    }
    fn ref_skip(&'a self, n: usize) -> Skip<<&'a Self as IntoIterator>::IntoIter> {
        self.into_iter().skip(n)
    }
    fn ref_take(&'a self, n: usize) -> Take<<&'a Self as IntoIterator>::IntoIter> {
        self.into_iter().take(n)
    }
    fn ref_chain<V>(
        &'a self,
        chained: &'a V,
    ) -> Chain<<&'a Self as IntoIterator>::IntoIter, <&'a V as IntoIterator>::IntoIter>
    where
        &'a V: IntoIterator<Item = <&'a Self as IntoIterator>::Item>,
    {
        self.into_iter().chain(chained)
    }
    fn ref_zip<V>(
        &'a self,
        other: &'a V,
    ) -> Zip<<&'a Self as IntoIterator>::IntoIter, <&'a V as IntoIterator>::IntoIter>
    where
        &'a V: IntoIterator<Item = <&'a Self as IntoIterator>::Item>,
    {
        self.into_iter().zip(other)
    }
    fn ref_max(&'a self) -> <&'a Self as IntoIterator>::Item
    where
        <&'a Self as IntoIterator>::Item: Ord,
    {
        self.into_iter().max().unwrap()
    }
    fn ref_max_by_key<B, F>(&'a self, f: F) -> <&'a Self as IntoIterator>::Item
    where
        F: FnMut(&<&'a Self as IntoIterator>::Item) -> B,
        B: Ord,
    {
        self.into_iter().max_by_key(f).unwrap()
    }
    fn ref_min(&'a self) -> <&'a Self as IntoIterator>::Item
    where
        <&'a Self as IntoIterator>::Item: Ord,
    {
        self.into_iter().min().unwrap()
    }
    fn ref_min_by_key<B, F>(&'a self, f: F) -> <&'a Self as IntoIterator>::Item
    where
        F: FnMut(&<&'a Self as IntoIterator>::Item) -> B,
        B: Ord,
    {
        self.into_iter().min_by_key(f).unwrap()
    }
    fn ref_sum<T: 'a + Sum<&'a T>>(&'a self) -> T
    where
        &'a Self: IntoIterator<Item = &'a T>,
    {
        self.into_iter().sum()
    }
    fn ref_map<F, T>(&'a self, f: F) -> Map<<&'a Self as IntoIterator>::IntoIter, F>
    where
        F: FnMut(<&'a Self as IntoIterator>::Item) -> T,
    {
        self.into_iter().map(f)
    }
    fn ref_all(&'a self, f: impl FnMut(<&'a Self as IntoIterator>::Item) -> bool) -> bool {
        self.into_iter().all(f)
    }
    fn ref_any(&'a self, f: impl FnMut(<&'a Self as IntoIterator>::Item) -> bool) -> bool {
        self.into_iter().any(f)
    }
    fn ref_step_by(&'a self, step: usize) -> StepBy<<&'a Self as IntoIterator>::IntoIter> {
        self.into_iter().step_by(step)
    }
    fn ref_filter<F: FnMut(&<&'a Self as IntoIterator>::Item) -> bool>(
        &'a self,
        f: F,
    ) -> Filter<<&'a Self as IntoIterator>::IntoIter, F> {
        self.into_iter().filter(f)
    }
    fn ref_fold<Acc, F>(&'a self, init: Acc, f: F) -> Acc
    where
        F: FnMut(Acc, <&'a Self as IntoIterator>::Item) -> Acc,
    {
        self.into_iter().fold(init, f)
    }
    fn ref_reduce<F>(&'a self, f: F) -> Option<<&'a Self as IntoIterator>::Item>
    where
        F: FnMut(
            <&'a Self as IntoIterator>::Item,
            <&'a Self as IntoIterator>::Item,
        ) -> <&'a Self as IntoIterator>::Item,
    {
        self.into_iter().reduce(f)
    }
    fn ref_position<P>(&'a self, predicate: P) -> Option<usize>
    where
        P: FnMut(<&'a Self as IntoIterator>::Item) -> bool,
    {
        self.into_iter().position(predicate)
    }
}

impl<'a, U: 'a> ItersRef<'a> for U where &'a U: IntoIterator {}
