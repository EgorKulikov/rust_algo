pub trait DoWith: Sized {
    fn do_with<F>(mut self, f: F) -> Self
    where
        F: (FnOnce(&mut Self)),
    {
        f(&mut self);
        self
    }
}

impl<T> DoWith for T {}
