pub trait OptionExt: Sized {
    fn as_opt(&self) -> Option<&Self> {
        Some(self)
    }

    fn to_opt(self) -> Option<Self> {
        Some(self)
    }
}

impl<T> OptionExt for T {}
