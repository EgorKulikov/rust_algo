use crate::collections::md_arr::arr5d::Arr5d;
use crate::misc::recursive_function::Callable5;

pub struct Memoization5d<F, Output>
where
    F: FnMut(
        &mut dyn Callable5<usize, usize, usize, usize, usize, Output>,
        usize,
        usize,
        usize,
        usize,
        usize,
    ) -> Output,
{
    f: std::cell::UnsafeCell<F>,
    res: Arr5d<Option<Output>>,
}

impl<F, Output: Clone> Memoization5d<F, Output>
where
    F: FnMut(
        &mut dyn Callable5<usize, usize, usize, usize, usize, Output>,
        usize,
        usize,
        usize,
        usize,
        usize,
    ) -> Output,
{
    pub fn new(d1: usize, d2: usize, d3: usize, d4: usize, d5: usize, f: F) -> Self {
        Self {
            f: std::cell::UnsafeCell::new(f),
            res: Arr5d::new(d1, d2, d3, d4, d5, None),
        }
    }
}

impl<F, Output: Clone> Callable5<usize, usize, usize, usize, usize, Output>
    for Memoization5d<F, Output>
where
    F: FnMut(
        &mut dyn Callable5<usize, usize, usize, usize, usize, Output>,
        usize,
        usize,
        usize,
        usize,
        usize,
    ) -> Output,
{
    fn call(&mut self, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize) -> Output {
        match self.res[(a1, a2, a3, a4, a5)].as_ref() {
            None => {
                let res = unsafe { (*self.f.get())(self, a1, a2, a3, a4, a5) };
                self.res[(a1, a2, a3, a4, a5)] = Some(res.clone());
                res
            }
            Some(res) => res.clone(),
        }
    }
}
