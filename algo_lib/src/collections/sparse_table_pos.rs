use crate::collections::bounds::clamp;
use crate::collections::vec_ext::gen_vec::VecGen;
use crate::misc::direction::Direction;
use crate::numbers::num_traits::bit_ops::BitOps;
use std::ops::RangeBounds;

pub struct SparseTableWithPos<T, F = fn(&T, &T) -> Direction> {
    table: Vec<Vec<T>>,
    pos: Vec<Vec<usize>>,
    f: F,
}

impl<T: Clone, F: Fn(&T, &T) -> Direction> SparseTableWithPos<T, F> {
    pub fn new(a: &[T], f: F) -> Self {
        let n = a.len();
        let mut table = Vec::with_capacity(n.highest_bit() + 1);
        table.push(a.to_vec());
        let mut pos: Vec<Vec<usize>> = Vec::with_capacity(n.highest_bit() + 1);
        pos.push((0..n).collect());
        table.gen_append(n.highest_bit(), |i, table| {
            let mut cur = Vec::with_capacity(n - (1 << i) + 1);
            let mut cur_pos = Vec::with_capacity(n - (1 << i) + 1);
            for j in 0..=n - (1 << i) {
                let direction = f(&table[i - 1][j], &table[i - 1][j + (1 << (i - 1))]);
                cur.push(match direction {
                    Direction::Left => {
                        cur_pos.push(pos[i - 1][j]);
                        table[i - 1][j].clone()
                    }
                    Direction::Right => {
                        cur_pos.push(pos[i - 1][j + (1 << (i - 1))]);
                        table[i - 1][j + (1 << (i - 1))].clone()
                    }
                });
            }
            pos.push(cur_pos);
            cur
        });
        Self { table, pos, f }
    }

    pub fn query(&self, range: impl RangeBounds<usize>) -> (T, usize) {
        let (from, to) = clamp(&range, self.table[0].len());
        assert!(from < to);
        let len = to - from;
        let level = len.highest_bit();
        let direction = (self.f)(
            &self.table[level][from],
            &self.table[level][to - (1 << level)],
        );
        match direction {
            Direction::Left => (self.table[level][from].clone(), self.pos[level][from]),
            Direction::Right => (
                self.table[level][to - (1 << level)].clone(),
                self.pos[level][to - (1 << level)],
            ),
        }
    }
}
