use crate::collections::min_max::MinimMaxim;

pub trait IterMinMaxPos<'a, T: Ord + 'a>: 'a
where
    &'a Self: IntoIterator<Item = T>,
{
    fn max_position(&'a self) -> Option<usize> {
        let mut res = None;
        let mut val = None;
        for (i, cur) in self.into_iter().enumerate() {
            if val.maxim(cur) {
                res = Some(i);
            }
        }
        res
    }

    fn min_position(&'a self) -> Option<usize> {
        let mut res = None;
        let mut val = None;
        for (i, cur) in self.into_iter().enumerate() {
            if val.minim(cur) {
                res = Some(i);
            }
        }
        res
    }
}

impl<'a, T: Ord + 'a, I: ?Sized + 'a> IterMinMaxPos<'a, T> for I where &'a I: IntoIterator<Item = T> {}
