use crate::collections::md_arr::arr3d::Arr3d;
use crate::misc::recursive_function::Callable3;

pub struct Memoization3d<F, Output>
where
    F: FnMut(&mut dyn Callable3<usize, usize, usize, Output>, usize, usize, usize) -> Output,
{
    f: std::cell::UnsafeCell<F>,
    res: Arr3d<Option<Output>>,
}

impl<F, Output: Clone> Memoization3d<F, Output>
where
    F: FnMut(&mut dyn Callable3<usize, usize, usize, Output>, usize, usize, usize) -> Output,
{
    pub fn new(d1: usize, d2: usize, d3: usize, f: F) -> Self {
        Self {
            f: std::cell::UnsafeCell::new(f),
            res: Arr3d::new(d1, d2, d3, None),
        }
    }
}

impl<F, Output: Clone> Callable3<usize, usize, usize, Output> for Memoization3d<F, Output>
where
    F: FnMut(&mut dyn Callable3<usize, usize, usize, Output>, usize, usize, usize) -> Output,
{
    fn call(&mut self, a1: usize, a2: usize, a3: usize) -> Output {
        match self.res[(a1, a2, a3)].as_ref() {
            None => {
                let res = unsafe { (*self.f.get())(self, a1, a2, a3) };
                self.res[(a1, a2, a3)] = Some(res.clone());
                res
            }
            Some(res) => res.clone(),
        }
    }
}
