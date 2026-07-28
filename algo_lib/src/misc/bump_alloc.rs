use std::alloc::{alloc, handle_alloc_error, Layout};
use std::cell::Cell;
use std::ptr::NonNull;

// Dense thread-local bump allocation for never-freed nodes; addresses are stable.
const CHUNK_SIZE: usize = 1 << 22;

thread_local! {
    static BUMP: Cell<(usize, usize)> = const { Cell::new((0, 0)) };
}

pub fn bump_alloc(layout: Layout) -> NonNull<u8> {
    BUMP.with(|bump| {
        let (cur, end) = bump.get();
        let aligned = (cur + layout.align() - 1) & !(layout.align() - 1);
        if aligned + layout.size() <= end {
            bump.set((aligned + layout.size(), end));
            unsafe { NonNull::new_unchecked(aligned as *mut u8) }
        } else {
            let chunk_size = CHUNK_SIZE.max(layout.size() + layout.align());
            let chunk_layout = Layout::from_size_align(chunk_size, 16).unwrap();
            let chunk = unsafe { alloc(chunk_layout) };
            if chunk.is_null() {
                handle_alloc_error(chunk_layout);
            }
            let start = chunk as usize;
            let aligned = (start + layout.align() - 1) & !(layout.align() - 1);
            bump.set((aligned + layout.size(), start + chunk_size));
            unsafe { NonNull::new_unchecked(aligned as *mut u8) }
        }
    })
}

pub fn bump_new<T>(value: T) -> NonNull<T> {
    let ptr = bump_alloc(Layout::new::<T>()).cast::<T>();
    unsafe { ptr.as_ptr().write(value) };
    ptr
}
