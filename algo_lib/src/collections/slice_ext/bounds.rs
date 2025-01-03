use std::ops::{Bound, RangeBounds};

pub trait Bounds<T: PartialOrd> {
    fn lower_bound(&self, el: &T) -> usize;
    fn upper_bound(&self, el: &T) -> usize;
    fn bin_search(&self, el: &T) -> Option<usize>;
    fn more(&self, el: &T) -> usize;
    fn more_or_eq(&self, el: &T) -> usize;
    fn less(&self, el: &T) -> usize {
        self.lower_bound(el)
    }
    fn less_or_eq(&self, el: &T) -> usize {
        self.upper_bound(el)
    }
    fn inside<'a>(&self, bounds: impl RangeBounds<&'a T>) -> usize
    where
        T: 'a;
}

impl<T: PartialOrd> Bounds<T> for [T] {
    fn lower_bound(&self, el: &T) -> usize {
        let mut left = 0;
        let mut right = self.len();
        while left < right {
            let mid = left + ((right - left) >> 1);
            if &self[mid] < el {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }

    fn upper_bound(&self, el: &T) -> usize {
        let mut left = 0;
        let mut right = self.len();
        while left < right {
            let mid = left + ((right - left) >> 1);
            if &self[mid] <= el {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }

    fn bin_search(&self, el: &T) -> Option<usize> {
        let at = self.lower_bound(el);
        if at == self.len() || &self[at] != el {
            None
        } else {
            Some(at)
        }
    }

    fn more(&self, el: &T) -> usize {
        self.len() - self.upper_bound(el)
    }

    fn more_or_eq(&self, el: &T) -> usize {
        self.len() - self.lower_bound(el)
    }

    fn inside<'a>(&self, bounds: impl RangeBounds<&'a T>) -> usize
    where
        T: 'a,
    {
        let to = match bounds.end_bound() {
            Bound::Included(el) => self.less_or_eq(el),
            Bound::Excluded(el) => self.less(el),
            Bound::Unbounded => self.len(),
        };
        let from = match bounds.start_bound() {
            Bound::Included(el) => self.less(el),
            Bound::Excluded(el) => self.less_or_eq(el),
            Bound::Unbounded => 0,
        };
        to - from
    }
}
