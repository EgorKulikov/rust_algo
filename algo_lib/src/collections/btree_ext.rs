use std::collections::{BTreeMap, BTreeSet};
use std::ops::Bound;

pub trait BTreeExt<'a, T> {
    type Output;
    fn next(&'a self, x: &'a T) -> Option<Self::Output>;
    fn prev(&'a self, x: &'a T) -> Option<Self::Output>;
    fn ceil(&'a self, x: &'a T) -> Option<Self::Output>;
    fn floor(&'a self, x: &'a T) -> Option<Self::Output>;
}

impl<'a, T: Ord + 'a> BTreeExt<'a, T> for BTreeSet<T> {
    type Output = &'a T;
    fn next(&'a self, x: &'a T) -> Option<Self::Output> {
        self.range((Bound::Excluded(x), Bound::Unbounded)).next()
    }

    fn ceil(&'a self, x: &'a T) -> Option<Self::Output> {
        self.range(x..).next()
    }

    fn prev(&'a self, x: &'a T) -> Option<Self::Output> {
        self.range(..x).next_back()
    }

    fn floor(&'a self, x: &'a T) -> Option<Self::Output> {
        self.range(..=x).next_back()
    }
}

impl<'a, K: Ord + 'a, V: 'a> BTreeExt<'a, K> for BTreeMap<K, V> {
    type Output = (&'a K, &'a V);

    fn next(&'a self, x: &'a K) -> Option<Self::Output> {
        self.range((Bound::Excluded(x), Bound::Unbounded)).next()
    }

    fn ceil(&'a self, x: &'a K) -> Option<Self::Output> {
        self.range(x..).next()
    }

    fn prev(&'a self, x: &'a K) -> Option<Self::Output> {
        self.range(..x).next_back()
    }

    fn floor(&'a self, x: &'a K) -> Option<Self::Output> {
        self.range(..=x).next_back()
    }
}
