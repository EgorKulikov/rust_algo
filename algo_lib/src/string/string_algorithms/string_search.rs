use crate::string::composite_slicelike::SlicelikeZip;
use crate::string::slicelike::Slicelike;
use crate::string::string_algorithms::z_algorithm::ZAlgorithm;

pub trait StringSearch {
    fn index_of(&self, pattern: &Self) -> Option<usize>;
    fn contains(&self, pattern: &Self) -> bool {
        self.index_of(pattern).is_some()
    }
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
}
