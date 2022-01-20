use std::cell::UnsafeCell;
use std::ptr::NonNull;

pub struct OwnedCell<T>(UnsafeCell<T>);

impl<T> OwnedCell<T> {
    pub fn new(val: T) -> Self {
        Self(UnsafeCell::new(val))
    }

    pub unsafe fn as_ref<'a>(&self) -> &'a T {
        NonNull::new_unchecked(self.0.get()).as_ref()
    }

    pub unsafe fn as_mut<'a>(&self) -> &'a mut T {
        NonNull::new_unchecked(self.0.get()).as_mut()
    }

    pub unsafe fn replace(&self, new_val: T) -> T {
        std::mem::replace(self.as_mut(), new_val)
    }
}
