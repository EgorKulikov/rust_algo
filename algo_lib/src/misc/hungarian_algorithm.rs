use crate::collections::bit_set::BitSet;
use crate::collections::md_arr::arr2d::Arr2d;
use crate::collections::slice_ext::legacy_fill::LegacyFill;
use crate::numbers::num_traits::ord::MinMax;

pub fn hungarian_algorithm(a: &Arr2d<i64>) -> i64 {
    assert_eq!(a.d1(), a.d2());
    let inf = i64::max_val() / 2;
    let n = a.d1();
    let mut u = vec![0; n + 1];
    let mut v = vec![0; n + 1];
    let mut p = vec![n; n + 1];
    let mut way = vec![n; n + 1];
    let mut min_v = vec![inf; n + 1];
    let mut used = BitSet::new(n + 1);
    for i in 0..n {
        p[n] = i;
        let mut j0 = n;
        used.fill(false);
        min_v.legacy_fill(inf);
        while p[j0] != n {
            used.set(j0);
            let i0 = p[j0];
            let mut delta = inf;
            let mut j1 = n;
            for j in 0..n {
                if !used[j] {
                    let cur = a[(i0, j)] - u[i0] - v[j];
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
            for j in 0..=n {
                if used[j] {
                    u[p[j]] += delta;
                    v[j] -= delta;
                } else {
                    min_v[j] -= delta;
                }
            }
            j0 = j1;
        }
        while j0 != n {
            let j1 = way[j0];
            p[j0] = p[j1];
            j0 = j1;
        }
    }
    -v[n]
}
