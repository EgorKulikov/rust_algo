use std::cell::UnsafeCell;

pub struct OwnedCell<T>(UnsafeCell<T>);

impl<T> OwnedCell<T> {
    pub fn new(val: T) -> Self {
        Self(UnsafeCell::new(val))
    }

    pub unsafe fn as_ref<'a>(&self) -> &'a T {
        &*self.0.get()
    }

    pub unsafe fn as_mut<'a>(&self) -> &'a mut T {
        &mut *self.0.get()
    }

    pub unsafe fn replace(&self, new_val: T) -> T {
        std::mem::replace(self.as_mut(), new_val)
    }
}
