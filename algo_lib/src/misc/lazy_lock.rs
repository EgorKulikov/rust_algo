use std::cell::UnsafeCell;
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::sync::Once;

union Data<T, F> {
    value: ManuallyDrop<T>,
    f: ManuallyDrop<F>,
}

// 1.80
pub struct LazyLock<T, F = fn() -> T> {
    once: Once,
    data: UnsafeCell<Data<T, F>>,
}

impl<T, F: FnOnce() -> T> LazyLock<T, F> {
    #[inline]
    pub const fn new(f: F) -> LazyLock<T, F> {
        LazyLock {
            once: Once::new(),
            data: UnsafeCell::new(Data {
                f: ManuallyDrop::new(f),
            }),
        }
    }

    #[inline]
    pub fn force(this: &LazyLock<T, F>) -> &T {
        this.once.call_once(|| {
            let data = unsafe { &mut *this.data.get() };
            let f = unsafe { ManuallyDrop::take(&mut data.f) };
            let value = f();
            data.value = ManuallyDrop::new(value);
        });

        unsafe { &(*this.data.get()).value }
    }
}

impl<T, F: FnOnce() -> T> Deref for LazyLock<T, F> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        LazyLock::force(self)
    }
}

unsafe impl<T: Sync + Send, F: Send> Sync for LazyLock<T, F> {}
