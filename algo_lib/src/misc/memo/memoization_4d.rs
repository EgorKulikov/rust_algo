use crate::collections::md_arr::arr4d::Arr4d;
use crate::misc::recursive_function::Callable4;

pub struct Memoization4d<F, Output>
where
    F: FnMut(
        &mut dyn Callable4<usize, usize, usize, usize, Output>,
        usize,
        usize,
        usize,
        usize,
    ) -> Output,
{
    f: std::cell::UnsafeCell<F>,
    res: Arr4d<Option<Output>>,
}

impl<F, Output: Clone> Memoization4d<F, Output>
where
    F: FnMut(
        &mut dyn Callable4<usize, usize, usize, usize, Output>,
        usize,
        usize,
        usize,
        usize,
    ) -> Output,
{
    pub fn new(d1: usize, d2: usize, d3: usize, d4: usize, f: F) -> Self {
        Self {
            f: std::cell::UnsafeCell::new(f),
            res: Arr4d::new(d1, d2, d3, d4, None),
        }
    }
}

impl<F, Output: Clone> Callable4<usize, usize, usize, usize, Output> for Memoization4d<F, Output>
where
    F: FnMut(
        &mut dyn Callable4<usize, usize, usize, usize, Output>,
        usize,
        usize,
        usize,
        usize,
    ) -> Output,
{
    fn call(&mut self, a1: usize, a2: usize, a3: usize, a4: usize) -> Output {
        match self.res[(a1, a2, a3, a4)].as_ref() {
            None => {
                let res = unsafe { (*self.f.get())(self, a1, a2, a3, a4) };
                self.res[(a1, a2, a3, a4)] = Some(res.clone());
                res
            }
            Some(res) => res.clone(),
        }
    }
}
