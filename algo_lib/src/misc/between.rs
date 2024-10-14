use std::ops::RangeInclusive;

pub fn between<T: Ord + Copy>(x: T, y: T) -> RangeInclusive<T> {
    x.min(y)..=x.max(y)
}
