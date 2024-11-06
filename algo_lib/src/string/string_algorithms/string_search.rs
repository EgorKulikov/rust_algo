use crate::string::composite_slicelike::SlicelikeZip;
use crate::string::slicelike::Slicelike;
use crate::string::string_algorithms::z_algorithm::ZAlgorithm;

pub trait StringSearch {
    fn index_of(&self, pattern: &Self) -> Option<usize>;
    fn str_contains(&self, pattern: &Self) -> bool {
        self.index_of(pattern).is_some()
    }
    fn all_matches(&self, pattern: &Self) -> Vec<usize>;
}

impl<S: Slicelike + ?Sized> StringSearch for S
where
    S::Output: PartialEq + Sized,
{
    fn index_of(&self, pattern: &Self) -> Option<usize> {
        pattern
            .chain(self)
            .z_algorithm()
            .into_iter()
            .skip(pattern.len())
            .position(|x| x >= pattern.len())
    }

    fn all_matches(&self, pattern: &Self) -> Vec<usize> {
        pattern
            .chain(self)
            .z_algorithm()
            .into_iter()
            .skip(pattern.len())
            .enumerate()
            .filter(|&(_, x)| x >= pattern.len())
            .map(|(i, _)| i)
            .collect()
    }
}
