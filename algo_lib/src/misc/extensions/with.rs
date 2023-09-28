pub trait With: Sized {
    fn with<F, R>(self, f: F) -> R
    where
        F: (FnOnce(Self) -> R),
    {
        f(self)
    }
}

impl<T> With for T {}
