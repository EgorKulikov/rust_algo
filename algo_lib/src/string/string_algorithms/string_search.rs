use crate::string::composite_slicelike::SlicelikeZip;
use crate::string::str::Str;
use crate::string::string_algorithms::z_algorithm::ZAlgorithm;

pub trait StringSearch {
    fn index_of(&self, pattern: &Self) -> Option<usize>;
    fn contains(&self, pattern: &Self) -> bool {
        self.index_of(pattern).is_some()
    }
}

impl StringSearch for Str<'_> {
    fn index_of(&self, pattern: &Self) -> Option<usize> {
        pattern
            .as_slice()
            .zip(self.as_slice())
            .z_algorithm()
            .iter()
            .skip(pattern.len())
            .position(|&x| x >= pattern.len())
    }
}
