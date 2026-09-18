use crate::collections::md_arr::arr2d::Arr2d;

/// Minimum cost of assigning every row to a distinct column (`rows <= cols`).
pub fn hungarian_algorithm(a: &Arr2d<i64>) -> i64 {
    assignment(a).0
}

/// Minimum-cost assignment of every row to a distinct column (`rows <=
/// cols`): the total cost and the column chosen for each row. Dense shortest
/// augmenting paths, O(rows^2 * cols), scanning only unvisited columns.
pub fn assignment(a: &Arr2d<i64>) -> (i64, Vec<usize>) {
    const NONE: usize = usize::MAX;
    let (n, m) = (a.d1(), a.d2());
    assert!(n <= m);
    let cost = a.as_slice();
    let mut u = vec![0i64; n];
    let mut v = vec![0i64; m];
    let mut col_of_row = vec![NONE; n];
    let mut row_of_col = vec![NONE; m];
    // Row minima as initial potentials, with a greedy matching on tight edges.
    for i in 0..n {
        let row = &cost[i * m..(i + 1) * m];
        let (best, &low) = row.iter().enumerate().min_by_key(|&(_, &c)| c).unwrap();
        u[i] = low;
        if row_of_col[best] == NONE {
            row_of_col[best] = i;
            col_of_row[i] = best;
        }
    }
    let mut shortest = vec![0i64; m];
    let mut path = vec![NONE; m];
    let mut remaining: Vec<usize> = Vec::with_capacity(m);
    let mut scanned_cols: Vec<usize> = Vec::with_capacity(m);
    let mut scanned_rows: Vec<usize> = Vec::with_capacity(n);
    for start in 0..n {
        if col_of_row[start] != NONE {
            continue;
        }
        shortest.fill(i64::MAX);
        remaining.clear();
        remaining.extend(0..m);
        scanned_cols.clear();
        scanned_rows.clear();
        let mut min_val = 0i64;
        let mut i = start;
        let sink = loop {
            scanned_rows.push(i);
            let row = &cost[i * m..(i + 1) * m];
            let base = min_val - u[i];
            let mut lowest = i64::MAX;
            let mut index = 0;
            for (at, &j) in remaining.iter().enumerate() {
                let r = base + row[j] - v[j];
                if r < shortest[j] {
                    shortest[j] = r;
                    path[j] = i;
                }
                let s = shortest[j];
                if s < lowest || (s == lowest && row_of_col[j] == NONE) {
                    lowest = s;
                    index = at;
                }
            }
            min_val = lowest;
            let j = remaining.swap_remove(index);
            scanned_cols.push(j);
            if row_of_col[j] == NONE {
                break j;
            }
            i = row_of_col[j];
        };
        u[start] += min_val;
        for &r in &scanned_rows {
            if r != start {
                u[r] += min_val - shortest[col_of_row[r]];
            }
        }
        for &j in &scanned_cols {
            v[j] -= min_val - shortest[j];
        }
        let mut j = sink;
        loop {
            let r = path[j];
            row_of_col[j] = r;
            std::mem::swap(&mut col_of_row[r], &mut j);
            if r == start {
                break;
            }
        }
    }
    let total = (0..n).map(|i| cost[i * m + col_of_row[i]]).sum();
    (total, col_of_row)
}

#[cfg(test)]
mod tests {
    use super::assignment;
    use crate::collections::bit_set::BitSet;
    use crate::collections::md_arr::arr2d::Arr2d;
    use crate::misc::random::{Random, RandomTrait};
    use crate::numbers::num_traits::ord::MinMax;

    // The previous O(n^2 m) implementation, kept as an oracle.
    fn reference(a: &Arr2d<i64>) -> i64 {
        let inf = i64::max_val() / 2;
        let n = a.d1();
        let m = a.d2();
        assert!(n <= m);
        let mut u = vec![0; n + 1];
        let mut v = vec![0; m + 1];
        let mut p = vec![0; m + 1];
        let mut way = vec![0; m + 1];
        let mut min_v = vec![inf; m + 1];
        let mut used = BitSet::new(m + 1);
        for i in 1..=n {
            p[0] = i;
            let mut j0 = 0;
            used.fill(false);
            min_v.fill(inf);
            while p[j0] != 0 {
                used.set(j0);
                let i0 = p[j0];
                let mut delta = inf;
                let mut j1 = 0;
                for j in 1..=m {
                    if !used[j] {
                        let cur = a[(i0 - 1, j - 1)] - u[i0] - v[j];
                        if cur < min_v[j] {
                            min_v[j] = cur;
                            way[j] = j0;
                        }
                        if min_v[j] < delta {
                            delta = min_v[j];
                            j1 = j;
                        }
                    }
                }
                for j in 0..=m {
                    if used[j] {
                        u[p[j]] += delta;
                        v[j] -= delta;
                    } else {
                        min_v[j] -= delta;
                    }
                }
                j0 = j1;
            }
            while j0 != 0 {
                let j1 = way[j0];
                p[j0] = p[j1];
                j0 = j1;
            }
        }
        -v[0]
    }

    fn brute(a: &Arr2d<i64>, row: usize, used: &mut Vec<bool>) -> i64 {
        if row == a.d1() {
            return 0;
        }
        let mut best = i64::MAX;
        for j in 0..a.d2() {
            if !used[j] {
                used[j] = true;
                best = best.min(a[(row, j)] + brute(a, row + 1, used));
                used[j] = false;
            }
        }
        best
    }

    fn check(a: &Arr2d<i64>, expected: i64) {
        let (cost, cols) = assignment(a);
        assert_eq!(cost, expected);
        assert_eq!(super::hungarian_algorithm(a), expected);
        let mut seen = vec![false; a.d2()];
        let mut sum = 0;
        for (i, &j) in cols.iter().enumerate() {
            assert!(!seen[j], "column used twice");
            seen[j] = true;
            sum += a[(i, j)];
        }
        assert_eq!(sum, expected);
    }

    #[test]
    fn small_matches_brute_force() {
        let mut rng = Random::new_with_seed(261);
        for _ in 0..300 {
            let n = rng.gen_range(1..=6usize);
            let m = rng.gen_range(n..=7usize);
            let wide = rng.gen_bool();
            let a = Arr2d::with_gen(n, m, |_, _| {
                if wide {
                    rng.gen_range(-1_000_000_000..=1_000_000_000i64)
                } else {
                    rng.gen_range(-3..=3i64)
                }
            });
            let expected = brute(&a, 0, &mut vec![false; m]);
            check(&a, expected);
        }
    }

    #[test]
    fn larger_matches_previous_implementation() {
        let mut rng = Random::new_with_seed(262);
        for &(n, m) in &[(40usize, 40usize), (30, 70), (100, 100), (1, 50)] {
            let a = Arr2d::with_gen(n, m, |_, _| rng.gen_range(-1000..=1000i64));
            check(&a, reference(&a));
            let ties = Arr2d::with_gen(n, m, |_, _| rng.gen_range(0..=2i64));
            check(&ties, reference(&ties));
        }
    }
}
