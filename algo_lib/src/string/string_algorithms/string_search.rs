use crate::string::slicelike::chain;
use crate::string::string_algorithms::z_algorithm::ZAlgorithm;

pub trait StringSearch {
    fn index_of(&self, pattern: &Self) -> Option<usize>;
    fn str_contains(&self, pattern: &Self) -> bool {
        self.index_of(pattern).is_some()
    }
    fn all_matches(&self, pattern: &Self) -> Vec<usize>;
}

impl<T: PartialEq + Sized> StringSearch for [T] {
    fn index_of(&self, pattern: &Self) -> Option<usize> {
        chain(pattern, self)
            .z_algorithm()
            .into_iter()
            .skip(pattern.len())
            .position(|x| x >= pattern.len())
    }

    fn all_matches(&self, pattern: &Self) -> Vec<usize> {
        chain(pattern, self)
            .z_algorithm()
            .into_iter()
            .skip(pattern.len())
            .enumerate()
            .filter(|&(_, x)| x >= pattern.len())
            .map(|(i, _)| i)
            .collect()
    }
}
