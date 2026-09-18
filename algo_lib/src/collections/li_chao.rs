//! Li Chao tree: minimum of a set of lines (or line segments) at a fixed,
//! sorted set of query coordinates. Adding a line costs O(log n), a segment
//! O(log^2 n), a query O(log n).

#[derive(Clone, Copy)]
struct Line {
    a: i64,
    b: i64,
}

impl Line {
    #[inline]
    fn at(&self, x: i64) -> i64 {
        self.a * x + self.b
    }
}

#[derive(Clone)]
pub struct LiChao {
    /// Sorted distinct query coordinates, padded to a power of two with the
    /// last value.
    xs: Vec<i64>,
    n: usize,
    nodes: Vec<Option<Line>>,
}

impl LiChao {
    /// Tree over the given query coordinates (any order, duplicates allowed).
    pub fn with_points(mut xs: Vec<i64>) -> Self {
        xs.sort_unstable();
        xs.dedup();
        assert!(!xs.is_empty());
        let n = xs.len();
        let size = n.next_power_of_two();
        let last = xs[n - 1];
        xs.resize(size, last);
        Self {
            xs,
            n,
            nodes: vec![None; 2 * size],
        }
    }

    /// Tree over every integer in `lo..hi`; meant for small ranges.
    pub fn new(lo: i64, hi: i64) -> Self {
        assert!(
            lo < hi && hi - lo <= 1 << 24,
            "use with_points for large domains"
        );
        Self::with_points((lo..hi).collect())
    }

    fn size(&self) -> usize {
        self.nodes.len() / 2
    }

    /// Inserts `line` into node `k` covering coordinate indices `l..r`.
    fn add_at(&mut self, mut k: usize, mut l: usize, mut r: usize, mut line: Line) {
        loop {
            let Some(cur) = self.nodes[k] else {
                self.nodes[k] = Some(line);
                return;
            };
            let m = (l + r) / 2;
            let left_better = line.at(self.xs[l]) < cur.at(self.xs[l]);
            let mid_better = line.at(self.xs[m]) < cur.at(self.xs[m]);
            if mid_better {
                self.nodes[k] = Some(line);
                line = cur;
            }
            if r - l == 1 {
                return;
            }
            if left_better != mid_better {
                k *= 2;
                r = m;
            } else {
                k = 2 * k + 1;
                l = m;
            }
        }
    }

    pub fn add_line(&mut self, a: i64, b: i64) {
        let size = self.size();
        self.add_at(1, 0, size, Line { a, b });
    }

    /// Adds `a x + b` only for coordinates `x` in `from..to`.
    pub fn add_segment(&mut self, from: i64, to: i64, a: i64, b: i64) {
        let real = &self.xs[..self.n];
        let lo = real.partition_point(|&x| x < from);
        let hi = real.partition_point(|&x| x < to);
        if lo >= hi {
            return;
        }
        let size = self.size();
        let (mut l, mut r) = (lo + size, hi + size);
        let line = Line { a, b };
        let mut cover = Vec::new();
        while l < r {
            if l & 1 == 1 {
                cover.push(l);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                cover.push(r);
            }
            l >>= 1;
            r >>= 1;
        }
        for k in cover {
            let level = (usize::BITS - 1 - k.leading_zeros()) as usize;
            let width = size >> level;
            let start = (k - (1 << level)) * width;
            self.add_at(k, start, start + width, line);
        }
    }

    /// Minimum over the inserted lines at query coordinate `x` (which must be
    /// one of the tree's coordinates), if any line covers it.
    pub fn min(&self, x: i64) -> Option<i64> {
        let at = self.xs[..self.n]
            .binary_search(&x)
            .expect("unknown coordinate");
        let mut k = at + self.size();
        let mut best: Option<i64> = None;
        while k >= 1 {
            if let Some(line) = self.nodes[k] {
                let v = line.at(x);
                best = Some(best.map_or(v, |b| b.min(v)));
            }
            k >>= 1;
        }
        best
    }
}

#[cfg(test)]
mod tests {
    use super::LiChao;
    use crate::misc::random::{Random, RandomTrait};

    #[test]
    fn lines_and_segments_match_naive() {
        let mut rng = Random::new_with_seed(101);
        for case in 0..6 {
            let (lo, hi) = [
                (0i64, 1),
                (0, 2),
                (-5, 5),
                (-1000, 1000),
                (-1_000_000_000, 1_000_000_000),
                (7, 8),
            ][case];
            let points: Vec<i64> = if hi - lo <= 2000 {
                (lo..hi).collect()
            } else {
                (0..500).map(|_| rng.gen_range(lo..hi)).collect()
            };
            let mut tree = if hi - lo <= 2000 && case % 2 == 0 {
                LiChao::new(lo, hi)
            } else {
                LiChao::with_points(points.clone())
            };
            let mut items: Vec<(i64, i64, i64, i64)> = Vec::new(); // from, to, a, b
            for _ in 0..300 {
                let a = rng.gen_range(-1000..=1000i64);
                let b = rng.gen_range(-1_000_000_000..=1_000_000_000i64);
                if rng.gen_bool() {
                    tree.add_line(a, b);
                    items.push((i64::MIN, i64::MAX, a, b));
                } else {
                    let f = rng.gen_range(lo..hi);
                    let t = rng.gen_range(f..=hi);
                    tree.add_segment(f, t, a, b);
                    items.push((f, t, a, b));
                }
                for _ in 0..5 {
                    let x = points[rng.gen_range(0..points.len())];
                    let expected = items
                        .iter()
                        .filter(|&&(f, t, _, _)| f <= x && x < t)
                        .map(|&(_, _, a, b)| a * x + b)
                        .min();
                    assert_eq!(tree.min(x), expected, "range {lo}..{hi} x={x}");
                }
            }
        }
    }
}
