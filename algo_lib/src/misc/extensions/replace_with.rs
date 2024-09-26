use std::{mem, ptr};

pub trait ReplaceWith<F: FnOnce(Self) -> Self>: Sized {
    fn replace_with(&mut self, f: F) {
        unsafe {
            let old = ptr::read(self);
            let new = on_unwind(move || f(old), || std::process::abort());
            ptr::write(self, new);
        }
    }
}

impl<T, F: FnOnce(Self) -> Self> ReplaceWith<F> for T {}

pub fn on_unwind<F: FnOnce() -> T, T, P: FnOnce()>(f: F, p: P) -> T {
    let x = OnDrop(mem::ManuallyDrop::new(p));
    let t = f();
    let mut x = mem::ManuallyDrop::new(x);
    unsafe { mem::ManuallyDrop::drop(&mut x.0) };
    t
}

struct OnDrop<F: FnOnce()>(mem::ManuallyDrop<F>);
impl<F: FnOnce()> Drop for OnDrop<F> {
    #[inline(always)]
    fn drop(&mut self) {
        (unsafe { ptr::read(&*self.0) })();
    }
}
