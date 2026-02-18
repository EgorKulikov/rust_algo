use crate::collections::bit_set::BitSet;
use crate::collections::md_arr::arr2d::Arr2d;
use crate::numbers::num_traits::ord::MinMax;

pub fn hungarian_algorithm(a: &Arr2d<i64>) -> i64 {
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
