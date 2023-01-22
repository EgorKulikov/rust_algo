use crate::misc::recursive_function::Callable;

pub struct Memoization1d<F, Output>
where
    F: FnMut(&mut dyn Callable<usize, Output>, usize) -> Output,
{
    f: std::cell::UnsafeCell<F>,
    res: Vec<Option<Output>>,
}

impl<F, Output: Clone> Memoization1d<F, Output>
where
    F: FnMut(&mut dyn Callable<usize, Output>, usize) -> Output,
{
    pub fn new(len: usize, f: F) -> Self {
        Self {
            f: std::cell::UnsafeCell::new(f),
            res: vec![None; len],
        }
    }
}

impl<F, Output: Clone> Callable<usize, Output> for Memoization1d<F, Output>
where
    F: FnMut(&mut dyn Callable<usize, Output>, usize) -> Output,
{
    fn call(&mut self, n: usize) -> Output {
        match self.res[n].as_ref() {
            None => {
                let res = unsafe { (*self.f.get())(self, n) };
                self.res[n] = Some(res.clone());
                res
            }
            Some(res) => res.clone(),
        }
    }
}
