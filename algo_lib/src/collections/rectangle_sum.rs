use crate::collections::fenwick::FenwickTree;
use crate::collections::slice_ext::radix_sort::radix_sort_by_key;
use crate::numbers::num_traits::algebra::AdditionMonoidWithSub;

/// Offline rectangle sums: for every query `(x1, y1, x2, y2)` the total
/// weight of points with `x1 <= x < x2` and `y1 <= y < y2`. One sweep over x
/// with a Fenwick tree over compressed y, O((n + q) log n); coordinates are
/// ordered through an offset so that the sorting passes are radix sorts.
pub fn rectangle_sums<W: AdditionMonoidWithSub + Copy>(
    points: &[(i64, i64, W)],
    queries: &[(i64, i64, i64, i64)],
) -> Vec<W> {
    let unsigned = |v: i64| (v as u64) ^ (1 << 63);
    let mut ys: Vec<u64> = points.iter().map(|p| unsigned(p.1)).collect();
    radix_sort_by_key(&mut ys, |&y| y);
    ys.dedup();
    let rank = |y: i64| ys.partition_point(|&v| v < unsigned(y));
    // (x, y rank, weight) sorted by x
    let mut pts: Vec<(u64, u32, W)> = points
        .iter()
        .map(|&(x, y, w)| (unsigned(x), rank(y) as u32, w))
        .collect();
    radix_sort_by_key(&mut pts, |p| p.0);
    // (x, query, is upper edge) sorted by x
    let mut events: Vec<(u64, u32, bool)> = Vec::with_capacity(2 * queries.len());
    for (i, &(x1, y1, x2, y2)) in queries.iter().enumerate() {
        if x1 < x2 && y1 < y2 {
            events.push((unsigned(x1), i as u32, false));
            events.push((unsigned(x2), i as u32, true));
        }
    }
    radix_sort_by_key(&mut events, |e| e.0);
    let mut tree = FenwickTree::new(ys.len());
    let mut res = vec![W::zero(); queries.len()];
    // Sum below the lower edge; the upper edge of the same query comes later
    // in the sweep, so unsigned weights never go negative.
    let mut below = vec![W::zero(); queries.len()];
    let mut at = 0;
    for (x, query, upper) in events {
        while at < pts.len() && pts[at].0 < x {
            tree.add(pts[at].1 as usize, pts[at].2);
            at += 1;
        }
        let (_, y1, _, y2) = queries[query as usize];
        let sum = tree.get(rank(y1)..rank(y2));
        if upper {
            res[query as usize] = sum - below[query as usize];
        } else {
            below[query as usize] = sum;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::rectangle_sums;
    use crate::misc::random::{Random, RandomTrait};

    #[test]
    fn matches_brute_force() {
        let mut rng = Random::new_with_seed(401);
        for round in 0..200 {
            let n = rng.gen_range(0..=80usize);
            let q = rng.gen_range(0..=80usize);
            let range = if round % 2 == 0 { 5i64 } else { 1_000_000_000 };
            let points: Vec<(i64, i64, i64)> = (0..n)
                .map(|_| {
                    (
                        rng.gen_range(-range..=range),
                        rng.gen_range(-range..=range),
                        rng.gen_range(-100..=100i64),
                    )
                })
                .collect();
            let queries: Vec<(i64, i64, i64, i64)> = (0..q)
                .map(|_| {
                    let (a, b) = (
                        rng.gen_range(-range..=range + 1),
                        rng.gen_range(-range..=range + 1),
                    );
                    let (c, d) = (
                        rng.gen_range(-range..=range + 1),
                        rng.gen_range(-range..=range + 1),
                    );
                    (a.min(b), c.min(d), a.max(b), c.max(d))
                })
                .collect();
            let expected: Vec<i64> = queries
                .iter()
                .map(|&(x1, y1, x2, y2)| {
                    points
                        .iter()
                        .filter(|p| x1 <= p.0 && p.0 < x2 && y1 <= p.1 && p.1 < y2)
                        .map(|p| p.2)
                        .sum()
                })
                .collect();
            assert_eq!(rectangle_sums(&points, &queries), expected);
            // unsigned weights
            let unsigned: Vec<(i64, i64, u64)> = points
                .iter()
                .map(|p| (p.0, p.1, (p.2 + 100) as u64))
                .collect();
            let expected: Vec<u64> = queries
                .iter()
                .map(|&(x1, y1, x2, y2)| {
                    unsigned
                        .iter()
                        .filter(|p| x1 <= p.0 && p.0 < x2 && y1 <= p.1 && p.1 < y2)
                        .map(|p| p.2)
                        .sum()
                })
                .collect();
            assert_eq!(rectangle_sums(&unsigned, &queries), expected);
        }
    }
}
