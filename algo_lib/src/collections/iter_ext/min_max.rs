use crate::collections::min_max::MinimMaxim;

pub trait IterMinMaxPos<T: Ord>: Iterator<Item = T> + Sized {
    fn max_position(self) -> Option<usize> {
        let mut res = None;
        let mut val = None;
        for (i, cur) in self.enumerate() {
            if val.maxim(cur) {
                res = Some(i);
            }
        }
        res
    }

    fn min_position(self) -> Option<usize> {
        let mut res = None;
        let mut val = None;
        for (i, cur) in self.enumerate() {
            if val.minim(cur) {
                res = Some(i);
            }
        }
        res
    }
}

impl<T: Ord, I: Iterator<Item = T> + Sized> IterMinMaxPos<T> for I {}
