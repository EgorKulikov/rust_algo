#[target_feature(enable = "avx2")]
unsafe fn fast_apply_avx2<T: Copy>(a: &mut [T], mut f: impl FnMut(T) -> T) {
    for val in a.iter_mut() {
        *val = f(*val);
    }
}

pub fn fast_apply<T: Copy>(a: &mut [T], f: impl FnMut(T) -> T) {
    unsafe { fast_apply_avx2(a, f) }
}

#[target_feature(enable = "avx2")]
unsafe fn fast_fold_avx2<T: Copy, R>(a: &[T], mut acc: R, mut f: impl FnMut(R, T) -> R) -> R {
    for val in a.iter() {
        acc = f(acc, *val);
    }
    acc
}

pub fn fast_fold<T: Copy, R>(a: &[T], acc: R, f: impl FnMut(R, T) -> R) -> R {
    unsafe { fast_fold_avx2(a, acc, f) }
}
