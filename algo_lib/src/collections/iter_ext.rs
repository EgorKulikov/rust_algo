use std::cmp::Ordering;

pub trait IterFind<T: PartialEq>: Iterator<Item = T> + Sized {
    fn find(mut self, item: T) -> Option<usize> {
        self.position(|r| r == item)
    }
}

impl<T: PartialEq, I: Iterator<Item = T>> IterFind<T> for I {}

pub trait EnumerateSortBy<T>: Iterator<Item = T> + Sized {
    fn enumerate_sort_by<F>(self, mut f: F) -> Vec<(usize, T)>
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        let mut res: Vec<_> = self.enumerate().collect();
        res.sort_by(|(i, vi), (j, vj)| {
            let res = f(vi, vj);
            if res != Ordering::Equal {
                res
            } else {
                i.cmp(j)
            }
        });
        res
    }
}

impl<T, I: Iterator<Item = T> + Sized> EnumerateSortBy<T> for I {}

pub trait EnumerateSort<T: Ord>: EnumerateSortBy<T> {
    fn enumerate_sort(self) -> Vec<(usize, T)> {
        self.enumerate_sort_by(|a, b| a.cmp(b))
    }
}

impl<T: Ord, I: Iterator<Item = T> + Sized> EnumerateSort<T> for I {}

pub trait CollectVec<T>: Iterator<Item = T> + Sized {
    fn collect_vec(self) -> Vec<T> {
        self.collect()
    }
}

impl<T, I: Iterator<Item = T> + Sized> CollectVec<T> for I {}
