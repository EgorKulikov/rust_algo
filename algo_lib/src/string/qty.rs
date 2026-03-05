use crate::collections::array_map::ArrayMap;
use std::ops::RangeBounds;

pub trait StrQty {
    fn qty(&self, bounds: impl RangeBounds<u8>) -> ArrayMap<u8, usize>;
    fn qty_lower(&self) -> ArrayMap<u8, usize>;
}

impl StrQty for [u8] {
    fn qty(&self, bounds: impl RangeBounds<u8>) -> ArrayMap<u8, usize> {
        let mut res = ArrayMap::new(bounds);
        for &c in self {
            res[c] += 1;
        }
        res
    }

    fn qty_lower(&self) -> ArrayMap<u8, usize> {
        self.qty(b'a'..=b'z')
    }
}
