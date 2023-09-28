use crate::collections::md_arr::arr2d::Arr2d;
use crate::misc::recursive_function::Callable2;

pub struct Memoization2d<F, Output>
where
    F: FnMut(&mut dyn Callable2<usize, usize, Output>, usize, usize) -> Output,
{
    f: std::cell::UnsafeCell<F>,
    res: Arr2d<Option<Output>>,
}

impl<F, Output: Clone> Memoization2d<F, Output>
where
    F: FnMut(&mut dyn Callable2<usize, usize, Output>, usize, usize) -> Output,
{
    pub fn new(d1: usize, d2: usize, f: F) -> Self {
        Self {
            f: std::cell::UnsafeCell::new(f),
            res: Arr2d::new(d1, d2, None),
        }
    }
}

impl<F, Output: Clone> Callable2<usize, usize, Output> for Memoization2d<F, Output>
where
    F: FnMut(&mut dyn Callable2<usize, usize, Output>, usize, usize) -> Output,
{
    fn call(&mut self, n: usize, m: usize) -> Output {
        match self.res[(n, m)].as_ref() {
            None => {
                let res = unsafe { (*self.f.get())(self, n, m) };
                self.res[(n, m)] = Some(res.clone());
                res
            }
            Some(res) => res.clone(),
        }
    }
}
