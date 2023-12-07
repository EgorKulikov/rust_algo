use std::ops::RangeBounds;

pub fn clamp(range: impl RangeBounds<usize>, n: usize) -> (usize, usize) {
    let start = match range.start_bound() {
        std::ops::Bound::Included(&x) => x,
        std::ops::Bound::Excluded(&x) => x + 1,
        std::ops::Bound::Unbounded => 0,
    };
    let end = match range.end_bound() {
        std::ops::Bound::Included(&x) => x + 1,
        std::ops::Bound::Excluded(&x) => x,
        std::ops::Bound::Unbounded => n,
    };
    (start, end.min(n))
}
