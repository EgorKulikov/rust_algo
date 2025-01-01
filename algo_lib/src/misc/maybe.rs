use std::mem;
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};

pub union Maybe<T> {
    pub value: ManuallyDrop<T>,
    pub none: (),
}

impl<T> Maybe<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: ManuallyDrop::new(value),
        }
    }

    pub fn none() -> Self {
        Self { none: () }
    }

    pub unsafe fn take(&mut self) -> T {
        unsafe { ManuallyDrop::into_inner(mem::replace(self, Maybe::none()).value) }
    }

    pub unsafe fn as_ref(&self) -> &T {
        self.value.deref()
    }

    pub unsafe fn as_mut(&mut self) -> &mut T {
        self.value.deref_mut()
    }

    pub unsafe fn drop(&mut self) {
        unsafe { ManuallyDrop::drop(&mut self.value) }
    }
}
