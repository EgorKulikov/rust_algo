use crate::collections::bit_set::BitSet;
use crate::collections::iter_ext::collect::IterCollect;
use crate::numbers::num_traits::as_index::AsIndex;

pub fn primality_table(n: usize) -> BitSet {
    let mut res = BitSet::new(n);
    res.fill(true);
    if n > 0 {
        res.unset(0);
    }
    if n > 1 {
        res.unset(1);
    }
    let mut i = 2;
    while i * i < n {
        if res[i] {
            for j in ((i * i)..n).step_by(i) {
                res.unset(j);
            }
        }
        i += 1;
    }
    res
}

pub fn primes<T: AsIndex>(n: usize) -> Vec<T> {
    primality_table(n)
        .into_iter()
        .map(|i| T::from_index(i))
        .collect_vec()
}

pub fn divisor_table<T: AsIndex + PartialEq>(n: usize) -> Vec<T> {
    let mut res = (0..n).map(|i| T::from_index(i)).collect_vec();
    let mut i = 2;
    while i * i < n {
        if res[i] == T::from_index(i) {
            for j in ((i * i)..n).step_by(i) {
                res[j] = T::from_index(i);
            }
        }
        i += 1;
    }
    res
}
