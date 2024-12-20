use std::iter::{Chain, Enumerate, Filter, Map, Rev, Skip, StepBy, Sum, Take, Zip};

pub trait Iters: IntoIterator + Sized {
    fn iter_enumerate(self) -> Enumerate<Self::IntoIter> {
        self.into_iter().enumerate()
    }
    fn iter_rev(self) -> Rev<Self::IntoIter>
    where
        Self::IntoIter: DoubleEndedIterator,
    {
        self.into_iter().rev()
    }
    fn iter_skip(self, n: usize) -> Skip<Self::IntoIter> {
        self.into_iter().skip(n)
    }
    fn iter_take(self, n: usize) -> Take<Self::IntoIter> {
        self.into_iter().take(n)
    }
    fn iter_chain<V: IntoIterator<Item = Self::Item>>(
        self,
        chained: V,
    ) -> Chain<Self::IntoIter, V::IntoIter> {
        self.into_iter().chain(chained)
    }
    fn iter_zip<V: IntoIterator>(self, other: V) -> Zip<Self::IntoIter, V::IntoIter> {
        self.into_iter().zip(other)
    }
    fn iter_max(self) -> Self::Item
    where
        Self::Item: Ord,
    {
        self.into_iter().max().unwrap()
    }
    fn iter_max_by_key<B, F>(self, f: F) -> Self::Item
    where
        F: FnMut(&Self::Item) -> B,
        B: Ord,
    {
        self.into_iter().max_by_key(f).unwrap()
    }
    fn iter_min(self) -> Self::Item
    where
        Self::Item: Ord,
    {
        self.into_iter().min().unwrap()
    }
    fn iter_min_by_key<B, F>(self, f: F) -> Self::Item
    where
        F: FnMut(&Self::Item) -> B,
        B: Ord,
    {
        self.into_iter().min_by_key(f).unwrap()
    }
    fn iter_sum(self) -> Self::Item
    where
        Self::Item: Sum<Self::Item>,
    {
        Sum::sum(self.into_iter())
    }
    fn iter_map<F, T>(self, f: F) -> Map<Self::IntoIter, F>
    where
        F: FnMut(Self::Item) -> T,
    {
        self.into_iter().map(f)
    }
    fn iter_all(self, f: impl FnMut(Self::Item) -> bool) -> bool {
        self.into_iter().all(f)
    }
    fn iter_any(self, f: impl FnMut(Self::Item) -> bool) -> bool {
        self.into_iter().any(f)
    }
    fn iter_step_by(self, step: usize) -> StepBy<Self::IntoIter> {
        self.into_iter().step_by(step)
    }
    fn iter_filter<F: FnMut(&Self::Item) -> bool>(self, f: F) -> Filter<Self::IntoIter, F> {
        self.into_iter().filter(f)
    }
    fn iter_fold<Acc, F>(self, init: Acc, f: F) -> Acc
    where
        F: FnMut(Acc, Self::Item) -> Acc,
    {
        self.into_iter().fold(init, f)
    }
    fn iter_reduce<F>(self, f: F) -> Option<Self::Item>
    where
        F: FnMut(Self::Item, Self::Item) -> Self::Item,
    {
        self.into_iter().reduce(f)
    }
    fn iter_position<P>(self, predicate: P) -> Option<usize>
    where
        P: FnMut(Self::Item) -> bool,
    {
        self.into_iter().position(predicate)
    }
    fn iter_find(self, val: Self::Item) -> Option<usize>
    where
        Self::Item: PartialEq,
    {
        self.into_iter().position(|x| x == val)
    }
    fn iter_count(self, val: Self::Item) -> usize
    where
        Self::Item: PartialEq,
    {
        self.into_iter().filter(|x| *x == val).count()
    }
}

impl<U> Iters for U where U: IntoIterator {}
