use crate::string::slicelike::Slicelike;
use std::ops::Index;

pub struct CompositeSlicelike<
    'a,
    T,
    S1: Slicelike<Output = T> + ?Sized,
    S2: Slicelike<Output = T> + ?Sized,
> {
    s1: &'a S1,
    s2: &'a S2,
    phantom: std::marker::PhantomData<T>,
}

pub trait SlicelikeZip: Slicelike
where
    Self::Output: Sized,
{
    fn zip<'a, S2: Slicelike<Output = Self::Output> + ?Sized>(
        &'a self,
        s2: &'a S2,
    ) -> CompositeSlicelike<'a, Self::Output, Self, S2> {
        CompositeSlicelike {
            s1: self,
            s2,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<S: Slicelike + ?Sized> SlicelikeZip for S where S::Output: Sized {}

impl<'a, T, S1: Slicelike<Output = T> + ?Sized, S2: Slicelike<Output = T> + ?Sized> Index<usize>
    for CompositeSlicelike<'a, T, S1, S2>
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index < self.s1.len() {
            &self.s1[index]
        } else {
            &self.s2[index - self.s1.len()]
        }
    }
}

impl<'a, T, S1: Slicelike<Output = T> + ?Sized, S2: Slicelike<Output = T> + ?Sized> Slicelike
    for CompositeSlicelike<'a, T, S1, S2>
{
    fn len(&self) -> usize {
        self.s1.len() + self.s2.len()
    }
}
