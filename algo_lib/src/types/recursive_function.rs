use std::marker::PhantomData;

pub trait Callable<Args, Output> {
    fn call(&mut self, args: Args) -> Output;
}

pub struct RecursiveFunction<F, Args, Output>
where
    F: FnMut(&mut dyn Callable<Args, Output>, Args) -> Output,
{
    f: F,
    phantom_args: PhantomData<Args>,
    phantom_output: PhantomData<Output>,
}

impl<F, Args, Output> RecursiveFunction<F, Args, Output>
where
    F: FnMut(&mut dyn Callable<Args, Output>, Args) -> Output,
{
    pub fn new(f: F) -> Self {
        Self {
            f,
            phantom_args: Default::default(),
            phantom_output: Default::default(),
        }
    }
}

impl<F, Args, Output> Callable<Args, Output> for RecursiveFunction<F, Args, Output>
where
    F: FnMut(&mut dyn Callable<Args, Output>, Args) -> Output,
{
    fn call(&mut self, args: Args) -> Output {
        let const_ptr = &self.f as *const F;
        let mut_ptr = const_ptr as *mut F;
        unsafe { (&mut *mut_ptr)(self, args) }
    }
}
