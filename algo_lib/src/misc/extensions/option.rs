pub trait OptionExt: Sized {
    fn as_opt(&self) -> Option<&Self> {
        Some(self)
    }

    fn to_opt(self) -> Option<Self> {
        Some(self)
    }

    fn take_if(self, val: bool) -> Option<Self> {
        if val {
            Some(self)
        } else {
            None
        }
    }
}

impl<T> OptionExt for T {}
