use crate::collections::bounds::clamp;
use crate::collections::vec_ext::gen_vec::VecGen;
use crate::numbers::num_traits::bit_ops::BitOps;
use std::ops::RangeBounds;

pub struct SparseTable<T, F = fn(&T, &T) -> T> {
    table: Vec<Vec<T>>,
    f: F,
}

impl<T, F: Fn(&T, &T) -> T> SparseTable<T, F> {
    pub fn new(a: Vec<T>, f: F) -> Self {
        let n = a.len();
        let mut table = Vec::with_capacity(n.highest_bit() + 1);
        table.push(a);
        table.gen_append(n.highest_bit(), |i, table| {
            let mut cur = Vec::with_capacity(n - (1 << i) + 1);
            for j in 0..=n - (1 << i) {
                cur.push(f(&table[i - 1][j], &table[i - 1][j + (1 << (i - 1))]));
            }
            cur
        });
        Self { table, f }
    }

    pub fn query(&self, range: impl RangeBounds<usize>) -> T {
        let (from, to) = clamp(&range, self.table[0].len());
        assert!(from < to);
        let len = to - from;
        let level = len.highest_bit();
        (self.f)(
            &self.table[level][from],
            &self.table[level][to - (1 << level)],
        )
    }
}
