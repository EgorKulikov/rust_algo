use std::cmp::Ordering;

pub trait IterPartialEqExt<T: PartialEq>: Iterator<Item = T> + Sized {
    fn find(mut self, item: T) -> Option<usize> {
        self.position(|r| r == item)
    }
}

impl<T: PartialEq, I: Iterator<Item = T>> IterPartialEqExt<T> for I {}

pub trait IterExt<T>: Iterator<Item = T> + Sized {
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
    fn collect_vec(self) -> Vec<T> {
        self.collect()
    }
}

impl<T, I: Iterator<Item = T> + Sized> IterExt<T> for I {}

pub trait IterOrdExt<T: Ord>: IterExt<T> {
    fn enumerate_sort(self) -> Vec<(usize, T)> {
        self.enumerate_sort_by(|a, b| a.cmp(b))
    }

    fn max_position(mut self) -> Option<usize> {
        let mut res = None;
        let mut val: Option<T> = None;
        let mut at = 0usize;
        while let Some(cur) = self.next() {
            if val.is_none() || *val.as_ref().unwrap() < cur {
                val = Some(cur);
                res = Some(at);
            }
            at += 1;
        }
        res
    }

    fn min_position(mut self) -> Option<usize> {
        let mut res = None;
        let mut val = None;
        let mut at = 0usize;
        while let Some(cur) = self.next() {
            if val.is_none() || *val.as_ref().unwrap() > cur {
                val = Some(cur);
                res = Some(at);
            }
            at += 1;
        }
        res
    }
}

impl<T: Ord, I: Iterator<Item = T> + Sized> IterOrdExt<T> for I {}
