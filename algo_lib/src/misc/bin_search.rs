use crate::numbers::num_traits::algebra::IntegerSemiRingWithSub;
use crate::numbers::real::Real;

pub fn search_last_false<T: Copy + PartialOrd + IntegerSemiRingWithSub>(
    mut left: T,
    mut right: T,
    mut pred: impl FnMut(T) -> bool,
) -> T {
    assert!(left <= right);
    debug_assert!(!pred(left));
    let two = T::one() + T::one();
    while left < right {
        let mid = left + (right - left + T::one()) / two;
        if pred(mid) {
            right = mid - T::one();
        } else {
            left = mid;
        }
    }
    left
}

pub fn search_first_true<T: Copy + PartialOrd + IntegerSemiRingWithSub>(
    mut left: T,
    mut right: T,
    mut pred: impl FnMut(T) -> bool,
) -> T {
    assert!(left <= right);
    debug_assert!(pred(right));
    let two = T::one() + T::one();
    while left < right {
        let mid = left + (right - left) / two;
        if pred(mid) {
            right = mid;
        } else {
            left = mid + T::one();
        }
    }
    left
}

pub fn search_real(mut left: Real, mut right: Real, mut pred: impl FnMut(Real) -> bool) -> Real {
    assert!(left <= right);
    for _ in 0..100 {
        let mid = (left + right) / 2;
        if pred(mid) {
            right = mid;
        } else {
            left = mid;
        }
    }
    (left + right) / 2
}
